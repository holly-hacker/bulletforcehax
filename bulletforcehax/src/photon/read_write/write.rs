#![allow(clippy::float_cmp)]
#![allow(clippy::many_single_char_names)] // required because of quaternion

use byteorder::{BigEndian, WriteBytesExt};
use std::io::Write;

use super::super::*;

pub fn write_debug_string(writer: &mut dyn Write, debug_string: Option<&str>) -> PhotonWriteResult<()> {
    match debug_string {
        Some(x) => write_value_of_type(writer, ProtocolValue::String(x)),
        None => write_value_of_type(writer, ProtocolValue::Null()),
    }
}

fn get_value_type(value: &ProtocolValue) -> u8 {
    match value {
        ProtocolValue::Null() => 42,
        ProtocolValue::Dictionary => 68,
        ProtocolValue::StringArray(_) => 97,
        ProtocolValue::Byte(_) => 98,
        ProtocolValue::Custom(_) => 99,
        ProtocolValue::Double(_) => 100,
        ProtocolValue::EventData => 101,
        ProtocolValue::Float(_) => 102,
        ProtocolValue::Hashtable(_) => 104,
        ProtocolValue::Integer(_) => 105,
        ProtocolValue::Short(_) => 107,
        ProtocolValue::Long(_) => 108,
        ProtocolValue::IntegerArray(_) => 110,
        ProtocolValue::Bool(_) => 111,
        ProtocolValue::OperationResponse => 112,
        ProtocolValue::OperationRequest => 113,
        ProtocolValue::String(_) => 115,
        ProtocolValue::ByteArray(_) => 120,
        ProtocolValue::Array(_) => 121,
        ProtocolValue::ObjectArray(_) => 122,
    }
}

pub fn write_value_of_type(c: &mut dyn Write, value: ProtocolValue) -> PhotonWriteResult<()> {
    let type_byte = get_value_type(&value);
    c.write_u8(type_byte)?;
    write_value_of_type_without_type(c, value)
}

pub fn write_value_of_type_without_type(c: &mut dyn Write, value: ProtocolValue) -> PhotonWriteResult<()> {
    match value {
        ProtocolValue::Null() => Ok(()),
        ProtocolValue::Bool(true) => Ok(c.write_u8(1)?),
        ProtocolValue::Bool(false) => Ok(c.write_u8(0)?),
        ProtocolValue::Byte(x) => Ok(c.write_u8(x)?),
        ProtocolValue::Short(x) => Ok(c.write_i16::<BigEndian>(x)?),
        ProtocolValue::Integer(x) => Ok(c.write_i32::<BigEndian>(x)?),
        ProtocolValue::Long(x) => Ok(c.write_i64::<BigEndian>(x)?),
        ProtocolValue::Float(x) => Ok(c.write_f32::<BigEndian>(x)?),
        ProtocolValue::Double(x) => Ok(c.write_f64::<BigEndian>(x)?),
        ProtocolValue::String(x) => {
            let bytes = x.as_bytes(); // as utf8 bytes
            c.write_u16::<BigEndian>(bytes.len() as u16)?;
            Ok(c.write_all(bytes)?)
        }
        ProtocolValue::ByteArray(bytes) => {
            c.write_u32::<BigEndian>(bytes.len() as u32)?;
            Ok(c.write_all(bytes.as_slice())?)
        }
        ProtocolValue::IntegerArray(ints) => {
            c.write_u32::<BigEndian>(ints.len() as u32)?;
            for i in ints {
                c.write_i32::<BigEndian>(i)?;
            }
            Ok(())
        }
        ProtocolValue::StringArray(strings) => {
            c.write_u16::<BigEndian>(strings.len() as u16)?;
            for s in strings {
                let bytes = s.as_bytes(); // as utf8 bytes
                c.write_u16::<BigEndian>(bytes.len() as u16)?;
                c.write_all(bytes)?;
            }
            Ok(())
        }
        ProtocolValue::Array(x) => {
            c.write_u16::<BigEndian>(x.len() as u16)?;
            // if we hit this, we may need to implement returning an empty array of nulls
            assert!(
                !x.is_empty(),
                "Tried to serialize empty Array! Not allowed since we need a protocol type."
            );
            let protocol_type = get_value_type(&x[0]);
            c.write_u8(protocol_type)?;
            for i in x {
                assert_eq!(protocol_type, get_value_type(&i), "Array types did not match!");
                write_value_of_type_without_type(c, i)?;
            }
            Ok(())
        }
        ProtocolValue::ObjectArray(x) => {
            c.write_u16::<BigEndian>(x.len() as u16)?;
            for i in x {
                write_value_of_type(c, i)?;
            }
            Ok(())
        }
        ProtocolValue::Dictionary => Err(PhotonWriteError::UnimplementedProtocolValueType(ProtocolValue::Dictionary)),
        ProtocolValue::Hashtable(x) => {
            c.write_u16::<BigEndian>(x.len() as u16)?;
            for (key, value) in x {
                write_value_of_type(c, key)?;
                write_value_of_type(c, value)?;
            }
            Ok(())
        }
        ProtocolValue::EventData => Err(PhotonWriteError::UnimplementedProtocolValueType(ProtocolValue::EventData)),
        ProtocolValue::OperationResponse => Err(PhotonWriteError::UnimplementedProtocolValueType(ProtocolValue::OperationResponse)),
        ProtocolValue::OperationRequest => Err(PhotonWriteError::UnimplementedProtocolValueType(ProtocolValue::OperationRequest)),
        ProtocolValue::Custom(custom) => {
            match custom {
                CustomType::Vector2(x, y) => {
                    c.write_u8(b'W')?;
                    c.write_u16::<BigEndian>(8)?;
                    c.write_f32::<BigEndian>(x)?;
                    c.write_f32::<BigEndian>(y)?;
                }
                CustomType::Vector3(x, y, z) => {
                    c.write_u8(b'V')?;
                    c.write_u16::<BigEndian>(12)?;
                    c.write_f32::<BigEndian>(x)?;
                    c.write_f32::<BigEndian>(y)?;
                    c.write_f32::<BigEndian>(z)?;
                }
                CustomType::Quaternion(x, y, z, w) => {
                    c.write_u8(b'Q')?;
                    c.write_u16::<BigEndian>(16)?;
                    c.write_f32::<BigEndian>(x)?;
                    c.write_f32::<BigEndian>(y)?;
                    c.write_f32::<BigEndian>(z)?;
                    c.write_f32::<BigEndian>(w)?;
                }
                CustomType::Player(id) => {
                    c.write_u8(b'P')?;
                    c.write_u16::<BigEndian>(4)?;
                    c.write_i32::<BigEndian>(id)?;
                }
                CustomType::Custom { id, data } => {
                    c.write_u8(id)?;
                    c.write_u16::<BigEndian>(data.len() as u16)?;
                    c.write_all(data.as_slice())?;
                }
            }
            Ok(())
        }
    }
}

pub fn write_parameter_table(c: &mut dyn Write, x: HashMap<u8, ProtocolValue>) -> PhotonWriteResult<()> {
    c.write_u16::<BigEndian>(x.len() as u16)?;
    for (key, value) in x {
        c.write_u8(key)?;
        write_value_of_type(c, value)?;
    }
    Ok(())
}
