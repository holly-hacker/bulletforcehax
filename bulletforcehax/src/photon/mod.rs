use std::collections::HashMap;
use std::hash::{Hash, Hasher};

mod errors;
pub use errors::*;
mod photon_impl;
mod read_write;

type ParameterTable<'a> = HashMap<u8, ProtocolValue<'a>>;

/// A parsed packet. Can be read/written using the `TryInto` and `TryFrom` methods.
///
/// # Example
///
/// ## Serializing a packet
/// ```rust
/// # use std::collections::HashMap;
/// # use bulletforcehax::photon::*;
/// # use std::convert::TryInto;
/// let mut params = HashMap::new();
/// params.insert(0x42, ProtocolValue::Float(13.37));
///
/// let packet = PhotonPacket::OperationRequest(0x20, params);
/// let bytes: Vec<u8> = packet.try_into().unwrap();
/// assert_eq!(bytes, vec![0xF3, 0x02, 0x20, 0, 0x01, 0x42, 0x66, 0x41, 0x55, 0xeb, 0x85]);
/// ```
///
/// ## Deserializing a packet
/// ```rust
/// # use std::collections::HashMap;
/// # use bulletforcehax::photon::*;
/// # use std::convert::TryFrom;
/// let bytes = vec![0xF3, 0x02, 0x20, 0, 0x01, 0x42, 0x66, 0x41, 0x55, 0xeb, 0x85];
/// let packet = PhotonPacket::try_from(bytes.as_slice()).expect("Deserializing failed");
///
/// if let PhotonPacket::OperationRequest(packet_type, params) = packet {
///     assert_eq!(packet_type, 0x20);
///     assert_eq!(params.len(), 1);
///     assert_eq!(params[&0x42], ProtocolValue::Float(13.37));
/// } else {
///     panic!("Expected OperationRequest");
/// }
/// ```
#[derive(Debug)]
pub enum PhotonPacket<'a> {
    // Init,
    // InitResponse,
    OperationRequest(u8, ParameterTable<'a>),
    OperationResponse(u8, ParameterTable<'a>, i16, Option<&'a str>),
    Event(u8, ParameterTable<'a>),
    InternalOperationRequest(u8, ParameterTable<'a>),
    InternalOperationResponse(u8, ParameterTable<'a>, i16, Option<&'a str>),
    // Message,
    // RawMessage,
}

/// A deserialized Photon value, converted to its Rust equivalent.
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
    /// array of predetermined type, `Array` in C#.
    Array(Vec<ProtocolValue<'a>>),
    /// array of arbitrary types
    ObjectArray(Vec<ProtocolValue<'a>>),
    ByteArray,
    StringArray,
    IntegerArray,
    /// hashmap of predefined types, `IDictionary/Dictionary<T1, T2>` in C#
    Dictionary,
    /// hashmap of arbitrary types, `Hashtable` or `Dictionary<object, object>` in C#
    Hashtable(HashMap<ProtocolValue<'a>, ProtocolValue<'a>>),
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
