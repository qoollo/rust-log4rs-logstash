use log4rs::append::Append;
use log4rs::config::{Deserialize, Deserializers};

use crate::appender::AppenderBuilder;
use anyhow::Result as AnyResult;
use log::Level as LogLevel;
use std::time::Duration;

struct AppenderDeserializer;

#[derive(Debug, serde::Deserialize)]
pub struct AppenderConfig {
    level: Option<LogLevel>,
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
        let appender = AppenderBuilder::default()
            .with_level(config.level)
            .with_hostname(&config.hostname)
            .with_port(config.port)
            .with_buffer_size(config.buffer_size)
            .with_buffer_lifetime(config.buffer_lifetime)
            .with_write_timeout(config.write_timeout)
            .with_connection_timeout(config.connection_timeout)
            .with_use_tls(config.use_tls.unwrap_or(false))
            .build()?;

        Ok(Box::new(appender))
    }
}

pub fn deserializers() -> Deserializers {
    let mut d = Deserializers::default();
    d.insert("logstash", AppenderDeserializer);
    d
}
