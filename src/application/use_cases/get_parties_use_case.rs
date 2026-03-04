use std::sync::Arc;

use axum::http::StatusCode;

use crate::{
    application::interfaces::result_get_party_interface::IResultGetParty,
    domain::{
        community::CommunityRepository,
        party::{IGetPartiesByParams, PartyRepository},
    },
    shared::{api_error::ApiErrorResponse, api_response::ApiResponse},
};

pub struct GetPartiesUseCase<PR: PartyRepository, CR: CommunityRepository> {
    party_repository: Arc<PR>,
    community_repository: Arc<CR>,
}

impl<PR: PartyRepository, CR: CommunityRepository> GetPartiesUseCase<PR, CR> {
    pub fn new(party_repository: Arc<PR>, community_repository: Arc<CR>) -> Self {
        Self {
            party_repository,
            community_repository,
        }
    }

    pub async fn execute(
        &self,
        user_id: i32,
        community_id: Option<i32>,
    ) -> Result<ApiResponse<Vec<IResultGetParty>>, (StatusCode, ApiErrorResponse)> {
        // If community_id filter provided, verify ownership
        if let Some(cid) = community_id {
            let belongs = self
                .community_repository
                .belongs_to_user(cid, user_id)
                .await
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ApiErrorResponse::new("Failed to verify community ownership".to_string()),
                    )
                })?;

            if !belongs {
                return Err((
                    StatusCode::FORBIDDEN,
                    ApiErrorResponse::new("Community does not belong to user".to_string()),
                ));
            }
        }

        // Get user's community IDs to filter parties
        let community_ids = if let Some(cid) = community_id {
            vec![cid]
        } else {
            self.community_repository
                .get_ids_by_user(user_id)
                .await
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ApiErrorResponse::new("Failed to fetch communities".to_string()),
                    )
                })?
        };

        if community_ids.is_empty() {
            return Ok(ApiResponse::success(vec![]));
        }

        let mut all_parties = Vec::new();
        for cid in community_ids {
            let params = IGetPartiesByParams {
                community_id: Some(cid),
                game_name: None,
                created_at: None,
                updated_at: None,
                teams_ids: None,
                team_winner_ids: None,
            };

            let parties = self
                .party_repository
                .get_by_params(params)
                .await
                .map_err(|_| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ApiErrorResponse::new("Failed to fetch parties".to_string()),
                    )
                })?;

            all_parties.extend(parties);
        }

        let result = all_parties
            .into_iter()
            .map(IResultGetParty::new)
            .collect();

        Ok(ApiResponse::success(result))
    }
}
