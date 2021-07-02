use serde::Serialize;
use serde_json::Value;
use std::{collections::HashMap, time::SystemTime};

#[derive(Debug, Default, Clone, Serialize)]
pub struct Event {
    #[serde(rename = "@timestamp")]
    timestamp: Option<SystemTime>,
    #[serde(flatten)]
    fields: HashMap<String, Value>,
    #[serde(flatten)]
    metadata: HashMap<String, Value>,
}

impl Event {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_timestamp(&mut self, timestamp: Option<SystemTime>) -> &mut Self {
        self.timestamp = timestamp;
        self
    }

    pub fn with_metadata(&mut self, key: &str, value: Value) -> &mut Self {
        self.metadata.insert(format!("@metadata.{}", key), value);
        self
    }

    pub fn with_field(&mut self, key: &str, value: Value) -> &mut Self {
        self.fields.insert(key.into(), value);
        self
    }
}
