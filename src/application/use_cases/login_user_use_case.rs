use axum::{http::StatusCode, Json};
use validator::Validate;

use crate::shared::api_response::ApiResponse;
use crate::domain::user::UserRepository;
use crate::presentation::dtos::{
    login_dto::LoginDto,
    auth_response_dto::{AuthResponseDto, UserResponseDto},
};
use crate::shared::{
    api_error::ApiErrorResponse,
    jwt_service::JwtService,
    password_service::PasswordService,
};

pub struct LoginUserUseCase {
    user_repository: Box<dyn UserRepository>,
}

impl LoginUserUseCase {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(
        &self,
        dto: LoginDto,
    ) -> Result<Json<ApiResponse<AuthResponseDto>>, (StatusCode, Json<ApiErrorResponse>)> {
        dto.validate()
            .map_err(|e| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ApiErrorResponse::new(e.to_string())),
                )
            })?;

        let user = self.user_repository.get_by_username(&dto.username).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse::new(e.to_string())),
            )
        })?.ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiErrorResponse::new("Invalid credentials".to_string())),
            )
        })?;

        let password_valid = PasswordService::verify_password(&dto.password, &user.password_hash)
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiErrorResponse::new(e.to_string())),
                )
            })?;

        if !password_valid {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiErrorResponse::new("Invalid credentials".to_string())),
            ));
        }

        let token = JwtService::generate_token(user.id, user.username.clone())
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiErrorResponse::new(e.to_string())),
                )
            })?;

        let auth_response = AuthResponseDto {
            token,
            user: UserResponseDto {
                id: user.id,
                username: user.username,
                email: user.email,
            },
        };

        Ok(Json(ApiResponse::success(auth_response)))
    }
}
