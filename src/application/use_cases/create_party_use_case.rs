use axum::http::StatusCode;

use crate::{
    domain::{
        community::CommunityRepository,
        party::{Party, PartyRepository},
        team::TeamRepository,
    },
    presentation::dtos::create_party_dto::CreatePartyDto,
    shared::api_error::ApiErrorResponse,
};
use std::sync::Arc;

pub struct CreatePartyUseCase<TR: TeamRepository, CR: CommunityRepository, PR: PartyRepository> {
    party_repository: Arc<PR>,
    team_repository: Arc<TR>,
    community_repository: Arc<CR>,
}

impl<TR: TeamRepository, CR: CommunityRepository, PR: PartyRepository>
    CreatePartyUseCase<TR, CR, PR>
{
    pub fn new(
        team_repository: Arc<TR>,
        community_repository: Arc<CR>,
        party_repository: Arc<PR>,
    ) -> Self {
        Self {
            team_repository,
            community_repository,
            party_repository,
        }
    }

    pub async fn execute(&self, dto: CreatePartyDto, user_id: i32) -> Result<(), (StatusCode, ApiErrorResponse)> {
        let community = self
            .community_repository
            .get_by_id_and_user(dto.community_id, user_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Failed to fetch community".to_string()),
                )
            })?
            .ok_or((
                StatusCode::BAD_REQUEST,
                ApiErrorResponse::new("Community not found".to_string()),
            ))?;

        let teams = self
            .team_repository
            .get_by_ids(dto.teams_ids.clone())
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Failed to fetch teams".to_string()),
                )
            })?;

        if teams.len() != dto.teams_ids.len() {
            return Err((
                StatusCode::BAD_REQUEST,
                ApiErrorResponse::new("Some teams not found".to_string()),
            ));
        }

        let party = Party::new(dto.game_name, teams, community.id);

        self.party_repository.insert(&party).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Failed to create party".to_string()),
            )
        })?;

        Ok(())
    }
}
