use std::ops::Deref;

use data::{Struct, Elements, Map, Set};

/// List.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct List {
    elements: Elements,
}
impl List {
    /// Makes a new `List` instance.
    pub fn new(elements: Elements) -> Self {
        List { elements }
    }
}
impl Deref for List {
    type Target = Elements;
    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}
impl From<Vec<bool>> for List {
    fn from(f: Vec<bool>) -> Self {
        List::new(Elements::Bool(f))
    }
}
impl From<Vec<i8>> for List {
    fn from(f: Vec<i8>) -> Self {
        List::new(Elements::I8(f))
    }
}
impl From<Vec<i16>> for List {
    fn from(f: Vec<i16>) -> Self {
        List::new(Elements::I16(f))
    }
}
impl From<Vec<i32>> for List {
    fn from(f: Vec<i32>) -> Self {
        List::new(Elements::I32(f))
    }
}
impl From<Vec<i64>> for List {
    fn from(f: Vec<i64>) -> Self {
        List::new(Elements::I64(f))
    }
}
impl From<Vec<f64>> for List {
    fn from(f: Vec<f64>) -> Self {
        List::new(Elements::Double(f))
    }
}
impl From<Vec<Vec<u8>>> for List {
    fn from(f: Vec<Vec<u8>>) -> Self {
        List::new(Elements::Binary(f))
    }
}
impl From<Vec<Struct>> for List {
    fn from(f: Vec<Struct>) -> Self {
        List::new(Elements::Struct(f))
    }
}
impl From<Vec<Map>> for List {
    fn from(f: Vec<Map>) -> Self {
        List::new(Elements::Map(f))
    }
}
impl From<Vec<Set>> for List {
    fn from(f: Vec<Set>) -> Self {
        List::new(Elements::Set(f))
    }
}
impl From<Vec<List>> for List {
    fn from(f: Vec<List>) -> Self {
        List::new(Elements::List(f))
    }
}
