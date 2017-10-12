//! This crate provides functionalites for encoding/deconding [Thrift][thrift] protocol.
//!
//! # Examples
//!
//! # References
//!
//! - [Thrift Protocol Structure][protocol-structure]
//! - [Thrift Binary protocol encoding][binary-encoding]
//! - [Thrift Compact protocol encoding][compact-encoding]
//!
//! [thrift]: https://thrift.apache.org/
//! [protocol-structure]: https://github.com/apache/thrift/blob/master/doc/specs/thrift-protocol-spec.md
//! [binary-encoding]: https://github.com/apache/thrift/blob/master/doc/specs/thrift-binary-protocol.md
//! [compact-encoding]: https://github.com/apache/thrift/blob/master/doc/specs/thrift-compact-protocol.md
//!
#![warn(missing_docs)]
extern crate byteorder;
#[macro_use]
extern crate trackable;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

macro_rules! track_io {
    ($expr:expr) => {
        track!($expr.map_err(<::Error as From<::std::io::Error>>::from))
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

/// This crate specific `Result` type.
pub type Result<T> = std::result::Result<T, Error>;
