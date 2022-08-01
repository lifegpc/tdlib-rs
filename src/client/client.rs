use super::ClientError;
use futures_util::lock::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::net::{TcpStream, ToSocketAddrs, UdpSocket};

/// [Client] builder
pub struct ClientBuilder {
    /// Set `TCP_NODELAY`
    _no_delay: bool,
    /// Use UDP Connections
    _use_udp: bool,
    _transport_type: TransportType,
}

impl ClientBuilder {
    /// Build the client
    /// * `address` - The address to connect to.
    pub async fn build<A: ToSocketAddrs>(self, address: A) -> Result<Client, ClientError> {
        let stream = if self._use_udp {
            let socket = UdpSocket::bind("0.0.0.0:0").await?;
            socket.connect(address).await?;
            SocketHelper::from(socket)
        } else {
            let stream = TcpStream::connect(address).await?;
            stream.set_nodelay(self._no_delay)?;
            SocketHelper::from(stream)
        };
        Ok(Client {
            stream,
            builder: self,
        })
    }

    /// Create a new builder
    pub fn new() -> Self {
        Self {
            _no_delay: false,
            _use_udp: false,
            _transport_type: TransportType::Full,
        }
    }

    /// Set `TCP_NODELAY`
    pub fn no_delay(mut self, no_delay: bool) -> Self {
        self._no_delay = no_delay;
        self
    }

    /// Set the transport type. Default: [TransportType::Full]
    pub fn transport_type(mut self, transport_type: TransportType) -> Self {
        self._transport_type = transport_type;
        self
    }

    /// Use UDP Connections
    pub fn use_udp(mut self, use_udp: bool) -> Self {
        self._use_udp = use_udp;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// The transport type which used to transport payload.
pub enum TransportType {
    /// The lightest protocol. Max length of the payload: `16777215`.
    /// [More](https://core.telegram.org/mtproto/mtproto-transports#abridged)
    Abridged,
    /// Max length of the payload: `4294967295`.
    /// [More](https://core.telegram.org/mtproto/mtproto-transports#intermediate)
    Intermediate,
    /// Can be to use with [obfuscation enabled](https://core.telegram.org/mtproto/mtproto-transports#transport-obfsucation) to bypass ISP blocks.
    /// Max length of the payload: `4294967280 - 4294967295`.
    /// [More](https://core.telegram.org/mtproto/mtproto-transports#padded-intermediate)
    PaddedIntermediate,
    /// The basic MTProto transport protocol.
    /// Max length of the payload: `4294967287`.
    /// [More](https://core.telegram.org/mtproto/mtproto-transports#full)
    Full,
}

/// Socket wrapper
enum Socket {
    /// TCP
    TCP(TcpStream),
    /// UDP
    UDP(UdpSocket),
}

/// Socket wrapper
struct SocketHelper {
    /// Socket
    stream: Mutex<Socket>,
    initialized: AtomicBool,
}

impl SocketHelper {
    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::SeqCst)
    }
}

impl From<TcpStream> for SocketHelper {
    fn from(stream: TcpStream) -> Self {
        Self {
            stream: Mutex::new(Socket::TCP(stream)),
            initialized: AtomicBool::new(false),
        }
    }
}

impl From<UdpSocket> for SocketHelper {
    fn from(stream: UdpSocket) -> Self {
        Self {
            stream: Mutex::new(Socket::UDP(stream)),
            initialized: AtomicBool::new(false),
        }
    }
}

/// A low api level client
pub struct Client {
    /// Internal streams
    stream: SocketHelper,
    /// The builder
    builder: ClientBuilder,
}
