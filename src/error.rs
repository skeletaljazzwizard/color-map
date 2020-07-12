// FIXME This would better solved by `anyhow`, `error-chain`, or `failure`
use std::fmt;

pub type Result<T> = std::result::Result<T, ColorMapError>;

#[derive(Debug)]
pub enum ColorMapError {
    InvalidArgumentKError(std::num::ParseIntError), // invalid k value
    InvalidArgumentImageError(image::error::ImageError), // invalid image path or file type
    MaxIterations(i32), // reached max iterations
}

impl fmt::Display for ColorMapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ColorMapError::InvalidArgumentKError(ref err) => write!(f, "K error: {}", err),
            ColorMapError::InvalidArgumentImageError(ref err) => write!(f, "image error: {}", err),
            ColorMapError::MaxIterations(ref count) => write!(f, "Kmeans max iterations limit of {} reached", count),
        }
    }
}

impl std::error::Error for ColorMapError {}

impl From<std::num::ParseIntError> for ColorMapError {
    fn from(err: std::num::ParseIntError) -> ColorMapError {
        ColorMapError::InvalidArgumentKError(err)
    }
}

impl From<image::error::ImageError> for ColorMapError {
    fn from(err: image::error::ImageError) -> ColorMapError {
        ColorMapError::InvalidArgumentImageError(err)
    }
}
