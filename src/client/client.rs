use super::ClientError;
use crate::objects::base::UnencryptedMessage;
use crate::objects::traits::{Deserialize, Serialize};
use bytes::BytesMut;
use futures_util::lock::Mutex;
use rand::{Rng, SeedableRng};
use std::ops::DerefMut;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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
            seq_no: AtomicU32::new(0),
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
    /// The lightest protocol. Max length of the payload: `67108864`.
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

impl TransportType {
    /// Returns true if current variant is [TransportType::Full]
    pub fn is_full(&self) -> bool {
        *self == Self::Full
    }
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

    pub async fn init_with(&self, ty: TransportType) -> Result<(), ClientError> {
        match ty {
            TransportType::Abridged => {
                self.send(&[0xef]).await?;
                self.initialized.store(true, Ordering::SeqCst);
                Ok(())
            }
            TransportType::Intermediate => {
                self.send(&[0xee, 0xee, 0xee, 0xee]).await?;
                self.initialized.store(true, Ordering::SeqCst);
                Ok(())
            }
            TransportType::PaddedIntermediate => {
                self.send(&[0xdd, 0xdd, 0xdd, 0xdd]).await?;
                self.initialized.store(true, Ordering::SeqCst);
                Ok(())
            }
            TransportType::Full => {
                self.initialized.store(true, Ordering::SeqCst);
                Ok(())
            }
        }
    }

    pub async fn recv_exact(&self, data: &mut [u8]) -> Result<(), ClientError> {
        let le = data.len();
        let mut s = self.recv(data).await?;
        while s < le {
            s += self.recv(&mut data[s..]).await?;
        }
        Ok(())
    }

    pub async fn recv(&self, data: &mut [u8]) -> Result<usize, ClientError> {
        match self.stream.lock().await.deref_mut() {
            Socket::TCP(stream) => Ok(stream.read(data).await?),
            Socket::UDP(stream) => Ok(stream.recv(data).await?),
        }
    }

    pub async fn send(&self, data: &[u8]) -> Result<usize, ClientError> {
        match self.stream.lock().await.deref_mut() {
            Socket::TCP(stream) => Ok(stream.write(data).await?),
            Socket::UDP(stream) => Ok(stream.send(data).await?),
        }
    }

    pub async fn send_all(&self, data: &[u8]) -> Result<(), ClientError> {
        let mut s = self.send(&data).await?;
        while s < data.len() {
            s += self.send(&data[s..]).await?;
        }
        Ok(())
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
    /// the TCP sequence number for this TCP connection: the first packet sent is numbered 0, the next one 1, etc.
    seq_no: AtomicU32,
}

impl Client {
    /// Gen data from payload.
    fn gen_payload(&self, data: Vec<u8>) -> Vec<u8> {
        match self.builder._transport_type {
            TransportType::Abridged => {
                if data.len() >= 508 {
                    let mut payload = Vec::with_capacity(data.len() + 4);
                    payload.push(0x7fu8);
                    payload.extend_from_slice(&((data.len() / 4) as u32).to_le_bytes()[..3]);
                    payload.extend_from_slice(&data);
                    payload
                } else {
                    let mut payload = Vec::with_capacity(data.len() + 1);
                    payload.push((data.len() / 4) as u8);
                    payload.extend_from_slice(&data);
                    payload
                }
            }
            TransportType::Intermediate => {
                let mut payload = Vec::with_capacity(data.len() + 4);
                let le = data.len() as u32;
                payload.extend_from_slice(&le.to_le_bytes());
                payload.extend_from_slice(&data);
                payload
            }
            TransportType::PaddedIntermediate => {
                let random = rand::rngs::StdRng::from_entropy().gen::<u32>() % 16;
                let mut payload = Vec::with_capacity(data.len() + 4 + random as usize);
                let le = data.len() as u32 + random;
                payload.extend_from_slice(&le.to_le_bytes());
                payload.extend_from_slice(&data);
                for _ in 0..random {
                    payload.push(rand::rngs::StdRng::from_entropy().gen());
                }
                payload
            }
            TransportType::Full => {
                let mut payload = Vec::with_capacity(data.len() + 12);
                let le = data.len() as u32 + 12;
                payload.extend_from_slice(&le.to_le_bytes());
                payload.extend_from_slice(&(self.seq_no.load(Ordering::SeqCst).to_le_bytes()));
                payload.extend_from_slice(&data);
                payload.extend_from_slice(&(crc32fast::hash(&payload).to_le_bytes()));
                payload
            }
        }
    }

    /// Send unencrypted data
    /// * `data` - unecrypted data
    pub async fn send_unencrypted<S: Serialize>(&self, data: &S) -> Result<(), ClientError> {
        if !self.stream.is_initialized() {
            self.stream
                .init_with(self.builder._transport_type.clone())
                .await?;
        }
        let mut d = Vec::with_capacity(20);
        d.extend_from_slice(&(0i64).serialize_to_bytes());
        let message_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            * (1 << 32);
        d.extend_from_slice(&message_id.to_le_bytes());
        let data = data.serialize_to_bytes();
        d.extend_from_slice(&(data.len() as u32).to_le_bytes());
        d.extend_from_slice(&data);
        let payload = self.gen_payload(d);
        if self.builder._transport_type.is_full() {
            self.seq_no.fetch_add(1, Ordering::SeqCst);
        }
        Ok(self.stream.send_all(&payload).await?)
    }

    /// Receive data
    pub async fn recv(&self) -> Result<BytesMut, ClientError> {
        if !self.stream.is_initialized() {
            return Err(ClientError::NotInitialized);
        }
        match self.builder._transport_type {
            TransportType::Abridged => {
                let mut le = [0u8; 1];
                self.stream.recv_exact(&mut le).await?;
                let data = if le[0] == 0x7f {
                    let mut le = [0u8; 3];
                    self.stream.recv_exact(&mut le).await?;
                    let le = [le[0], le[1], le[2], 0];
                    let le = (u32::from_le_bytes(le)) as usize * 4;
                    let mut data = BytesMut::with_capacity(le);
                    data.resize(le, 0);
                    self.stream.recv_exact(&mut data).await?;
                    data
                } else {
                    let le = le[0] as usize * 4;
                    let mut data = BytesMut::with_capacity(le);
                    data.resize(le, 0);
                    self.stream.recv_exact(&mut data).await?;
                    data
                };
                if data.len() == 4 {
                    return Err(ClientError::ServerError(i32::deserialize_from_bytes(
                        &data,
                    )?));
                }
                Ok(data)
            }
            TransportType::Intermediate => {
                let mut le = [0u8; 4];
                self.stream.recv_exact(&mut le).await?;
                let le = u32::from_le_bytes(le) as usize;
                let mut data = BytesMut::with_capacity(le);
                data.resize(le, 0);
                self.stream.recv_exact(&mut data).await?;
                if le == 4 {
                    return Err(ClientError::ServerError(i32::deserialize_from_bytes(
                        &data,
                    )?));
                }
                Ok(data)
            }
            TransportType::PaddedIntermediate => {
                let mut le = [0u8; 4];
                self.stream.recv_exact(&mut le).await?;
                let le = u32::from_le_bytes(le) as usize;
                let mut data = BytesMut::with_capacity(le);
                data.resize(le, 0);
                self.stream.recv_exact(&mut data).await?;
                if le == 4 {
                    return Err(ClientError::ServerError(i32::deserialize_from_bytes(
                        &data,
                    )?));
                }
                Ok(data)
            }
            TransportType::Full => {
                let mut h = crc32fast::Hasher::new();
                let mut le = [0u8; 4];
                self.stream.recv_exact(&mut le).await?;
                h.update(&le);
                let le = u32::from_le_bytes(le) as usize;
                let mut seq_no = [0u8; 4];
                self.stream.recv_exact(&mut seq_no).await?;
                h.update(&seq_no);
                let _seq_no = u32::from_le_bytes(seq_no);
                let mut data = BytesMut::with_capacity(le - 12);
                data.resize(le - 12, 0);
                self.stream.recv_exact(&mut data).await?;
                h.update(&data);
                let mut crc = [0u8; 4];
                self.stream.recv_exact(&mut crc).await?;
                let crc = u32::from_le_bytes(crc);
                if crc != h.finalize() {
                    return Err(ClientError::Crc32CheckFailed);
                }
                if le == 4 {
                    return Err(ClientError::ServerError(i32::deserialize_from_bytes(
                        &data,
                    )?));
                }
                Ok(data)
            }
        }
    }

    /// Receive unencrypted message
    pub async fn recv_unecrypted(&self) -> Result<UnencryptedMessage, ClientError> {
        Ok(UnencryptedMessage::deserialize_from_bytes(
            &self.recv().await?,
        )?)
    }
}
