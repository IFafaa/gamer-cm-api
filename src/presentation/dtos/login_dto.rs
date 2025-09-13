use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginDto {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,
    
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}
