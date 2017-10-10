#[derive(Debug)]
pub struct Message {
    pub name: String,
    pub kind: MessageKind,
    pub seq_id: u32,
    // TODO: body
}

#[derive(Debug, Clone, Copy)]
pub enum MessageKind {
    Call = 1,
    Reply = 2,
    Exception = 3,
    Oneway = 4,
}
