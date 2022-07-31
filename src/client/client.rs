use super::ClientError;
use std::any::Any;
use tokio::net::{TcpStream, ToSocketAddrs, UdpSocket};

/// [Client] builder
pub struct ClientBuilder {
    /// Set `TCP_NODELAY`
    _no_delay: bool,
    /// Use UDP Connections
    _use_udp: bool,
}

impl ClientBuilder {
    /// Build the client
    /// * `address` - The address to connect to.
    pub async fn build<A: ToSocketAddrs>(&self, address: A) -> Result<Client, ClientError> {
        let stream: Box<dyn Any> = if self._use_udp {
            let socket = UdpSocket::bind("0.0.0.0:0").await?;
            socket.connect(address).await?;
            Box::new(socket)
        } else {
            let stream = TcpStream::connect(address).await?;
            stream.set_nodelay(self._no_delay)?;
            Box::new(stream)
        };
        Ok(Client { stream })
    }

    /// Create a new builder
    pub fn new() -> Self {
        Self {
            _no_delay: false,
            _use_udp: false,
        }
    }

    /// Set `TCP_NODELAY`
    pub fn no_delay(mut self, no_delay: bool) -> Self {
        self._no_delay = no_delay;
        self
    }

    /// Use UDP Connections
    pub fn use_udp(mut self, use_udp: bool) -> Self {
        self._use_udp = use_udp;
        self
    }
}

/// A low api level client
pub struct Client {
    /// Internal streams
    stream: Box<dyn Any>,
}
