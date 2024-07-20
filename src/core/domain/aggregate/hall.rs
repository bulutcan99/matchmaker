use std::collections::HashMap;
use std::time::{Duration, Instant};

use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use crate::core::domain::aggregate::table::Table;
use crate::core::domain::aggregate::worker::Worker;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Hall {
	tables: HashMap<u8, Table>,
	workers: Vec<Worker>,
	last_match_time: Instant,
	end_time: Instant,
	matching_active: bool,
	meeting_counter: u8,
	break_time: bool,
}

impl Hall {
	pub fn new() -> Self {
		let now = Instant::now();
		Hall {
			tables: HashMap::new(),
			workers: Vec::new(),
			last_match_time: now,
			end_time: now + Duration::from_secs(300),
			matching_active: false,
			meeting_counter: 0,
			break_time: false,
		}
	}

	pub fn add_worker(&mut self, worker: Worker) {
		self.workers.push(worker);
	}

	pub fn add_table(&mut self, id: u8) {
		self.tables.insert(id, Table::new(id));
	}

	pub fn get_tables(&self) -> HashMap<u8, Table> {
		self.tables.clone()
	}

	pub fn start_matching(&mut self) -> Option<HashMap<u8, Table>> {
		let mut rng = thread_rng();
		// let mut users = self.workers.iter().map(|w| w.user.clone()).collect::<Vec<_>>();
		let mut workers = self.workers.iter().cloned().collect();
		workers.shuffle(&mut rng);

		let mut table_ids: Vec<u8> = self.tables.keys().cloned().collect();
		table_ids.shuffle(&mut rng);

		let mut table_iter = table_ids.iter();
		for worker in workers {
			if let Some(table_id) = table_iter.next() {
				if let Some(table) = self.tables.get_mut(table_id) {
					table.add_worker(worker);
				}
			}
		}
		self.last_match_time = Instant::now();
		Some(self.tables.clone())
	}

	pub fn check_and_update(&mut self) {
		let now = Instant::now();
		if self.break_time {
			if now.duration_since(self.last_match_time) >= Duration::from_secs(60) {
				self.break_time = false;
				println!("Break time is over. Starting new matching.");
				self.start_matching();
			}
		} else if now.duration_since(self.last_match_time) >= Duration::from_secs(120) {
			self.meeting_counter += 1;
			if self.meeting_counter % 2 == 0 {
				println!("Meeting interval. Break time for 1 minute.");
				self.break_time = true;
				self.last_match_time = Instant::now();
			} else {
				self.start_matching();
			}
		}

		if now >= self.end_time {
			self.time_is_up();
		}
	}

	fn time_is_up(&self) {
		println!("Time is up. Thanks for coming.");
	}
}