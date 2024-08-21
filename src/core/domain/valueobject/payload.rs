use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::domain::valueobject::date::Timestamp;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub user_id: Uuid,
    pub issued_at: Timestamp,
    pub expired_at: u64,
}
