//! RPC message.
use data::Struct;

/// RPC message.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Message {
    method_name: String,
    kind: MessageKind,
    sequence_id: i32,
    body: Struct,
}
impl Message {
    /// Makes a new `Message` instance.
    pub fn new(method_name: &str, kind: MessageKind, sequence_id: i32, body: Struct) -> Self {
        Message {
            method_name: method_name.to_owned(),
            kind,
            sequence_id,
            body,
        }
    }

    /// Makes a new `Message` instance which has the kind `MessageKind::Call`.
    pub fn call(method_name: &str, sequence_id: i32, body: Struct) -> Self {
        Self::new(method_name, MessageKind::Call, sequence_id, body)
    }

    /// Makes a new `Message` instance which has the kind `MessageKind::Reply`.
    pub fn reply(method_name: &str, sequence_id: i32, body: Struct) -> Self {
        Self::new(method_name, MessageKind::Reply, sequence_id, body)
    }

    /// Makes a new `Message` instance which has the kind `MessageKind::Exception`.
    pub fn exception(method_name: &str, sequence_id: i32, body: Struct) -> Self {
        Self::new(method_name, MessageKind::Exception, sequence_id, body)
    }

    /// Makes a new `Message` instance which has the kind `MessageKind::Oneway`.
    pub fn oneway(method_name: &str, sequence_id: i32, body: Struct) -> Self {
        Self::new(method_name, MessageKind::Oneway, sequence_id, body)
    }

    /// Returns the method name of this message.
    pub fn method_name(&self) -> &str {
        &self.method_name
    }

    /// Returns the kind of this message.
    pub fn kind(&self) -> MessageKind {
        self.kind
    }

    /// Returns the sequence id of this message.
    pub fn sequence_id(&self) -> i32 {
        self.sequence_id
    }

    /// Returns the body of this message.
    pub fn body(&self) -> &Struct {
        &self.body
    }
}

/// The kind of a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[allow(missing_docs)]
pub enum MessageKind {
    Call = 1,
    Reply = 2,
    Exception = 3,
    Oneway = 4,
}
impl MessageKind {
    pub(crate) fn from_u8(n: u8) -> Option<Self> {
        match n {
            1 => Some(MessageKind::Call),
            2 => Some(MessageKind::Reply),
            3 => Some(MessageKind::Exception),
            4 => Some(MessageKind::Oneway),
            _ => None,
        }
    }
}
