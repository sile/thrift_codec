use std::io::Write;
use byteorder::{WriteBytesExt, BigEndian};

use Result;
use collections::{List, Set, Map};
use message::Message;
use structure::Struct;
use value::{Value, ValueRef, Values, ValueKind};

pub trait BinaryEncode {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()>;
}
impl BinaryEncode for i8 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_i8(*self))?;
        Ok(())
    }
}
impl BinaryEncode for u8 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_u8(*self))?;
        Ok(())
    }
}
impl BinaryEncode for i16 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_i16::<BigEndian>(*self))?;
        Ok(())
    }
}
impl BinaryEncode for i32 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_i32::<BigEndian>(*self))?;
        Ok(())
    }
}
impl BinaryEncode for i64 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_i64::<BigEndian>(*self))?;
        Ok(())
    }
}
impl BinaryEncode for f64 {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_f64::<BigEndian>(*self))?;
        Ok(())
    }
}
impl BinaryEncode for bool {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!((*self as i8).encode(writer))
    }
}
impl<'a> BinaryEncode for &'a [u8] {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        assert!(self.len() <= 0x7FFF_FFFF);
        track_io!(writer.write_u32::<BigEndian>(self.len() as u32))?;
        track_io!(writer.write_all(self))?;
        Ok(())
    }
}
impl<'a> BinaryEncode for &'a str {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!(self.as_bytes().encode(writer))
    }
}
impl BinaryEncode for String {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!(self.as_bytes().encode(writer))
    }
}
impl BinaryEncode for Message {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_u16::<BigEndian>(0b1000_0000_0000_0001))?; // version
        track_io!(writer.write_u8(0))?; // unused
        track_io!(writer.write_u8(self.kind as u8))?;
        track!(self.name.as_str().encode(writer))?;
        track!((self.seq_id as i32).encode(writer))?;
        track!(self.body.encode(writer))?;
        Ok(())
    }
}
impl BinaryEncode for ValueKind {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track_io!(writer.write_u8(*self as u8))?;
        Ok(())
    }
}
impl BinaryEncode for Value {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        match *self {
            Value::Bool(x) => track!(x.encode(writer)),
            Value::Byte(x) => track!(x.encode(writer)),
            Value::Double(x) => track!(x.encode(writer)),
            Value::I16(x) => track!(x.encode(writer)),
            Value::I32(x) => track!(x.encode(writer)),
            Value::I64(x) => track!(x.encode(writer)),
            Value::String(ref x) => track!(x.encode(writer)),
            Value::Struct(ref x) => track!(x.encode(writer)),
            Value::Map(ref x) => track!(x.encode(writer)),
            Value::Set(ref x) => track!(x.encode(writer)),
            Value::List(ref x) => track!(x.encode(writer)),
        }
    }
}
impl<'a> BinaryEncode for ValueRef<'a> {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        match *self {
            ValueRef::Bool(x) => track!(x.encode(writer)),
            ValueRef::Byte(x) => track!(x.encode(writer)),
            ValueRef::Double(x) => track!(x.encode(writer)),
            ValueRef::I16(x) => track!(x.encode(writer)),
            ValueRef::I32(x) => track!(x.encode(writer)),
            ValueRef::I64(x) => track!(x.encode(writer)),
            ValueRef::String(x) => track!(x.encode(writer)),
            ValueRef::Struct(x) => track!(x.encode(writer)),
            ValueRef::Map(x) => track!(x.encode(writer)),
            ValueRef::Set(x) => track!(x.encode(writer)),
            ValueRef::List(x) => track!(x.encode(writer)),
        }
    }
}
impl BinaryEncode for Values {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        match *self {
            Values::Bool(ref v) => {
                for x in v {
                    track!(x.encode(writer))?;
                }
            }
            Values::Byte(ref v) => {
                for x in v {
                    track!(x.encode(writer))?;
                }
            }
            Values::Double(ref v) => {
                for x in v {
                    track!(x.encode(writer))?;
                }
            }
            Values::I16(ref v) => {
                for x in v {
                    track!(x.encode(writer))?;
                }
            }
            Values::I32(ref v) => {
                for x in v {
                    track!(x.encode(writer))?;
                }
            }
            Values::I64(ref v) => {
                for x in v {
                    track!(x.encode(writer))?;
                }
            }
            Values::String(ref v) => {
                for x in v {
                    track!(x.encode(writer))?;
                }
            }
            Values::Struct(ref v) => {
                for x in v {
                    track!(x.encode(writer))?;
                }
            }
            Values::Map(ref v) => {
                for x in v {
                    track!(x.encode(writer))?;
                }
            }
            Values::Set(ref v) => {
                for x in v {
                    track!(x.encode(writer))?;
                }
            }
            Values::List(ref v) => {
                for x in v {
                    track!(x.encode(writer))?;
                }
            }
        }
        Ok(())
    }
}
impl BinaryEncode for Struct {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        for field in &self.fields {
            track!(field.value.kind().encode(writer))?;
            track!((field.id as i16).encode(writer))?;
            track!(field.value.encode(writer))?;
        }
        track_io!(writer.write_u8(0))?; // stop field
        Ok(())
    }
}
impl BinaryEncode for List {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!(self.elements.kind().encode(writer))?;
        track!((self.elements.len() as i32).encode(writer))?;
        track!(self.elements.encode(writer))?;
        Ok(())
    }
}
impl BinaryEncode for Set {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        track!(self.elements.kind().encode(writer))?;
        track!((self.elements.len() as i32).encode(writer))?;
        track!(self.elements.encode(writer))?;
        Ok(())
    }
}
impl BinaryEncode for Map {
    fn encode<W: Write>(&self, writer: &mut W) -> Result<()> {
        assert_eq!(self.keys.len(), self.values.len());
        track!(self.keys.kind().encode(writer))?;
        track!(self.values.kind().encode(writer))?;
        track!((self.keys.len() as i32).encode(writer))?;
        for i in 0..self.keys.len() {
            track!(self.keys.get(i).unwrap().encode(writer))?;
            track!(self.values.get(i).unwrap().encode(writer))?;
        }
        Ok(())
    }
}
