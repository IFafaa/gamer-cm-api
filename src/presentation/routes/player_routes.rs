use std::sync::Arc;

use crate::{
    application::use_cases::{
        create_player_into_community_use_case::CreatePlayerIntoCommunityUseCase,
        delete_player_of_community_use_case::DeletePlayerOfCommunityUseCase,
        update_player_use_case::UpdatePlayerUseCase,
    },
    infra::db::{
        community_repository::PgCommunityRepository, player_repository::PgPlayerRepository,
    },
    presentation::{
        dtos::{
            create_player_into_community_dto::CreatePlayerIntoCommunityDto,
            update_player_dto::UpdatePlayerDto,
        },
        middleware::auth_middleware::AuthenticatedUser,
    },
    shared::{api_error::ApiErrorResponse, state::AppState, validate_dto::validate_dto},
};
use axum::{
    Router,
    extract::{Json, Path, State, Extension},
    http::StatusCode,
    routing::{delete, post, put},
};

pub fn player_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_player_into_community))
        .route("/{id}", put(update_player))
        .route("/{id}", delete(delete_player_of_community))
}

async fn create_player_into_community(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(dto): Json<CreatePlayerIntoCommunityDto>,
) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    validate_dto(&dto)?;

    let player_repository = PgPlayerRepository::new(state.db.clone());
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = CreatePlayerIntoCommunityUseCase::new(
        Arc::new(player_repository),
        Arc::new(community_repository),
    );

    use_case
        .execute(dto, user.id)
        .await
        .map_err(|(status, error)| (status, Json(error)))
}

async fn update_player(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
    Json(dto): Json<UpdatePlayerDto>,
) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    validate_dto(&dto)?;

    let player_repository = PgPlayerRepository::new(state.db.clone());
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = UpdatePlayerUseCase::new(
        Arc::new(player_repository),
        Arc::new(community_repository),
    );

    use_case
        .execute(id, user.id, dto)
        .await
        .map_err(|(status, error)| (status, Json(error)))
}

async fn delete_player_of_community(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<i32>,
) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    let player_repository = PgPlayerRepository::new(state.db.clone());
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = DeletePlayerOfCommunityUseCase::new(
        Arc::new(player_repository),
        Arc::new(community_repository),
    );

    use_case
        .execute(id, user.id)
        .await
        .map_err(|(status, error)| (status, Json(error)))
}
