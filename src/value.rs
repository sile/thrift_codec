use collections::{Map, Set, List};
use structure::Struct;

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    Byte(u8),
    Double(f64),
    I16(i16),
    I32(i32),
    I64(i64),
    String(String),
    Struct(Struct),
    Map(Map),
    Set(Set),
    List(List),
}
impl Value {
    pub fn kind(&self) -> ValueKind {
        match *self {
            Value::Bool(_) => ValueKind::Bool,
            Value::Byte(_) => ValueKind::Byte,
            Value::Double(_) => ValueKind::Double,
            Value::I16(_) => ValueKind::I16,
            Value::I32(_) => ValueKind::I32,
            Value::I64(_) => ValueKind::I64,
            Value::String(_) => ValueKind::String,
            Value::Struct(_) => ValueKind::Struct,
            Value::Map(_) => ValueKind::Map,
            Value::Set(_) => ValueKind::Set,
            Value::List(_) => ValueKind::List,
        }
    }
}

#[derive(Debug)]
pub enum ValueRef<'a> {
    Bool(bool),
    Byte(u8),
    Double(f64),
    I16(i16),
    I32(i32),
    I64(i64),
    String(&'a String),
    Struct(&'a Struct),
    Map(&'a Map),
    Set(&'a Set),
    List(&'a List),
}
impl<'a> ValueRef<'a> {
    pub fn kind(&self) -> ValueKind {
        match *self {
            ValueRef::Bool(_) => ValueKind::Bool,
            ValueRef::Byte(_) => ValueKind::Byte,
            ValueRef::Double(_) => ValueKind::Double,
            ValueRef::I16(_) => ValueKind::I16,
            ValueRef::I32(_) => ValueKind::I32,
            ValueRef::I64(_) => ValueKind::I64,
            ValueRef::String(_) => ValueKind::String,
            ValueRef::Struct(_) => ValueKind::Struct,
            ValueRef::Map(_) => ValueKind::Map,
            ValueRef::Set(_) => ValueKind::Set,
            ValueRef::List(_) => ValueKind::List,
        }
    }
}

#[derive(Debug)]
pub enum Values {
    Bool(Vec<bool>),
    Byte(Vec<u8>),
    Double(Vec<f64>),
    I16(Vec<i16>),
    I32(Vec<i32>),
    I64(Vec<i64>),
    String(Vec<String>),
    Struct(Vec<Struct>),
    Map(Vec<Map>),
    Set(Vec<Set>),
    List(Vec<List>),
}
impl Values {
    pub fn get(&self, index: usize) -> Option<ValueRef> {
        match *self {
            Values::Bool(ref v) => v.get(index).cloned().map(ValueRef::Bool),
            Values::Byte(ref v) => v.get(index).cloned().map(ValueRef::Byte),
            Values::Double(ref v) => v.get(index).cloned().map(ValueRef::Double),
            Values::I16(ref v) => v.get(index).cloned().map(ValueRef::I16),
            Values::I32(ref v) => v.get(index).cloned().map(ValueRef::I32),
            Values::I64(ref v) => v.get(index).cloned().map(ValueRef::I64),
            Values::String(ref v) => v.get(index).map(ValueRef::String),
            Values::Struct(ref v) => v.get(index).map(ValueRef::Struct),
            Values::Map(ref v) => v.get(index).map(ValueRef::Map),
            Values::Set(ref v) => v.get(index).map(ValueRef::Set),
            Values::List(ref v) => v.get(index).map(ValueRef::List),
        }
    }
    pub fn kind(&self) -> ValueKind {
        match *self {
            Values::Bool(_) => ValueKind::Bool,
            Values::Byte(_) => ValueKind::Byte,
            Values::Double(_) => ValueKind::Double,
            Values::I16(_) => ValueKind::I16,
            Values::I32(_) => ValueKind::I32,
            Values::I64(_) => ValueKind::I64,
            Values::String(_) => ValueKind::String,
            Values::Struct(_) => ValueKind::Struct,
            Values::Map(_) => ValueKind::Map,
            Values::Set(_) => ValueKind::Set,
            Values::List(_) => ValueKind::List,
        }
    }
    pub fn len(&self) -> usize {
        match *self {
            Values::Bool(ref v) => v.len(),
            Values::Byte(ref v) => v.len(),
            Values::Double(ref v) => v.len(),
            Values::I16(ref v) => v.len(),
            Values::I32(ref v) => v.len(),
            Values::I64(ref v) => v.len(),
            Values::String(ref v) => v.len(),
            Values::Struct(ref v) => v.len(),
            Values::Map(ref v) => v.len(),
            Values::Set(ref v) => v.len(),
            Values::List(ref v) => v.len(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ValueKind {
    Bool = 2,
    Byte = 3,
    Double = 4,
    I16 = 6,
    I32 = 8,
    I64 = 10,
    String = 11,
    Struct = 12,
    Map = 13,
    Set = 14,
    List = 15,
}
