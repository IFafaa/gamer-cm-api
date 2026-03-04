use axum::http::StatusCode;
use std::sync::Arc;

use crate::{
    domain::{community::CommunityRepository, team::TeamRepository},
    presentation::dtos::update_team_dto::UpdateTeamDto,
    shared::{api_error::ApiErrorResponse, date_time::DateTime},
};

pub struct UpdateTeamUseCase<TR: TeamRepository, CR: CommunityRepository> {
    team_repository: Arc<TR>,
    community_repository: Arc<CR>,
}

impl<TR: TeamRepository, CR: CommunityRepository> UpdateTeamUseCase<TR, CR> {
    pub fn new(team_repository: Arc<TR>, community_repository: Arc<CR>) -> Self {
        Self {
            team_repository,
            community_repository,
        }
    }

    pub async fn execute(
        &self,
        team_id: i32,
        user_id: i32,
        dto: UpdateTeamDto,
    ) -> Result<(), (StatusCode, ApiErrorResponse)> {
        let mut team = self
            .team_repository
            .get_by_id(team_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal server error".to_string()),
                )
            })?
            .ok_or((
                StatusCode::NOT_FOUND,
                ApiErrorResponse::new("Team not found".to_string()),
            ))?;

        let belongs = self
            .community_repository
            .belongs_to_user(team.community_id, user_id)
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
                ApiErrorResponse::new("Team does not belong to user".to_string()),
            ));
        }

        // Check duplicate name in same community
        let already_exists = self
            .team_repository
            .exists(dto.name.clone(), team.community_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal server error".to_string()),
                )
            })?;

        if already_exists && team.name != dto.name {
            return Err((
                StatusCode::CONFLICT,
                ApiErrorResponse::new("Team with this name already exists".to_string()),
            ));
        }

        team.name = dto.name;
        team.updated_at = DateTime::now();

        self.team_repository.save(&team).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Internal server error".to_string()),
            )
        })?;

        Ok(())
    }
}
