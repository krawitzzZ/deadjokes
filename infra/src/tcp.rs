use std::io::{self, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Mutex;
use std::time::Duration;

type Stream = Box<dyn Write + Sync + Send>;

pub(crate) struct AdvancedTcpStream {
    hostname: String,
    port: u16,
    stream: Mutex<Option<Stream>>,
    use_tls: bool,
}

impl AdvancedTcpStream {
    pub(crate) fn new(hostname: String, port: u16, use_tls: bool) -> Self {
        Self {
            hostname,
            port,
            stream: Mutex::new(None),
            use_tls,
        }
    }

    pub(crate) fn send_bytes(&self, bytes: &[u8]) -> io::Result<()> {
        let mut stream = self.stream.lock().map_err(|e| {
            io::Error::new(
                io::ErrorKind::AddrNotAvailable,
                format!("failed to lock TCP stream mutex: {e}"),
            )
        })?;
        let should_repeat = self.send_bytes_inner(&mut stream, bytes)?;
        if should_repeat {
            self.send_bytes_inner(&mut stream, bytes)?;
        }
        Ok(())
    }

    pub(crate) fn flush(&self) -> io::Result<()> {
        let mut stream = self.stream.lock().map_err(|e| {
            io::Error::new(
                io::ErrorKind::AddrNotAvailable,
                format!("failed to lock TCP stream mutex: {e}"),
            )
        })?;
        let recreated = self.recreate_stream_if_needed(&mut stream)?;
        if !recreated {
            stream.as_mut().expect("should be some").flush()?;
        }
        Ok(())
    }

    fn send_bytes_inner(&self, stream: &mut Option<Stream>, bytes: &[u8]) -> io::Result<bool> {
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

    fn recreate_stream_if_needed(&self, stream: &mut Option<Stream>) -> io::Result<bool> {
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

    fn create_connection(&self) -> io::Result<TcpStream> {
        let addr = (self.hostname.as_str(), self.port)
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::AddrNotAvailable,
                    format!("address not available: {}:{}", self.hostname, self.port),
                )
            })?;
        let stream = TcpStream::connect_timeout(&addr, Duration::from_secs(10))?;
        Ok(stream)
    }

    fn create_tcp_connection(&self) -> io::Result<Stream> {
        Ok(Box::new(self.create_connection()?))
    }

    fn create_tls_connection(&self) -> io::Result<Stream> {
        use native_tls::HandshakeError;
        let conn = native_tls::TlsConnector::new().map_err(|e| {
            io::Error::new(
                io::ErrorKind::AddrNotAvailable,
                format!("failed to create tls connector: {e}"),
            )
        })?;
        let stream = self.create_connection()?;
        let mut stream = conn.connect(self.hostname.as_str(), stream);
        while let Err(err) = stream {
            match err {
                HandshakeError::Failure(err) => {
                    return Err(io::Error::new(
                        io::ErrorKind::ConnectionRefused,
                        format!("TCP connection handshake error: {err}"),
                    ))
                }
                HandshakeError::WouldBlock(block) => {
                    stream = block.handshake();
                }
            }
        }
        Ok(Box::new(stream.expect("handshake completed")))
    }
}
