use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to generate image from raw data")]
    FailedImageFromRaw,
    #[error("No free image found")]
    MissingFreeImage,
    #[error("Unsupported image format")]
    UnsupportedImageFormat,
    #[error("Drawing action has not started")]
    DrawingNotStarted,
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Bug: reversal list is not set")]
    ReversalNotSet,
    #[error("Codec error: {0}")]
    CodecError(#[from] bincode::Error),
}
