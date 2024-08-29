use anyhow::Error;
use async_trait::async_trait;
use uuid::Uuid;
use crate::core::application::usecase::auth::error::TokenError;
use crate::core::application::usecase::auth::token::Token;
use crate::core::domain::valueobject::payload::Payload;

#[async_trait]
pub trait TokenMaker: Send + Sync {
    async fn generate_token(user: &str, salt: Uuid) -> Result<Token, TokenError>
    fn decode_token( token: &Token,salt: Uuid ) -> Result<String, Error>;
}
