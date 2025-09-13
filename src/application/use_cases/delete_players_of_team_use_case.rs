use std::sync::Arc;

use crate::{
    domain::{
        player::PlayerRepository,
        team::TeamRepository,
    },
    presentation::dtos::delete_players_of_team_dto::DeletePlayersOfTeamDto,
    shared::api_error::ApiErrorResponse,
};

pub struct DeletePlayersOfTeamUseCase<PR: PlayerRepository, TR: TeamRepository> {
    player_repository: Arc<PR>,
    team_repository: Arc<TR>,
}

impl<PR: PlayerRepository, TR: TeamRepository> DeletePlayersOfTeamUseCase<PR, TR> {
    pub fn new(
        player_repository: Arc<PR>,
        team_repository: Arc<TR>,
    ) -> Self {
        Self {
            player_repository,
            team_repository,
        }
    }

    pub async fn execute(
        &self,
        dto: DeletePlayersOfTeamDto,
    ) -> Result<(), (axum::http::StatusCode, ApiErrorResponse)> {
        let mut team = self
            .team_repository
            .get_by_id(dto.team_id)
            .await
            .map_err(|_| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal server error".to_string()),
                )
            })?
            .ok_or_else(|| {
                (
                    axum::http::StatusCode::NOT_FOUND,
                    ApiErrorResponse::new("Team not found".to_string()),
                )
            })?;

        if let Some(name) = dto.name {
            team.name = name;
        }

        let mut updated_players = Vec::new();

        for mut player in team.players.clone() {
            player.disable();
            self.player_repository.save(&player).await.map_err(|_| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Failed to disable player".to_string()),
                )
            })?;
        }

        for player_id in dto.player_ids {
            let mut player = self
                .player_repository
                .get_by_id(player_id)
                .await
                .map_err(|_| {
                    (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        ApiErrorResponse::new("Internal server error".to_string()),
                    )
                })?
                .ok_or_else(|| {
                    (
                        axum::http::StatusCode::NOT_FOUND,
                        ApiErrorResponse::new(format!("Player with ID {} not found", player_id)),
                    )
                })?;

            if player.community_id != team.community_id {
                return Err((
                    axum::http::StatusCode::BAD_REQUEST,
                    ApiErrorResponse::new(format!(
                        "Player {} is not in the same community as team {}",
                        player.nickname, team.name
                    )),
                ));
            }

            if !player.enabled {
                player.enabled = true;
                player.updated_at = crate::shared::date_time::DateTime::now();
                self.player_repository.save(&player).await.map_err(|_| {
                    (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        ApiErrorResponse::new("Failed to re-enable player".to_string()),
                    )
                })?;
            }

            updated_players.push(player);
        }

        team.players = updated_players;
        team.updated_at = crate::shared::date_time::DateTime::now();

        self.team_repository.save(&team).await.map_err(|_| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Failed to save team".to_string()),
            )
        })?;

        Ok(())
    }
}
