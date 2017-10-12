use data::{DataRef, DataKind, Elements};

use {Result, ErrorKind};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Map(Option<Inner>);
impl Map {
    pub fn empty() -> Self {
        Map(None)
    }
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
    pub fn from_keys_and_values(keys: Elements, values: Elements) -> Result<Self> {
        track_assert_eq!(keys.len(), values.len(), ErrorKind::InvalidInput);
        Ok(Map(Some(Inner { keys, values })))
    }
    pub fn get(&self, index: usize) -> Option<(DataRef, DataRef)> {
        self.0.as_ref().and_then(|inner| inner.get(index))
    }
    pub fn iter(&self) -> MapIter {
        MapIter {
            map: self,
            index: 0,
        }
    }
    pub fn key_kind(&self) -> Option<DataKind> {
        self.0.as_ref().map(|inner| inner.keys.kind())
    }
    pub fn value_kind(&self) -> Option<DataKind> {
        self.0.as_ref().map(|inner| inner.values.kind())
    }
    pub fn len(&self) -> usize {
        self.0.as_ref().map_or(0, |inner| inner.len())
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[derive(Debug, Clone)]
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
