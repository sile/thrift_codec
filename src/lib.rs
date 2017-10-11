extern crate byteorder;
#[macro_use]
extern crate trackable;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

macro_rules! track_io {
    ($expr:expr) => {
        track!($expr.map_err(::Error::from))
    }
}

pub use error::{Error, ErrorKind};
pub use decode::{BinaryDecode, CompactDecode};
pub use encode::{BinaryEncode, CompactEncode};

pub mod data;
pub mod message;

mod decode;
mod encode;
mod error;

// mod constants {
//     pub mod compact {
//         pub const PROTOCOL_ID: u8 = 0x82;
//         pub const VERSION: u8 = 0b0_0001;

//         pub const FIELD_TYPE_BOOLEAN_TRUE: u8 = 1;
//         pub const FIELD_TYPE_BOOLEAN_FALSE: u8 = 2;
//         pub const FIELD_TYPE_BYTE: u8 = 3;
//         pub const FIELD_TYPE_I16: u8 = 4;
//         pub const FIELD_TYPE_I32: u8 = 5;
//         pub const FIELD_TYPE_I64: u8 = 6;
//         pub const FIELD_TYPE_DOUBLE: u8 = 7;
//         pub const FIELD_TYPE_BINARY: u8 = 8; // for binary and string fields
//         pub const FIELD_TYPE_LIST: u8 = 9;
//         pub const FIELD_TYPE_SET: u8 = 10;
//         pub const FIELD_TYPE_MAP: u8 = 11;
//         pub const FIELD_TYPE_STRUCT: u8 = 12; // for structs and union fields

//         pub const ELEMENT_TYPE_BOOL: u8 = 2;
//         pub const ELEMENT_TYPE_BYTE: u8 = 3;
//         pub const ELEMENT_TYPE_DOUBLE: u8 = 4;
//         pub const ELEMENT_TYPE_I16: u8 = 6;
//         pub const ELEMENT_TYPE_I32: u8 = 8;
//         pub const ELEMENT_TYPE_I64: u8 = 10;
//         pub const ELEMENT_TYPE_STRING: u8 = 11; // for binary and string fields
//         pub const ELEMENT_TYPE_STRUCT: u8 = 12; // for struct and union fields
//         pub const ELEMENT_TYPE_MAP: u8 = 13;
//         pub const ELEMENT_TYPE_SET: u8 = 14;
//         pub const ELEMENT_TYPE_LIST: u8 = 15;
//     }
// }

pub type Result<T> = std::result::Result<T, Error>;
