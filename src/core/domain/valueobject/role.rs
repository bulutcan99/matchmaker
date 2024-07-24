use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Role {
	Admin,
	Moderator,
	User,
}


impl AsRef<str> for Role {
	fn as_ref(&self) -> &str {
		match self {
			Role::Admin => "Admin",
			Role::Moderator => "Moderator",
			Role::User => "User",
		}
	}
}