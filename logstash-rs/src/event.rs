use chrono::{DateTime, Local, Utc};
use serde::Serialize;
use serde_json::Value;
use std::{collections::HashMap, time::SystemTime};

#[derive(Debug, Default, Clone, Serialize)]
pub struct Event {
    #[serde(rename = "@timestamp")]
    #[serde(with = "my_date_format")]
    timestamp: Option<DateTime<Utc>>,
    #[serde(flatten)]
    fields: HashMap<String, Value>,
    #[serde(flatten)]
    metadata: HashMap<String, Value>,
}

impl Event {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_with_time_now() -> Self {
        Self {
            timestamp: Some(Utc::now()),
            ..Default::default()
        }
    }

    pub fn set_timestamp(&mut self, timestamp: Option<SystemTime>) -> &mut Self {
        self.timestamp = timestamp.map(|t| t.into());
        self
    }

    pub fn set_metadata(&mut self, key: &str, value: Value) -> &mut Self {
        self.metadata.insert(format!("@metadata.{}", key), value);
        self
    }

    pub fn set_field(&mut self, key: &str, value: Value) -> &mut Self {
        self.fields.insert(key.into(), value);
        self
    }
}

mod my_date_format {
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
