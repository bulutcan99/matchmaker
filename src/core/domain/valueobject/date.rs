use chrono::{DateTime, Local, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Timestamp {
    datetime: DateTime<Utc>,
}

impl Timestamp {
    pub fn new(datetime: DateTime<Utc>) -> Self {
        Self { datetime }
    }

    pub fn now_utc() -> Self {
        Self {
            datetime: Utc::now(),
        }
    }

    pub fn to_local(&self) -> DateTime<Local> {
        self.datetime.with_timezone(&Local)
    }

    pub fn to_naive(&self) -> NaiveDateTime {
        self.datetime.naive_utc()
    }

    pub fn convert_to_offset(&self) -> OffsetDateTime {
        self.datetime
            .naive_utc()?
            .and_time(self.datetime.time())
            .unwrap()
            .to_offset(Utc::now().offset().unwrap())
    }
}
