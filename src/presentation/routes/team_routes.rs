use std::sync::Arc;

use axum::{
    Router,
    extract::{Json, State, Extension},
    http::StatusCode,
    routing::{post, patch},
};

use crate::{
    application::use_cases::{
        add_players_into_team_use_case::AddPlayersIntoTeamUseCase,
        create_team_into_community_use_case::CreateTeamIntoCommunityUseCase,
        delete_players_of_team_use_case::DeletePlayersOfTeamUseCase,
    },
    infra::db::{
        community_repository::PgCommunityRepository, player_repository::PgPlayerRepository,
        team_repository::PgTeamRepository,
    },
    presentation::{
        dtos::{
            add_players_into_team_dto::AddPlayersIntoTeamDto,
            create_team_into_community_dto::CreateTeamIntoCommunityDto,
            delete_players_of_team_dto::DeletePlayersOfTeamDto,
        },
        middleware::auth_middleware::AuthenticatedUser,
    },
    shared::{api_error::ApiErrorResponse, state::AppState, validate_dto::validate_dto},
};

pub fn team_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_team_into_community))
        .route("/add-players", post(add_players_into_team))
        .route("/delete-players", patch(delete_players_of_team))
}

async fn create_team_into_community(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(dto): Json<CreateTeamIntoCommunityDto>,
) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    validate_dto(&dto)?;

    let team_repository = PgTeamRepository::new(state.db.clone());
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = CreateTeamIntoCommunityUseCase::new(
        Arc::new(team_repository),
        Arc::new(community_repository),
    );

    use_case
        .execute(dto, user.id)
        .await
        .map_err(|(status, error)| (status, Json(error)))
}

async fn add_players_into_team(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(dto): Json<AddPlayersIntoTeamDto>,
) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    dto.validate()?;

    let team_repository = PgTeamRepository::new(state.db.clone());
    let player_repository = PgPlayerRepository::new(state.db.clone());
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = AddPlayersIntoTeamUseCase::new(
        Arc::new(player_repository),
        Arc::new(team_repository),
        Arc::new(community_repository),
    );

    use_case
        .execute(dto, user.id)
        .await
        .map_err(|(status, error)| (status, Json(error)))
}

async fn delete_players_of_team(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(dto): Json<DeletePlayersOfTeamDto>,
) -> Result<(), (StatusCode, Json<ApiErrorResponse>)> {
    dto.validate()?;

    let team_repository = PgTeamRepository::new(state.db.clone());
    let player_repository = PgPlayerRepository::new(state.db.clone());
    let community_repository = PgCommunityRepository::new(state.db.clone());
    let use_case = DeletePlayersOfTeamUseCase::new(
        Arc::new(player_repository),
        Arc::new(team_repository),
        Arc::new(community_repository),
    );

    use_case
        .execute(dto, user.id)
        .await
        .map_err(|(status, error)| (status, Json(error)))
}
