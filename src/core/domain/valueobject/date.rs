use chrono::{DateTime, Local, Utc};
use sqlx::types::time::OffsetDateTime;

pub struct DateService;

impl DateService {
	pub fn get_current_timestamp_utc() -> DateTime<Utc> {
		Local::now().to_utc()
	}

	pub fn get_current_timestamp() -> DateTime<Local> {
		Local::now()
	}

	pub fn convert_to_offset(datetime: DateTime<Local>) -> OffsetDateTime {
		datetime.naive_utc()?
			.and_time(datetime.time())
			.unwrap()
			.to_offset(Utc::now().offset().unwrap())
	}
}