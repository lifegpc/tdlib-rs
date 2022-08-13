/// Deserialize error
#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum DeserializeError {
    /// Error message
    String(String),
    /// IO error
    IoError(std::io::Error),
    /// Null error
    NulError(std::ffi::NulError),
    /// Failed decode string with UTF-8
    Utf8Error(std::str::Utf8Error),
}

impl From<&str> for DeserializeError {
    fn from(s: &str) -> Self {
        DeserializeError::String(s.to_string())
    }
}
