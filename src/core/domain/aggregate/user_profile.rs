use std::fmt;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::core::domain::entity::company::Company;
use crate::core::domain::entity::user::User;
use crate::core::domain::valueobject::position::Position;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserProfile {
	pub user: User,
	pub company: Option<Company>,
	pub position: Option<Position>,
}

impl Display for UserProfile {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "Name: {}", self.user.name)
	}
}