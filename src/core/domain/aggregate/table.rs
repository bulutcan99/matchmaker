use serde::{Deserialize, Serialize};

use crate::core::domain::aggregate::employment::Employment;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Table {
    id: u8,
    worker1: Option<Employment>,
    worker2: Option<Employment>,
}

impl Table {
    pub fn new(id: u8) -> Self {
        Table {
            id,
            worker1: None,
            worker2: None,
        }
    }

    pub fn add_worker(&mut self, worker: Employment) {
        worker.sit_table(self.id);
        if self.worker1.is_none() {
            self.worker1 = Some(worker);
            println!("Worker1 set");
        } else if self.worker2.is_none() {
            self.worker2 = Some(worker);
            println!("Worker2 set");
        } else {
            println!("No more workers can be added to this table.");
        }
    }

    fn worker_to_string(worker: &Option<Employment>) -> String {
        match worker {
            Some(u) => u.to_string(),
            None => "Unknown User".to_string(),
        }
    }

    pub fn talk(&self) {
        let worker1_str = Self::worker_to_string(&self.worker1);
        let worker2_str = Self::worker_to_string(&self.worker2);

        println!("{} and {} are talking", worker1_str, worker2_str);
    }

    pub fn share_qr(&self) {
        let worker1_str = Self::worker_to_string(&self.worker1);
        let worker2_str = Self::worker_to_string(&self.worker2);

        println!("{} and {} are sharing qr", worker1_str, worker2_str);
    }

    pub fn is_full(&self) -> bool {
        self.worker1.is_some() && self.worker2.is_some()
    }
}
