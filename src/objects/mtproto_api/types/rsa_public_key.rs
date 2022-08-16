use crate::objects::traits::Serialize;
use bytes::BytesMut;
use openssl::{bn::BigNum, pkey::Public, rsa::Rsa, sha::Sha1};

/// RSA Public key
///
/// Source: `rsa_public_key n:string e:string = RSAPublicKey;`
#[derive(Clone, Debug, tdlib_rs_impl::Deserialize, tdlib_rs_impl::Serialize)]
pub struct RSAPublicKey {
    pub n: BytesMut,
    pub e: BytesMut,
}

impl RSAPublicKey {
    /// Return sha1 hash of the public key
    pub fn sha1(&self) -> BytesMut {
        let mut hasher = Sha1::new();
        hasher.update(&self.n.serialize_to_bytes());
        hasher.update(&self.e.serialize_to_bytes());
        BytesMut::from(hasher.finish().as_slice())
    }

    /// Return 64 lower-order bits of SHA1
    pub fn sha1_as_i64(&self) -> i64 {
        let mut data = self.sha1();
        data.reverse();
        let data = [
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        ];
        i64::from_be_bytes(data)
    }
}

impl TryInto<Rsa<Public>> for RSAPublicKey {
    type Error = openssl::error::ErrorStack;
    fn try_into(self) -> Result<Rsa<Public>, Self::Error> {
        let s = &self;
        s.try_into()
    }
}

impl TryInto<Rsa<Public>> for &RSAPublicKey {
    type Error = openssl::error::ErrorStack;
    fn try_into(self) -> Result<Rsa<Public>, Self::Error> {
        Rsa::<Public>::from_public_components(
            BigNum::from_slice(&self.n)?,
            BigNum::from_slice(&self.e)?,
        )
    }
}

impl From<&Rsa<Public>> for RSAPublicKey {
    fn from(key: &Rsa<Public>) -> Self {
        let n = key.n().to_vec();
        let e = key.e().to_vec();
        let n: &[u8] = &n;
        let e: &[u8] = &e;
        RSAPublicKey {
            n: BytesMut::from(n),
            e: BytesMut::from(e),
        }
    }
}

impl From<Rsa<Public>> for RSAPublicKey {
    fn from(key: Rsa<Public>) -> Self {
        Self::from(&key)
    }
}
