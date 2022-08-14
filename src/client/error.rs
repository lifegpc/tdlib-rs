#[derive(Debug, derive_more::Display, derive_more::From)]
/// Client error
pub enum ClientError {
    /// The client is not initialized.
    NotInitialized,
    /// Failed to deserialize response
    DeserializeError(crate::objects::DeserializeError),
    /// Network error
    TokioError(tokio::io::Error),
    /// Server error
    ServerError(i32),
}
