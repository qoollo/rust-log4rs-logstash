use log4rs::append::Append;
use log4rs::config::{Deserialize, Deserializers};

use crate::appender::AppenderBuilder;
use anyhow::Result as AnyResult;
use log::Level as LogLevel;
use std::time::Duration;

struct AppenderDeserializer;

#[derive(Debug, serde::Deserialize)]
pub struct AppenderConfig {
    level: LogLevel,
    ignore_buffer_level: Option<LogLevel>,
    hostname: String,
    port: u16,
    buffer_size: Option<usize>,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    buffer_lifetime: Option<Duration>,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    write_timeout: Option<Duration>,
    #[serde(default)]
    #[serde(with = "humantime_serde")]
    connection_timeout: Option<Duration>,
    use_tls: Option<bool>,
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
        builder
            .with_level(config.level)
            .with_hostname(&config.hostname)
            .with_port(config.port)
            .with_use_tls(config.use_tls.unwrap_or(false));
        if let Some(buffer_size) = config.buffer_size {
            builder.with_buffer_size(buffer_size);
        }
        if let Some(buffer_lifetime) = config.buffer_lifetime {
            builder.with_buffer_lifetime(buffer_lifetime);
        }
        if let Some(write_timeout) = config.write_timeout {
            builder.with_write_timeout(write_timeout);
        }
        if let Some(connection_timeout) = config.connection_timeout {
            builder.with_connection_timeout(connection_timeout);
        }
        if let Some(ignore_level) = config.ignore_buffer_level {
            builder.with_ignore_buffer_level(ignore_level);
        }
        let appender = builder.build()?;

        Ok(Box::new(appender))
    }
}

pub fn deserializers() -> Deserializers {
    let mut d = Deserializers::default();
    d.insert("logstash", AppenderDeserializer);
    d
}
