use std::fmt;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::core::domain::entity::company::Company;
use crate::core::domain::entity::user::User;
use crate::core::domain::valueobject::position::Position;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Worker {
	pub user: User,
	pub company: Option<Company>,
	pub position: Option<Position>,
}

impl Display for Worker {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "User: {}", self.user.name)
	}
}


impl Worker {
	pub fn sit_table(&self, table_id: u8) {
		println!("{}, sitted {}", self.user.name, table_id)
	}
}