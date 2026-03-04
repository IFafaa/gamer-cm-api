use axum::{Json, http::StatusCode};
use validator::Validate;

use crate::domain::user::{User, UserRepository};
use crate::presentation::dtos::{
    auth_response_dto::{AuthResponseDto, UserResponseDto},
    register_dto::RegisterDto,
};
use crate::shared::api_response::ApiResponse;
use crate::shared::{
    api_error::ApiErrorResponse, jwt_service::JwtService, password_service::PasswordService,
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
        dto.validate().map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ApiErrorResponse::new(e.to_string())),
            )
        })?;

        if self
            .user_repository
            .get_by_username(&dto.username)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiErrorResponse::new(e.to_string())),
                )
            })?
            .is_some()
        {
            return Err((
                StatusCode::CONFLICT,
                Json(ApiErrorResponse::new("Username already exists".to_string())),
            ));
        }

        if self
            .user_repository
            .get_by_email(&dto.email)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiErrorResponse::new(e.to_string())),
                )
            })?
            .is_some()
        {
            return Err((
                StatusCode::CONFLICT,
                Json(ApiErrorResponse::new("Email already exists".to_string())),
            ));
        }

        let password_hash = PasswordService::hash_password(&dto.password).map_err(|e| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::User;
    use crate::shared::date_time::DateTime;
    use mockall::mock;

    mock! {
        pub UserRepo {}
        #[async_trait::async_trait]
        impl UserRepository for UserRepo {
            async fn insert(&self, user: &User) -> anyhow::Result<User>;
            async fn get_by_username(&self, username: &str) -> anyhow::Result<Option<User>>;
            async fn get_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;
            async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<User>>;
            async fn update(&self, user: &User) -> anyhow::Result<()>;
            async fn delete(&self, id: i32) -> anyhow::Result<()>;
        }
    }

    fn make_inserted_user() -> User {
        User {
            id: 1,
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            password_hash: "hash".to_string(),
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            enabled: true,
        }
    }

    fn valid_dto() -> RegisterDto {
        RegisterDto {
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            password: "secret123".to_string(),
        }
    }

    #[tokio::test]
    async fn test_register_success() {
        unsafe {
            std::env::set_var("JWT_SECRET", "test-secret");
        }
        let mut mock = MockUserRepo::new();
        mock.expect_get_by_username().returning(|_| Ok(None));
        mock.expect_get_by_email().returning(|_| Ok(None));
        mock.expect_insert().returning(|_| Ok(make_inserted_user()));

        let use_case = RegisterUserUseCase::new(Box::new(mock));
        let result = use_case.execute(valid_dto()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_duplicate_username_returns_conflict() {
        let mut mock = MockUserRepo::new();
        mock.expect_get_by_username()
            .returning(|_| Ok(Some(make_inserted_user())));

        let use_case = RegisterUserUseCase::new(Box::new(mock));
        let result = use_case.execute(valid_dto()).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().0, StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn test_register_duplicate_email_returns_conflict() {
        let mut mock = MockUserRepo::new();
        mock.expect_get_by_username().returning(|_| Ok(None));
        mock.expect_get_by_email()
            .returning(|_| Ok(Some(make_inserted_user())));

        let use_case = RegisterUserUseCase::new(Box::new(mock));
        let result = use_case.execute(valid_dto()).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().0, StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn test_register_invalid_dto_returns_bad_request() {
        let mock = MockUserRepo::new();
        let use_case = RegisterUserUseCase::new(Box::new(mock));
        let dto = RegisterDto {
            username: "ab".to_string(), // too short
            email: "alice@example.com".to_string(),
            password: "secret123".to_string(),
        };
        let result = use_case.execute(dto).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().0, StatusCode::BAD_REQUEST);
    }
}
