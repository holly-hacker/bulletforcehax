use super::*;

#[derive(Debug)]
pub enum PacketReadError {
    InvalidMagic(u8),
    UnknownPacketType(u8),
    UnknownEventType(u8),
    UnknownOperationType(u8),
    UnknownInternalOperationType(u8),
    UnknownProtocolValueType(u8),
    UnimplementedProtocolValueType(ProtocolValue<'static>),
    UnexpectedProtocolValue,
    IOError(std::io::Error),
    EncodingError(std::str::Utf8Error),
}

impl From<std::io::Error> for PacketReadError {
    fn from(error: std::io::Error) -> Self {
        PacketReadError::IOError(error)
    }
}

impl From<std::str::Utf8Error> for PacketReadError {
    fn from(error: std::str::Utf8Error) -> Self {
        PacketReadError::EncodingError(error)
    }
}
