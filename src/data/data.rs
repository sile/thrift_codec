use data::{Struct, Map, Set, List};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum Data {
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Double(f64),
    Binary(Vec<u8>),
    Struct(Struct),
    Map(Map),
    Set(Set),
    List(List),
}
impl Data {
    pub fn kind(&self) -> DataKind {
        match *self {
            Data::Bool(_) => DataKind::Bool,
            Data::I8(_) => DataKind::I8,
            Data::I16(_) => DataKind::I16,
            Data::I32(_) => DataKind::I32,
            Data::I64(_) => DataKind::I64,
            Data::Double(_) => DataKind::Double,
            Data::Binary(_) => DataKind::Binary,
            Data::Struct(_) => DataKind::Struct,
            Data::Map(_) => DataKind::Map,
            Data::Set(_) => DataKind::Set,
            Data::List(_) => DataKind::List,
        }
    }
    pub fn as_ref(&self) -> DataRef {
        match *self {
            Data::Bool(ref v) => DataRef::Bool(v),
            Data::I8(ref v) => DataRef::I8(v),
            Data::I16(ref v) => DataRef::I16(v),
            Data::I32(ref v) => DataRef::I32(v),
            Data::I64(ref v) => DataRef::I64(v),
            Data::Double(ref v) => DataRef::Double(v),
            Data::Binary(ref v) => DataRef::Binary(v),
            Data::Struct(ref v) => DataRef::Struct(v),
            Data::Map(ref v) => DataRef::Map(v),
            Data::Set(ref v) => DataRef::Set(v),
            Data::List(ref v) => DataRef::List(v),
        }
    }
}
impl From<bool> for Data {
    fn from(f: bool) -> Self {
        Data::Bool(f)
    }
}
impl From<i8> for Data {
    fn from(f: i8) -> Self {
        Data::I8(f)
    }
}
impl From<i16> for Data {
    fn from(f: i16) -> Self {
        Data::I16(f)
    }
}
impl From<i32> for Data {
    fn from(f: i32) -> Self {
        Data::I32(f)
    }
}
impl From<i64> for Data {
    fn from(f: i64) -> Self {
        Data::I64(f)
    }
}
impl From<f64> for Data {
    fn from(f: f64) -> Self {
        Data::Double(f)
    }
}
impl<'a> From<&'a str> for Data {
    fn from(f: &'a str) -> Self {
        Data::Binary(f.as_bytes().to_owned())
    }
}
impl From<String> for Data {
    fn from(f: String) -> Self {
        Data::Binary(f.into())
    }
}
impl<'a> From<&'a [u8]> for Data {
    fn from(f: &'a [u8]) -> Self {
        Data::Binary(f.to_owned())
    }
}
impl From<Vec<u8>> for Data {
    fn from(f: Vec<u8>) -> Self {
        Data::Binary(f)
    }
}
impl From<Struct> for Data {
    fn from(f: Struct) -> Self {
        Data::Struct(f)
    }
}
impl From<Map> for Data {
    fn from(f: Map) -> Self {
        Data::Map(f)
    }
}
impl From<Set> for Data {
    fn from(f: Set) -> Self {
        Data::Set(f)
    }
}
impl From<List> for Data {
    fn from(f: List) -> Self {
        Data::List(f)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum DataRef<'a> {
    Bool(&'a bool),
    I8(&'a i8),
    I16(&'a i16),
    I32(&'a i32),
    I64(&'a i64),
    Double(&'a f64),
    Binary(&'a [u8]),
    Struct(&'a Struct),
    Map(&'a Map),
    Set(&'a Set),
    List(&'a List),
}
impl<'a> DataRef<'a> {
    pub fn kind(&self) -> DataKind {
        match *self {
            DataRef::Bool(_) => DataKind::Bool,
            DataRef::I8(_) => DataKind::I8,
            DataRef::I16(_) => DataKind::I16,
            DataRef::I32(_) => DataKind::I32,
            DataRef::I64(_) => DataKind::I64,
            DataRef::Double(_) => DataKind::Double,
            DataRef::Binary(_) => DataKind::Binary,
            DataRef::Struct(_) => DataKind::Struct,
            DataRef::Map(_) => DataKind::Map,
            DataRef::Set(_) => DataKind::Set,
            DataRef::List(_) => DataKind::List,
        }
    }
    pub fn to_owned(&self) -> Data {
        match *self {
            DataRef::Bool(v) => Data::Bool(v.to_owned()),
            DataRef::I8(v) => Data::I8(v.to_owned()),
            DataRef::I16(v) => Data::I16(v.to_owned()),
            DataRef::I32(v) => Data::I32(v.to_owned()),
            DataRef::I64(v) => Data::I64(v.to_owned()),
            DataRef::Double(v) => Data::Double(v.to_owned()),
            DataRef::Binary(v) => Data::Binary(v.to_owned()),
            DataRef::Struct(v) => Data::Struct(v.to_owned()),
            DataRef::Map(v) => Data::Map(v.to_owned()),
            DataRef::Set(v) => Data::Set(v.to_owned()),
            DataRef::List(v) => Data::List(v.to_owned()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum DataKind {
    Bool = 2,
    I8 = 3,
    I16 = 6,
    I32 = 8,
    I64 = 10,
    Double = 4,
    Binary = 11,
    Struct = 12,
    Map = 13,
    Set = 14,
    List = 15,
}
