use std::io::Read;

use Result;

pub trait BinaryDecode: Sized {
    fn binary_decode<R: Read>(reader: &mut R) -> Result<Self>;
}

pub trait CompactDecode: Sized {
    fn compact_decode<R: Read>(reader: &mut R) -> Result<Self>;
}
