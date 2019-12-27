#[derive(Debug)]
pub enum Packet { // TODO: add data
    Init,
    InitResponse,
    OperationRequest,
    OperationResponse,
    Event,
    InternalOperationRequest,
    InternalOperationResponse,
    Message,
    RawMessage,
}

impl Packet {
    pub fn read(data: &[u8]) -> Result<Packet, PacketReadError> {
        if data.len() <= 2 { return Err(PacketReadError::PacketTooShort(data.len())); }
        if data[0] != 0xF3 { return Err(PacketReadError::InvalidMagic(data[0])); }

        let packet_type: u8 = data[1]; // TODO: read

        match packet_type {
            0 => Ok(Packet::Init),
            1 => Ok(Packet::InitResponse),
            2 => Ok(Packet::OperationRequest),
            3 => Ok(Packet::OperationResponse),
            4 => Ok(Packet::Event),
            6 => Ok(Packet::InternalOperationRequest),
            7 => Ok(Packet::InternalOperationResponse),
            8 => Ok(Packet::Message),
            9 => Ok(Packet::RawMessage),
            _ => Err(PacketReadError::UnknownPacketType(packet_type))
        }
    }
}

#[derive(Debug)]
pub enum PacketReadError {
    PacketTooShort(usize),
    InvalidMagic(u8),
    UnknownPacketType(u8),
}
