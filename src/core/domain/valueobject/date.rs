use std::ops::Add;

use anyhow::{anyhow, Error};
use chrono::{DateTime, Duration, Local, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use tower_cookies::cookie::time::format_description::well_known::Rfc3339;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Timestamp {
    pub datetime: DateTime<Utc>,
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

impl Add<u64> for Timestamp {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        let duration = Duration::milliseconds(rhs as i64);
        let new_datetime = self.datetime + duration;
        Timestamp::new(new_datetime)
    }
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

    pub fn get_expire_time(&self, expire: u64) -> u64 {
        let current_time = Timestamp::now_utc();
        let expire_time = current_time + expire;
        expire_time.datetime.timestamp() as u64
    }

    pub fn to_unix_timestamp(&self) -> u64 {
        self.datetime.timestamp() as u64
    }
}

pub fn parse_utc(moment: &str) -> Result<OffsetDateTime, Error> {
    OffsetDateTime::parse(moment, &Rfc3339).map_err(|_| anyhow!("Error while parsing date-time!"))
}
