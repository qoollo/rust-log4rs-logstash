use crate::prelude::*;
use std::fmt::Write as FMTWrite;
use std::io::Write as IOWrite;
use std::net::TcpStream;
use std::sync::Mutex;

pub(crate) struct AdvancedTcpStream {
    hostname: String,
    port: u16,
    use_tls: bool,
    stream: Mutex<Option<Box<dyn IOWrite + Sync + Send>>>,
}

impl AdvancedTcpStream {
    pub(crate) fn new(hostname: String, port: u16, use_tls: bool) -> Self {
        Self {
            hostname,
            port,
            use_tls,
            stream: Mutex::new(None),
        }
    }

    pub(crate) fn send_bytes(&self, bytes: &[u8]) -> Result<()> {
        self.recreate_stream_if_needed()?;
        let mut stream = self
            .stream
            .try_lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock stream mutex"))?;
        if let Some(stream) = stream.as_deref_mut() {
            stream.write_all(bytes)?;
        }
        Ok(())
    }

    fn recreate_stream_if_needed(&self) -> Result<()> {
        let mut stream = self
            .stream
            .try_lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock stream mutex"))?;
        if stream.is_none() {
            *stream = Some(match self.use_tls {
                true => Box::new(self.create_connection()?),
                false => Box::new(self.create_tls_connection()?),
            });
        }
        Ok(())
    }

    fn create_connection(&self) -> Result<TcpStream> {
        Ok(TcpStream::connect((self.hostname.as_str(), self.port))?)
    }

    fn create_tls_connection(&self) -> Result<TcpStream> {
        Ok(TcpStream::connect((self.hostname.as_str(), self.port))?)
    }

    fn flush(&self) -> Result<()> {
        let mut stream = self
            .stream
            .try_lock()
            .map_err(|_| anyhow::anyhow!("Failed to lock stream mutex"))?;
        if let Some(stream) = stream.as_deref_mut() {
            stream.flush()?;
        }
        Ok(())
    }
}

pub struct TcpSender {
    stream: AdvancedTcpStream,
}

impl TcpSender {
    pub fn new(hostname: String, port: u16, use_tls: bool) -> Self {
        Self {
            stream: AdvancedTcpStream::new(hostname, port, use_tls),
        }
    }
}

impl Sender for TcpSender {
    fn send(&self, event: LogStashRecord) -> Result<()> {
        let mut event = serde_json::to_string(&event)?;
        event.write_char('\n')?;
        self.stream.send_bytes(event.as_bytes())?;
        Ok(())
    }

    fn send_batch(&self, events: Vec<LogStashRecord>) -> Result<()> {
        if events.is_empty() {
            return Ok(());
        }
        let mut buf = vec![];
        for event in events {
            serde_json::to_writer(&mut buf, &event)?;
            buf.push('\n' as u8);
        }
        self.stream.send_bytes(&buf)?;
        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.stream.flush()?;
        Ok(())
    }
}

impl log::Log for TcpSender {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let record = LogStashRecord::from_record(record);
        let _ = self.send(record);
    }

    fn flush(&self) {
        let _ = Sender::flush(self);
    }
}
