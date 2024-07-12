use chrono::{DateTime,Local,Utc};

pub struct DateService;

impl DateService{
	pub fn get_current_timestamp_utc() -> DateTime<Utc>{
		Local::now().to_utc()
	}

	pub fn get_current_timestamp() -> DateTime<Local>{
		Local::now()
	}
}