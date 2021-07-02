pub mod buffer;
pub mod error;
pub mod event;
pub mod output;
pub mod result;
pub use buffer::BufferedSender;
pub use event::Event;
pub use output::tcp::TcpSender;
pub use result::Result;

pub type BufferedTCPSender = BufferedSender<TcpSender>;

pub trait Sender {
    fn send(&mut self, event: &Event) -> Result<()>;
    fn send_batch(&mut self, events: &[Event]) -> Result<()>;
    fn flush(&mut self) -> Result<()>;
}

mod prelude {
    pub use super::*;
    pub use crate::error::Error;
}
