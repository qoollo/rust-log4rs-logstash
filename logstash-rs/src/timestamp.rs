use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Serialize, Serializer};
use std::time::SystemTime;

const DEFAULT_TIME_PRECISION: TimePrecision = TimePrecision::Millis;

#[derive(Debug, Clone)]
pub struct Timestamp {
    date_time: DateTime<Utc>,
    precision: TimePrecision,
}

impl Timestamp {
    pub fn new(date_time: DateTime<Utc>, precision: TimePrecision) -> Self {
        Self {
            date_time,
            precision,
        }
    }

    pub fn now() -> Self {
        Self {
            date_time: Utc::now(),
            precision: DEFAULT_TIME_PRECISION,
        }
    }

    pub fn now_with_precision(precision: TimePrecision) -> Self {
        Self {
            date_time: Utc::now(),
            precision,
        }
    }

    pub fn set_date_time(&mut self, date_time: DateTime<Utc>) {
        self.date_time = date_time;
    }

    pub fn set_precision(&mut self, precision: TimePrecision) {
        self.precision = precision;
    }
}

impl From<SystemTime> for Timestamp {
    fn from(value: SystemTime) -> Self {
        let datetime = DateTime::from(value);
        Timestamp::new(datetime, DEFAULT_TIME_PRECISION)
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(value: DateTime<Utc>) -> Self {
        Timestamp::new(value, DEFAULT_TIME_PRECISION)
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let precision = self.precision.into();
        let date_time_string = self.date_time.to_rfc3339_opts(precision, true);

        serializer.serialize_str(&date_time_string)
    }
}

#[derive(Debug, serde::Deserialize, Copy, Clone)]
pub enum TimePrecision {
    /// Format whole seconds only, with no decimal point nor subseconds.
    Secs,
    /// Use fixed 3 subsecond digits.
    Millis,
    /// Use fixed 6 subsecond digits.
    Micros,
    /// Use fixed 9 subsecond digits.
    Nanos,
}

impl From<TimePrecision> for SecondsFormat {
    fn from(val: TimePrecision) -> Self {
        match val {
            TimePrecision::Secs => SecondsFormat::Secs,
            TimePrecision::Millis => SecondsFormat::Millis,
            TimePrecision::Micros => SecondsFormat::Micros,
            TimePrecision::Nanos => SecondsFormat::Nanos,
        }
    }
}
