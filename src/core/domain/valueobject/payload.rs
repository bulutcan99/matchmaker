use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
}
