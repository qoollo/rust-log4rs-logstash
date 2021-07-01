pub mod event;
pub mod output;
use anyhow::Result as AnyResult;
use event::Event;

trait Sender {
    fn send(&mut self, event: &Event) -> AnyResult<()>;
    fn send_batch(&mut self, event: &[Event]) -> AnyResult<()>;
}
