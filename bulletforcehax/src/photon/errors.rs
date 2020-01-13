use super::*;

pub type PacketReadResult<T> = Result<T, PacketReadError>;
pub type PacketWriteResult<T> = Result<T, PacketWriteError>;

#[derive(Debug)]
pub enum PacketReadError {
    UnknownEventType(u8),
    UnknownOperationType(u8),
    UnknownInternalOperationType(u8),
    UnimplementedEventType(Event<'static>),
    UnimplementedOperationType(Operation<'static>),
    UnimplementedInternalOperationType(InternalOperation),
    UnexpectedProtocolValue,
    CouldNotFindKey(u8),
    CouldNotFindKeyProtocolValue(ProtocolValue<'static>),
    PhotonError(PhotonReadError),
}

impl From<PhotonReadError> for PacketReadError {
    fn from(error: PhotonReadError) -> Self {
        PacketReadError::PhotonError(error)
    }
}

#[derive(Debug)]
pub enum PacketWriteError {
    UnimplementedEventType(Event<'static>),
    UnimplementedOperationType(Operation<'static>),
    UnimplementedInternalOperationType(InternalOperation),
    PhotonError(PhotonWriteError),
}

impl From<PhotonWriteError> for PacketWriteError {
    fn from(error: PhotonWriteError) -> Self {
        PacketWriteError::PhotonError(error)
    }
}
