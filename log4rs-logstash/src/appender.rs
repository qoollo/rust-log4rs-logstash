use anyhow::Result as AnyResult;
use log::Level as LogLevel;
use log::Record;
use log4rs::append::Append;
use std::time::Duration;

#[derive(Debug)]
pub struct Appender {}

#[derive(Debug)]
pub struct AppenderBuilder {
    level: LogLevel,
    hostname: String,
    port: u16,
    buffer_size: Option<usize>,
    buffer_lifetime: Option<Duration>,
    write_timeout: Option<Duration>,
    connection_timeout: Option<Duration>,
}

impl Default for AppenderBuilder {
    fn default() -> AppenderBuilder {
        AppenderBuilder {
            level: LogLevel::Warn,
            hostname: "127.0.0.1".to_string(),
            port: 5044,
            buffer_size: Some(1024),
            buffer_lifetime: Some(Duration::from_secs(1)),
            write_timeout: Some(Duration::from_secs(10)),
            connection_timeout: Some(Duration::from_secs(10)),
        }
    }
}

impl AppenderBuilder {
    /// Sets threshold for this logger to level.
    pub fn with_level(mut self, level: LogLevel) -> AppenderBuilder {
        self.level = level;
        self
    }

    /// Sets the hostname of the remote server.
    pub fn with_hostname(mut self, hostname: &str) -> AppenderBuilder {
        self.hostname = hostname.to_string();
        self
    }

    /// Sets the port of the remote server.
    pub fn with_port(mut self, port: u16) -> AppenderBuilder {
        self.port = port;
        self
    }

    /// Sets the upperbound limit on the number of records that can be placed in the buffer, once
    /// this size has been reached, the buffer will be sent to the remote server.
    pub fn with_buffer_size(mut self, buffer_size: Option<usize>) -> AppenderBuilder {
        self.buffer_size = buffer_size;
        self
    }

    /// Sets the maximum lifetime of the buffer before send it to the remote server.
    pub fn with_buffer_lifetime(mut self, buffer_duration: Option<Duration>) -> AppenderBuilder {
        self.buffer_lifetime = buffer_duration;
        self
    }

    /// Sets the timemout for write operation.
    pub fn with_write_timeout(mut self, timeout: Option<Duration>) -> AppenderBuilder {
        self.write_timeout = timeout;
        self
    }

    /// Sets the timeout for network connections.
    pub fn with_connection_timeout(mut self, timeout: Option<Duration>) -> AppenderBuilder {
        self.connection_timeout = timeout;
        self
    }

    /// Invoke the builder and return a [`Appender`](struct.Appender.html).
    pub fn build(self) -> AnyResult<Appender> {
        Ok(Appender {})
    }
}

impl Appender {
    pub fn builder() -> AppenderBuilder {
        AppenderBuilder::default()
    }
}

impl Append for Appender {
    fn append(&self, _record: &Record) -> AnyResult<()> {
        Ok(())
    }
    fn flush(&self) {}
}
