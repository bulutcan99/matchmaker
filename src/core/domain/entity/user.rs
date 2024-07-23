use anyhow::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::domain::valueobject::date::DateService;
use crate::core::domain::valueobject::role::Role;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
	pub id: Option<Uuid>,
	pub name: String,
	pub surname: String,
	pub email: String,
	pub role: Role,
	pub password_hash: String,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

impl User {
	pub fn new_user(name: String, surname: String, email: String, password_hash: String) -> Self {
		User {
			id,
			name,
			surname,
			email,
			password_hash,
			role: Role::User,
			created_at: DateService::get_current_timestamp_utc(),
			updated_at: DateService::get_current_timestamp_utc(),
		}
	}

	pub fn new_admin(name: String, surname: String, email: String, password_hash: String) -> Self {
		User {
			id,
			name,
			surname,
			email,
			password_hash,
			role: Role::Admin,
			created_at: DateService::get_current_timestamp_utc(),
			updated_at: DateService::get_current_timestamp_utc(),
		}
	}

	pub fn update(
		&mut self,
		name: Option<String>,
		surname: Option<String>,
		email: Option<String>,
		password_hash: Option<String>,
	) -> Result<(), Error> {
		if let Some(password_hash) = password_hash {
			self.password_hash = password_hash;
		}
		if let Some(name) = name {
			self.name = name;
		}
		if let Some(surname) = surname {
			self.surname = surname;
		}
		if let Some(email) = email {
			self.email = email;
		}
		self.updated_at = Utc::now();

		Ok(())
	}

	pub fn update_role(&mut self, role: Option<Role>) -> Result<(), Error> {
		if let Some(role) = role {
			self.role = role;
		}

		Ok(())
	}
}

