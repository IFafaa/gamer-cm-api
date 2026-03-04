use axum::http::StatusCode;

use crate::{
    domain::community::{Community, CommunityRepository},
    presentation::dtos::create_community_dto::CreateCommunityDto,
    shared::api_error::ApiErrorResponse,
};
use std::sync::Arc;

pub struct CreateCommunityUseCase<R: CommunityRepository> {
    community_repository: Arc<R>,
}

impl<R: CommunityRepository> CreateCommunityUseCase<R> {
    pub fn new(community_repository: Arc<R>) -> Self {
        Self {
            community_repository,
        }
    }

    pub async fn execute(
        &self,
        dto: CreateCommunityDto,
        user_id: i32,
    ) -> Result<(), (StatusCode, ApiErrorResponse)> {
        if dto.name.is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                ApiErrorResponse::new("Community name cannot be empty".to_string()),
            ));
        }

        let already_exists = self
            .community_repository
            .exists(dto.name.clone(), user_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal server error".to_string()),
                )
            })?;
        if already_exists {
            return Err((
                StatusCode::CONFLICT,
                ApiErrorResponse::new("Community already exists".to_string()),
            ));
        }

        let community = Community::new(dto.name, user_id);
        self.community_repository
            .insert(&community)
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

    #[tokio::test]
    async fn test_create_community_success() {
        let mut mock = MockCommunityRepo::new();
        mock.expect_exists().returning(|_, _| Ok(false));
        mock.expect_insert().returning(|_| Ok(()));

        let use_case = CreateCommunityUseCase::new(Arc::new(mock));
        let dto = CreateCommunityDto {
            name: "Pro Gamers".to_string(),
        };
        let result = use_case.execute(dto, 1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_community_empty_name_returns_bad_request() {
        let mock = MockCommunityRepo::new();
        let use_case = CreateCommunityUseCase::new(Arc::new(mock));
        let dto = CreateCommunityDto {
            name: "".to_string(),
        };
        let result = use_case.execute(dto, 1).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().0, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_create_community_already_exists_returns_conflict() {
        let mut mock = MockCommunityRepo::new();
        mock.expect_exists().returning(|_, _| Ok(true));

        let use_case = CreateCommunityUseCase::new(Arc::new(mock));
        let dto = CreateCommunityDto {
            name: "Existing".to_string(),
        };
        let result = use_case.execute(dto, 1).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().0, StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn test_create_community_repository_error_returns_internal_error() {
        let mut mock = MockCommunityRepo::new();
        mock.expect_exists()
            .returning(|_, _| Err(anyhow::anyhow!("db error")));

        let use_case = CreateCommunityUseCase::new(Arc::new(mock));
        let dto = CreateCommunityDto {
            name: "My Community".to_string(),
        };
        let result = use_case.execute(dto, 1).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().0, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
