use chrono::{DateTime, Utc};
use log::Level;
use serde::Serialize;
use serde_json::Value;
use std::{collections::HashMap, time::SystemTime};

#[derive(Debug, Default, Clone, Serialize)]
pub struct LogStashRecord {
    #[serde(rename = "@timestamp")]
    #[serde(with = "logstash_date_format")]
    timestamp: Option<DateTime<Utc>>,
    module: Option<String>,
    file: Option<String>,
    line: Option<u32>,
    level: Option<Level>,
    target: String,
    #[serde(flatten)]
    fields: HashMap<String, Value>,
}

impl LogStashRecord {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_with_time_now() -> Self {
        Self {
            timestamp: Some(Utc::now()),
            ..Default::default()
        }
    }

    pub fn from_record(record: &log::Record) -> Self {
        let mut event = LogStashRecord::new_with_time_now();
        let meta = record.metadata();

        event.module = record.module_path().map(|p| p.into());
        event.file = record.file().map(|p| p.into());
        event.line = record.line();
        event.level = Some(meta.level());
        event.target = meta.target().into();
        event.add_data("message", record.args().to_string().into());
        event
    }

    pub fn set_timestamp(&mut self, timestamp: Option<SystemTime>) -> &mut Self {
        self.timestamp = timestamp.map(|t| t.into());
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

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(date) = date {
            let s = date.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_none()
        }
    }
}
