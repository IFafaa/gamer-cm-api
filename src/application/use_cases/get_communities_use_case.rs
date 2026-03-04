use axum::http::StatusCode;

use crate::{
    application::interfaces::result_get_community_interface::IResultGetCommunity,
    domain::community::CommunityRepository,
    shared::{
        api_error::ApiErrorResponse, api_response::ApiResponse, pagination::PaginationParams,
    },
};
use std::sync::Arc;

pub struct GetCommunitiesUseCase<R: CommunityRepository> {
    community_repository: Arc<R>,
}

impl<R: CommunityRepository> GetCommunitiesUseCase<R> {
    pub fn new(community_repository: Arc<R>) -> Self {
        Self {
            community_repository,
        }
    }

    pub async fn execute(
        &self,
        user_id: i32,
        pagination: PaginationParams,
    ) -> Result<ApiResponse<Vec<IResultGetCommunity>>, (StatusCode, ApiErrorResponse)> {
        let communities = self
            .community_repository
            .get_all_by_user(user_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal Server Error".to_string()),
                )
            })?;

        let result: Vec<IResultGetCommunity> = communities
            .into_iter()
            .map(IResultGetCommunity::new)
            .collect();

        let total = result.len();
        let meta = pagination.meta(total);
        let paginated = pagination.apply(result);

        Ok(ApiResponse::with_pagination(paginated, meta))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::community::Community;
    use mockall::mock;

    mock! {
        pub CommunityRepo {}
        #[async_trait::async_trait]
        impl CommunityRepository for CommunityRepo {
            async fn insert(&self, community: &Community) -> anyhow::Result<()>;
            async fn exists(&self, name: String, user_id: i32) -> anyhow::Result<bool>;
            async fn get_all_by_user(&self, user_id: i32) -> anyhow::Result<Vec<Community>>;
            async fn get_by_id_and_user(&self, id: i32, user_id: i32) -> anyhow::Result<Option<Community>>;
            async fn belongs_to_user(&self, community_id: i32, user_id: i32) -> anyhow::Result<bool>;
            async fn get_ids_by_user(&self, user_id: i32) -> anyhow::Result<Vec<i32>>;
            async fn save(&self, community: &Community) -> anyhow::Result<()>;
        }
    }

    fn make_community(id: i32, name: &str) -> Community {
        let mut c = Community::new(name.to_string(), 1);
        c.id = id;
        c
    }

    #[tokio::test]
    async fn returns_all_results_within_single_page() {
        let mut mock = MockCommunityRepo::new();
        mock.expect_get_all_by_user()
            .returning(|_| Ok(vec![make_community(1, "Alpha"), make_community(2, "Beta")]));

        let use_case = GetCommunitiesUseCase::new(Arc::new(mock));
        let pagination = PaginationParams { page: 1, limit: 10 };
        let result = use_case.execute(1, pagination).await;
        assert!(result.is_ok(), "Expected Ok result");
        let json = serde_json::to_value(result.ok().unwrap()).unwrap();
        assert_eq!(json["data"].as_array().unwrap().len(), 2);
        assert_eq!(json["meta"]["total"], 2);
        assert_eq!(json["meta"]["page"], 1);
        assert_eq!(json["meta"]["has_next_page"], false);
        assert_eq!(json["meta"]["has_previous_page"], false);
    }

    #[tokio::test]
    async fn returns_correct_slice_on_second_page() {
        let mut mock = MockCommunityRepo::new();
        mock.expect_get_all_by_user().returning(|_| {
            Ok((1..=7).map(|i| make_community(i, &format!("C{}", i))).collect())
        });

        let use_case = GetCommunitiesUseCase::new(Arc::new(mock));
        let pagination = PaginationParams { page: 2, limit: 3 };
        let result = use_case.execute(1, pagination).await;
        assert!(result.is_ok(), "Expected Ok result");
        let json = serde_json::to_value(result.ok().unwrap()).unwrap();
        assert_eq!(json["data"].as_array().unwrap().len(), 3);
        assert_eq!(json["meta"]["total"], 7);
        assert_eq!(json["meta"]["page"], 2);
        assert_eq!(json["meta"]["total_pages"], 3);
        assert_eq!(json["meta"]["has_next_page"], true);
        assert_eq!(json["meta"]["has_previous_page"], true);
    }

    #[tokio::test]
    async fn returns_empty_data_when_offset_exceeds_total() {
        let mut mock = MockCommunityRepo::new();
        mock.expect_get_all_by_user()
            .returning(|_| Ok(vec![make_community(1, "Only")]));

        let use_case = GetCommunitiesUseCase::new(Arc::new(mock));
        let pagination = PaginationParams { page: 5, limit: 10 };
        let result = use_case.execute(1, pagination).await;
        assert!(result.is_ok(), "Expected Ok result");
        let json = serde_json::to_value(result.ok().unwrap()).unwrap();
        assert_eq!(json["data"].as_array().unwrap().len(), 0);
        assert_eq!(json["meta"]["total"], 1);
    }

    #[tokio::test]
    async fn returns_internal_error_on_repository_failure() {
        let mut mock = MockCommunityRepo::new();
        mock.expect_get_all_by_user()
            .returning(|_| Err(anyhow::anyhow!("db error")));

        let use_case = GetCommunitiesUseCase::new(Arc::new(mock));
        let pagination = PaginationParams::default();
        let result = use_case.execute(1, pagination).await;

        assert!(result.is_err());
        assert_eq!(result.err().unwrap().0, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
