use axum::http::StatusCode;

use crate::{
    domain::community::CommunityRepository,
    domain::player::{Player, PlayerRepository},
    presentation::dtos::create_player_into_community_dto::CreatePlayerIntoCommunityDto,
    shared::api_error::ApiErrorResponse,
};
use std::sync::Arc;

pub struct CreatePlayerIntoCommunityUseCase<PR: PlayerRepository, CR: CommunityRepository> {
    player_repository: Arc<PR>,
    community_repository: Arc<CR>,
}

impl<PR: PlayerRepository, CR: CommunityRepository> CreatePlayerIntoCommunityUseCase<PR, CR> {
    pub fn new(player_repository: Arc<PR>, community_repository: Arc<CR>) -> Self {
        Self {
            player_repository,
            community_repository,
        }
    }

    pub async fn execute(
        &self,
        dto: CreatePlayerIntoCommunityDto,
        user_id: i32,
    ) -> Result<(), (StatusCode, ApiErrorResponse)> {
        let community = self
            .community_repository
            .get_by_id_and_user(dto.community_id, user_id)
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

        let already_exists = self
            .player_repository
            .exists(dto.nickname.clone(), community.id)
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
                ApiErrorResponse::new("Player already exists in the community".to_string()),
            ));
        }

        let player = Player::new(dto.nickname, dto.community_id);
        self.player_repository.insert(&player).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Internal server error".to_string()),
            )
        })?;
        Ok(())
    }
}
