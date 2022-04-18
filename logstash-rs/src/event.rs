use chrono::{DateTime, Utc};
use log::Level;
use serde::Serialize;
use serde_json::Value;
use std::{collections::HashMap, time::SystemTime};

#[derive(Debug, Clone, Serialize)]
pub struct LogStashRecord {
    #[serde(rename = "@timestamp")]
    #[serde(with = "logstash_date_format")]
    pub timestamp: DateTime<Utc>,
    pub module: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub level: Level,
    pub target: String,
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

impl Default for LogStashRecord {
    fn default() -> Self {
        Self {
            timestamp: Utc::now(),
            module: Default::default(),
            file: Default::default(),
            line: Default::default(),
            level: Level::Warn,
            target: Default::default(),
            fields: Default::default(),
        }
    }
}

impl LogStashRecord {
    /// Initialize record with current time in `timestamp` field
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now(),
            ..Default::default()
        }
    }

    pub fn from_record(record: &log::Record) -> Self {
        let mut event = LogStashRecord::new();
        let meta = record.metadata();

        event.module = record.module_path().map(|p| p.into());
        event.file = record.file().map(|p| p.into());
        event.line = record.line();
        event.level = meta.level();
        event.target = meta.target().into();
        event.add_data("message", record.args().to_string().into());
        event
    }

    pub fn set_timestamp(&mut self, timestamp: SystemTime) -> &mut Self {
        self.timestamp = timestamp.into();
        self
    }

    pub fn add_metadata(&mut self, key: &str, value: Value) -> &mut Self {
        self.fields.insert(format!("@metadata.{}", key), value);
        self
    }

    pub fn add_data(&mut self, key: &str, value: Value) -> &mut Self {
        self.fields.insert(key.into(), value);
        self
    }
}

mod logstash_date_format {
    use chrono::{DateTime, Utc};
    use serde::{self, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        serializer.serialize_str(&s)
    }
}
