use std::collections::HashMap;
use std::time::{Duration, Instant};

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::core::domain::aggregate::user_profile::UserProfile;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Table {
	id: u8,
	user1: Option<UserProfile>,
	user2: Option<UserProfile>,
}

impl Table {
	pub fn new(id: u8) -> Self {
		Table {
			id,
			user1: None,
			user2: None,
		}
	}

	pub fn add_user(&mut self, user: UserProfile) {
		if self.user1.is_none() {
			self.user1 = Some(user);
		} else if self.user2.is_none() {
			self.user2 = Some(user);
		}
	}

	pub fn talk(&self) {
		let user1_str = match &self.user1 {
			Some(user) => user.to_string(),
			None => "Unknown User1".to_string(),
		};

		let user2_str = match &self.user2 {
			Some(user) => user.to_string(),
			None => "Unknown User2".to_string(),
		};

		println!("{} and {} are talking", user1_str, user2_str);
	}

	pub fn share_qr(&self) {
		let user1_str = match &self.user1 {
			Some(user) => user.to_string(),
			None => "Unknown User1".to_string(),
		};

		let user2_str = match &self.user2 {
			Some(user) => user.to_string(),
			None => "Unknown User2".to_string(),
		};

		println!("{} and {} are sharing qr", user1_str, user2_str);
	}
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Hall {
	admin: UserProfile,
	tables: HashMap<u8, Table>,
	users: Vec<UserProfile>,
	last_match_time: Instant,
}

impl Hall {
	pub fn new(admin: UserProfile) -> Self {
		Hall {
			admin,
			tables: HashMap::new(),
			users: Vec::new(),
			last_match_time: Instant::now(),
		}
	}

	pub fn add_user(&mut self, user: UserProfile) {
		self.users.push(user);
	}

	pub fn add_table(&mut self, id: u8) {
		self.tables.insert(id, Table::new(id));
	}

	pub fn start_matching(&mut self) {
		let mut rng = rand::thread_rng();
		let mut users = self.users.clone();
		users.shuffle(&mut rng);

		let mut table_ids: Vec<u8> = self.tables.keys().cloned().collect();
		table_ids.shuffle(&mut rng);

		let mut table_iter = table_ids.iter();
		for user in users {
			if let Some(table_id) = table_iter.next() {
				if let Some(table) = self.tables.get_mut(table_id) {
					table.add_user(user);
				}
			}
		}
		self.last_match_time = Instant::now();
	}

	pub fn check_and_update(&mut self) {
		if Instant::now().duration_since(self.last_match_time) >= Duration::from_secs(120) {
			self.start_matching(); // Start matching again if 2 minutes have passed
		}
	}
}
