use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    user_id: Uuid,
    issued_at: DateTime<Local>,
    expired_at: u64,
}
