extern crate byteorder;
#[macro_use]
extern crate trackable;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

macro_rules! track_io {
    ($expr:expr) => {
        track!($expr.map_err(|e: ::std::io::Error| ::Error::from(e)))
    }
}

pub use error::{Error, ErrorKind};
pub use decode::{BinaryDecode, CompactDecode};
pub use encode::{BinaryEncode, CompactEncode};

pub mod data;
pub mod message;

mod constants;
mod decode;
mod encode;
mod error;
mod zigzag;

pub type Result<T> = std::result::Result<T, Error>;
