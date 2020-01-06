use super::*;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use read_write::{read_debug_string, read_parameter_table, write_debug_string, write_parameter_table};
use std::io::Cursor;

impl PhotonPacket<'_> {
    pub fn read<'a>(data: &'a [u8]) -> PhotonReadResult<PhotonPacket<'a>> {
        let ref mut c = Cursor::new(data);
        let magic = c.read_u8()?;
        if magic != 0xF3 {
            return Err(PhotonReadError::InvalidMagic(magic));
        }

        let packet_type: u8 = c.read_u8()?;
        match packet_type {
            2 => Ok(PhotonPacket::OperationRequest(c.read_u8()?, read_parameter_table(c)?)),
            3 => {
                let operation_type = c.read_u8()?;
                let return_code = c.read_i16::<BigEndian>()?;
                let debug_string = read_debug_string(c)?;
                let parameter_table = read_parameter_table(c)?;
                Ok(PhotonPacket::OperationResponse(
                    operation_type,
                    parameter_table,
                    return_code,
                    debug_string,
                ))
            }
            4 => Ok(PhotonPacket::Event(c.read_u8()?, read_parameter_table(c)?)),
            6 => Ok(PhotonPacket::InternalOperationRequest(c.read_u8()?, read_parameter_table(c)?)),
            7 => {
                let operation_type = c.read_u8()?;
                let return_code = c.read_i16::<BigEndian>()?;
                let debug_string = read_debug_string(c)?;
                let parameter_table = read_parameter_table(c)?;
                Ok(PhotonPacket::InternalOperationResponse(
                    operation_type,
                    parameter_table,
                    return_code,
                    debug_string,
                ))
            }
            0 | 1 | 8 | 9 => Err(PhotonReadError::UnimplementedPacketType(packet_type)),
            _ => Err(PhotonReadError::UnknownPacketType(packet_type)),
        }
    }

    fn get_type(&self) -> u8 {
        match self {
            // PhotonPacket::Init => 0,
            // PhotonPacket::InitResponse => 1,
            PhotonPacket::OperationRequest(_, _) => 2,
            PhotonPacket::OperationResponse(_, _, _, _) => 3,
            PhotonPacket::Event(_, _) => 4,
            PhotonPacket::InternalOperationRequest(_, _) => 6,
            PhotonPacket::InternalOperationResponse(_, _, _, _) => 7,
            // PhotonPacket::Message => 8,
            // PhotonPacket::RawMessage => 9,
        }
    }

    pub fn into_vec(self) -> PhotonWriteResult<Vec<u8>> {
        let mut vec = Vec::new();
        let ref mut writer = vec;

        writer.write_u8(0xF3)?;
        writer.write_u8(self.get_type())?;

        match self {
            PhotonPacket::OperationRequest(packet_type, params) => {
                writer.write_u8(packet_type)?;
                write_parameter_table(writer, params)?;
            }
            PhotonPacket::OperationResponse(packet_type, params, return_value, debug_string) => {
                writer.write_u8(packet_type)?;
                writer.write_i16::<BigEndian>(return_value)?;
                write_debug_string(writer, debug_string)?;
                write_parameter_table(writer, params)?;
            }
            PhotonPacket::Event(packet_type, params) => {
                writer.write_u8(packet_type)?;
                write_parameter_table(writer, params)?;
            }
            PhotonPacket::InternalOperationRequest(packet_type, params) => {
                writer.write_u8(packet_type)?;
                write_parameter_table(writer, params)?;
            }
            PhotonPacket::InternalOperationResponse(packet_type, params, return_value, debug_string) => {
                writer.write_u8(packet_type)?;
                writer.write_i16::<BigEndian>(return_value)?;
                write_debug_string(writer, debug_string)?;
                write_parameter_table(writer, params)?;
            }
        }

        Ok(vec)
    }
}
