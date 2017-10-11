use std::io::Write;
use byteorder::{WriteBytesExt, BigEndian};

use Result;
use collections::{Map, Set, List};
use constants::compact::*;
use message::Message;
use structure::Struct;
use value::{Value, ValueRef, Values};

pub trait CompactEncode {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()>;
}
impl CompactEncode for i8 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_i8(*self))
    }
}
impl CompactEncode for u8 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_u8(*self))
    }
}
impl CompactEncode for i16 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!((*self as i32).encode(writer))?;
        Ok(())
    }
}
impl CompactEncode for i32 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!(write_var32(writer, i32_to_zigzag(*self)))?;
        Ok(())
    }
}
impl CompactEncode for i64 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!(write_var64(writer, i64_to_zigzag(*self)))?;
        Ok(())
    }
}
impl CompactEncode for f64 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_f64::<BigEndian>(*self))?;
        Ok(())
    }
}
impl CompactEncode for bool {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!((*self as u8).encode(writer))
    }
}
impl<'a> CompactEncode for &'a [u8] {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!(write_var32(writer, self.len() as u32))?;
        track_io!(writer.write_all(self))?;
        Ok(())
    }
}
impl CompactEncode for Vec<u8> {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!(self.as_slice().encode(writer))
    }
}
impl<'a> CompactEncode for &'a str {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!(self.as_bytes().encode(writer))
    }
}
impl CompactEncode for String {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!(self.as_bytes().encode(writer))
    }
}
impl CompactEncode for Message {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_u8(PROTOCOL_ID))?;
        track_io!(writer.write_u8(((self.kind as u8) << 5) | VERSION))?;
        track!(write_var32(writer, self.seq_id))?;
        track!(self.name.encode(writer))?;
        track!(self.body.encode(writer))?;
        Ok(())
    }
}
impl CompactEncode for Struct {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mut prev_field_id = 0;
        for field in &self.fields {
            let field_id_delta = if field.id > prev_field_id && (field.id - prev_field_id) <= 15 {
                (field.id - prev_field_id) as u8
            } else {
                0b0000
            };
            let field_type = match field.value {
                Value::Bool(true) => FIELD_TYPE_BOOLEAN_TRUE,
                Value::Bool(false) => FIELD_TYPE_BOOLEAN_FALSE,
                Value::Byte(_) => FIELD_TYPE_BYTE,
                Value::Double(_) => FIELD_TYPE_DOUBLE,
                Value::I16(_) => FIELD_TYPE_I16,
                Value::I32(_) => FIELD_TYPE_I32,
                Value::I64(_) => FIELD_TYPE_I64,
                Value::String(_) => FIELD_TYPE_BINARY,
                Value::Struct(_) => FIELD_TYPE_STRUCT,
                Value::List(_) => FIELD_TYPE_LIST,
                Value::Set(_) => FIELD_TYPE_SET,
                Value::Map(_) => FIELD_TYPE_MAP,
            };
            track_io!(writer.write_u8((field_id_delta << 4) | field_type))?;
            if field_id_delta == 0 {
                (field.id as i16).encode(writer)?;
            }
            prev_field_id = field.id;
            match field.value {
                Value::Bool(_) => {}
                Value::Byte(ref v) => track!(v.encode(writer))?,
                Value::Double(ref v) => track!(v.encode(writer))?,
                Value::I16(ref v) => track!(v.encode(writer))?,
                Value::I32(ref v) => track!(v.encode(writer))?,
                Value::I64(ref v) => track!(v.encode(writer))?,
                Value::String(ref v) => track!(v.encode(writer))?,
                Value::Struct(ref v) => track!(v.encode(writer))?,
                Value::List(ref v) => track!(v.encode(writer))?,
                Value::Set(ref v) => track!(v.encode(writer))?,
                Value::Map(ref v) => track!(v.encode(writer))?,
            }
        }
        track_io!(writer.write_u8(0))?; // stop field
        Ok(())
    }
}
impl<'a> CompactEncode for ValueRef<'a> {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        match *self {
            ValueRef::Bool(v) => track!(v.encode(writer)),
            ValueRef::Byte(v) => track!(v.encode(writer)),
            ValueRef::Double(v) => track!(v.encode(writer)),
            ValueRef::I16(v) => track!(v.encode(writer)),
            ValueRef::I32(v) => track!(v.encode(writer)),
            ValueRef::I64(v) => track!(v.encode(writer)),
            ValueRef::String(v) => track!(v.encode(writer)),
            ValueRef::Struct(v) => track!(v.encode(writer)),
            ValueRef::List(v) => track!(v.encode(writer)),
            ValueRef::Set(v) => track!(v.encode(writer)),
            ValueRef::Map(v) => track!(v.encode(writer)),
        }
    }
}
impl CompactEncode for List {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        let kind = element_type(&self.elements);
        let size = self.elements.len();
        if size < 15 {
            track_io!(writer.write_u8((size as u8) << 4 | kind))?;
        } else {
            track_io!(writer.write_u8(0b1111_0000 | kind))?;
            track!(write_var32(writer, size as u32))?;
        }
        for i in 0..size {
            track!(self.elements.get(i).unwrap().encode(writer))?;
        }
        Ok(())
    }
}
impl CompactEncode for Set {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        let kind = element_type(&self.elements);
        let size = self.elements.len();
        if size < 15 {
            track_io!(writer.write_u8((size as u8) << 4 | kind))?;
        } else {
            track_io!(writer.write_u8(0b1111_0000 | kind))?;
            track!(write_var32(writer, size as u32))?;
        }
        for i in 0..size {
            track!(self.elements.get(i).unwrap().encode(writer))?;
        }
        Ok(())
    }
}
impl CompactEncode for Map {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        assert_eq!(self.keys.len(), self.values.len()); // TODO
        if self.keys.len() == 0 {
            track_io!(writer.write_u8(0))?;
        } else {
            track!(write_var32(writer, self.keys.len() as u32))?;

            let key_type = element_type(&self.keys);
            let value_type = element_type(&self.values);
            track_io!(writer.write_u8((key_type << 4) | value_type))?;
        }
        for i in 0..self.keys.len() {
            track!(self.keys.get(i).unwrap().encode(writer))?;
            track!(self.values.get(i).unwrap().encode(writer))?;
        }
        Ok(())
    }
}

fn element_type(v: &Values) -> u8 {
    match *v {
        Values::Bool(_) => ELEMENT_TYPE_BOOL,
        Values::Byte(_) => ELEMENT_TYPE_BYTE,
        Values::Double(_) => ELEMENT_TYPE_DOUBLE,
        Values::I16(_) => ELEMENT_TYPE_I16,
        Values::I32(_) => ELEMENT_TYPE_I32,
        Values::I64(_) => ELEMENT_TYPE_I64,
        Values::String(_) => ELEMENT_TYPE_STRING,
        Values::Struct(_) => ELEMENT_TYPE_STRUCT,
        Values::List(_) => ELEMENT_TYPE_LIST,
        Values::Set(_) => ELEMENT_TYPE_SET,
        Values::Map(_) => ELEMENT_TYPE_MAP,
    }
}

fn write_var32<W: Write>(writer: &mut W, mut n: u32) -> Result<()> {
    // TODO: i32 version
    loop {
        let mut b = (n & 0b0111_1111) as u8;
        n = n >> 7;
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

fn write_var64<W: Write>(writer: &mut W, mut n: u64) -> Result<()> {
    // TODO: i64 version
    loop {
        let mut b = (n & 0b0111_1111) as u8;
        n = n >> 7;
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

fn i32_to_zigzag(n: i32) -> u32 {
    ((n << 1) ^ (n >> 31)) as u32
}

fn i64_to_zigzag(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zigzag_works() {
        assert_eq!(i32_to_zigzag(0), 0);
        assert_eq!(i32_to_zigzag(-1), 1);
        assert_eq!(i32_to_zigzag(1), 2);
        assert_eq!(i32_to_zigzag(-2), 3);
        assert_eq!(i32_to_zigzag(2), 4);
    }
}
