use log4rs::append::Append;
use log4rs::config::{Deserialize, Deserializers};
use serde_json::Value;

use crate::appender::AppenderBuilder;
use anyhow::Result as AnyResult;
use log::Level as LogLevel;
use std::collections::HashMap;
use std::time::Duration;

struct AppenderDeserializer {
    extra_fields: Option<HashMap<String, Value>>
}

pub trait DeserializersExt {
    /// Register logstash deserializer
    fn with_logstash(self) -> Self;
    /// Register logstash deserializer with additional extra fields
    fn with_logstash_extra(self, extra_fields: HashMap<String, Value>) -> Self;
}

#[derive(Debug, serde::Deserialize)]
pub struct AppenderConfig {
    ignore_buffer_level: Option<LogLevel>,
    hostname: String,
    port: u16,
    buffer_size: Option<usize>,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    buffer_lifetime: Option<Duration>,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    connection_timeout: Option<Duration>,
    use_tls: Option<bool>,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    error_period: Option<Duration>,
    extra_fields: Option<HashMap<String, Value>>,
    log_queue_len: Option<usize>,
}

impl AppenderDeserializer {
    fn new(extra_fields: Option<HashMap<String, Value>>) -> Self {
        Self {
            extra_fields
        }
    }
}

impl Default for AppenderDeserializer {
    fn default() -> Self {
        Self {
            extra_fields: None
        }
    }
}

impl Deserialize for AppenderDeserializer {
    type Trait = dyn Append;
    type Config = AppenderConfig;

    fn deserialize(
        &self,
        config: Self::Config,
        _deserializers: &Deserializers,
    ) -> AnyResult<Box<Self::Trait>> {
        let mut builder = AppenderBuilder::default();
        builder = builder
            .with_hostname(&config.hostname)
            .with_port(config.port)
            .with_use_tls(config.use_tls.unwrap_or(false));
        if let Some(buffer_size) = config.buffer_size {
            builder = builder.with_buffer_size(buffer_size);
        }
        if let Some(buffer_lifetime) = config.buffer_lifetime {
            builder = builder.with_buffer_lifetime(buffer_lifetime);
        }
        if let Some(connection_timeout) = config.connection_timeout {
            builder = builder.with_connection_timeout(connection_timeout);
        }
        if let Some(ignore_level) = config.ignore_buffer_level {
            builder = builder.with_ignore_buffer_level(ignore_level);
        }
        if let Some(error_period) = config.error_period {
            builder = builder.with_error_period(error_period);
        }
        if let Some(log_queue_len) = config.log_queue_len {
            builder = builder.with_log_queue_len(log_queue_len);
        }

        let mut extra_fields = self.extra_fields.clone().unwrap_or_default();
        if let Some(config_extra_fields) = config.extra_fields {
            extra_fields.extend(config_extra_fields);   
        }

        builder = builder.with_extra_fields(extra_fields);

        let appender = builder.build()?;

        Ok(Box::new(appender))
    }
}

/// Returns default Deserializers extended with logstash appender
pub fn deserializers() -> Deserializers {
    let mut d = Deserializers::default();
    register_deserializer(&mut d, None);
    d
}

/// Register deserializer for logstash appender
pub fn register_deserializer(deserializers: &mut Deserializers, extra_fields: Option<HashMap<String, Value>>) {
    deserializers.insert("logstash", AppenderDeserializer::new(extra_fields));
}

impl DeserializersExt for Deserializers {
    fn with_logstash(mut self) -> Self {
        register_deserializer(&mut self, None);
        self
    }

    fn with_logstash_extra(mut self, extra_fields: HashMap<String, Value>) -> Self {
        register_deserializer(&mut self, Some(extra_fields));
        self
    }
}
