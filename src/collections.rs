use value::Values;

#[derive(Debug)]
pub struct List {
    pub elements: Values,
}

#[derive(Debug)]
pub struct Set {
    pub elements: Values,
}

#[derive(Debug)]
pub struct Map {
    pub keys: Values,
    pub values: Values,
}
