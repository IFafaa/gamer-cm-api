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
