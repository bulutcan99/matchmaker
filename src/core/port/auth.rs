use anyhow::Error;

use crate::core::domain::entity::user::User;
use crate::core::domain::valueobject::payload::Payload;

//access and refresh token will be implemented
pub trait TokenMaker: Send + Sync {
	async fn generate_token(&self, user: &User) -> String;
	async fn decode_token(&self, token: &str) -> Result<Payload, Error>;
}
