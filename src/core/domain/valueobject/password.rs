use argonautica::{Hasher, Verifier};
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

use crate::shared::config::config::Config;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
            .with_salt(email)
            .with_secret_key(get_secret_key())
            .hash()
            .unwrap();
        Ok(HashedPassword(hashed_password))
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, PasswordError> {
        let mut verifier = Verifier::default();
        let result = verifier
            .with_hash(&self.0)
            .with_password(password)
            .with_secret_key(get_secret_key())
            .verify()
            .map_err(|_| PasswordError::InvalidPassword)?;

        Ok(result)
    }

    pub fn as_string(&self) -> String {
        self.0.to_owned()
    }
}

impl From<String> for HashedPassword {
    fn from(s: String) -> Self {
        HashedPassword(s)
    }
}

fn get_secret_key() -> &'static str {
    let settings = Config::get();
    &settings.auth.password.secret
}
