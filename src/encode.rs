use std::io::Write;
use byteorder::{WriteBytesExt, BigEndian};

use {Result, ErrorKind};

pub trait BinaryEncode {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()>;
}
impl BinaryEncode for bool {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_u8(*self as u8))
    }
}
impl BinaryEncode for u8 {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_u8(*self))
    }
}
impl BinaryEncode for i8 {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_i8(*self))
    }
}
impl BinaryEncode for i16 {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_i16::<BigEndian>(*self))
    }
}
impl BinaryEncode for i32 {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_i32::<BigEndian>(*self))
    }
}
impl BinaryEncode for i64 {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_i64::<BigEndian>(*self))
    }
}
impl BinaryEncode for f64 {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_f64::<BigEndian>(*self))
    }
}
impl<'a> BinaryEncode for &'a [u8] {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_assert!(self.len() <= 0x7FFF_FFFF, ErrorKind::InvalidInput);
        track!((self.len() as i32).binary_encode(writer))?;
        track_io!(writer.write_all(self))?;
        Ok(())
    }
}

pub trait CompactEncode {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()>;
}
