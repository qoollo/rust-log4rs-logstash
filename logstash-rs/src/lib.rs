pub mod buffer;
pub mod error;
pub mod event;
pub mod output;
pub use anyhow::Result;
pub use buffer::BufferedSender;
pub use event::LogStashRecord;
pub use output::tcp::TcpSender;

pub trait Sender: Sync + Send + 'static {
    fn send(&self, event: LogStashRecord) -> Result<()>;
    fn send_batch(&self, events: Vec<LogStashRecord>) -> Result<()>;
    fn flush(&self) -> Result<()>;
}

mod prelude {
    pub use super::*;
    pub use crate::error::Error;
}
