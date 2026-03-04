use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginDto {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,

    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_login_dto_passes_validation() {
        let dto = LoginDto {
            username: "alice".to_string(),
            password: "pass123".to_string(),
        };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_empty_username_fails_validation() {
        let dto = LoginDto {
            username: "".to_string(),
            password: "pass123".to_string(),
        };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_empty_password_fails_validation() {
        let dto = LoginDto {
            username: "alice".to_string(),
            password: "".to_string(),
        };
        assert!(dto.validate().is_err());
    }
}
