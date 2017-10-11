use collections::List;
use value::Value;

// TODO: Union, Exception
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Struct {
    pub fields: Vec<Field>,
}
impl Struct {
    pub fn new(fields: Vec<Field>) -> Self {
        Struct { fields }
    }
}
impl<A> From<(A,)> for Struct
where
    A: Into<Value>,
{
    fn from((a,): (A,)) -> Self {
        Struct::new(vec![Field::new(1, a.into())])
    }
}
impl<A, B> From<(A, B)> for Struct
where
    A: Into<Value>,
    B: Into<Value>,
{
    fn from((a, b): (A, B)) -> Self {
        Struct::new(vec![Field::new(1, a.into()), Field::new(2, b.into())])
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Field {
    pub id: i16,
    pub value: Value,
}
impl Field {
    pub fn new(id: i16, value: Value) -> Self {
        Field { id, value }
    }
    pub fn structure(id: i16, value: Struct) -> Self {
        Self::new(id, Value::Struct(value))
    }
    pub fn list(id: i16, value: List) -> Self {
        Self::new(id, Value::List(value))
    }
}
