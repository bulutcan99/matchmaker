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
        OffsetDateTime::from_unix_timestamp(self.datetime.timestamp()).unwrap()
    }
}

impl From<OffsetDateTime> for Timestamp {
    fn from(odt: OffsetDateTime) -> Self {
        let dt = Utc::now();
        let naive_utc = dt.naive_utc();
        let offset = dt.offset().clone();
        let dt_new = DateTime::<Utc>::from_naive_utc_and_offset(naive_utc, offset);
        Self { datetime: dt_new }
    }
}
