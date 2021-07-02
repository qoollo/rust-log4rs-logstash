use std::time::Duration;

use crate::{event::Event, Sender};

struct BufferedSender<S: Sender> {
    sender: S,
    buffer: Vec<Event>,
    buffer_size: Option<usize>,
    buffer_lifetime: Option<Duration>,
}

impl<S: Sender> BufferedSender<S> {
    pub fn new(sender: S, buffer_size: Option<usize>, buffer_lifetime: Option<Duration>) -> Self {
        Self {
            sender,
            buffer: vec![],
            buffer_size,
            buffer_lifetime,
        }
    }
}

impl<S: Sender> Sender for BufferedSender<S> {
    fn send(&mut self, event: &Event) -> anyhow::Result<()> {
        self.sender.send(event)
    }

    fn send_batch(&mut self, events: &[Event]) -> anyhow::Result<()> {
        self.sender.send_batch(events)
    }
}
