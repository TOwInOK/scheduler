use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("fail to fetch file: {0}")]
    Io(#[from] std::io::Error),
    #[error("ron error: {0}")]
    RonSpanned(#[from] ron::error::SpannedError),
    #[error("ron error: {0}")]
    Ron(#[from] ron::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
