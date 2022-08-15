use crate::objects::traits::Serialize;
use bytes::BytesMut;
use rsa::{pkcs1::DecodeRsaPublicKey, BigUint, PublicKeyParts, RsaPublicKey};
use sha1::{Digest, Sha1};

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
        hasher.update(&self.n.serialize());
        hasher.update(&self.e.serialize());
        BytesMut::from(hasher.finalize().as_slice())
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

impl TryInto<RsaPublicKey> for RSAPublicKey {
    type Error = rsa::errors::Error;
    fn try_into(self) -> Result<RsaPublicKey, Self::Error> {
        RsaPublicKey::new(
            BigUint::from_bytes_be(&self.n),
            BigUint::from_bytes_be(&self.e),
        )
    }
}

impl From<RsaPublicKey> for RSAPublicKey {
    fn from(key: RsaPublicKey) -> Self {
        let n = key.n().to_bytes_be();
        let e = key.e().to_bytes_be();
        let n: &[u8] = &n;
        let e: &[u8] = &e;
        RSAPublicKey {
            n: BytesMut::from(n),
            e: BytesMut::from(e),
        }
    }
}

impl<'a> DecodeRsaPublicKey for RSAPublicKey {
    fn from_pkcs1_der(bytes: &[u8]) -> rsa::pkcs1::Result<Self> {
        let key = RsaPublicKey::from_pkcs1_der(bytes)?;
        Ok(Self::from(key))
    }
}
