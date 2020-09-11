// FIXME This would better solved by `anyhow`, `error-chain`, or `failure`
use std::fmt;

pub type Result<T> = std::result::Result<T, ColorMapError>;

#[derive(Debug)]
pub enum ColorMapError {
    InvalidArgumentKError(std::num::ParseIntError), // invalid k value
    InvalidArgumentImageError(image::error::ImageError), // invalid image path or file type
    MaxIterations(i32), // reached max iterations
    WriteColorError(std::io::Error),
}

impl fmt::Display for ColorMapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ColorMapError::InvalidArgumentKError(ref err) => writeln!(f, "K error: {}", err),
            ColorMapError::InvalidArgumentImageError(ref err) => writeln!(f, "image error: {}", err),
            ColorMapError::MaxIterations(ref count) => writeln!(f, "Kmeans max iterations limit of {} reached", count),
            ColorMapError::WriteColorError(ref err) => writeln!(f, "write color error: {}", err),
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

impl From<std::io::Error> for ColorMapError {
    fn from(err: std::io::Error) -> ColorMapError {
        ColorMapError::WriteColorError(err)
    }
}
