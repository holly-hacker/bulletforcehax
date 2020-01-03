use super::*;

pub type PacketReadResult<T> = Result<T, PacketReadError>;

#[derive(Debug)]
pub enum PacketReadError {
    InvalidMagic(u8),
    UnknownPacketType(u8),
    UnknownEventType(u8),
    UnknownOperationType(u8),
    UnknownInternalOperationType(u8),
    UnimplementedEventType(Event<'static>),
    UnimplementedOperationType(Operation<'static>),
    UnimplementedInternalOperationType(InternalOperation),
    UnknownProtocolValueType(u8),
    UnimplementedProtocolValueType(ProtocolValue<'static>),
    UnexpectedProtocolValue,
    IOError(std::io::Error),
    EncodingError(std::str::Utf8Error),
    CouldNotFindKey(u8),
    CouldNotFindKeyProtocolValue(ProtocolValue<'static>),
    Other(String),
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
