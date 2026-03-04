use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UpdatePlayerDto {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Nickname must be between 1 and 50 characters"
    ))]
    pub nickname: String,
}
