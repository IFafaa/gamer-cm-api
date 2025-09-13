use serde::Serialize;

#[derive(Serialize)]
pub struct AuthResponseDto {
    pub token: String,
    pub user: UserResponseDto,
}

#[derive(Serialize)]
pub struct UserResponseDto {
    pub id: i32,
    pub username: String,
    pub email: String,
}
