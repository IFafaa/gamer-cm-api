use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UpdateCommunityDto {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Name must be between 1 and 50 characters"
    ))]
    pub name: String,
}
