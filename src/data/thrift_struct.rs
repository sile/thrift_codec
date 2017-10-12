use data::Data;

/// Structure.
///
/// # Examples
///
/// ```
/// use thrift_codec::data::{Struct, Field};
///
/// let a = Struct::new(vec![Field::new(1, "foo"), Field::new(2, "bar")]);
/// let b = Struct::from(("foo", "bar"));
/// assert_eq!(a, b);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Struct {
    fields: Vec<Field>,
}
impl Struct {
    /// Makes a new `Struct` instance.
    pub fn new(fields: Vec<Field>) -> Self {
        Struct { fields }
    }

    /// Returns the fields of this.
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
}
impl<A> From<(A,)> for Struct
where
    A: Into<Data>,
{
    fn from(fields: (A,)) -> Self {
        Struct::new(vec![Field::new(1, fields.0)])
    }
}
impl<A, B> From<(A, B)> for Struct
where
    A: Into<Data>,
    B: Into<Data>,
{
    fn from(fields: (A, B)) -> Self {
        Struct::new(vec![Field::new(1, fields.0), Field::new(2, fields.1)])
    }
}
impl<A, B, C> From<(A, B, C)> for Struct
where
    A: Into<Data>,
    B: Into<Data>,
    C: Into<Data>,
{
    fn from(fields: (A, B, C)) -> Self {
        Struct::new(vec![
            Field::new(1, fields.0),
            Field::new(2, fields.1),
            Field::new(3, fields.2),
        ])
    }
}
impl<A, B, C, D> From<(A, B, C, D)> for Struct
where
    A: Into<Data>,
    B: Into<Data>,
    C: Into<Data>,
    D: Into<Data>,
{
    fn from(fields: (A, B, C, D)) -> Self {
        Struct::new(vec![
            Field::new(1, fields.0),
            Field::new(2, fields.1),
            Field::new(3, fields.2),
            Field::new(4, fields.3),
        ])
    }
}
impl<A, B, C, D, E> From<(A, B, C, D, E)> for Struct
where
    A: Into<Data>,
    B: Into<Data>,
    C: Into<Data>,
    D: Into<Data>,
    E: Into<Data>,
{
    fn from(fields: (A, B, C, D, E)) -> Self {
        Struct::new(vec![
            Field::new(1, fields.0),
            Field::new(2, fields.1),
            Field::new(3, fields.2),
            Field::new(4, fields.3),
            Field::new(5, fields.4),
        ])
    }
}

/// A struct field.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Field {
    id: i16,
    data: Data,
}
impl Field {
    /// Makes a new `Field` instance.
    pub fn new<T>(id: i16, data: T) -> Self
    where
        T: Into<Data>,
    {
        Field {
            id,
            data: data.into(),
        }
    }

    /// Returns the identifier of this field.
    pub fn id(&self) -> i16 {
        self.id
    }

    /// Returns the data of this field.
    pub fn data(&self) -> &Data {
        &self.data
    }
}
