use super::*;

pub type PhotonReadResult<T> = Result<T, PhotonReadError>;
pub type PhotonWriteResult<T> = Result<T, PhotonWriteError>;

#[derive(Debug)]
pub enum PhotonReadError {
    InvalidMagic(u8),
    UnknownPacketType(u8),
    UnknownProtocolValueType(u8),
    UnimplementedPacketType(PhotonPacket<'static>),
    UnimplementedProtocolValueType(ProtocolValue<'static>),
    InvalidDebugStringType,
    IOError(std::io::Error),
    EncodingError(std::str::Utf8Error),
}

impl From<std::io::Error> for PhotonReadError {
    fn from(error: std::io::Error) -> Self {
        PhotonReadError::IOError(error)
    }
}

impl From<std::str::Utf8Error> for PhotonReadError {
    fn from(error: std::str::Utf8Error) -> Self {
        PhotonReadError::EncodingError(error)
    }
}

#[derive(Debug)]
pub enum PhotonWriteError {
    UnimplementedPacketType(PhotonPacket<'static>),
    UnimplementedProtocolValueType(ProtocolValue<'static>),
    IOError(std::io::Error),
    EncodingError(std::str::Utf8Error),
}

impl From<std::io::Error> for PhotonWriteError {
    fn from(error: std::io::Error) -> Self {
        PhotonWriteError::IOError(error)
    }
}

impl From<std::str::Utf8Error> for PhotonWriteError {
    fn from(error: std::str::Utf8Error) -> Self {
        PhotonWriteError::EncodingError(error)
    }
}
