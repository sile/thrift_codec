use structure::Struct;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Message {
    pub name: String,
    pub kind: MessageKind,
    pub seq_id: u32,
    pub body: Struct,
}
impl Message {
    pub fn oneway(name: &str, seq_id: u32, body: Struct) -> Self {
        Message {
            name: name.to_owned(),
            kind: MessageKind::Oneway,
            seq_id,
            body,
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
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
