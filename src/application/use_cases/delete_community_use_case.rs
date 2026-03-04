use axum::http::StatusCode;

use crate::{domain::community::CommunityRepository, shared::api_error::ApiErrorResponse};
use std::sync::Arc;

pub struct DeleteCommunityUseCase<R: CommunityRepository> {
    community_repository: Arc<R>,
}

impl<R: CommunityRepository> DeleteCommunityUseCase<R> {
    pub fn new(community_repository: Arc<R>) -> Self {
        Self {
            community_repository,
        }
    }

    pub async fn execute(&self, community_id: i32, user_id: i32) -> Result<(), (StatusCode, ApiErrorResponse)> {
        let mut community = self
            .community_repository
            .get_by_id_and_user(community_id, user_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal server error".to_string()),
                )
            })?
            .ok_or((
                StatusCode::NOT_FOUND,
                ApiErrorResponse::new("Community not found".to_string()),
            ))?;

        if !community.is_enabled() {
            return Err((
                StatusCode::BAD_REQUEST,
                ApiErrorResponse::new("Community is already disabled".to_string()),
            ));
        }

        community.disable();

        self.community_repository
            .save(&community)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal server error".to_string()),
                )
            })?;
        Ok(())
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

    fn make_community(enabled: bool) -> Community {
        let mut c = Community::new("Test Community".to_string(), 1);
        if !enabled {
            c.disable();
        }
        c
    }

    #[tokio::test]
    async fn test_delete_community_success() {
        let mut mock = MockCommunityRepo::new();
        mock.expect_get_by_id_and_user()
            .returning(|_, _| Ok(Some(make_community(true))));
        mock.expect_save().returning(|_| Ok(()));

        let use_case = DeleteCommunityUseCase::new(Arc::new(mock));
        let result = use_case.execute(1, 1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_community_not_found_returns_not_found() {
        let mut mock = MockCommunityRepo::new();
        mock.expect_get_by_id_and_user().returning(|_, _| Ok(None));

        let use_case = DeleteCommunityUseCase::new(Arc::new(mock));
        let result = use_case.execute(99, 1).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().0, StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_already_disabled_community_returns_bad_request() {
        let mut mock = MockCommunityRepo::new();
        mock.expect_get_by_id_and_user()
            .returning(|_, _| Ok(Some(make_community(false))));

        let use_case = DeleteCommunityUseCase::new(Arc::new(mock));
        let result = use_case.execute(1, 1).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().0, StatusCode::BAD_REQUEST);
    }
}
