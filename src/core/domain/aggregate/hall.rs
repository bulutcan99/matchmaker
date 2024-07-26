use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

use crate::core::domain::aggregate::employment::Employment;
use crate::core::domain::aggregate::table::Table;

//todo: masalara oturma logic'i daha temiz ve dusunulerek ilerlenicek
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Hall {
    tables: HashMap<u8, Table>,
    workers: Vec<Employment>,
    last_match_time: SystemTime,
    end_time: SystemTime,
    matching_active: bool,
    meeting_counter: u8,
    break_time: bool,
}

impl Hall {
    pub fn new() -> Self {
        let now = SystemTime::now();
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

    pub fn add_worker(&mut self, worker: Employment) {
        self.workers.push(worker);
    }

    pub fn add_table(&mut self, id: u8) {
        self.tables.insert(id, Table::new(id));
    }

    pub fn get_tables(&self) -> HashMap<u8, Table> {
        self.tables.clone()
    }

    /*
    pub fn start_matching(&mut self) -> Option<HashMap<u8, Table>> {
        use SliceRandom;
        use thread_rng;

        let mut rng = thread_rng();
        let mut workers = self.workers.clone();
        println!("LEN: {}", workers.len());
        workers.shuffle(&mut rng);

        let mut table_ids: Vec<u8> = self.tables.keys().cloned().collect();
        table_ids.shuffle(&mut rng);

        let mut table_iter = table_ids.iter();
        let mut seated_workers: HashSet<u16> = HashSet::new();

        for table_id in table_iter {
            if let Some(table) = self.tables.get_mut(table_id) {
                // Add workers to the current table until the table is full or no workers left
                let mut i = 0;
                while i < workers.len() {
                    let worker = workers.remove(0); // Get the first worker
                    if !seated_workers.contains(&worker.user.id.unwrap()) {
                        table.add_worker(worker.clone());
                        seated_workers.insert(worker.user.id.unwrap()); // Mark this worker as seated
                    } else {
                        // Re-add the worker to the end of the list if they were not seated
                        workers.push(worker);
                    }
                    i += 1;
                    if table.is_full() { // Assuming Table has an `is_full` method
                        break;
                    }
                }
            }
        }

        self.last_match_time = SystemTime::now();
        Some(self.tables.clone())
    }
    pub fn check_and_update(&mut self) {
        let now = SystemTime::now();
        if self.break_time {
            if now.duration_since(self.last_match_time).unwrap_or(Duration::from_secs(0)) >= Duration::from_secs(10) {
                self.break_time = false;
                println!("Break time is over. Starting new matching.");
                self.start_matching();
            }
        } else if now.duration_since(self.last_match_time).unwrap_or(Duration::from_secs(0)) >= Duration::from_secs(20) {
            self.meeting_counter += 1;
            if self.meeting_counter % 2 == 0 {
                println!("Meeting interval. Break time for 1 minute.");
                self.break_time = true;
                self.last_match_time = SystemTime::now();
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
    */
}
