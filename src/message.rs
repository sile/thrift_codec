use std::borrow::Cow;

use data::Struct;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Message {
    method_name: Cow<'static, str>,
    kind: MessageKind,
    sequence_id: i32,
    body: Struct,
}
impl Message {
    pub fn new<T>(method_name: T, kind: MessageKind, sequence_id: i32, body: Struct) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Message {
            method_name: method_name.into(),
            kind,
            sequence_id,
            body,
        }
    }
    pub fn call<T>(method_name: T, sequence_id: i32, body: Struct) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self::new(method_name, MessageKind::Call, sequence_id, body)
    }
    pub fn reply<T>(method_name: T, sequence_id: i32, body: Struct) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self::new(method_name, MessageKind::Reply, sequence_id, body)
    }
    pub fn exception<T>(method_name: T, sequence_id: i32, body: Struct) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self::new(method_name, MessageKind::Exception, sequence_id, body)
    }
    pub fn oneway<T>(method_name: T, sequence_id: i32, body: Struct) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self::new(method_name, MessageKind::Oneway, sequence_id, body)
    }
    pub fn method_name(&self) -> &str {
        &self.method_name
    }
    pub fn kind(&self) -> MessageKind {
        self.kind
    }
    pub fn sequence_id(&self) -> i32 {
        self.sequence_id
    }
    pub fn body(&self) -> &Struct {
        &self.body
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum MessageKind {
    Call = 1,
    Reply = 2,
    Exception = 3,
    Oneway = 4,
}
impl MessageKind {
    // pub(crate) fn from_u8(n: u8) -> Option<Self> {
    //     match n {
    //         1 => Some(MessageKind::Call),
    //         2 => Some(MessageKind::Reply),
    //         3 => Some(MessageKind::Exception),
    //         4 => Some(MessageKind::Oneway),
    //         _ => None,
    //     }
    // }
}
