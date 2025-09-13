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
