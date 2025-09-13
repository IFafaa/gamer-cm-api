use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::user::{User, UserRepository};

pub struct PgUserRepository {
    pub pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn insert(&self, user: &User) -> anyhow::Result<User> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO users (username, email, password_hash, created_at, updated_at, enabled)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
            user.username,
            user.email,
            user.password_hash,
            user.created_at,
            user.updated_at,
            user.enabled,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(User {
            id: rec.id,
            username: user.username.clone(),
            email: user.email.clone(),
            password_hash: user.password_hash.clone(),
            created_at: user.created_at,
            updated_at: user.updated_at,
            enabled: user.enabled,
        })
    }

    async fn get_by_username(&self, username: &str) -> anyhow::Result<Option<User>> {
        let row = sqlx::query!(
            r#"
            SELECT id, username, email, password_hash, created_at, updated_at, enabled
            FROM users
            WHERE username = $1 AND enabled = true
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.id,
            username: r.username,
            email: r.email,
            password_hash: r.password_hash,
            created_at: r.created_at,
            updated_at: r.updated_at,
            enabled: r.enabled,
        }))
    }

    async fn get_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        let row = sqlx::query!(
            r#"
            SELECT id, username, email, password_hash, created_at, updated_at, enabled
            FROM users
            WHERE email = $1 AND enabled = true
            "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.id,
            username: r.username,
            email: r.email,
            password_hash: r.password_hash,
            created_at: r.created_at,
            updated_at: r.updated_at,
            enabled: r.enabled,
        }))
    }

    async fn get_by_id(&self, id: i32) -> anyhow::Result<Option<User>> {
        let row = sqlx::query!(
            r#"
            SELECT id, username, email, password_hash, created_at, updated_at, enabled
            FROM users
            WHERE id = $1 AND enabled = true
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| User {
            id: r.id,
            username: r.username,
            email: r.email,
            password_hash: r.password_hash,
            created_at: r.created_at,
            updated_at: r.updated_at,
            enabled: r.enabled,
        }))
    }

    async fn update(&self, user: &User) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE users 
            SET username = $1, email = $2, password_hash = $3, updated_at = $4, enabled = $5
            WHERE id = $6
            "#,
            user.username,
            user.email,
            user.password_hash,
            user.updated_at,
            user.enabled,
            user.id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE users 
            SET enabled = false, updated_at = NOW()
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
