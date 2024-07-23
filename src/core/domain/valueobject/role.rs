use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Role {
	Admin,
	Moderator,
	User,
}