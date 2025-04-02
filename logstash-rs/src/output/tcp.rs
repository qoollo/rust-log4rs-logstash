use crate::prelude::*;
use std::fmt::Write as FMTWrite;
use std::io::Write as IOWrite;
use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::sync::Mutex;
use std::time::Duration;

type Stream = Box<dyn IOWrite + Sync + Send>;

pub(crate) struct AdvancedTcpStream {
    hostname: String,
    port: u16,
    use_tls: bool,
    stream: Mutex<Option<Stream>>,
    connection_timeout: Option<Duration>,
}

impl AdvancedTcpStream {
    pub(crate) fn new(
        hostname: String,
        port: u16,
        use_tls: bool,
        connection_timeout: Option<Duration>,
    ) -> Self {
        Self {
            hostname,
            port,
            use_tls,
            stream: Mutex::new(None),
            connection_timeout,
        }
    }

    pub(crate) fn send_bytes(&self, bytes: &[u8]) -> Result<()> {
        let mut stream = self.stream.lock()?;
        let should_repeat = self.send_bytes_inner(&mut stream, bytes)?;
        if should_repeat {
            self.send_bytes_inner(&mut stream, bytes)?;
        }
        Ok(())
    }

    fn send_bytes_inner(&self, stream: &mut Option<Stream>, bytes: &[u8]) -> Result<bool> {
        let recreated = self.recreate_stream_if_needed(stream)?;
        if let Err(err) = stream.as_mut().expect("should be some").write_all(bytes) {
            *stream = None;
            if !recreated {
                return Ok(true);
            }
            return Err(err.into());
        }
        Ok(false)
    }

    fn recreate_stream_if_needed(&self, stream: &mut Option<Stream>) -> Result<bool> {
        if stream.is_none() {
            *stream = Some(if self.use_tls {
                self.create_tls_connection()?
            } else {
                self.create_tcp_connection()?
            });
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn create_connection(&self) -> Result<TcpStream> {
        let addr = (self.hostname.as_str(), self.port)
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| Error::AddressResolution(self.hostname.clone(), self.port))?;
        let stream = if let Some(timeout) = self.connection_timeout {
            TcpStream::connect_timeout(&addr, timeout)?
        } else {
            TcpStream::connect(addr)?
        };
        Ok(stream)
    }

    fn create_tcp_connection(&self) -> Result<Stream> {
        Ok(Box::new(self.create_connection()?))
    }

    #[cfg(all(feature = "tls", feature = "rustls"))]
    fn create_tls_connection(&self) -> Result<Stream> {
        compile_error!("Select one of 'tls' or 'rustls' feature");
        unreachable!();
    }

    #[cfg(all(feature = "tls", not(feature = "rustls")))]
    fn create_tls_connection(&self) -> Result<Stream> {
        use native_tls::HandshakeError;
        let conn = native_tls::TlsConnector::new()?;
        let stream = self.create_connection()?;
        let mut stream = conn.connect(self.hostname.as_str(), stream);
        while let Err(err) = stream {
            match err {
                HandshakeError::Failure(err) => return Err(err.into()),
                HandshakeError::WouldBlock(block) => {
                    stream = block.handshake();
                }
            }
        }
        Ok(Box::new(stream.expect("handshake completed")))
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
        let stream = self.create_connection()?;
        let stream = rustls_crate::StreamOwned::new(session, stream);
        Ok(Box::new(stream))
    }

    #[cfg(all(not(feature = "tls"), not(feature = "rustls")))]
    fn create_tls_connection(&self) -> Result<Stream> {
        panic!("TLS is not supported. Please enable 'tls' feature")
    }

    fn flush(&self) -> Result<()> {
        let mut stream = self.stream.lock()?;
        let recreated = self.recreate_stream_if_needed(&mut stream)?;
        if !recreated {
            stream.as_mut().expect("should be some").flush()?;
        }
        Ok(())
    }
}

pub struct TcpSender {
    stream: AdvancedTcpStream,
}

impl TcpSender {
    pub fn new(
        hostname: String,
        port: u16,
        use_tls: bool,
        connection_timeout: Option<Duration>,
    ) -> Self {
        Self {
            stream: AdvancedTcpStream::new(hostname, port, use_tls, connection_timeout),
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
            buf.push(b'\n');
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
