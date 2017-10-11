use std::io::Write;
use byteorder::{WriteBytesExt, BigEndian};

use {Result, ErrorKind};
use constants;
use message::Message;
use data::{Data, DataRef, Struct, Map, Set, List};

pub trait BinaryEncode {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()>;
}
impl BinaryEncode for bool {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_u8(*self as u8))
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
impl BinaryEncode for Message {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_u16::<BigEndian>(
            (1 << 15) | constants::BINARY_PROTOCOL_VERSION,
        ))?;
        track_io!(writer.write_u8(0))?;
        track_io!(writer.write_u8(self.kind() as u8))?;
        track!(self.method_name().as_bytes().binary_encode(writer))?;
        track!(self.sequence_id().binary_encode(writer))?;
        track!(self.body().binary_encode(writer))?;
        Ok(())
    }
}
impl BinaryEncode for Data {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!(self.as_ref().binary_encode(writer))
    }
}
impl<'a> BinaryEncode for DataRef<'a> {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        match *self {
            DataRef::Bool(v) => track!(v.binary_encode(writer)),
            DataRef::I8(v) => track!(v.binary_encode(writer)),
            DataRef::I16(v) => track!(v.binary_encode(writer)),
            DataRef::I32(v) => track!(v.binary_encode(writer)),
            DataRef::I64(v) => track!(v.binary_encode(writer)),
            DataRef::Double(v) => track!(v.binary_encode(writer)),
            DataRef::Binary(v) => track!(v.binary_encode(writer)),
            DataRef::Struct(v) => track!(v.binary_encode(writer)),
            DataRef::Map(v) => track!(v.binary_encode(writer)),
            DataRef::Set(v) => track!(v.binary_encode(writer)),
            DataRef::List(v) => track!(v.binary_encode(writer)),
        }
    }
}
impl BinaryEncode for Struct {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        for field in self.fields() {
            track_io!(writer.write_u8(field.data().kind() as u8))?;
            track!(field.id().binary_encode(writer))?;
            track!(field.data().binary_encode(writer))?;
        }
        track_io!(writer.write_u8(0))?;
        Ok(())
    }
}
impl BinaryEncode for Map {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_assert!(self.len() <= 0x7FFF_FFFF, ErrorKind::InvalidInput);

        let key_kind = track_assert_some!(self.key_kind(), ErrorKind::InvalidInput);
        track_io!(writer.write_u8(key_kind as u8))?;

        let value_kind = track_assert_some!(self.value_kind(), ErrorKind::InvalidInput);
        track_io!(writer.write_u8(value_kind as u8))?;

        track!((self.len() as i32).binary_encode(writer))?;
        for (k, v) in self.iter() {
            track!(k.binary_encode(writer))?;
            track!(v.binary_encode(writer))?;
        }
        Ok(())
    }
}
impl BinaryEncode for Set {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_assert!(self.len() <= 0x7FFF_FFFF, ErrorKind::InvalidInput);
        track_io!(writer.write_u8(self.kind() as u8))?;
        track!((self.len() as i32).binary_encode(writer))?;
        for e in self.iter() {
            track!(e.binary_encode(writer))?;
        }
        Ok(())
    }
}
impl BinaryEncode for List {
    fn binary_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_assert!(self.len() <= 0x7FFF_FFFF, ErrorKind::InvalidInput);
        track_io!(writer.write_u8(self.kind() as u8))?;
        track!((self.len() as i32).binary_encode(writer))?;
        for e in self.iter() {
            track!(e.binary_encode(writer))?;
        }
        Ok(())
    }
}

pub trait CompactEncode {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()>;
}
