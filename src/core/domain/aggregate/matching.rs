use std::collections::HashMap;

use crate::core::domain::aggregate::user_profile::UserProfile;
use crate::core::domain::valueobject::role::Role;

pub struct Hall {
	admin: Role::Admin,
	tables: HashMap<u8, Table>,
}

impl Hall {
	pub fn new(admin: Role::Admin, tables: HashMap<u8, Table>) -> Self {
		Hall {
			admin,
			tables,
		}
	}

	pub fn matching(&mut self) {
		// Implement matching logic here
	}

	pub fn add_table(&mut self, id: u8, table: Table) {
		self.tables.insert(id, table);
	}

	pub fn get_table(&self, id: u8) -> Option<&Table> {
		self.tables.get(&id)
	}
}

pub struct Table {
	id: u8,
	user1: UserProfile,
	user2: UserProfile,
}

impl Table {
	pub fn new(id: u8, user1: UserProfile, user2: UserProfile) -> Self {
		Table {
			id,
			user1,
			user2,
		}
	}

	pub fn talk(&self) {
		println!("{} and {} are talking", self.user1, self.user2);
	}

	pub fn share_qr(&self) {
		println!("{} and {} are sharing QR code", self.user1, self.user2);
	}
}