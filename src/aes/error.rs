/// Encrypt Error
#[derive(Debug, derive_more::From)]
pub enum EncryptError {
    /// The origin data is too long.
    DataTooLong,
    /// OpenSSL Error
    OpenSSLError(openssl::error::ErrorStack),
    /// Aes key error
    AesKeyError(openssl::aes::KeyError),
}
