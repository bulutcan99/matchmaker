use anyhow::Error;
use async_trait::async_trait;

use crate::core::domain::entity::user::User;
use crate::core::domain::valueobject::payload::Payload;

#[async_trait]
pub trait TokenMaker: Send + Sync {
    async fn generate_token(&self, user: &User) -> String;
    fn decode_token(&self, token: &str) -> Result<Payload, Error>;
}
