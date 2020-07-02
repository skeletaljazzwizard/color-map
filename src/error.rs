// FIXME This would better solved by `anyhow`, `error-chain`, or `failure`
use std::fmt;

pub type Result<T> = std::result::Result<T, SoupError>;

#[derive(Debug)]
pub enum SoupError {
    InvalidArgumentKError(std::num::ParseIntError), // invalid k value
    InvalidArgumentImageError(image::error::ImageError), // invalid image path or file type
    MaxIterations(i32), // reached max iterations
}

impl fmt::Display for SoupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SoupError::InvalidArgumentKError(ref err) => write!(f, "K error: {}", err),
            SoupError::InvalidArgumentImageError(ref err) => write!(f, "image error: {}", err),
            SoupError::MaxIterations(ref count) => write!(f, "Kmeans max iterations limit of {} reached", count),
        }
    }
}

impl std::error::Error for SoupError {}

impl From<std::num::ParseIntError> for SoupError {
    fn from(err: std::num::ParseIntError) -> SoupError {
        SoupError::InvalidArgumentKError(err)
    }
}

impl From<image::error::ImageError> for SoupError {
    fn from(err: image::error::ImageError) -> SoupError {
        SoupError::InvalidArgumentImageError(err)
    }
}
