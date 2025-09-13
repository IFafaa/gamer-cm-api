use axum::{http::StatusCode, Json};
use validator::Validate;

use crate::shared::api_response::ApiResponse;
use crate::domain::user::{User, UserRepository};
use crate::presentation::dtos::{
    register_dto::RegisterDto,
    auth_response_dto::{AuthResponseDto, UserResponseDto},
};
use crate::shared::{
    api_error::ApiErrorResponse,
    jwt_service::JwtService,
    password_service::PasswordService,
};

pub struct RegisterUserUseCase {
    user_repository: Box<dyn UserRepository>,
}

impl RegisterUserUseCase {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(
        &self,
        dto: RegisterDto,
    ) -> Result<Json<ApiResponse<AuthResponseDto>>, (StatusCode, Json<ApiErrorResponse>)> {
        dto.validate()
            .map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ApiErrorResponse::new(e.to_string())),
                )
            })?;

        if self.user_repository.get_by_username(&dto.username).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse::new(e.to_string())),
            )
        })?.is_some() {
            return Err((
                StatusCode::CONFLICT,
                Json(ApiErrorResponse::new("Username already exists".to_string())),
            ));
        }

        if self.user_repository.get_by_email(&dto.email).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse::new(e.to_string())),
            )
        })?.is_some() {
            return Err((
                StatusCode::CONFLICT,
                Json(ApiErrorResponse::new("Email already exists".to_string())),
            ));
        }

        let password_hash = PasswordService::hash_password(&dto.password)
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiErrorResponse::new(e.to_string())),
                )
            })?;

        let user = User::new(dto.username.clone(), dto.email, password_hash);
        let created_user = self.user_repository.insert(&user).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse::new(e.to_string())),
            )
        })?;

        let token = JwtService::generate_token(created_user.id, created_user.username.clone())
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiErrorResponse::new(e.to_string())),
                )
            })?;

        let auth_response = AuthResponseDto {
            token,
            user: UserResponseDto {
                id: created_user.id,
                username: created_user.username,
                email: created_user.email,
            },
        };

        Ok(Json(ApiResponse::success(auth_response)))
    }
}
