#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum DeserializeError {
    String(String),
    IoError(std::io::Error),
}

impl From<&str> for DeserializeError {
    fn from(s: &str) -> Self {
        DeserializeError::String(s.to_string())
    }
}
