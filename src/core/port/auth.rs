use async_trait::async_trait;
use uuid::Uuid;

use crate::core::application::usecase::auth::error::TokenError;
use crate::core::application::usecase::auth::token::Token;

#[async_trait]
pub trait TokenMaker: Send + Sync {
    fn generate_token(user: &str, salt: &Uuid) -> Result<Token, TokenError>;
    fn validate_token(token: &Token, salt: &Uuid) -> Result<(), TokenError>;
}
