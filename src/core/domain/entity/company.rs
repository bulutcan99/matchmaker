use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Id;

use crate::core::domain::valueobject::sector::Sector;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Company {
	pub id: Option<Id>,
	pub name: String,
	pub description: String,
	pub sector: Sector,
	pub url: String,
	pub foundation_date: u16,
	pub created_at: DateTime<Local>,
	pub updated_at: DateTime<Local>,
}

