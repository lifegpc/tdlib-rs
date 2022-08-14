#[derive(Debug, derive_more::Display, derive_more::From)]
/// Client error
pub enum ClientError {
    /// The client is not initialized.
    NotInitialized,
    DeserializeError(crate::objects::DeserializeError),
    TokioError(tokio::io::Error),
}
