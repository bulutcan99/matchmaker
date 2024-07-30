use argonautica::{Hasher, Verifier};
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashedPassword(String);

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Invalid password")]
    InvalidPassword,
}
impl HashedPassword {
    pub fn new(password: &str, email: &str, secret_key: &str) -> Result<Self, PasswordError> {
        let mut hasher = Hasher::default();
        let hashed_password = hasher
            .with_password(password)
            .with_salt(&email)
            .with_secret_key(secret_key)
            .hash()
            .unwrap();
        Ok(HashedPassword(hashed_password))
    }

    pub fn verify_password(&self, password: &str, secret_key: &str) -> Result<bool, PasswordError> {
        let mut verifier = Verifier::default();
        let result = verifier
            .with_hash(&self.0)
            .with_password(password)
            .with_secret_key(secret_key)
            .verify()
            .map_err(|_| PasswordError::InvalidPassword)?;

        Ok(result)
    }
}
