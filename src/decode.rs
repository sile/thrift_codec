use std;
use std::io::Read;
use byteorder::{ReadBytesExt, BigEndian};
use trackable::error::ErrorKindExt;

use {Result, ErrorKind};
use collections::{List, Set, Map};
use constants::compact::*;
use message::{Message, MessageKind};
use structure::{Struct, Field};
use value::{Value, Values};

pub trait CompactDecode: Sized {
    fn decode<R: Read>(reader: &mut R) -> Result<Self>;
}
impl CompactDecode for bool {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let n = track_io!(reader.read_u8())?;
        track_assert!(n < 2, ErrorKind::InvalidInput, "n={}", n);
        Ok(n == 1)
    }
}
impl CompactDecode for u8 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        track_io!(reader.read_u8())
    }
}
impl CompactDecode for i8 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        track_io!(reader.read_i8())
    }
}
impl CompactDecode for i16 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let n = track!(i32::decode(reader))?;
        track_assert!(
            std::i16::MIN as i32 <= n && n <= std::i16::MAX as i32,
            ErrorKind::InvalidInput,
            "n={}",
            n
        );
        Ok(n as i16)
    }
}
impl CompactDecode for i32 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let n = track_io!(read_var32(reader))?;
        Ok(zigzag_to_i32(n))
    }
}
impl CompactDecode for i64 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let n = track_io!(read_var64(reader))?;
        Ok(zigzag_to_i64(n))
    }
}
impl CompactDecode for f64 {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        track_io!(reader.read_f64::<BigEndian>())
    }
}
impl CompactDecode for Vec<u8> {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let len = track_io!(read_var32(reader))? as usize;
        let mut buf = vec![0; len];
        track_io!(reader.read_exact(&mut buf))?;
        Ok(buf)
    }
}
impl CompactDecode for String {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let buf = track!(Vec::decode(reader))?;
        track!(String::from_utf8(buf).map_err(|e| {
            ErrorKind::InvalidInput.cause(e).into()
        }))
    }
}
impl CompactDecode for Message {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let protocol_id = track_io!(reader.read_u8())?;
        track_assert_eq!(protocol_id, PROTOCOL_ID, ErrorKind::InvalidInput);

        let b = track_io!(reader.read_u8())?;
        let kind = track_assert_some!(MessageKind::from_u8(b >> 5), ErrorKind::InvalidInput);
        let version = b & 0b0001_1111;
        track_assert_eq!(version, VERSION, ErrorKind::InvalidInput);

        let seq_id = track!(read_var32(reader))?;
        let name = track!(String::decode(reader))?;
        let body = track!(Struct::decode(reader), "name={:?}", name)?;

        Ok(Message {
            name,
            kind,
            seq_id,
            body,
        })
    }
}
impl CompactDecode for Struct {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let mut prev_field_id = 0;
        let mut fields = Vec::new();
        loop {
            let b = track_io!(reader.read_u8())?;
            if b == 0 {
                break;
            }
            let field_id_delta = b >> 4;
            let field_type = b & 0b1111;
            let field_id = if field_id_delta != 0 {
                prev_field_id + field_id_delta as i16
            } else {
                track!(i16::decode(reader))?
            };
            prev_field_id = field_id;
            let value = match field_type {
                FIELD_TYPE_BOOLEAN_TRUE => Value::Bool(true),
                FIELD_TYPE_BOOLEAN_FALSE => Value::Bool(false),
                FIELD_TYPE_BYTE => Value::Byte(track!(u8::decode(reader))?),
                FIELD_TYPE_I16 => Value::I16(track!(i16::decode(reader))?),
                FIELD_TYPE_I32 => Value::I32(track!(i32::decode(reader))?),
                FIELD_TYPE_I64 => Value::I64(track!(i64::decode(reader))?),
                FIELD_TYPE_DOUBLE => Value::Double(track!(f64::decode(reader))?),
                FIELD_TYPE_BINARY => Value::String(track!(String::decode(reader))?), // TODO
                FIELD_TYPE_LIST => Value::List(track!(List::decode(reader))?),
                FIELD_TYPE_SET => Value::Set(track!(Set::decode(reader))?),
                FIELD_TYPE_MAP => Value::Map(track!(Map::decode(reader))?),
                FIELD_TYPE_STRUCT => Value::Struct(track!(Struct::decode(reader))?),
                _ => track_panic!(ErrorKind::InvalidInput, "field={}", field_type),
            };
            fields.push(Field {
                id: field_id,
                value,
            });
        }
        Ok(Struct { fields })
    }
}
impl CompactDecode for List {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let b = track_io!(reader.read_u8())?;
        let mut size = (b >> 4) as usize;
        let element_type = b & 0b1111;
        if size == 0b1111 {
            size = track!(i32::decode(reader))? as usize;
            track_assert!(15 <= size, ErrorKind::InvalidInput, "size={}", size);
        }

        fn values<T: CompactDecode, R: Read>(reader: &mut R, size: usize) -> Result<Vec<T>> {
            (0..size)
                .map(|_| track!(T::decode(reader), "size={}", size))
                .collect()
        }
        let elements = match element_type {
            ELEMENT_TYPE_BOOL => track!(values(reader, size)).map(Values::Bool)?,
            ELEMENT_TYPE_BYTE => track!(values(reader, size)).map(Values::Byte)?,
            ELEMENT_TYPE_DOUBLE => track!(values(reader, size)).map(Values::Double)?,
            ELEMENT_TYPE_I16 => track!(values(reader, size)).map(Values::I16)?,
            ELEMENT_TYPE_I32 => track!(values(reader, size)).map(Values::I32)?,
            ELEMENT_TYPE_I64 => track!(values(reader, size)).map(Values::I64)?,
            ELEMENT_TYPE_STRING => track!(values(reader, size)).map(Values::String)?,
            ELEMENT_TYPE_STRUCT => track!(values(reader, size)).map(Values::Struct)?,
            ELEMENT_TYPE_MAP => track!(values(reader, size)).map(Values::Map)?,
            ELEMENT_TYPE_SET => track!(values(reader, size)).map(Values::Set)?,
            ELEMENT_TYPE_LIST => track!(values(reader, size)).map(Values::List)?,
            _ => track_panic!(ErrorKind::InvalidInput, "type={}", element_type),
        };
        Ok(List { elements })
    }
}
impl CompactDecode for Set {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        let b = track_io!(reader.read_u8())?;
        let mut size = (b >> 4) as usize;
        let element_type = b & 0b1111;
        if size == 0b1111 {
            size = track!(i32::decode(reader))? as usize;
            track_assert!(15 <= size, ErrorKind::InvalidInput, "size={}", size);
        }

        fn values<T: CompactDecode, R: Read>(reader: &mut R, size: usize) -> Result<Vec<T>> {
            (0..size).map(|_| track!(T::decode(reader))).collect()
        }
        let elements = match element_type {
            ELEMENT_TYPE_BOOL => track!(values(reader, size)).map(Values::Bool)?,
            ELEMENT_TYPE_BYTE => track!(values(reader, size)).map(Values::Byte)?,            
            ELEMENT_TYPE_DOUBLE => track!(values(reader, size)).map(Values::Double)?,
            ELEMENT_TYPE_I16 => track!(values(reader, size)).map(Values::I16)?,
            ELEMENT_TYPE_I32 => track!(values(reader, size)).map(Values::I32)?,
            ELEMENT_TYPE_I64 => track!(values(reader, size)).map(Values::I64)?,
            ELEMENT_TYPE_STRING => track!(values(reader, size)).map(Values::String)?,
            ELEMENT_TYPE_STRUCT => track!(values(reader, size)).map(Values::Struct)?,
            ELEMENT_TYPE_MAP => track!(values(reader, size)).map(Values::Map)?,
            ELEMENT_TYPE_SET => track!(values(reader, size)).map(Values::Set)?,
            ELEMENT_TYPE_LIST => track!(values(reader, size)).map(Values::List)?,
            _ => track_panic!(ErrorKind::InvalidInput, "type={}", element_type),
        };
        Ok(Set { elements })
    }
}
impl CompactDecode for Map {
    fn decode<R: Read>(reader: &mut R) -> Result<Self> {
        // TODO:
        let size = track!(i32::decode(reader))?;
        if size == 0 {
            // XXX:
            Ok(Map {
                keys: Values::Bool(Vec::new()),
                values: Values::Bool(Vec::new()),
            })
        } else {
            track_assert!(size > 0, ErrorKind::InvalidInput);
            let b = track_io!(reader.read_u8())?;
            let key_type = b >> 4;
            let value_type = b & 0b1111;

            fn init_values(kind: u8) -> Result<Values> {
                // TODO: with_capacity
                match kind {
                    ELEMENT_TYPE_BOOL => Ok(Values::Bool(Vec::new())),
                    ELEMENT_TYPE_BYTE => Ok(Values::Byte(Vec::new())),
                    ELEMENT_TYPE_DOUBLE => Ok(Values::Double(Vec::new())),
                    ELEMENT_TYPE_I16 => Ok(Values::I16(Vec::new())),
                    ELEMENT_TYPE_I32 => Ok(Values::I32(Vec::new())),
                    ELEMENT_TYPE_I64 => Ok(Values::I64(Vec::new())),
                    ELEMENT_TYPE_STRING => Ok(Values::String(Vec::new())),
                    ELEMENT_TYPE_STRUCT => Ok(Values::Struct(Vec::new())),
                    ELEMENT_TYPE_MAP => Ok(Values::Map(Vec::new())),
                    ELEMENT_TYPE_SET => Ok(Values::Set(Vec::new())),
                    ELEMENT_TYPE_LIST => Ok(Values::List(Vec::new())),
                    _ => track_panic!(ErrorKind::InvalidInput, "type={}", kind),
                }
            }
            let mut keys = track!(init_values(key_type))?;
            let mut values = track!(init_values(value_type))?;
            fn push<R: Read>(reader: &mut R, vec: &mut Values) -> Result<()> {
                match *vec {
                    Values::Bool(ref mut v) => v.push(track!(CompactDecode::decode(reader))?),
                    Values::Byte(ref mut v) => v.push(track!(CompactDecode::decode(reader))?),
                    Values::Double(ref mut v) => v.push(track!(CompactDecode::decode(reader))?),
                    Values::I16(ref mut v) => v.push(track!(CompactDecode::decode(reader))?),
                    Values::I32(ref mut v) => v.push(track!(CompactDecode::decode(reader))?),
                    Values::I64(ref mut v) => v.push(track!(CompactDecode::decode(reader))?),
                    Values::String(ref mut v) => v.push(track!(CompactDecode::decode(reader))?),
                    Values::Struct(ref mut v) => v.push(track!(CompactDecode::decode(reader))?),
                    Values::Map(ref mut v) => v.push(track!(CompactDecode::decode(reader))?),
                    Values::Set(ref mut v) => v.push(track!(CompactDecode::decode(reader))?),
                    Values::List(ref mut v) => v.push(track!(CompactDecode::decode(reader))?),
                };
                Ok(())
            }
            for _ in 0..size {
                track!(push(reader, &mut keys))?;
                track!(push(reader, &mut values))?;
            }
            Ok(Map { keys, values })
        }
    }
}

fn read_var32<R: Read>(reader: &mut R) -> Result<u32> {
    // TODO: overflow check
    // TODO: i32 version
    let mut n = 0;
    for i in 0.. {
        track_assert!(i < 5, ErrorKind::InvalidInput);
        let b = track_io!(reader.read_u8())?;
        n += ((b & 0b0111_1111) as u32) << (i * 7);
        if (b & 0b1000_0000) == 0 {
            break;
        }
    }
    Ok(n)
}

fn read_var64<R: Read>(reader: &mut R) -> Result<u64> {
    // TODO: overflow check
    // TODO: i64 version
    let mut n = 0;
    for i in 0.. {
        track_assert!(i < 10, ErrorKind::InvalidInput);
        let b = track_io!(reader.read_u8())?;
        n += ((b & 0b0111_1111) as u64) << (i * 7);
        if (b & 0b1000_0000) == 0 {
            break;
        }
    }
    Ok(n)
}

fn zigzag_to_i32(n: u32) -> i32 {
    (n >> 1) as i32 ^ -(n as i32 & 1)
}

fn zigzag_to_i64(n: u64) -> i64 {
    (n >> 1) as i64 ^ -(n as i64 & 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zigzag_works() {
        assert_eq!(zigzag_to_i32(0), 0);
        assert_eq!(zigzag_to_i32(1), -1);
        assert_eq!(zigzag_to_i32(2), 1);
        assert_eq!(zigzag_to_i32(3), -2);
        assert_eq!(zigzag_to_i32(4), 2);
    }
}
