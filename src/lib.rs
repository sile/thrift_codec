//! This crate provides functionalities for encoding/deconding [Thrift][thrift] protocol.
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
//! # Examples
//!
//! Encodes a message:
//!
//! ```
//! use thrift_codec::CompactEncode;
//! use thrift_codec::data::Struct;
//! use thrift_codec::message::Message;
//!
//! let message = Message::oneway("foo_method", 1, Struct::from(("arg1", 2)));
//! let mut buf = Vec::new();
//! message.compact_encode(&mut buf).unwrap();
//! assert_eq!(
//!     buf,
//!     [130, 129, 1, 10, 102, 111, 111, 95, 109, 101, 116,
//!     104, 111, 100, 24, 4, 97, 114, 103, 49, 21, 4, 0]
//! );
//! ```
//!
//! Decodes the above binary:
//!
//! ```
//! use thrift_codec::CompactDecode;
//! use thrift_codec::data::Struct;
//! use thrift_codec::message::Message;
//!
//! let bytes = [
//!     130, 129, 1, 10, 102, 111, 111, 95, 109, 101, 116,
//!     104, 111, 100, 24, 4, 97, 114, 103, 49, 21, 4, 0
//! ];
//!
//! let message = Message::compact_decode(&mut &bytes[..]).unwrap();
//! let expected = Message::oneway("foo_method", 1, Struct::from(("arg1", 2)));
//! assert_eq!(message, expected);
//! ```
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

#[cfg(test)]
mod test {
    use data::Struct;
    use message::Message;
    use super::*;

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn it_works() {
        let message = Message::oneway("foo_method", 1, Struct::from(("arg1", 2)));
        let mut buf = Vec::new();
        track_try_unwrap!(message.compact_encode(&mut buf));
        assert_eq!(
            buf,
            [130, 129, 1, 10, 102, 111, 111, 95, 109, 101, 116,
             104, 111, 100, 24, 4, 97, 114, 103, 49, 21, 4, 0]
        );

        let decoded = track_try_unwrap!(Message::compact_decode(&mut &buf[..]));
        assert_eq!(decoded, message);
    }
}
