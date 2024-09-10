use crate::shared::mailer::mailer::MailerError;
use chrono::{Duration, Utc};
use log::info;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct VerificationToken {
    pub token: String,
    pub expires_at: i64,
}

impl VerificationToken {
    pub fn new() -> Self {
        let token = Uuid::new_v4().to_string();
        let expires_at = Utc::now()
            .checked_add_signed(Duration::minutes(30))
            .unwrap()
            .timestamp();

        VerificationToken { token, expires_at }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn is_valid(&self) -> bool {
        Utc::now().timestamp() < self.expires_at
    }
}

async fn verify_user_provided_token(
    provided_token: &str,
    stored_token: &VerificationToken,
) -> Result<(), MailerError> {
    if stored_token.token == provided_token && stored_token.is_valid() {
        info!("Token is valid, user verified.");
        Ok(())
    } else {
        Err(MailerError::InvalidToken)
    }
}
