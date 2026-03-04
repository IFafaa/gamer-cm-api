use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateCommunityDto {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Name must be between 1 and 50 characters"
    ))]
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_create_community_dto_passes_validation() {
        let dto = CreateCommunityDto { name: "Pro Gamers".to_string() };
        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_empty_name_fails_validation() {
        let dto = CreateCommunityDto { name: "".to_string() };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_name_too_long_fails_validation() {
        let dto = CreateCommunityDto { name: "x".repeat(51) };
        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_name_at_max_length_passes_validation() {
        let dto = CreateCommunityDto { name: "a".repeat(50) };
        assert!(dto.validate().is_ok());
    }
}
