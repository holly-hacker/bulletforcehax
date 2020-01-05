use byteorder::{BigEndian, ReadBytesExt};
use std::collections::HashMap;
use std::io::Cursor;

use super::super::*;

/// Version of read_value that returns an error on a non-string
pub fn read_string<'a>(c: &mut Cursor<&'a [u8]>) -> PacketReadResult<Option<&'a str>> {
    match read_value(c)? {
        ProtocolValue::Null() => Ok(None),
        ProtocolValue::String(string) => Ok(Some(string)),
        _ => Err(PacketReadError::UnexpectedProtocolValue),
    }
}

pub fn read_value<'a>(c: &mut Cursor<&'a [u8]>) -> PacketReadResult<ProtocolValue<'a>> {
    let protocol_type = c.read_u8()?;
    read_value_of_type(c, protocol_type)
}

pub fn read_value_of_type<'a>(c: &mut Cursor<&'a [u8]>, protocol_type: u8) -> PacketReadResult<ProtocolValue<'a>> {
    match protocol_type {
        42 => Ok(ProtocolValue::Null()),
        68 => Err(PacketReadError::UnimplementedProtocolValueType(ProtocolValue::Dictionary)),
        97 => Err(PacketReadError::UnimplementedProtocolValueType(ProtocolValue::StringArray)),
        98 => Ok(ProtocolValue::Byte(c.read_u8()?)),
        99 => Err(PacketReadError::UnimplementedProtocolValueType(ProtocolValue::Custom)),
        100 => Ok(ProtocolValue::Double(c.read_f64::<BigEndian>()?)),
        101 => Err(PacketReadError::UnimplementedProtocolValueType(ProtocolValue::EventData)),
        102 => Ok(ProtocolValue::Float(c.read_f32::<BigEndian>()?)),
        104 => Ok(ProtocolValue::Hashtable(read_hash_table(c)?)),
        105 => Ok(ProtocolValue::Integer(c.read_u32::<BigEndian>()?)),
        107 => Ok(ProtocolValue::Short(c.read_u16::<BigEndian>()?)),
        108 => Ok(ProtocolValue::Long(c.read_u64::<BigEndian>()?)),
        110 => Err(PacketReadError::UnimplementedProtocolValueType(ProtocolValue::IntegerArray)),
        111 => Ok(ProtocolValue::Bool(c.read_u8()? != 0)),
        112 => Err(PacketReadError::UnimplementedProtocolValueType(ProtocolValue::OperationResponse)),
        113 => Err(PacketReadError::UnimplementedProtocolValueType(ProtocolValue::OperationRequest)),
        115 => {
            let len = c.read_u16::<BigEndian>()? as usize;
            let pos = c.position() as usize;

            let return_slice = &(*c.get_ref())[pos..pos + len];
            c.set_position((pos + len) as u64);
            let str_slice = std::str::from_utf8(return_slice)?;
            Ok(ProtocolValue::String(str_slice))
        }
        120 => Err(PacketReadError::UnimplementedProtocolValueType(ProtocolValue::ByteArray)),
        121 => Ok(ProtocolValue::Array(read_value_array_of_same_type(c)?)),
        122 => Ok(ProtocolValue::ObjectArray(read_value_array(c)?)),
        _ => Err(PacketReadError::UnknownProtocolValueType(protocol_type)),
    }
}

pub fn read_value_array_of_same_type<'a>(c: &mut Cursor<&'a [u8]>) -> PacketReadResult<Vec<ProtocolValue<'a>>> {
    let len = c.read_u16::<BigEndian>()?;
    let protocol_type = c.read_u8()?;
    let mut ret = Vec::new();
    for _i in 0..len {
        ret.push(read_value_of_type(c, protocol_type)?);
    }
    Ok(ret)
}

pub fn read_value_array<'a>(c: &mut Cursor<&'a [u8]>) -> PacketReadResult<Vec<ProtocolValue<'a>>> {
    let len = c.read_u16::<BigEndian>()?;
    let mut ret = Vec::new();
    for _i in 0..len {
        ret.push(read_value(c)?);
    }
    Ok(ret)
}

pub fn read_hash_table<'a>(c: &mut Cursor<&'a [u8]>) -> PacketReadResult<HashMap<ProtocolValue<'a>, ProtocolValue<'a>>> {
    let mut ret = HashMap::new();
    let len = c.read_u16::<BigEndian>()?;
    for _i in 0..len {
        ret.insert(read_value(c)?, read_value(c)?);
    }
    Ok(ret)
}

pub fn read_parameter_table<'a>(c: &mut Cursor<&'a [u8]>) -> PacketReadResult<HashMap<u8, ProtocolValue<'a>>> {
    let mut ret = HashMap::new();
    let len = c.read_u16::<BigEndian>()?;
    for _i in 0..len {
        ret.insert(c.read_u8()?, read_value(c)?);
    }
    Ok(ret)
}
