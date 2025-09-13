use serde::Serialize;
use time::PrimitiveDateTime;

use crate::shared::date_time::DateTime;

#[derive(Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub enabled: bool,
}

impl User {
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        let now = DateTime::now();
        Self {
            id: 0,
            username,
            email,
            password_hash,
            created_at: now,
            updated_at: now,
            enabled: true,
        }
    }
}

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn insert(&self, user: &User) -> anyhow::Result<User>;
    async fn get_by_username(&self, username: &str) -> anyhow::Result<Option<User>>;
    async fn get_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;
    async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<User>>;
    async fn update(&self, user: &User) -> anyhow::Result<()>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
}
