use axum::http::StatusCode;

use crate::{
    domain::{community::CommunityRepository, player::PlayerRepository},
    shared::api_error::ApiErrorResponse,
};
use std::sync::Arc;

pub struct DeletePlayerOfCommunityUseCase<R: PlayerRepository, CR: CommunityRepository> {
    player_repository: Arc<R>,
    community_repository: Arc<CR>,
}

impl<R: PlayerRepository, CR: CommunityRepository> DeletePlayerOfCommunityUseCase<R, CR> {
    pub fn new(player_repository: Arc<R>, community_repository: Arc<CR>) -> Self {
        Self {
            player_repository,
            community_repository,
        }
    }

    pub async fn execute(
        &self,
        player_id: i32,
        user_id: i32,
    ) -> Result<(), (StatusCode, ApiErrorResponse)> {
        let mut player = self
            .player_repository
            .get_by_id(player_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal server error".to_string()),
                )
            })?
            .ok_or((
                StatusCode::NOT_FOUND,
                ApiErrorResponse::new("Player not found".to_string()),
            ))?;

        let belongs = self
            .community_repository
            .belongs_to_user(player.community_id, user_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Failed to verify ownership".to_string()),
                )
            })?;

        if !belongs {
            return Err((
                StatusCode::FORBIDDEN,
                ApiErrorResponse::new("Player does not belong to user".to_string()),
            ));
        }

        if !player.is_enabled() {
            return Err((
                StatusCode::BAD_REQUEST,
                ApiErrorResponse::new("Player is already disabled".to_string()),
            ));
        }

        player.disable();

        self.player_repository.save(&player).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Internal server error".to_string()),
            )
        })?;
        Ok(())
    }
}
