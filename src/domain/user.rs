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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_new_sets_fields() {
        let user = User::new(
            "alice".to_string(),
            "alice@example.com".to_string(),
            "hashed_pass".to_string(),
        );
        assert_eq!(user.username, "alice");
        assert_eq!(user.email, "alice@example.com");
        assert_eq!(user.password_hash, "hashed_pass");
        assert_eq!(user.id, 0);
        assert!(user.enabled);
    }

    #[test]
    fn test_user_new_is_enabled_by_default() {
        let user = User::new("bob".to_string(), "bob@test.com".to_string(), "hash".to_string());
        assert!(user.enabled);
    }

    #[test]
    fn test_user_clone() {
        let user = User::new("carol".to_string(), "carol@test.com".to_string(), "h".to_string());
        let cloned = user.clone();
        assert_eq!(cloned.username, user.username);
        assert_eq!(cloned.email, user.email);
    }
}
