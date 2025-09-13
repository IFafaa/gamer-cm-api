use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn new(user_id: i32, username: String) -> Self {
        let now = chrono::Utc::now().timestamp() as usize;
        Self {
            sub: user_id.to_string(),
            username,
            exp: now + (24 * 60 * 60), // 24 hours
            iat: now,
        }
    }
}

pub struct JwtService;

impl JwtService {
    fn get_secret() -> String {
        env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string())
    }

    pub fn generate_token(user_id: i32, username: String) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims::new(user_id, username);
        let secret = Self::get_secret();
        encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
    }

    pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let secret = Self::get_secret();
        let validation = Validation::new(Algorithm::HS256);
        decode::<Claims>(token, &DecodingKey::from_secret(secret.as_ref()), &validation)
            .map(|data| data.claims)
    }

    pub fn extract_user_id_from_token(token: &str) -> Result<i32, anyhow::Error> {
        let claims = Self::validate_token(token)?;
        claims.sub.parse().map_err(|e| anyhow::anyhow!("Invalid user ID in token: {}", e))
    }
}
