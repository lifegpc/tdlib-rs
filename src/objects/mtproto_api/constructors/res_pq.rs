use crate::objects::traits::TypeId;
use bytes::BytesMut;

/// Factorize error
#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum FactorizeError {
    /// OpenSSL Error
    OpenSSLError(openssl::error::ErrorStack),
    /// No prime found
    NotFound,
}

#[derive(Clone, Debug, tdlib_rs_impl::Deserialize, tdlib_rs_impl::Serialize)]
/// The response type for function [super::super::functions::req_pq_multi]
///
/// Origin: `resPQ#05162463 nonce:int128 server_nonce:int128 pq:string server_public_key_fingerprints:Vector<long> = ResPQ;`
pub struct resPQ {
    /// Selected randomly by the client (random number) and identifies the client within this communication.
    pub nonce: i128,
    /// Selected randomly by the server.
    pub server_nonce: i128,
    /// A representation of a natural number (in binary big endian format).
    /// This number is the product of two different odd prime numbers.
    /// Normally, pq is less than or equal to 2^63-1.
    pub pq: BytesMut,
    /// A list of public RSA key fingerprints (64 lower-order bits of SHA1 (server_public_key);
    /// the public key is represented as a bare type `rsa_public_key n:string e:string = RSAPublicKey`,
    /// where, as usual, n and е are numbers in big endian format serialized as strings of bytes,
    /// following which SHA1 is computed) received by the server.
    pub server_public_key_fingerprints: Box<Vec<i64>>,
}

impl resPQ {
    /// Returns (p, q)
    pub fn pq_factorize(&self) -> Result<(Vec<u8>, Vec<u8>), FactorizeError> {
        match crate::prime::pq_factorize(&self.pq)? {
            Some(s) => Ok(s),
            None => Err(FactorizeError::NotFound),
        }
    }
}

impl TypeId for resPQ {
    fn type_id2() -> u32 {
        0x05162463
    }
}
