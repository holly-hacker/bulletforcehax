use byteorder::{BigEndian, WriteBytesExt};
use std::io::Write;

use super::super::*;

fn get_value_type(value: &ProtocolValue) -> u8 {
    match value {
        ProtocolValue::Null() => 42,
        ProtocolValue::Dictionary => 68,
        ProtocolValue::StringArray => 97,
        ProtocolValue::Byte(_) => 98,
        ProtocolValue::Custom => 99,
        ProtocolValue::Double(_) => 100,
        ProtocolValue::EventData => 101,
        ProtocolValue::Float(_) => 102,
        ProtocolValue::Hashtable(_) => 104,
        ProtocolValue::Integer(_) => 105,
        ProtocolValue::Short(_) => 107,
        ProtocolValue::Long(_) => 108,
        ProtocolValue::IntegerArray => 110,
        ProtocolValue::Bool(_) => 111,
        ProtocolValue::OperationResponse => 112,
        ProtocolValue::OperationRequest => 113,
        ProtocolValue::String(_) => 115,
        ProtocolValue::ByteArray => 120,
        ProtocolValue::Array(_) => 121,
        ProtocolValue::ObjectArray(_) => 122,
    }
}

pub fn write_value_of_type(c: &mut dyn Write, value: ProtocolValue) -> PacketWriteResult<()> {
    let type_byte = get_value_type(&value);
    c.write_u8(type_byte)?;
    write_value_of_type_without_type(c, value)
}

pub fn write_value_of_type_without_type(c: &mut dyn Write, value: ProtocolValue) -> PacketWriteResult<()> {
    match value {
        ProtocolValue::Null() => Ok(()),
        ProtocolValue::Bool(true) => Ok(c.write_u8(1)?),
        ProtocolValue::Bool(false) => Ok(c.write_u8(0)?),
        ProtocolValue::Byte(x) => Ok(c.write_u8(x)?),
        ProtocolValue::Short(x) => Ok(c.write_u16::<BigEndian>(x)?),
        ProtocolValue::Integer(x) => Ok(c.write_u32::<BigEndian>(x)?),
        ProtocolValue::Long(x) => Ok(c.write_u64::<BigEndian>(x)?),
        ProtocolValue::Float(x) => Ok(c.write_f32::<BigEndian>(x)?),
        ProtocolValue::Double(x) => Ok(c.write_f64::<BigEndian>(x)?),
        ProtocolValue::String(x) => {
            let bytes = x.as_bytes(); // as utf8 bytes
            c.write_u16::<BigEndian>(bytes.len() as u16)?;
            Ok(c.write_all(bytes)?)
        }
        ProtocolValue::ByteArray => Err(PacketWriteError::UnimplementedProtocolValueType(ProtocolValue::ByteArray)),
        ProtocolValue::IntegerArray => Err(PacketWriteError::UnimplementedProtocolValueType(ProtocolValue::IntegerArray)),
        ProtocolValue::StringArray => Err(PacketWriteError::UnimplementedProtocolValueType(ProtocolValue::StringArray)),
        ProtocolValue::Array(x) => {
            c.write_u16::<BigEndian>(x.len() as u16)?;
            // if we hit this, we may need to implement returning an empty array of nulls
            assert!(x.len() > 0, "Tried to serialize empty Array! Not allowed since we need a protocol type.");
            let protocol_type = get_value_type(&x[0]);
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
        ProtocolValue::Dictionary => Err(PacketWriteError::UnimplementedProtocolValueType(ProtocolValue::Dictionary)),
        ProtocolValue::Hashtable(x) => {
            c.write_u16::<BigEndian>(x.len() as u16)?;
            for (key, value) in x {
                write_value_of_type(c, key)?;
                write_value_of_type(c, value)?;
            }
            Ok(())
        }
        ProtocolValue::EventData => Err(PacketWriteError::UnimplementedProtocolValueType(ProtocolValue::EventData)),
        ProtocolValue::OperationResponse => Err(PacketWriteError::UnimplementedProtocolValueType(ProtocolValue::OperationResponse)),
        ProtocolValue::OperationRequest => Err(PacketWriteError::UnimplementedProtocolValueType(ProtocolValue::OperationRequest)),
        ProtocolValue::Custom => Err(PacketWriteError::UnimplementedProtocolValueType(ProtocolValue::Custom)),
    }
}
