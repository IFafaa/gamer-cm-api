use axum::http::StatusCode;
use std::sync::Arc;

use crate::{
    domain::{community::CommunityRepository, player::PlayerRepository},
    presentation::dtos::update_player_dto::UpdatePlayerDto,
    shared::{api_error::ApiErrorResponse, date_time::DateTime},
};

pub struct UpdatePlayerUseCase<PR: PlayerRepository, CR: CommunityRepository> {
    player_repository: Arc<PR>,
    community_repository: Arc<CR>,
}

impl<PR: PlayerRepository, CR: CommunityRepository> UpdatePlayerUseCase<PR, CR> {
    pub fn new(player_repository: Arc<PR>, community_repository: Arc<CR>) -> Self {
        Self {
            player_repository,
            community_repository,
        }
    }

    pub async fn execute(
        &self,
        player_id: i32,
        user_id: i32,
        dto: UpdatePlayerDto,
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

        // Check duplicate nickname in same community
        let already_exists = self
            .player_repository
            .exists(dto.nickname.clone(), player.community_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal server error".to_string()),
                )
            })?;

        if already_exists && player.nickname != dto.nickname {
            return Err((
                StatusCode::CONFLICT,
                ApiErrorResponse::new("Player with this nickname already exists".to_string()),
            ));
        }

        player.nickname = dto.nickname;
        player.updated_at = DateTime::now();

        self.player_repository.save(&player).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Internal server error".to_string()),
            )
        })?;

        Ok(())
    }
}
