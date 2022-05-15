use std::sync::mpsc::SendError;
use std::sync::PoisonError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    FmtError(#[from] std::fmt::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[cfg(all(feature = "tls", not(feature = "rustls")))]
    #[error(transparent)]
    TlsError(#[from] native_tls::Error),
    #[error("sender thread stopped: {0}")]
    SenderThreadStopped(String),
    #[error("address resolution error: {0}:{1}")]
    AddressResolution(String, u16),
    #[error("fatal internal error: {0}")]
    FatalInternal(String),
    #[cfg(all(not(feature = "tls"), feature = "rustls"))]
    #[error("rustls client: {0}")]
    InvalidDNSName(#[from] rustls_crate::client::InvalidDnsNameError),
    #[cfg(all(not(feature = "tls"), feature = "rustls"))]
    #[error("rustls: {0}")]
    Rustls(#[from] rustls_crate::Error),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(err: PoisonError<T>) -> Self {
        Self::FatalInternal(err.to_string())
    }
}

impl<T> From<SendError<T>> for Error {
    fn from(err: SendError<T>) -> Self {
        Self::SenderThreadStopped(err.to_string())
    }
}
