use crate::prelude::*;
use std::fmt::Write as FMTWrite;
use std::io::Write as IOWrite;
use std::net::TcpStream;
use std::sync::Mutex;

type Stream = Box<dyn IOWrite + Sync + Send>;

pub(crate) struct AdvancedTcpStream {
    hostname: String,
    port: u16,
    use_tls: bool,
    stream: Mutex<Option<Stream>>,
}

impl AdvancedTcpStream {
    pub(crate) fn new(hostname: String, port: u16, use_tls: bool) -> Self {
        Self {
            hostname,
            port,
            use_tls,
            stream: Mutex::new(None),
        }
    }

    pub(crate) fn send_bytes(&self, bytes: &[u8]) -> Result<()> {
        let mut stream = self
            .stream
            .try_lock()
            .map_err(|_| Error::lock_stream_mutex())?;
        self.recreate_stream_if_needed(&mut stream)?;
        if let Some(Err(err)) = stream.as_mut().map(|stream| stream.write_all(bytes)) {
            *stream = None;
            return Err(err.into());
        }
        Ok(())
    }

    fn recreate_stream_if_needed(&self, stream: &mut Option<Stream>) -> Result<()> {
        if stream.is_none() {
            *stream = Some(if self.use_tls {
                self.create_connection()?
            } else {
                self.create_tls_connection()?
            });
        }
        Ok(())
    }

    fn create_connection(&self) -> Result<Stream> {
        Ok(Box::new(TcpStream::connect((
            self.hostname.as_str(),
            self.port,
        ))?))
    }

    #[cfg(all(feature = "tls", feature = "rustls"))]
    fn create_tls_connection(&self) -> Result<Stream> {
        compile_error!("Select one of 'tls' or 'rustls' feature");
        unreachable!();
    }

    #[cfg(all(feature = "tls", not(feature = "rustls")))]
    fn create_tls_connection(&self) -> Result<Stream> {
        let conn = native_tls::TlsConnector::new()?;
        let stream = TcpStream::connect((self.hostname.as_str(), self.port))?;
        let stream = conn.connect(self.hostname.as_str(), stream)?;
        Ok(Box::new(stream))
    }

    #[cfg(all(not(feature = "tls"), feature = "rustls"))]
    fn create_tls_connection(&self) -> Result<Stream> {
        use std::convert::TryInto;
        use std::sync::Arc;
        let mut root_store = rustls_crate::RootCertStore::empty();
        root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
            rustls_crate::OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        }));
        let config = rustls_crate::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        let session = rustls_crate::ClientConnection::new(
            Arc::new(config),
            self.hostname.as_str().try_into()?,
        )?;
        let stream = TcpStream::connect((self.hostname.as_str(), self.port))?;
        let stream = rustls_crate::StreamOwned::new(session, stream);
        Ok(Box::new(stream))
    }

    #[cfg(all(not(feature = "tls"), not(feature = "rustls")))]
    fn create_tls_connection(&self) -> Result<Stream> {
        panic!("TLS is not supported. Please enable 'tls' feature")
    }

    fn flush(&self) -> Result<()> {
        let mut stream = self
            .stream
            .try_lock()
            .map_err(|_| Error::lock_stream_mutex())?;
        if let Some(Err(err)) = stream.as_mut().map(|stream| stream.flush()) {
            *stream = None;
            return Err(err.into());
        }
        Ok(())
    }
}

pub struct TcpSender {
    stream: AdvancedTcpStream,
}

impl TcpSender {
    pub fn new(hostname: String, port: u16, use_tls: bool) -> Self {
        Self {
            stream: AdvancedTcpStream::new(hostname, port, use_tls),
        }
    }
}

impl Sender for TcpSender {
    fn send(&self, event: LogStashRecord) -> Result<()> {
        let mut event = serde_json::to_string(&event)?;
        event.write_char('\n')?;
        self.stream.send_bytes(event.as_bytes())?;
        Ok(())
    }

    fn send_batch(&self, events: Vec<LogStashRecord>) -> Result<()> {
        if events.is_empty() {
            return Ok(());
        }
        let mut buf = vec![];
        for event in events {
            serde_json::to_writer(&mut buf, &event)?;
            buf.push('\n' as u8);
        }
        self.stream.send_bytes(&buf)?;
        Ok(())
    }

    fn flush(&self) -> Result<()> {
        self.stream.flush()?;
        Ok(())
    }
}

impl log::Log for TcpSender {
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
