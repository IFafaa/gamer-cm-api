use axum::http::StatusCode;

use crate::{
    domain::{community::CommunityRepository, party::PartyRepository, team::TeamRepository},
    presentation::dtos::end_party_dto::EndPartyDto,
    shared::api_error::ApiErrorResponse,
};
use std::sync::Arc;

pub struct EndPartyUseCase<PR: PartyRepository, TR: TeamRepository, CR: CommunityRepository> {
    party_repository: Arc<PR>,
    team_repository: Arc<TR>,
    community_repository: Arc<CR>,
}

impl<PR: PartyRepository, TR: TeamRepository, CR: CommunityRepository>
    EndPartyUseCase<PR, TR, CR>
{
    pub fn new(
        party_repository: Arc<PR>,
        team_repository: Arc<TR>,
        community_repository: Arc<CR>,
    ) -> Self {
        Self {
            party_repository,
            team_repository,
            community_repository,
        }
    }

    pub async fn execute(
        &self,
        dto: EndPartyDto,
        user_id: i32,
    ) -> Result<(), (StatusCode, ApiErrorResponse)> {
        let party = self
            .party_repository
            .get_by_id(dto.party_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Failed to fetch party".to_string()),
                )
            })?
            .ok_or((
                StatusCode::NOT_FOUND,
                ApiErrorResponse::new("Party not found".to_string()),
            ))?;

        let belongs = self
            .community_repository
            .belongs_to_user(party.community_id, user_id)
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
                ApiErrorResponse::new("Party does not belong to user".to_string()),
            ));
        }

        if party.is_finished() {
            return Err((
                StatusCode::BAD_REQUEST,
                ApiErrorResponse::new("Party is already finished".to_string()),
            ));
        }

        let team_winner = match dto.team_winner_id {
            Some(team_id) => Some(
                self.team_repository
                    .get_by_id(team_id)
                    .await
                    .map_err(|_| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            ApiErrorResponse::new("Failed to fetch team".to_string()),
                        )
                    })?
                    .ok_or((
                        StatusCode::BAD_REQUEST,
                        ApiErrorResponse::new("Team not found".to_string()),
                    ))?,
            ),
            None => None,
        };

        if let Some(ref team) = team_winner {
            let is_team_not_in_party = !party.teams.iter().any(|t| t.id == team.id);
            if is_team_not_in_party {
                return Err((
                    StatusCode::BAD_REQUEST,
                    ApiErrorResponse::new("Team not part of the party".to_string()),
                ));
            }
        }

        let mut party = party;
        let winner_id = team_winner.map(|team| team.id);
        party.end(winner_id);

        self.party_repository.save(&party).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Failed to save party".to_string()),
            )
        })?;

        Ok(())
    }
}
