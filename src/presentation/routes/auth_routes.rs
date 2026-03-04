use axum::extract::State;
use axum::{Json, Router, routing::post};

use crate::application::use_cases::{
    login_user_use_case::LoginUserUseCase, register_user_use_case::RegisterUserUseCase,
};
use crate::presentation::dtos::{login_dto::LoginDto, register_dto::RegisterDto};
use crate::shared::state::AppState;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(
    State(app_state): State<AppState>,
    Json(dto): Json<RegisterDto>,
) -> Result<
    Json<
        crate::shared::api_response::ApiResponse<
            crate::presentation::dtos::auth_response_dto::AuthResponseDto,
        >,
    >,
    (
        axum::http::StatusCode,
        Json<crate::shared::api_error::ApiErrorResponse>,
    ),
> {
    let user_repository = Box::new(crate::infra::db::user_repository::PgUserRepository::new(
        app_state.db.clone(),
    ));
    let use_case = RegisterUserUseCase::new(user_repository);
    use_case.execute(dto).await
}

async fn login(
    State(app_state): State<AppState>,
    Json(dto): Json<LoginDto>,
) -> Result<
    Json<
        crate::shared::api_response::ApiResponse<
            crate::presentation::dtos::auth_response_dto::AuthResponseDto,
        >,
    >,
    (
        axum::http::StatusCode,
        Json<crate::shared::api_error::ApiErrorResponse>,
    ),
> {
    let user_repository = Box::new(crate::infra::db::user_repository::PgUserRepository::new(
        app_state.db.clone(),
    ));
    let use_case = LoginUserUseCase::new(user_repository);
    use_case.execute(dto).await
}
