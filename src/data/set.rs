use std::ops::Deref;

use data::{Struct, Elements, Map, List};

/// Set.
///
/// Note that internally this has the same representation with `List`.
/// No duplicate elements are removed.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Set {
    elements: Elements,
}
impl Set {
    /// Makes a new `Set` instance.
    pub fn new(elements: Elements) -> Self {
        Set { elements }
    }
}
impl Deref for Set {
    type Target = Elements;
    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}
impl From<Vec<bool>> for Set {
    fn from(f: Vec<bool>) -> Self {
        Set::new(Elements::Bool(f))
    }
}
impl From<Vec<i8>> for Set {
    fn from(f: Vec<i8>) -> Self {
        Set::new(Elements::I8(f))
    }
}
impl From<Vec<i16>> for Set {
    fn from(f: Vec<i16>) -> Self {
        Set::new(Elements::I16(f))
    }
}
impl From<Vec<i32>> for Set {
    fn from(f: Vec<i32>) -> Self {
        Set::new(Elements::I32(f))
    }
}
impl From<Vec<i64>> for Set {
    fn from(f: Vec<i64>) -> Self {
        Set::new(Elements::I64(f))
    }
}
impl From<Vec<f64>> for Set {
    fn from(f: Vec<f64>) -> Self {
        Set::new(Elements::Double(f))
    }
}
impl From<Vec<Vec<u8>>> for Set {
    fn from(f: Vec<Vec<u8>>) -> Self {
        Set::new(Elements::Binary(f))
    }
}
impl From<Vec<Struct>> for Set {
    fn from(f: Vec<Struct>) -> Self {
        Set::new(Elements::Struct(f))
    }
}
impl From<Vec<Map>> for Set {
    fn from(f: Vec<Map>) -> Self {
        Set::new(Elements::Map(f))
    }
}
impl From<Vec<Set>> for Set {
    fn from(f: Vec<Set>) -> Self {
        Set::new(Elements::Set(f))
    }
}
impl From<Vec<List>> for Set {
    fn from(f: Vec<List>) -> Self {
        Set::new(Elements::List(f))
    }
}
