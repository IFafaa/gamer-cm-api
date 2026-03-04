use axum::{Json, http::StatusCode};
use validator::Validate;

use crate::domain::user::UserRepository;
use crate::presentation::dtos::{
    auth_response_dto::{AuthResponseDto, UserResponseDto},
    login_dto::LoginDto,
};
use crate::shared::api_response::ApiResponse;
use crate::shared::{
    api_error::ApiErrorResponse, jwt_service::JwtService, password_service::PasswordService,
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
        dto.validate().map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(ApiErrorResponse::new(e.to_string())),
            )
        })?;

        let user = self
            .user_repository
            .get_by_username(&dto.username)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiErrorResponse::new(e.to_string())),
                )
            })?
            .ok_or_else(|| {
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

        let token = JwtService::generate_token(user.id, user.username.clone()).map_err(|e| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::User;
    use crate::shared::date_time::DateTime;
    use crate::shared::password_service::PasswordService;
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

    fn make_user_with_password(password: &str) -> User {
        let hash = PasswordService::hash_password(password).unwrap();
        User {
            id: 1,
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            password_hash: hash,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
            enabled: true,
        }
    }

    #[tokio::test]
    async fn test_login_success() {
        unsafe {
            std::env::set_var("JWT_SECRET", "test-secret");
        }
        let user = make_user_with_password("correct_pass");
        let mut mock = MockUserRepo::new();
        mock.expect_get_by_username()
            .returning(move |_| Ok(Some(user.clone())));

        let use_case = LoginUserUseCase::new(Box::new(mock));
        let dto = LoginDto {
            username: "alice".to_string(),
            password: "correct_pass".to_string(),
        };
        let result = use_case.execute(dto).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_login_user_not_found_returns_unauthorized() {
        let mut mock = MockUserRepo::new();
        mock.expect_get_by_username().returning(|_| Ok(None));

        let use_case = LoginUserUseCase::new(Box::new(mock));
        let dto = LoginDto {
            username: "ghost".to_string(),
            password: "pass".to_string(),
        };
        let result = use_case.execute(dto).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().0, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_login_wrong_password_returns_unauthorized() {
        let user = make_user_with_password("correct_pass");
        let mut mock = MockUserRepo::new();
        mock.expect_get_by_username()
            .returning(move |_| Ok(Some(user.clone())));

        let use_case = LoginUserUseCase::new(Box::new(mock));
        let dto = LoginDto {
            username: "alice".to_string(),
            password: "wrong_pass".to_string(),
        };
        let result = use_case.execute(dto).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().0, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_login_empty_credentials_returns_bad_request() {
        let mock = MockUserRepo::new();
        let use_case = LoginUserUseCase::new(Box::new(mock));
        let dto = LoginDto {
            username: "".to_string(),
            password: "".to_string(),
        };
        let result = use_case.execute(dto).await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().0, StatusCode::BAD_REQUEST);
    }
}
