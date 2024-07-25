use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::domain::valueobject::date::Timestamp;
use crate::core::domain::valueobject::sector::Sector;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Company {
    pub id: Option<Uuid>,
    pub name: String,
    pub foundation_date: u16,
    pub description: String,
    pub url: String,
    pub sector: Sector,
    pub updated_at: Timestamp,
    pub created_at: Timestamp,
}

impl Company {
    pub fn new(
        name: String,
        foundation_date: u16,
        description: String,
        url: String,
        sector: Sector,
    ) -> Self {
        Company {
            id,
            name,
            foundation_date,
            description,
            url,
            sector,
            created_at: Timestamp::now_utc(),
            updated_at: Timestamp::now_utc(),
        }
    }

    pub fn get_sector_string(&self) -> &str {
        self.sector.to_string()
    }
}
