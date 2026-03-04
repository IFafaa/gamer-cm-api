use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterDto {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Username must be between 3 and 50 characters"
    ))]
    pub username: String,

    #[validate(email(message = "Must be a valid email"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_dto() -> RegisterDto {
        RegisterDto {
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            password: "secure123".to_string(),
        }
    }

    #[test]
    fn test_valid_register_dto_passes_validation() {
        assert!(valid_dto().validate().is_ok());
    }

    #[test]
    fn test_username_too_short_fails_validation() {
        let dto = RegisterDto {
            username: "ab".to_string(),
            ..valid_dto()
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_username_too_long_fails_validation() {
        let dto = RegisterDto {
            username: "a".repeat(51),
            ..valid_dto()
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_invalid_email_fails_validation() {
        let dto = RegisterDto {
            email: "not-an-email".to_string(),
            ..valid_dto()
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_password_too_short_fails_validation() {
        let dto = RegisterDto {
            password: "abc".to_string(),
            ..valid_dto()
        };
        assert!(dto.validate().is_err());
    }
}
