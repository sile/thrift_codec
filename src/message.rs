use structure::Struct;

#[derive(Debug)]
pub struct Message {
    pub name: String,
    pub kind: MessageKind,
    pub seq_id: u32,
    pub body: Struct,
}

#[derive(Debug, Clone, Copy)]
pub enum MessageKind {
    Call = 1,
    Reply = 2,
    Exception = 3,
    Oneway = 4,
}
impl MessageKind {
    pub fn from_u8(n: u8) -> Option<Self> {
        match n {
            1 => Some(MessageKind::Call),
            2 => Some(MessageKind::Reply),
            3 => Some(MessageKind::Exception),
            4 => Some(MessageKind::Oneway),
            _ => None,
        }
    }
}
