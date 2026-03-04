use axum::http::StatusCode;

use crate::{
    domain::{community::CommunityRepository, party::PartyRepository},
    shared::api_error::ApiErrorResponse,
};
use std::sync::Arc;

pub struct DeletePartyUseCase<PR: PartyRepository, CR: CommunityRepository> {
    party_repository: Arc<PR>,
    community_repository: Arc<CR>,
}

impl<PR: PartyRepository, CR: CommunityRepository> DeletePartyUseCase<PR, CR> {
    pub fn new(party_repository: Arc<PR>, community_repository: Arc<CR>) -> Self {
        Self {
            party_repository,
            community_repository,
        }
    }

    pub async fn execute(
        &self,
        party_id: i32,
        user_id: i32,
    ) -> Result<(), (StatusCode, ApiErrorResponse)> {
        let mut party = self
            .party_repository
            .get_by_id(party_id)
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

        if !party.is_enabled() {
            return Err((
                StatusCode::BAD_REQUEST,
                ApiErrorResponse::new("Party is already disabled".to_string()),
            ));
        }

        party.disable();

        self.party_repository.save(&party).await.map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                ApiErrorResponse::new("Failed to save party".to_string()),
            )
        })?;

        Ok(())
    }
}
