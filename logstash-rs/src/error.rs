use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("fatal: failed to send {0} to channel")]
    SendToChannel(String),
    #[error("fatal: failed to lock stream mutex")]
    LockStreamMutex,
}

impl Error {
    pub fn send_to_channel(message: impl Into<String>) -> Self {
        Self::SendToChannel(message.into())
    }

    pub fn lock_stream_mutex() -> Self {
        Self::LockStreamMutex
    }
}
