use value::Value;

// TODO: Union, Exception
#[derive(Debug)]
pub struct Struct {
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub struct Field {
    pub id: i16,
    pub value: Value,
}
