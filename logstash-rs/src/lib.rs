pub mod buffer;
pub mod error;
pub mod event;
pub mod output;
pub use buffer::BufferedSender;
pub use error::Error;
pub use event::LogStashRecord;
pub use output::tcp::TcpSender;

pub type Result<T> = core::result::Result<T, Error>;

pub trait Sender: Sync + Send + 'static {
    fn send(&self, event: LogStashRecord) -> Result<()>;
    fn send_batch(&self, events: Vec<LogStashRecord>) -> Result<()>;
    fn flush(&self) -> Result<()>;
}

mod prelude {
    pub use super::*;
}
