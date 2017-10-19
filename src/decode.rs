use std::io::Read;
use byteorder::{ReadBytesExt, BigEndian, LittleEndian};

use {Result, Error, ErrorKind};
use constants;
use message::{Message, MessageKind};
use data::{Data, DataKind, Struct, Map, Set, List, Field, Elements};
use zigzag;

/// This trait allows to decode objects which encoded by the [Thrift Binary protocol encoding][encoding].
///
/// [encoding]: https://github.com/apache/thrift/blob/master/doc/specs/thrift-binary-protocol.md
pub trait BinaryDecode: Sized {
    /// Decodes an object.
    fn binary_decode<R: Read>(reader: &mut R) -> Result<Self>;
}
impl BinaryDecode for bool {
    fn binary_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let b = track_io!(reader.read_u8())?;
        track_assert!(b < 2, ErrorKind::InvalidInput, "b={}", b);
        Ok(b == 1)
    }
}
impl BinaryDecode for i8 {
    fn binary_decode<R: Read>(reader: &mut R) -> Result<Self> {
        track_io!(reader.read_i8())
    }
}
impl BinaryDecode for i16 {
    fn binary_decode<R: Read>(reader: &mut R) -> Result<Self> {
        track_io!(reader.read_i16::<BigEndian>())
    }
}
impl BinaryDecode for i32 {
    fn binary_decode<R: Read>(reader: &mut R) -> Result<Self> {
        track_io!(reader.read_i32::<BigEndian>())
    }
}
impl BinaryDecode for i64 {
    fn binary_decode<R: Read>(reader: &mut R) -> Result<Self> {
        track_io!(reader.read_i64::<BigEndian>())
    }
}
impl BinaryDecode for f64 {
    fn binary_decode<R: Read>(reader: &mut R) -> Result<Self> {
        track_io!(reader.read_f64::<BigEndian>())
    }
}
impl BinaryDecode for Vec<u8> {
    fn binary_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let size = track_io!(reader.read_i32::<BigEndian>())?;
        track_assert!(size >= 0, ErrorKind::InvalidInput, "size={}", size);

        let mut buf = vec![0; size as usize];
        track_io!(reader.read_exact(&mut buf))?;
        Ok(buf)
    }
}
impl BinaryDecode for Message {
    fn binary_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let version = track_io!(reader.read_u16::<BigEndian>())?;
        track_assert_eq!(
            version >> 15,
            1,
            ErrorKind::Other,
            "Old format is unsupported"
        );
        track_assert_eq!(
            version & 0x7FFF,
            constants::BINARY_PROTOCOL_VERSION,
            ErrorKind::InvalidInput
        );
        let _unused = track_io!(reader.read_u8())?;
        let kind = track_io!(reader.read_u8())? & 0b111;
        let kind = track_assert_some!(
            MessageKind::from_u8(kind),
            ErrorKind::InvalidInput,
            "kind={}",
            kind
        );
        let name = track!(Vec::binary_decode(reader))?;
        let name = track!(String::from_utf8(name).map_err(Error::from))?;
        let sequence_id = track_io!(reader.read_i32::<BigEndian>())?;
        let body = track!(Struct::binary_decode(reader))?;
        Ok(Message::new(&name, kind, sequence_id, body))
    }
}
impl BinaryDecode for Struct {
    fn binary_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let mut fields = Vec::new();
        loop {
            let kind = track_io!(reader.read_u8())?;
            if kind == 0 {
                break;
            }
            let kind = track_assert_some!(DataKind::from_u8(kind), ErrorKind::InvalidInput);

            let id = track_io!(reader.read_i16::<BigEndian>())?;
            let data = track!(binary_decode_data(reader, kind))?;
            fields.push(Field::new(id, data));
        }
        Ok(Struct::new(fields))
    }
}
impl BinaryDecode for Map {
    fn binary_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let key_kind = track_io!(reader.read_u8())?;
        let key_kind = track_assert_some!(DataKind::from_u8(key_kind), ErrorKind::InvalidInput);
        let value_kind = track_io!(reader.read_u8())?;
        let value_kind = track_assert_some!(DataKind::from_u8(value_kind), ErrorKind::InvalidInput);
        let size = track_io!(reader.read_i32::<BigEndian>())?;
        track_assert!(size >= 0, ErrorKind::InvalidInput, "size={}", size);

        let mut keys = Elements::new(key_kind);
        let mut values = Elements::new(value_kind);
        for i in 0..size {
            track!(binary_decode_element(reader, &mut keys), "i={}", i)?;
            track!(binary_decode_element(reader, &mut values), "i={}", i)?;
        }
        track!(Map::from_keys_and_values(keys, values))
    }
}
impl BinaryDecode for Set {
    fn binary_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let kind = track_io!(reader.read_u8())?;
        let kind = track_assert_some!(DataKind::from_u8(kind), ErrorKind::InvalidInput);
        let size = track_io!(reader.read_i32::<BigEndian>())?;
        track_assert!(size >= 0, ErrorKind::InvalidInput, "size={}", size);

        let mut elements = Elements::new(kind);
        for i in 0..size {
            track!(binary_decode_element(reader, &mut elements), "i={}", i)?;
        }
        Ok(Set::new(elements))
    }
}
impl BinaryDecode for List {
    fn binary_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let kind = track_io!(reader.read_u8())?;
        let kind = track_assert_some!(DataKind::from_u8(kind), ErrorKind::InvalidInput);
        let size = track_io!(reader.read_i32::<BigEndian>())?;
        track_assert!(size >= 0, ErrorKind::InvalidInput, "size={}", size);

        let mut elements = Elements::new(kind);
        for i in 0..size {
            track!(binary_decode_element(reader, &mut elements), "i={}", i)?;
        }
        Ok(List::new(elements))
    }
}
fn binary_decode_data<R: Read>(reader: &mut R, kind: DataKind) -> Result<Data> {
    let data = match kind {
        DataKind::Bool => Data::Bool(track!(BinaryDecode::binary_decode(reader))?),
        DataKind::I8 => Data::I8(track!(BinaryDecode::binary_decode(reader))?),
        DataKind::I16 => Data::I16(track!(BinaryDecode::binary_decode(reader))?),
        DataKind::I32 => Data::I32(track!(BinaryDecode::binary_decode(reader))?),
        DataKind::I64 => Data::I64(track!(BinaryDecode::binary_decode(reader))?),
        DataKind::Double => Data::Double(track!(BinaryDecode::binary_decode(reader))?),
        DataKind::Binary => Data::Binary(track!(BinaryDecode::binary_decode(reader))?),
        DataKind::Struct => Data::Struct(track!(BinaryDecode::binary_decode(reader))?),
        DataKind::Map => Data::Map(track!(BinaryDecode::binary_decode(reader))?),
        DataKind::Set => Data::Set(track!(BinaryDecode::binary_decode(reader))?),
        DataKind::List => Data::List(track!(BinaryDecode::binary_decode(reader))?),
    };
    Ok(data)
}
fn binary_decode_element<R: Read>(reader: &mut R, elements: &mut Elements) -> Result<()> {
    match *elements {
        Elements::Bool(ref mut v) => v.push(track!(BinaryDecode::binary_decode(reader))?),
        Elements::I8(ref mut v) => v.push(track!(BinaryDecode::binary_decode(reader))?),
        Elements::I16(ref mut v) => v.push(track!(BinaryDecode::binary_decode(reader))?),
        Elements::I32(ref mut v) => v.push(track!(BinaryDecode::binary_decode(reader))?),
        Elements::I64(ref mut v) => v.push(track!(BinaryDecode::binary_decode(reader))?),
        Elements::Double(ref mut v) => v.push(track!(BinaryDecode::binary_decode(reader))?),
        Elements::Binary(ref mut v) => v.push(track!(BinaryDecode::binary_decode(reader))?),
        Elements::Struct(ref mut v) => v.push(track!(BinaryDecode::binary_decode(reader))?),
        Elements::Map(ref mut v) => v.push(track!(BinaryDecode::binary_decode(reader))?),
        Elements::Set(ref mut v) => v.push(track!(BinaryDecode::binary_decode(reader))?),
        Elements::List(ref mut v) => v.push(track!(BinaryDecode::binary_decode(reader))?),
    };
    Ok(())
}

/// This trait allows to decode objects which encoded by the [Thrift Compact protocol encoding][encoding].
///
/// [encoding]: https://github.com/apache/thrift/blob/master/doc/specs/thrift-compact-protocol.md
pub trait CompactDecode: Sized {
    /// Decodes an object.
    fn compact_decode<R: Read>(reader: &mut R) -> Result<Self>;
}
impl CompactDecode for bool {
    fn compact_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let b = track_io!(reader.read_u8())?;
        track_assert!(b < 2, ErrorKind::InvalidInput, "b={}", b);
        Ok(b == 1)
    }
}
impl CompactDecode for i8 {
    fn compact_decode<R: Read>(reader: &mut R) -> Result<Self> {
        track_io!(reader.read_i8())
    }
}
impl CompactDecode for i16 {
    fn compact_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let v = track!(i32::compact_decode(reader))?;
        track_assert_eq!(v, i32::from(v as i16), ErrorKind::InvalidInput);
        Ok(v as i16)
    }
}
impl CompactDecode for i32 {
    fn compact_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let n = track!(read_varint(reader))?;
        track_assert!(n <= 0xFFFF_FFFF, ErrorKind::InvalidInput);
        Ok(zigzag::to_i32(n as u32))
    }
}
impl CompactDecode for i64 {
    fn compact_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let n = track!(read_varint(reader))?;
        Ok(zigzag::to_i64(n))
    }
}
impl CompactDecode for f64 {
    fn compact_decode<R: Read>(reader: &mut R) -> Result<Self> {
        // [NOTE]
        //
        // The [specification] says "We are using big-endian",
        // but actually, implementations are using little-endian.
        // (e.g., https://github.com/apache/thrift/blob/8b8a8efea13d1c97f856053af0a5c0e6a8a76354/lib/java/src/org/apache/thrift/protocol/TCompactProtocol.java#L845)
        //
        // [specification]: https://github.com/apache/thrift/blob/8b8a8efea13d1c97f856053af0a5c0e6a8a76354/doc/specs/thrift-compact-protocol.md
        track_io!(reader.read_f64::<LittleEndian>())
    }
}
impl CompactDecode for Vec<u8> {
    fn compact_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let size = track!(read_varint(reader))?;
        track_assert!(size <= 0x7FFF_FFFF, ErrorKind::InvalidInput);

        let mut buf = vec![0; size as usize];
        track_io!(reader.read_exact(&mut buf[..]))?;
        Ok(buf)
    }
}
impl CompactDecode for Message {
    fn compact_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let protocol_id = track_io!(reader.read_u8())?;
        track_assert_eq!(
            protocol_id,
            constants::COMPACT_PROTOCOL_ID,
            ErrorKind::InvalidInput
        );

        let kind_and_version = track_io!(reader.read_u8())?;
        let kind = track_assert_some!(
            MessageKind::from_u8(kind_and_version >> 5),
            ErrorKind::InvalidInput
        );
        track_assert_eq!(
            kind_and_version & 0b1_1111,
            constants::COMPACT_PROTOCOL_VERSION,
            ErrorKind::InvalidInput
        );

        let sequence_id = track!(read_varint(reader))?;
        track_assert!(sequence_id <= 0xFFFF_FFFF, ErrorKind::InvalidInput);
        let sequence_id = sequence_id as i32;

        let name = track!(Vec::compact_decode(reader))?;
        let name = track!(String::from_utf8(name).map_err(Error::from))?;

        let body = track!(Struct::compact_decode(reader))?;
        Ok(Message::new(&name, kind, sequence_id, body))
    }
}
impl CompactDecode for Struct {
    fn compact_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let mut prev_id = 0;
        let mut fields = Vec::new();
        loop {
            let b = track_io!(reader.read_u8())?;
            if b == 0 {
                break;
            }
            let id_delta = b >> 4;
            let kind = b & 0b1111;
            let id = if id_delta != 0 {
                prev_id + i16::from(id_delta)
            } else {
                track!(i16::compact_decode(reader))?
            };
            prev_id = id;
            let data = match kind {
                constants::COMPACT_FIELD_BOOLEAN_TRUE => Data::Bool(true),
                constants::COMPACT_FIELD_BOOLEAN_FALSE => Data::Bool(false),
                constants::COMPACT_FIELD_I8 => Data::I8(
                    track!(CompactDecode::compact_decode(reader))?,
                ),
                constants::COMPACT_FIELD_I16 => Data::I16(
                    track!(CompactDecode::compact_decode(reader))?,
                ),
                constants::COMPACT_FIELD_I32 => Data::I32(
                    track!(CompactDecode::compact_decode(reader))?,
                ),
                constants::COMPACT_FIELD_I64 => Data::I64(
                    track!(CompactDecode::compact_decode(reader))?,
                ),
                constants::COMPACT_FIELD_DOUBLE => Data::Double(
                    track!(CompactDecode::compact_decode(reader))?,
                ),
                constants::COMPACT_FIELD_BINARY => Data::Binary(
                    track!(CompactDecode::compact_decode(reader))?,
                ),
                constants::COMPACT_FIELD_LIST => Data::List(
                    track!(CompactDecode::compact_decode(reader))?,
                ),
                constants::COMPACT_FIELD_SET => Data::Set(
                    track!(CompactDecode::compact_decode(reader))?,
                ),
                constants::COMPACT_FIELD_MAP => Data::Map(
                    track!(CompactDecode::compact_decode(reader))?,
                ),
                constants::COMPACT_FIELD_STRUCT => Data::Struct(
                    track!(CompactDecode::compact_decode(reader))?,
                ),
                _ => track_panic!(ErrorKind::InvalidInput, "kind={}", kind),
            };
            fields.push(Field::new(id, data));
        }
        Ok(Struct::new(fields))
    }
}
impl CompactDecode for Map {
    fn compact_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let first = track_io!(reader.read_u8())?;
        if first == 0 {
            Ok(Map::empty())
        } else {
            let size = {
                let first = [first];
                let mut reader = first.chain(&mut *reader);
                track!(read_varint(&mut reader))?
            };
            track_assert!(size <= 0x7FFF_FFFF, ErrorKind::InvalidInput);

            let kinds = track_io!(reader.read_u8())?;
            let key_kind =
                track_assert_some!(DataKind::from_u8(kinds >> 4), ErrorKind::InvalidInput);
            let value_kind =
                track_assert_some!(DataKind::from_u8(kinds & 0b1111), ErrorKind::InvalidInput);

            let mut keys = Elements::new(key_kind);
            let mut values = Elements::new(value_kind);
            for i in 0..size {
                track!(compact_decode_element(reader, &mut keys), "i={}", i)?;
                track!(compact_decode_element(reader, &mut values), "i={}", i)?;
            }
            track!(Map::from_keys_and_values(keys, values))
        }
    }
}
impl CompactDecode for Set {
    fn compact_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let size_and_kind = track_io!(reader.read_u8())?;
        let mut size = i32::from(size_and_kind >> 4);
        let kind = size_and_kind & 0b1111;
        if size == 0b1111 {
            size = track!(i32::compact_decode(reader))?;
            track_assert!(15 <= size, ErrorKind::InvalidInput, "size={}", size);
        }
        let kind = track_assert_some!(DataKind::from_u8(kind), ErrorKind::InvalidInput);

        let mut elements = Elements::new(kind);
        for i in 0..size {
            track!(compact_decode_element(reader, &mut elements), "i={}", i)?;
        }
        Ok(Set::new(elements))
    }
}
impl CompactDecode for List {
    fn compact_decode<R: Read>(reader: &mut R) -> Result<Self> {
        let size_and_kind = track_io!(reader.read_u8())?;
        let mut size = i32::from(size_and_kind >> 4);
        let kind = size_and_kind & 0b1111;
        if size == 0b1111 {
            size = track!(i32::compact_decode(reader))?;
            track_assert!(15 <= size, ErrorKind::InvalidInput, "size={}", size);
        }
        let kind = track_assert_some!(DataKind::from_u8(kind), ErrorKind::InvalidInput);

        let mut elements = Elements::new(kind);
        for i in 0..size {
            track!(compact_decode_element(reader, &mut elements), "i={}", i)?;
        }
        Ok(List::new(elements))
    }
}


// [NOTE]
//
// The [specification] says "We are using big-endian",
// but actually, implementations are using little-endian.
// (e.g., https://github.com/apache/thrift/blob/8b8a8efea13d1c97f856053af0a5c0e6a8a76354/lib/java/src/org/apache/thrift/protocol/TCompactProtocol.java#L796)
//
// [specification]: https://github.com/apache/thrift/blob/8b8a8efea13d1c97f856053af0a5c0e6a8a76354/doc/specs/thrift-compact-protocol.md
fn read_varint<R: Read>(reader: &mut R) -> Result<u64> {
    let mut n = 0;
    for i in 0.. {
        track_assert!(i < 10, ErrorKind::InvalidInput);
        let b = track_io!(reader.read_u8())?;
        n += u64::from(b & 0b0111_1111) << (i * 7);
        if (b & 0b1000_0000) == 0 {
            break;
        }
    }
    Ok(n)
}

fn compact_decode_element<R: Read>(reader: &mut R, elements: &mut Elements) -> Result<()> {
    match *elements {
        Elements::Bool(ref mut v) => v.push(track!(CompactDecode::compact_decode(reader))?),
        Elements::I8(ref mut v) => v.push(track!(CompactDecode::compact_decode(reader))?),
        Elements::I16(ref mut v) => v.push(track!(CompactDecode::compact_decode(reader))?),
        Elements::I32(ref mut v) => v.push(track!(CompactDecode::compact_decode(reader))?),
        Elements::I64(ref mut v) => v.push(track!(CompactDecode::compact_decode(reader))?),
        Elements::Double(ref mut v) => v.push(track!(CompactDecode::compact_decode(reader))?),
        Elements::Binary(ref mut v) => v.push(track!(CompactDecode::compact_decode(reader))?),
        Elements::Struct(ref mut v) => v.push(track!(CompactDecode::compact_decode(reader))?),
        Elements::Map(ref mut v) => v.push(track!(CompactDecode::compact_decode(reader))?),
        Elements::Set(ref mut v) => v.push(track!(CompactDecode::compact_decode(reader))?),
        Elements::List(ref mut v) => v.push(track!(CompactDecode::compact_decode(reader))?),
    };
    Ok(())
}
