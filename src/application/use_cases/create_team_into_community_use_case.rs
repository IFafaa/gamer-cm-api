use axum::http::StatusCode;

use crate::{
    domain::{
        community::CommunityRepository,
        team::{Team, TeamRepository},
    },
    presentation::dtos::create_team_into_community_dto::CreateTeamIntoCommunityDto,
    shared::api_error::ApiErrorResponse,
};
use std::sync::Arc;

pub struct CreateTeamIntoCommunityUseCase<TR: TeamRepository, CR: CommunityRepository> {
    team_repository: Arc<TR>,
    community_repository: Arc<CR>,
}

impl<TR: TeamRepository, CR: CommunityRepository> CreateTeamIntoCommunityUseCase<TR, CR> {
    pub fn new(team_repository: Arc<TR>, community_repository: Arc<CR>) -> Self {
        Self {
            team_repository,
            community_repository,
        }
    }

    pub async fn execute(
        &self,
        dto: CreateTeamIntoCommunityDto,
        user_id: i32,
    ) -> Result<(), (StatusCode, ApiErrorResponse)> {
        let already_exists = self
            .team_repository
            .exists(dto.name.clone(), dto.community_id)
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
                ApiErrorResponse::new("Team already exists".to_string()),
            ));
        }

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

        let team = Team::new(dto.name, community.id);
        self.team_repository.insert(&team).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Internal server error".to_string()),
            )
        })?;
        Ok(())
    }
}
