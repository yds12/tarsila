use lapix::Error as LapixError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Image error: {0}")]
    LapixError(#[from] LapixError),
}
