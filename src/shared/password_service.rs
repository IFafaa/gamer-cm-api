use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{rand_core::OsRng, SaltString};

pub struct PasswordService;

impl PasswordService {
    pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(hash)?;
        let argon2 = Argon2::default();
        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_produces_valid_hash() {
        let hash = PasswordService::hash_password("mypassword123").unwrap();
        assert!(!hash.is_empty());
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_verify_correct_password_returns_true() {
        let password = "secret_pass_42";
        let hash = PasswordService::hash_password(password).unwrap();
        let result = PasswordService::verify_password(password, &hash).unwrap();
        assert!(result);
    }

    #[test]
    fn test_verify_wrong_password_returns_false() {
        let hash = PasswordService::hash_password("correct_password").unwrap();
        let result = PasswordService::verify_password("wrong_password", &hash).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_same_password_produces_different_hashes() {
        let hash1 = PasswordService::hash_password("password").unwrap();
        let hash2 = PasswordService::hash_password("password").unwrap();
        // salts are random — hashes must differ
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_verify_invalid_hash_returns_error() {
        let result = PasswordService::verify_password("password", "not_a_valid_hash");
        assert!(result.is_err());
    }
}
