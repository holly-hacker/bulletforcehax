use std::collections::HashMap;
use std::hash::{Hash, Hasher};

mod errors;
pub use errors::*;
mod photon_impl;
mod read_write;

type ParameterTable<'a> = HashMap<u8, ProtocolValue<'a>>;

#[derive(Debug)]
pub enum PhotonPacket<'a> {
    Init,
    InitResponse,
    OperationRequest(u8, ParameterTable<'a>),
    OperationResponse(u8, ParameterTable<'a>, i16, Option<&'a str>),
    Event(u8, ParameterTable<'a>),
    InternalOperationRequest(u8, ParameterTable<'a>),
    InternalOperationResponse(u8, ParameterTable<'a>, i16, Option<&'a str>),
    Message,
    RawMessage,
}

#[derive(Debug, PartialEq)]
pub enum ProtocolValue<'a> {
    Null(),
    Bool(bool),
    Byte(u8),
    Short(u16),
    Integer(u32),
    Long(u64),
    Float(f32),
    Double(f64),
    String(&'a str),
    OperationRequest,
    OperationResponse,
    EventData,
    Array(Vec<ProtocolValue<'a>>), // An array of predetermined type, C# type is Array.
    ObjectArray(Vec<ProtocolValue<'a>>),
    ByteArray,
    StringArray,
    IntegerArray,
    Dictionary, // Map<Object, Object>, predefined types, C# type is IDictionary/Dictionary<T1, T2>, TODO
    Hashtable(HashMap<ProtocolValue<'a>, ProtocolValue<'a>>), // Map<Object, Object>, random types, C# type is Hashtable/Dictionary<object, object>
    Custom,
}

// this may not work, I'm not sure yet
impl Eq for ProtocolValue<'_> {}

// really annoying and ugly hack, but I wouldn't know how to fix it
impl Hash for ProtocolValue<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ProtocolValue::Null() => 0.hash(state),
            ProtocolValue::Bool(x) => x.hash(state),
            ProtocolValue::Byte(x) => x.hash(state),
            ProtocolValue::Short(x) => x.hash(state),
            ProtocolValue::Integer(x) => x.hash(state),
            ProtocolValue::Long(x) => x.hash(state),
            ProtocolValue::Float(x) => unsafe { std::mem::transmute::<f32, u32>(*x) }.hash(state),
            ProtocolValue::Double(x) => unsafe { std::mem::transmute::<f64, u64>(*x) }.hash(state),
            ProtocolValue::String(x) => x.hash(state),
            _ => panic!("Tried to hash {:?}", self),
        }
    }
}
