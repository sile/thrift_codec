use std::io::Write;
use byteorder::{WriteBytesExt, BigEndian, LittleEndian};

use {Result, ErrorKind};
use constants;
use message::Message;
use data::{Data, DataRef, DataKind, Struct, Map, Set, List};
use zigzag;

/// This trait allows to encode objects to the binaries specified by
/// the [Thrift Binary protocol encoding][encoding].
///
/// [encoding]: https://github.com/apache/thrift/blob/master/doc/specs/thrift-binary-protocol.md
pub trait BinaryEncode {
    /// Encodes an object.
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

/// This trait allows to encode objects to the binaries specified by
/// the [Thrift Compact protocol encoding][encoding].
///
/// [encoding]: https://github.com/apache/thrift/blob/master/doc/specs/thrift-compact-protocol.md
pub trait CompactEncode {
    /// Encodes an object.
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()>;
}
impl CompactEncode for bool {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_u8(*self as u8))
    }
}
impl CompactEncode for i8 {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_i8(*self))
    }
}
impl CompactEncode for i16 {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!(i32::from(*self).compact_encode(writer))
    }
}
impl CompactEncode for i32 {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!(write_varint(writer, u64::from(zigzag::from_i32(*self))))
    }
}
impl CompactEncode for i64 {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!(write_varint(writer, zigzag::from_i64(*self)))
    }
}
impl CompactEncode for f64 {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        // [NOTE]
        //
        // The [specification] says "We are using big-endian",
        // but actually, implementations are using little-endian.
        // (e.g., https://github.com/apache/thrift/blob/8b8a8efea13d1c97f856053af0a5c0e6a8a76354/lib/java/src/org/apache/thrift/protocol/TCompactProtocol.java#L466)
        //
        // [specification]: https://github.com/apache/thrift/blob/8b8a8efea13d1c97f856053af0a5c0e6a8a76354/doc/specs/thrift-compact-protocol.md
        track_io!(writer.write_f64::<LittleEndian>(*self))
    }
}
impl<'a> CompactEncode for &'a [u8] {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_assert!(self.len() <= 0x7FFF_FFFF, ErrorKind::InvalidInput);
        track!(write_varint(writer, self.len() as u64))?;
        track_io!(writer.write_all(self))?;
        Ok(())
    }
}
impl CompactEncode for Message {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_u8(constants::COMPACT_PROTOCOL_ID))?;
        track_io!(writer.write_u8(
            ((self.kind() as u8) << 5) |
                constants::COMPACT_PROTOCOL_VERSION,
        ))?;
        track!(write_varint(writer, u64::from(self.sequence_id() as u32)))?;
        track!(self.method_name().as_bytes().compact_encode(writer))?;
        track!(self.body().compact_encode(writer))?;
        Ok(())
    }
}
impl CompactEncode for Data {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!(self.as_ref().compact_encode(writer))
    }
}
impl<'a> CompactEncode for DataRef<'a> {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        match *self {
            DataRef::Bool(v) => track!(v.compact_encode(writer)),
            DataRef::I8(v) => track!(v.compact_encode(writer)),
            DataRef::I16(v) => track!(v.compact_encode(writer)),
            DataRef::I32(v) => track!(v.compact_encode(writer)),
            DataRef::I64(v) => track!(v.compact_encode(writer)),
            DataRef::Double(v) => track!(v.compact_encode(writer)),
            DataRef::Binary(v) => track!(v.compact_encode(writer)),
            DataRef::Struct(v) => track!(v.compact_encode(writer)),
            DataRef::Map(v) => track!(v.compact_encode(writer)),
            DataRef::Set(v) => track!(v.compact_encode(writer)),
            DataRef::List(v) => track!(v.compact_encode(writer)),
        }
    }
}
impl CompactEncode for Struct {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mut prev_field_id = 0;
        for field in self.fields() {
            let mut delta = field.id() - prev_field_id;
            if !(0 < delta && delta <= 15) {
                delta = 0;
            }

            let kind = match *field.data() {
                Data::Bool(true) => constants::COMPACT_FIELD_BOOLEAN_TRUE,
                Data::Bool(false) => constants::COMPACT_FIELD_BOOLEAN_FALSE,
                Data::I8(_) => constants::COMPACT_FIELD_I8,
                Data::I16(_) => constants::COMPACT_FIELD_I16,
                Data::I32(_) => constants::COMPACT_FIELD_I32,
                Data::I64(_) => constants::COMPACT_FIELD_I64,
                Data::Double(_) => constants::COMPACT_FIELD_DOUBLE,
                Data::Binary(_) => constants::COMPACT_FIELD_BINARY,
                Data::Struct(_) => constants::COMPACT_FIELD_STRUCT,
                Data::Map(_) => constants::COMPACT_FIELD_MAP,
                Data::Set(_) => constants::COMPACT_FIELD_SET,
                Data::List(_) => constants::COMPACT_FIELD_LIST,
            };
            track_io!(writer.write_u8((delta << 4) as u8 | kind))?;
            if delta == 0 {
                track!(field.id().compact_encode(writer))?;
            }
            if field.data().kind() != DataKind::Bool {
                track!(field.data().compact_encode(writer))?;
            }
            prev_field_id = field.id();
        }
        track_io!(writer.write_u8(0))?;
        Ok(())
    }
}
impl CompactEncode for Map {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_assert!(self.len() <= 0x7FFF_FFFF, ErrorKind::InvalidInput);
        track!((self.len() as i32).compact_encode(writer))?;
        for (i, (k, v)) in self.iter().enumerate() {
            if i == 0 {
                track_io!(writer.write_u8(((k.kind() as u8) << 4) | v.kind() as u8))?;
            }
            track!(k.compact_encode(writer))?;
            track!(v.compact_encode(writer))?;
        }
        Ok(())
    }
}
impl CompactEncode for Set {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        let len = self.len();
        track_assert!(len <= 0x7FFF_FFFF, ErrorKind::InvalidInput);
        if len < 15 {
            track_io!(writer.write_u8((len << 4) as u8 | self.kind() as u8))?;
        } else {
            track_io!(writer.write_u8(0b1111_0000 | self.kind() as u8))?;
            track!(write_varint(writer, len as u64))?;
        }
        for e in self.iter() {
            track!(e.compact_encode(writer))?;
        }
        Ok(())
    }
}
impl CompactEncode for List {
    fn compact_encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        let len = self.len();
        track_assert!(len <= 0x7FFF_FFFF, ErrorKind::InvalidInput);
        if len < 15 {
            track_io!(writer.write_u8((len << 4) as u8 | self.kind() as u8))?;
        } else {
            track_io!(writer.write_u8(0b1111_0000 | self.kind() as u8))?;
            track!(write_varint(writer, len as u64))?;
        }
        for e in self.iter() {
            track!(e.compact_encode(writer))?;
        }
        Ok(())
    }
}

// [NOTE]
//
// The [specification] says "We are using big-endian",
// but actually, implementations are using little-endian.
// (e.g., https://github.com/apache/thrift/blob/8b8a8efea13d1c97f856053af0a5c0e6a8a76354/lib/java/src/org/apache/thrift/protocol/TCompactProtocol.java#L435)
//
// [specification]: https://github.com/apache/thrift/blob/8b8a8efea13d1c97f856053af0a5c0e6a8a76354/doc/specs/thrift-compact-protocol.md
fn write_varint<W: Write>(writer: &mut W, mut n: u64) -> Result<()> {
    loop {
        let mut b = (n & 0b0111_1111) as u8;
        n >>= 7;
        if n != 0 {
            b |= 0b1000_0000;
        }
        track_io!(writer.write_u8(b))?;
        if n == 0 {
            break;
        }
    }
    Ok(())
}
