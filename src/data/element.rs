use data::{DataRef, DataKind, Struct, Map, Set, List};

/// A sequence of the values of a data kind.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[allow(missing_docs)]
pub enum Elements {
    Bool(Vec<bool>),
    I8(Vec<i8>),
    I16(Vec<i16>),
    I32(Vec<i32>),
    I64(Vec<i64>),
    Double(Vec<f64>),
    Binary(Vec<Vec<u8>>),
    Struct(Vec<Struct>),
    Map(Vec<Map>),
    Set(Vec<Set>),
    List(Vec<List>),
}
impl Elements {
    /// Makes an empty sequence which can have the elements belonging to `kind`.
    pub fn new(kind: DataKind) -> Self {
        match kind {
            DataKind::Bool => Elements::Bool(Vec::new()),
            DataKind::I8 => Elements::I8(Vec::new()),
            DataKind::I16 => Elements::I16(Vec::new()),
            DataKind::I32 => Elements::I32(Vec::new()),
            DataKind::I64 => Elements::I64(Vec::new()),
            DataKind::Double => Elements::Double(Vec::new()),
            DataKind::Binary => Elements::Binary(Vec::new()),
            DataKind::Struct => Elements::Struct(Vec::new()),
            DataKind::Map => Elements::Map(Vec::new()),
            DataKind::Set => Elements::Set(Vec::new()),
            DataKind::List => Elements::List(Vec::new()),
        }
    }

    /// Returns the element placed at the specified index.
    pub fn get(&self, index: usize) -> Option<DataRef> {
        match *self {
            Elements::Bool(ref v) => v.get(index).map(DataRef::Bool),
            Elements::I8(ref v) => v.get(index).map(DataRef::I8),
            Elements::I16(ref v) => v.get(index).map(DataRef::I16),
            Elements::I32(ref v) => v.get(index).map(DataRef::I32),
            Elements::I64(ref v) => v.get(index).map(DataRef::I64),
            Elements::Double(ref v) => v.get(index).map(DataRef::Double),
            Elements::Binary(ref v) => v.get(index).map(|e| DataRef::Binary(e.as_ref())),
            Elements::Struct(ref v) => v.get(index).map(DataRef::Struct),
            Elements::Map(ref v) => v.get(index).map(DataRef::Map),
            Elements::Set(ref v) => v.get(index).map(DataRef::Set),
            Elements::List(ref v) => v.get(index).map(DataRef::List),
        }
    }

    /// Returns the element count of this sequence.
    pub fn len(&self) -> usize {
        match *self {
            Elements::Bool(ref v) => v.len(),
            Elements::I8(ref v) => v.len(),
            Elements::I16(ref v) => v.len(),
            Elements::I32(ref v) => v.len(),
            Elements::I64(ref v) => v.len(),
            Elements::Double(ref v) => v.len(),
            Elements::Binary(ref v) => v.len(),
            Elements::Struct(ref v) => v.len(),
            Elements::Map(ref v) => v.len(),
            Elements::Set(ref v) => v.len(),
            Elements::List(ref v) => v.len(),                                    
        }
    }

    /// Returns `true` if this sequence has no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the elements of this sequence.
    pub fn iter(&self) -> ElementIter {
        ElementIter {
            elements: self,
            index: 0,
        }
    }

    /// Returns the kind of the elements in this sequence.
    pub fn kind(&self) -> DataKind {
        match *self {
            Elements::Bool(_) => DataKind::Bool,
            Elements::I8(_) => DataKind::I8,
            Elements::I16(_) => DataKind::I16,
            Elements::I32(_) => DataKind::I32,
            Elements::I64(_) => DataKind::I64,
            Elements::Double(_) => DataKind::Double,
            Elements::Binary(_) => DataKind::Binary,
            Elements::Struct(_) => DataKind::Struct,
            Elements::Map(_) => DataKind::Map,
            Elements::Set(_) => DataKind::Set,
            Elements::List(_) => DataKind::List,
        }
    }
}
impl From<Vec<bool>> for Elements {
    fn from(f: Vec<bool>) -> Self {
        Elements::Bool(f)
    }
}
impl From<Vec<i8>> for Elements {
    fn from(f: Vec<i8>) -> Self {
        Elements::I8(f)
    }
}
impl From<Vec<i16>> for Elements {
    fn from(f: Vec<i16>) -> Self {
        Elements::I16(f)
    }
}
impl From<Vec<i32>> for Elements {
    fn from(f: Vec<i32>) -> Self {
        Elements::I32(f)
    }
}
impl From<Vec<i64>> for Elements {
    fn from(f: Vec<i64>) -> Self {
        Elements::I64(f)
    }
}
impl From<Vec<f64>> for Elements {
    fn from(f: Vec<f64>) -> Self {
        Elements::Double(f)
    }
}
impl From<Vec<Vec<u8>>> for Elements {
    fn from(f: Vec<Vec<u8>>) -> Self {
        Elements::Binary(f)
    }
}
impl From<Vec<Struct>> for Elements {
    fn from(f: Vec<Struct>) -> Self {
        Elements::Struct(f)
    }
}
impl From<Vec<Map>> for Elements {
    fn from(f: Vec<Map>) -> Self {
        Elements::Map(f)
    }
}
impl From<Vec<Set>> for Elements {
    fn from(f: Vec<Set>) -> Self {
        Elements::Set(f)
    }
}
impl From<Vec<List>> for Elements {
    fn from(f: Vec<List>) -> Self {
        Elements::List(f)
    }
}

/// An iterator which traverse the elements of a `Elements`.
#[derive(Debug)]
pub struct ElementIter<'a> {
    elements: &'a Elements,
    index: usize,
}
impl<'a> Iterator for ElementIter<'a> {
    type Item = DataRef<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.elements.get(self.index - 1)
    }
}
