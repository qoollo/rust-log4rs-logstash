use log::Level;

use crate::prelude::*;
use std::{
    sync::mpsc::{self, TrySendError},
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
pub(crate) enum Command {
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
        ignore_buffer: Level,
        error_period: Duration,
        log_queue_len: usize,
    ) -> Self {
        let sender = BufferedSenderThread::new(
            sender,
            buffer_size,
            buffer_lifetime,
            ignore_buffer,
            error_period,
            log_queue_len,
        )
        .run();
        Self { sender }
    }
}

impl Sender for BufferedSender {
    fn send(&self, event: LogStashRecord) -> Result<()> {
        let important = event.level <= Level::Warn;
        let result = self.sender.try_send(Command::Send(event));
        process_result(result, important)
    }

    fn send_batch(&self, events: Vec<LogStashRecord>) -> Result<()> {
        let important = events.iter().any(|e| e.level <= Level::Warn);
        let result = self.sender.try_send(Command::SendBatch(events));
        process_result(result, important)
    }

    fn flush(&self) -> Result<()> {
        let result = self.sender.try_send(Command::Flush);
        process_result(result, false)
    }
}

fn process_result<T>(r: std::result::Result<(), TrySendError<T>>, log_full: bool) -> Result<()> {
    match r {
        Err(TrySendError::Disconnected(..)) => {
            Err(Error::SenderThreadStopped(r.unwrap_err().to_string()))
        }
        Err(TrySendError::Full(..)) if log_full => Err(Error::BufferFull()),
        _ => Ok(()),
    }
}

#[derive(Debug)]
struct BufferedSenderThread<S: Sender> {
    sender: S,
    buffer: Vec<LogStashRecord>,
    buffer_size: Option<usize>,
    buffer_lifetime: Option<Duration>,
    deadline: Option<Instant>,
    ignore_buffer: Level,
    error_period: Duration,
    log_queue_len: usize,
}

impl<S: Sender> BufferedSenderThread<S> {
    fn new(
        sender: S,
        buffer_size: Option<usize>,
        buffer_lifetime: Option<Duration>,
        ignore_buffer: Level,
        error_period: Duration,
        log_queue_len: usize,
    ) -> Self {
        Self {
            sender,
            buffer: Vec::with_capacity(buffer_size.unwrap_or(0)),
            buffer_size,
            buffer_lifetime,
            deadline: None,
            ignore_buffer,
            error_period,
            log_queue_len,
        }
    }

    fn run(self) -> mpsc::SyncSender<Command> {
        let (sender, receiver) = mpsc::sync_channel(self.log_queue_len);
        self.run_thread(receiver);
        sender
    }

    fn next_deadline(&self) -> Option<Instant> {
        if self.buffer.is_empty() && self.buffer_size.is_some() {
            return self.buffer_lifetime.map(|lt| Instant::now() + lt);
        }
        None
    }

    fn run_thread(mut self, receiver: mpsc::Receiver<Command>) {
        std::thread::spawn::<_, Result<()>>(move || {
            {
                let mut last_error: Option<Instant> = None;
                loop {
                    let cmd = match self.deadline {
                        Some(deadline) => receiver
                            .recv_timeout(deadline.saturating_duration_since(Instant::now())),
                        None => receiver
                            .recv()
                            .map_err(|_| mpsc::RecvTimeoutError::Disconnected),
                    };

                    if let Ok(Command::SendBatch(_) | Command::Send(_)) = &cmd {
                        self.deadline = self.next_deadline();
                    }

                    match cmd {
                        Ok(Command::Flush) | Err(mpsc::RecvTimeoutError::Timeout) => self.flush(),
                        Ok(Command::Send(event)) => self.send(event),
                        Ok(Command::SendBatch(events)) => self.send_batch(events),
                        Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    }
                    .or_else(|err| {
                        if last_error
                            .as_ref()
                            .map(|x| x.elapsed() > self.error_period)
                            .unwrap_or(true)
                        {
                            println!("logstash logger error: {}", err);
                            last_error = Some(Instant::now());
                        }
                        if matches!(
                            err,
                            Error::FatalInternal(..) | Error::SenderThreadStopped(..)
                        ) {
                            Result::Err(err)
                        } else {
                            Result::Ok(())
                        }
                    })?;
                }
                Ok(())
            }
            .map_err(|err| {
                println!("fatal logger error: {}", err);
                err
            })
        });
    }

    fn send(&mut self, event: LogStashRecord) -> Result<()> {
        if event.level >= self.ignore_buffer {
            self.sender.send(event)?;
        } else if let Some(max_size) = self.buffer_size {
            self.buffer.push(event);
            if self.buffer.len() >= max_size {
                self.flush()?;
            }
        } else {
            self.sender.send(event)?;
        }
        Ok(())
    }

    fn send_batch(&mut self, events: Vec<LogStashRecord>) -> Result<()> {
        for event in events {
            self.send(event)?;
        }
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        if !self.buffer.is_empty() {
            let buffer = std::mem::replace(
                &mut self.buffer,
                Vec::with_capacity(self.buffer_size.unwrap_or_default()),
            );
            self.sender.send_batch(buffer)?;
        }
        self.sender.flush()?;
        self.deadline = None;
        Ok(())
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
