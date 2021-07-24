use crate::prelude::*;

#[derive(Debug)]
pub struct BufferedSender<S: Sender> {
    sender: S,
    buffer: Vec<Event>,
    buffer_size: Option<usize>,
}

impl<S: Sender> BufferedSender<S> {
    pub fn new(sender: S, buffer_size: Option<usize>) -> Self {
        Self {
            sender,
            buffer: Vec::with_capacity(buffer_size.unwrap_or(0)),
            buffer_size,
        }
    }
}

impl<S: Sender> Sender for BufferedSender<S> {
    fn send(&mut self, event: &Event) -> Result<()> {
        if let Some(max_size) = self.buffer_size {
            self.buffer.push(event.clone());
            if self.buffer.len() >= max_size {
                self.sender.send_batch(&self.buffer)?;
                self.buffer.clear();
            }
        } else {
            self.sender.send(event)?;
        }
        Ok(())
    }

    fn send_batch(&mut self, events: &[Event]) -> Result<()> {
        for event in events {
            self.send(event)?;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        if !self.buffer.is_empty() {
            self.sender.send_batch(&self.buffer)?;
        }
        self.sender.flush()
    }
}
