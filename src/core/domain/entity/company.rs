use chrono::{ DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Id;
use crate::core::domain::valueobject::sector::Sector;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub  struct Company {
    id: Option<Id>,
    name: String,
    description: String,
    sector: Sector,
    url: String,
    foundation_date: u16,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

