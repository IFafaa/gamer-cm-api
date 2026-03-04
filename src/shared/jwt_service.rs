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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_token() {
        unsafe { std::env::set_var("JWT_SECRET", "test-secret"); }
        let token = JwtService::generate_token(1, "alice".to_string()).unwrap();
        let claims = JwtService::validate_token(&token).unwrap();
        assert_eq!(claims.sub, "1");
        assert_eq!(claims.username, "alice");
    }

    #[test]
    fn test_extract_user_id_from_token() {
        unsafe { std::env::set_var("JWT_SECRET", "test-secret"); }
        let token = JwtService::generate_token(42, "bob".to_string()).unwrap();
        let user_id = JwtService::extract_user_id_from_token(&token).unwrap();
        assert_eq!(user_id, 42);
    }

    #[test]
    fn test_validate_invalid_token_returns_error() {
        unsafe { std::env::set_var("JWT_SECRET", "test-secret"); }
        let result = JwtService::validate_token("not.a.valid.token");
        assert!(result.is_err());
    }

    #[test]
    fn test_token_contains_exp_in_future() {
        unsafe { std::env::set_var("JWT_SECRET", "test-secret"); }
        let token = JwtService::generate_token(1, "user".to_string()).unwrap();
        let claims = JwtService::validate_token(&token).unwrap();
        let now = chrono::Utc::now().timestamp() as usize;
        assert!(claims.exp > now);
    }
}
