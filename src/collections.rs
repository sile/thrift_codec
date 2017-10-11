use structure::Struct;
use value::Values;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct List {
    pub elements: Values,
}
impl List {
    pub fn structs(v: Vec<Struct>) -> Self {
        List { elements: Values::Struct(v) }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Set {
    pub elements: Values,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Map {
    pub keys: Values,
    pub values: Values,
}
