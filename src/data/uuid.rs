/// Uuid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Uuid([u8; 16]);

impl Uuid {
    /// Makes a new `Uuid` instance.
    pub fn new(uuid: [u8; 16]) -> Self {
        Self(uuid)
    }

    /// Returns the UUID bytes.
    pub fn get(self) -> [u8; 16] {
        self.0
    }
}
