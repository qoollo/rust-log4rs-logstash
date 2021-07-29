use crate::prelude::*;
use std::{
    sync::mpsc,
    time::{Duration, Instant},
};

enum Command {
    Send(LogStashRecord),
    SendBatch(Vec<LogStashRecord>),
    Flush,
}

pub struct BufferedSender {
    sender: mpsc::SyncSender<Command>,
}

impl BufferedSender {
    pub fn new<S: Sender>(
        sender: S,
        buffer_size: Option<usize>,
        buffer_lifetime: Option<Duration>,
    ) -> Self {
        let sender = BufferedSenderThread::new(sender, buffer_size, buffer_lifetime).run();
        Self { sender }
    }
}

impl Sender for BufferedSender {
    fn send(&self, event: LogStashRecord) -> Result<()> {
        self.sender
            .send(Command::Send(event))
            .map_err(|_| anyhow::anyhow!("Failed to send record to channel"))?;
        Ok(())
    }

    fn send_batch(&self, events: Vec<LogStashRecord>) -> Result<()> {
        self.sender
            .send(Command::SendBatch(events))
            .map_err(|_| anyhow::anyhow!("Failed to send record to channel"))?;
        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.sender
            .send(Command::Flush)
            .map_err(|_| anyhow::anyhow!("Failed to send record to channel"))?;
        Ok(())
    }
}

#[derive(Debug)]
struct BufferedSenderThread<S: Sender> {
    sender: S,
    buffer: Vec<LogStashRecord>,
    buffer_size: Option<usize>,
    buffer_lifetime: Option<Duration>,
}

impl<S: Sender> BufferedSenderThread<S> {
    fn new(sender: S, buffer_size: Option<usize>, buffer_lifetime: Option<Duration>) -> Self {
        let buffer_size = match buffer_size {
            Some(s) if s < 2 => None,
            x => x,
        };
        Self {
            sender,
            buffer: Vec::with_capacity(buffer_size.unwrap_or(0)),
            buffer_size,
            buffer_lifetime,
        }
    }

    fn run(self) -> mpsc::SyncSender<Command> {
        let (sender, reciever) = mpsc::sync_channel(1);
        self.run_thread(reciever);
        sender
    }

    fn run_thread(mut self, reciever: mpsc::Receiver<Command>) {
        std::thread::spawn(move || {
            let mut deadline: Option<Instant> = None;
            loop {
                let cmd = match deadline {
                    Some(deadline) => {
                        reciever.recv_timeout(deadline.saturating_duration_since(Instant::now()))
                    }
                    None => reciever
                        .recv()
                        .map_err(|_| mpsc::RecvTimeoutError::Disconnected),
                };

                match (&cmd, self.buffer_lifetime) {
                    (Ok(Command::SendBatch(_) | Command::Send(_)), Some(lifetime))
                        if self.buffer.is_empty() && self.buffer_size.is_some() =>
                    {
                        deadline = Some(Instant::now() + lifetime);
                    }
                    _ => {}
                }
                match cmd {
                    Ok(Command::Flush) | Err(mpsc::RecvTimeoutError::Timeout) => {
                        self.flush();
                        deadline = None;
                    }
                    Ok(Command::Send(event)) => self.send(event),
                    Ok(Command::SendBatch(events)) => self.send_batch(events),
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
        });
    }

    fn send(&mut self, event: LogStashRecord) {
        if let Some(max_size) = self.buffer_size {
            self.buffer.push(event);
            if self.buffer.len() >= max_size {
                self.flush();
            }
        } else {
            let _ = self.sender.send(event);
        }
    }

    fn send_batch(&mut self, events: Vec<LogStashRecord>) {
        for event in events {
            self.send(event);
        }
    }

    fn flush(&mut self) {
        if !self.buffer.is_empty() {
            let buffer = std::mem::replace(&mut self.buffer, vec![]);
            let _ = self.sender.send_batch(buffer);
        }
        let _ = self.sender.flush();
    }
}

impl log::Log for BufferedSender {
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
