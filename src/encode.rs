use std::io::Write;

use Result;

pub trait BinaryEncode {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()>;
}

pub trait CompactEncode {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()>;
}
