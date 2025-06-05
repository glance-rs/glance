use std::io;

use derive_more::From;

pub type Result<T> = core::result::Result<T, CoreError>;

#[derive(Debug, From)]
pub enum CoreError {
    #[from]
    Image(image::ImageError),

    #[from]
    Minifb(minifb::Error),

    OutOfBounds(String),

    #[from]
    Io(io::Error),

    InvalidCast(String),
    InvalidData(String),
}

impl core::fmt::Display for CoreError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for CoreError {}
