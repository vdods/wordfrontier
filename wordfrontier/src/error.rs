#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("malformed input: {}", .0)]
    MalformedInputError(String),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    RusqliteError(#[from] rusqlite::Error),
}
