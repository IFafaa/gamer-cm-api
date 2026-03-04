use std::sync::Arc;

use axum::http::StatusCode;

use crate::{
    application::interfaces::result_get_party_interface::IResultGetParty,
    domain::{community::CommunityRepository, party::PartyRepository},
    shared::{api_error::ApiErrorResponse, api_response::ApiResponse},
};

pub struct GetPartyByIdUseCase<PR: PartyRepository, CR: CommunityRepository> {
    party_repository: Arc<PR>,
    community_repository: Arc<CR>,
}

impl<PR: PartyRepository, CR: CommunityRepository> GetPartyByIdUseCase<PR, CR> {
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
    ) -> Result<ApiResponse<IResultGetParty>, (StatusCode, ApiErrorResponse)> {
        let party = self
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

        let result = IResultGetParty::new(party);
        Ok(ApiResponse::success(result))
    }
}
