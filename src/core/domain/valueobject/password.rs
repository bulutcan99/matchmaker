use argonautica::{Hasher, Verifier};
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

use crate::config::Settings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashedPassword(String);

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Invalid password")]
    InvalidPassword,
}
impl HashedPassword {
    pub fn new(password: &str, email: &str) -> Result<Self, PasswordError> {
        let mut hasher = Hasher::default();
        let hashed_password = hasher
            .with_password(password)
            .with_salt(&email)
            .with_secret_key(get_secret_key().unwrap())
            .hash()
            .unwrap();
        Ok(HashedPassword(hashed_password))
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, PasswordError> {
        let mut verifier = Verifier::default();
        let result = verifier
            .with_hash(&self.0)
            .with_password(password)
            .with_secret_key(get_secret_key().unwrap())
            .verify()
            .map_err(|_| PasswordError::InvalidPassword)?;

        Ok(result)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn get_secret_key() -> Option<&'static str> {
    let settings = Settings::get();
    settings.password.secret_key.as_deref()
}
