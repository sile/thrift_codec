use data::{DataRef, DataKind, Elements};

use {Result, ErrorKind};

/// Map.
///
/// Internally this is represented by the data structure called "associative array".
/// No duplicate keys are removed.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Map(Option<Inner>);
impl Map {
    /// Makes an empty `Map` instance.
    ///
    /// Note that the returning value could not be encoded by the binary protocol encoding.
    pub fn empty() -> Self {
        Map(None)
    }

    /// Makes a new `Map` instance.
    pub fn new<I, K, V>(pairs: I) -> Self
    where
        I: Iterator<Item = (K, V)>,
        Vec<K>: Into<Elements>,
        Vec<V>: Into<Elements>,
    {
        let mut keys = Vec::new();
        let mut values = Vec::new();
        for (k, v) in pairs {
            keys.push(k);
            values.push(v);
        }
        Map(Some(Inner {
            keys: keys.into(),
            values: values.into(),
        }))
    }

    /// Makes a new `Map` instance from the separate `keys` and `values`.
    ///
    /// # Errors
    ///
    /// If the lengths of `keys` and `values` are differed,
    /// this function will return an error which kind is `ErrorKind::InvalidInput`.
    pub fn from_keys_and_values(keys: Elements, values: Elements) -> Result<Self> {
        track_assert_eq!(keys.len(), values.len(), ErrorKind::InvalidInput);
        Ok(Map(Some(Inner { keys, values })))
    }

    /// Returns the entry placed at the specified index.
    pub fn get(&self, index: usize) -> Option<(DataRef, DataRef)> {
        self.0.as_ref().and_then(|inner| inner.get(index))
    }

    /// Returns an iterator over this map.
    pub fn iter(&self) -> MapIter {
        MapIter {
            map: self,
            index: 0,
        }
    }

    /// Returns the kind of the keys in this map.
    pub fn key_kind(&self) -> Option<DataKind> {
        self.0.as_ref().map(|inner| inner.keys.kind())
    }

    /// Returns the kind of the values in this map.
    pub fn value_kind(&self) -> Option<DataKind> {
        self.0.as_ref().map(|inner| inner.values.kind())
    }

    /// Returns the number of the entries in this map.
    pub fn len(&self) -> usize {
        self.0.as_ref().map_or(0, |inner| inner.len())
    }

    /// Returns `true` if this map has no entries.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
struct Inner {
    keys: Elements,
    values: Elements,
}
impl Inner {
    pub fn get(&self, index: usize) -> Option<(DataRef, DataRef)> {
        self.keys.get(index).map(|k| {
            (k, self.values.get(index).expect("Never fails"))
        })
    }
    pub fn len(&self) -> usize {
        self.keys.len()
    }
}

/// An iterator which traverse the entries of a `Map`.
#[derive(Debug)]
pub struct MapIter<'a> {
    map: &'a Map,
    index: usize,
}
impl<'a> Iterator for MapIter<'a> {
    type Item = (DataRef<'a>, DataRef<'a>);
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.map.get(self.index - 1)
    }
}
