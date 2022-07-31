#[derive(Debug, derive_more::Display, derive_more::From)]
/// Client error
pub enum ClientError {
    TokioError(tokio::io::Error),
}
