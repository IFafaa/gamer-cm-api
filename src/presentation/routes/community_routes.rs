use std::sync::Arc;

use crate::{
    application::{
        interfaces::result_get_community_interface::IResultGetCommunity,
        use_cases::{
            create_community_use_case::CreateCommunityUseCase,
            delete_community_use_case::DeleteCommunityUseCase,
            get_communities_use_case::GetCommunitiesUseCase,
            get_community_by_id_use_case::GetCommunityByIdUseCase,
            update_community_use_case::UpdateCommunityUseCase,
        },
    },
    infra::db::community_repository::PgCommunityRepository,
    presentation::{
        dtos::{
            create_community_dto::CreateCommunityDto, update_community_dto::UpdateCommunityDto,
        },
        middleware::auth_middleware::AuthenticatedUser,
    },
    shared::{
        api_error::ApiErrorResponse, api_response::ApiResponse, pagination::PaginationParams,
        state::AppState, validate_dto::validate_dto,
    },
};
use axum::{
    Router,
    extract::{Extension, Json, Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
};

pub fn community_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_community))
        .route("/", get(get_communities))
        .route("/{id}", get(get_community_by_id))
        .route("/{id}", put(update_community))
        .route("/{id}", delete(delete_community))
}

async fn create_community(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(dto): Json<CreateCommunityDto>,
) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    validate_dto(&dto)?;

    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = CreateCommunityUseCase::new(Arc::new(community_repository));

    use_case
        .execute(dto, user.id)
        .await
        .map_err(|(status, error)| (status, Json(error)))
}

async fn get_communities(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<IResultGetCommunity>>>, (StatusCode, Json<ApiErrorResponse>)> {
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = GetCommunitiesUseCase::new(Arc::new(community_repository));

    use_case
        .execute(user.id, pagination)
        .await
        .map(Json)
        .map_err(|(status, error)| (status, Json(error)))
}

async fn get_community_by_id(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
) -> Result<Json<ApiResponse<IResultGetCommunity>>, (StatusCode, Json<ApiErrorResponse>)> {
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = GetCommunityByIdUseCase::new(Arc::new(community_repository));

    use_case
        .execute(id, user.id)
        .await
        .map(|response| Json(response))
        .map_err(|(status, error)| (status, Json(error)))
}

async fn update_community(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
    Json(dto): Json<UpdateCommunityDto>,
) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    validate_dto(&dto)?;

    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = UpdateCommunityUseCase::new(Arc::new(community_repository));

    use_case
        .execute(id, user.id, dto)
        .await
        .map_err(|(status, error)| (status, Json(error)))
}

async fn delete_community(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = DeleteCommunityUseCase::new(Arc::new(community_repository));

    use_case
        .execute(id, user.id)
        .await
        .map_err(|(status, error)| (status, Json(error)))
}
