use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::domain::valueobject::sector::Sector;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Company {
	pub id: Option<Uuid>,
	pub foundation_date: u16,
	pub name: String,
	pub description: String,
	pub url: String,
	pub sector: Sector,
	pub created_at: DateTime<Local>,
	pub updated_at: DateTime<Local>,
}

impl Company {
	pub fn get_sector_string(&self) -> &str {
		self.sector.to_string()
	}
}