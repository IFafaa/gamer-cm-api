use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Extension, Path, State},
    http::StatusCode,
    routing::get,
};

use crate::{
    application::{
        interfaces::result_get_stats_interface::CommunityStats,
        use_cases::get_community_stats_use_case::GetCommunityStatsUseCase,
    },
    infra::db::community_repository::PgCommunityRepository,
    presentation::middleware::auth_middleware::AuthenticatedUser,
    shared::{api_error::ApiErrorResponse, api_response::ApiResponse, state::AppState},
};

pub fn stats_routes() -> Router<AppState> {
    Router::new().route("/communities/{id}", get(get_community_stats))
}

async fn get_community_stats(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<CommunityStats>>, (StatusCode, Json<ApiErrorResponse>)> {
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = GetCommunityStatsUseCase::new(Arc::new(community_repository), state.db.clone());

    use_case
        .execute(id, user.id)
        .await
        .map(Json)
        .map_err(|(status, error)| (status, Json(error)))
}
