use anyhow::Result as AnyResult;
use log::Level as LogLevel;
use log::Record;
use log4rs::append::Append;
use logstash_rs::LogStashRecord;
use logstash_rs::Sender;
use logstash_rs::{BufferedSender, TcpSender};
use std::time::Duration;

pub struct Appender<S>
where
    S: Sender,
{
    sender: S,
}

impl<S: Sender> std::fmt::Debug for Appender<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::Appender", module_path!())
    }
}

#[derive(Debug)]
pub struct AppenderBuilder {
    level: Option<LogLevel>,
    hostname: String,
    port: u16,
    buffer_size: Option<usize>,
    buffer_lifetime: Option<Duration>,
    write_timeout: Option<Duration>,
    connection_timeout: Option<Duration>,
    use_tls: bool,
}

impl Default for AppenderBuilder {
    fn default() -> AppenderBuilder {
        AppenderBuilder {
            level: None,
            hostname: "127.0.0.1".to_string(),
            port: 5044,
            buffer_size: Some(1024),
            buffer_lifetime: Some(Duration::from_secs(1)),
            write_timeout: Some(Duration::from_secs(10)),
            connection_timeout: Some(Duration::from_secs(10)),
            use_tls: false,
        }
    }
}

impl AppenderBuilder {
    /// Sets threshold for this logger to level.
    pub fn with_level(&mut self, level: Option<LogLevel>) -> &mut AppenderBuilder {
        self.level = level;
        self
    }

    /// Sets the hostname of the remote server.
    pub fn with_hostname(&mut self, hostname: &str) -> &mut AppenderBuilder {
        self.hostname = hostname.to_string();
        self
    }

    /// Sets the port of the remote server.
    pub fn with_port(&mut self, port: u16) -> &mut AppenderBuilder {
        self.port = port;
        self
    }

    /// Sets the upperbound limit on the number of records that can be placed in the buffer, once
    /// this size has been reached, the buffer will be sent to the remote server.
    pub fn with_buffer_size(&mut self, buffer_size: Option<usize>) -> &mut AppenderBuilder {
        self.buffer_size = buffer_size;
        self
    }

    /// Sets the maximum lifetime of the buffer before send it to the remote server.
    pub fn with_buffer_lifetime(
        &mut self,
        buffer_duration: Option<Duration>,
    ) -> &mut AppenderBuilder {
        self.buffer_lifetime = buffer_duration;
        self
    }

    /// Sets the timemout for write operation.
    pub fn with_write_timeout(&mut self, timeout: Option<Duration>) -> &mut AppenderBuilder {
        self.write_timeout = timeout;
        self
    }

    /// Sets the timeout for network connections.
    pub fn with_connection_timeout(&mut self, timeout: Option<Duration>) -> &mut AppenderBuilder {
        self.connection_timeout = timeout;
        self
    }

    /// Sets the timeout for network connections.
    pub fn with_use_tls(&mut self, use_tls: bool) -> &mut AppenderBuilder {
        self.use_tls = use_tls;
        self
    }

    /// Invoke the builder and return a [`Appender`](struct.Appender.html).
    pub fn build(&self) -> AnyResult<Appender<BufferedSender>> {
        Ok(Appender {
            sender: BufferedSender::new(
                TcpSender::new(self.hostname.clone(), self.port, self.use_tls),
                self.buffer_size,
                self.buffer_lifetime,
            ),
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
        self.sender.send(LogStashRecord::from_record(record))?;
        Ok(())
    }
    fn flush(&self) {
        match self.try_flush() {
            Err(err) => {
                println!("Logstash appender failed to flush: {}", err);
            }
            _ => {}
        }
    }
}
