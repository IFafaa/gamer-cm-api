use axum::http::StatusCode;
use std::sync::Arc;

use crate::{
    domain::community::CommunityRepository,
    presentation::dtos::update_community_dto::UpdateCommunityDto,
    shared::{api_error::ApiErrorResponse, date_time::DateTime},
};

pub struct UpdateCommunityUseCase<R: CommunityRepository> {
    community_repository: Arc<R>,
}

impl<R: CommunityRepository> UpdateCommunityUseCase<R> {
    pub fn new(community_repository: Arc<R>) -> Self {
        Self {
            community_repository,
        }
    }

    pub async fn execute(
        &self,
        community_id: i32,
        user_id: i32,
        dto: UpdateCommunityDto,
    ) -> Result<(), (StatusCode, ApiErrorResponse)> {
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

        // Check if name already exists for this user
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

        if already_exists && community.name != dto.name {
            return Err((
                StatusCode::CONFLICT,
                ApiErrorResponse::new("Community with this name already exists".to_string()),
            ));
        }

        community.name = dto.name;
        community.updated_at = DateTime::now();

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
