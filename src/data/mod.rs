//! Data types.
pub use self::data_inner::{Data, DataRef, DataKind};
pub use self::element::Elements;
pub use self::list::List;
pub use self::map::Map;
pub use self::set::Set;
pub use self::thrift_struct::{Struct, Field};

mod data_inner;
mod element;
mod list;
mod map;
mod set;
mod thrift_struct;

pub mod iterators {
    //! Iterators.
    pub use super::element::ElementIter;
    pub use super::map::MapIter;
}
