extern crate byteorder;
#[macro_use]
extern crate trackable;

macro_rules! track_io {
    ($expr:expr) => {
        track!($expr.map_err(::Error::from))
    }
}

pub use error::{Error, ErrorKind};

pub mod collections;
pub mod encode;
pub mod message;
pub mod structure;
pub mod value;

mod error;

pub type Result<T> = std::result::Result<T, Error>;
