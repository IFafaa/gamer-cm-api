use axum::http::StatusCode;

use crate::{
    application::interfaces::result_get_community_interface::IResultGetCommunity,
    domain::community::CommunityRepository,
    shared::{api_error::ApiErrorResponse, api_response::ApiResponse},
};
use std::sync::Arc;

pub struct GetCommunityByIdUseCase<R: CommunityRepository> {
    community_repository: Arc<R>,
}

impl<R: CommunityRepository> GetCommunityByIdUseCase<R> {
    pub fn new(community_repository: Arc<R>) -> Self {
        Self {
            community_repository,
        }
    }

    pub async fn execute(
        &self,
        community_id: i32,
        user_id: i32,
    ) -> Result<ApiResponse<IResultGetCommunity>, (StatusCode, ApiErrorResponse)> {
        let community = self
            .community_repository
            .get_by_id_and_user(community_id, user_id)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorResponse::new("Internal Server Error".to_string()),
                )
            })?
            .ok_or((
                StatusCode::NOT_FOUND,
                ApiErrorResponse::new("Community not found".to_string()),
            ))?;

        let result = IResultGetCommunity::new(community);
        Ok(ApiResponse::success(result))
    }
}
