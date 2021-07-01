use std::net::TcpStream;

use crate::{event::Event, Sender};

struct TcpSender {
    hostname: String,
    port: u16,
    stream: Option<TcpStream>,
}

impl TcpSender {
    pub fn new(hostname: String, port: u16) -> Self {
        Self {
            hostname,
            port,
            stream: None,
        }
    }
}

impl Sender for TcpSender {
    fn send(&mut self, event: &Event) -> anyhow::Result<()> {
        let stream = if let Some(stream) = &self.stream {
            stream
        } else {
            let stream = TcpStream::connect((self.hostname.as_str(), self.port))?;
            self.stream = Some(stream);
            self.stream.as_mut().unwrap()
        };
        Ok(())
    }

    fn send_batch(&mut self, event: &[Event]) -> anyhow::Result<()> {
        todo!();
        Ok(())
    }
}
