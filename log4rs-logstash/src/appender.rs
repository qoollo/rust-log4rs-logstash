use anyhow::Result as AnyResult;
use log::Level as LogLevel;
use log::Record;
use log4rs::append::Append;
use logstash_rs::LogStashRecord;
use logstash_rs::Sender;
use logstash_rs::{BufferedSender, TcpSender};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

pub struct Appender<S> {
    sender: S,
    extra_fields: HashMap<String, Value>,
}

impl<S> std::fmt::Debug for Appender<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::Appender", module_path!())
    }
}

#[derive(Debug)]
pub struct AppenderBuilder {
    hostname: String,
    port: u16,
    buffer_size: Option<usize>,
    buffer_lifetime: Option<Duration>,
    connection_timeout: Option<Duration>,
    ignore_buffer: LogLevel,
    use_tls: bool,
    error_period: Duration,
    extra_fields: HashMap<String, Value>,
    log_queue_len: usize,
}

impl Default for AppenderBuilder {
    fn default() -> AppenderBuilder {
        AppenderBuilder {
            hostname: "127.0.0.1".to_string(),
            port: 5044,
            buffer_size: Some(1024),
            buffer_lifetime: Some(Duration::from_secs(1)),
            connection_timeout: Some(Duration::from_secs(10)),
            use_tls: false,
            ignore_buffer: LogLevel::Error,
            error_period: Duration::from_secs(10),
            extra_fields: Default::default(),
            log_queue_len: 1000,
        }
    }
}

impl AppenderBuilder {
    /// Sets threshold for this logger to level.
    pub fn with_ignore_buffer_level(mut self, level: LogLevel) -> AppenderBuilder {
        self.ignore_buffer = level;
        self
    }

    /// Sets the hostname of the remote server.
    pub fn with_hostname(mut self, hostname: impl Into<String>) -> AppenderBuilder {
        self.hostname = hostname.into();
        self
    }

    /// Sets the port of the remote server.
    pub fn with_port(mut self, port: u16) -> AppenderBuilder {
        self.port = port;
        self
    }

    /// Sets the upperbound limit on the number of records that can be placed in the buffer, once
    /// this size has been reached, the buffer will be sent to the remote server.
    /// If buffer size is 0 or 1 then buffer is not used
    pub fn with_buffer_size(mut self, buffer_size: usize) -> AppenderBuilder {
        if buffer_size < 2 {
            self.buffer_size = None;
        } else {
            self.buffer_size = Some(buffer_size);
        }
        self
    }

    /// Sets the maximum lifetime of the buffer before send it to the remote server.
    pub fn with_buffer_lifetime(mut self, buffer_duration: Duration) -> AppenderBuilder {
        self.buffer_lifetime = Some(buffer_duration);
        self
    }

    /// Sets the timeout for network connections.
    pub fn with_connection_timeout(mut self, timeout: Duration) -> AppenderBuilder {
        self.connection_timeout = Some(timeout);
        self
    }

    /// Use tls connection.
    pub fn with_use_tls(mut self, use_tls: bool) -> AppenderBuilder {
        self.use_tls = use_tls;
        self
    }

    /// Print period for internal logstash errors.
    pub fn with_error_period(mut self, error_period: Duration) -> AppenderBuilder {
        self.error_period = error_period;
        self
    }

    /// Maximum length of log message queue
    pub fn with_log_queue_len(mut self, log_queue_len: usize) -> AppenderBuilder {
        self.log_queue_len = log_queue_len;
        self
    }

    /// Additional fields to send to logstash
    pub fn with_extra_fields(mut self, extra_fields: HashMap<String, Value>) -> AppenderBuilder {
        self.extra_fields = extra_fields;
        self
    }

    /// Invoke the builder and return a [`Appender`](struct.Appender.html).
    pub fn build(self) -> AnyResult<Appender<BufferedSender>> {
        Ok(Appender {
            sender: BufferedSender::new(
                TcpSender::new(
                    self.hostname,
                    self.port,
                    self.use_tls,
                    self.connection_timeout,
                ),
                self.buffer_size,
                self.buffer_lifetime,
                self.ignore_buffer,
                self.error_period,
                self.log_queue_len,
            ),
            extra_fields: self.extra_fields,
        })
    }
}

impl<S> Appender<S>
where
    S: Sender + Sync + Send + 'static,
{
    pub fn builder() -> AppenderBuilder {
        AppenderBuilder::default()
    }

    fn try_flush(&self) -> AnyResult<()> {
        self.sender.flush()?;
        Ok(())
    }
}

impl<S> Append for Appender<S>
where
    S: Sender + Sync + Send + 'static,
{
    fn append(&self, record: &Record) -> AnyResult<()> {
        self.sender
            .send(LogStashRecord::from_record(record).with_data_from_map(&self.extra_fields))?;
        Ok(())
    }
    fn flush(&self) {
        if let Err(err) = self.try_flush() {
            eprintln!("Logstash appender failed to flush: {}", err);
        }
    }
}
