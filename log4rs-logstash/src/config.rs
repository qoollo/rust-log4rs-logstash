use log4rs::append::Append;
use log4rs::config::{Deserialize, Deserializers};
use serde_json::Value;

use crate::appender::AppenderBuilder;
use anyhow::Result as AnyResult;
use log::Level as LogLevel;
use std::collections::HashMap;
use std::time::Duration;

struct AppenderDeserializer;

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
        builder = builder.with_extra_fields(config.extra_fields.unwrap_or_default());
        let appender = builder.build()?;

        Ok(Box::new(appender))
    }
}

pub fn deserializers() -> Deserializers {
    let mut d = Deserializers::default();
    d.insert("logstash", AppenderDeserializer);
    d
}
