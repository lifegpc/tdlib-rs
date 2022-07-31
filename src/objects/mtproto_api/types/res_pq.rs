use crate::objects::traits::TypeId;
use std::ffi::CString;

/// The response type for function [super::super::functions::req_pq_multi]
///
/// Origin: `resPQ#05162463 nonce:int128 server_nonce:int128 pq:string server_public_key_fingerprints:Vector<long> = ResPQ;`
pub struct ResPq {
    /// Selected randomly by the client (random number) and identifies the client within this communication.
    pub nonce: i128,
    /// Selected randomly by the server.
    pub server_nonce: i128,
    /// A representation of a natural number (in binary big endian format).
    /// This number is the product of two different odd prime numbers.
    /// Normally, pq is less than or equal to 2^63-1.
    pub pq: CString,
    /// A list of public RSA key fingerprints (64 lower-order bits of SHA1 (server_public_key)
    pub server_public_key_fingerprints: Box<Vec<i64>>,
}

impl TypeId for ResPq {
    fn type_id(&self) -> u32 {
        0x05162463
    }
}
