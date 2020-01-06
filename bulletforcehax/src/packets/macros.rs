use super::*;
use either::Either;
use log::debug;
use std::collections::HashMap;

macro_rules! gen_protocol_type_functions {
    ($type_name:ident, $type:ty, $protocol_type:path) => {
        paste::item! {
            pub fn [<unwrap_protocol_ $type_name>]<'a>(protocol_type: ProtocolValue<'a>) -> PacketReadResult<$type> {
                match protocol_type {
                    $protocol_type(i) => Ok(i),
                    _ => Err(PacketReadError::UnexpectedProtocolValue),
                }
            }

            #[allow(dead_code)]
            pub fn [<get_protocol_ $type_name>]<'a>(
                map: &mut HashMap<ProtocolValue<'a>, ProtocolValue<'a>>,
                key: ProtocolValue<'static>,
            ) -> PacketReadResult<$type> {
                match map.remove(&key) {
                    Some(val) => Ok([<unwrap_protocol_ $type_name>](val)?),
                    None => {
                        debug!("Couldn't find key {:?} in {:?}", key, map);
                        Err(PacketReadError::CouldNotFindKeyProtocolValue(key))
                    }
                }
            }

            #[allow(dead_code)]
            pub fn [<get_u8_ $type_name>]<'a>(map: &mut HashMap<u8, ProtocolValue<'a>>, param_code: u8) -> PacketReadResult<$type> {
                match map.remove(&param_code) {
                    Some(val) => Ok([<unwrap_protocol_ $type_name>](val)?),
                    None => {
                        debug!("Couldn't find key {} in {:?}", param_code, map);
                        Err(PacketReadError::CouldNotFindKey(param_code))
                    }
                }
            }
        }
    };
}

gen_protocol_type_functions!(string, &'a str, ProtocolValue::String);
gen_protocol_type_functions!(bool, bool, ProtocolValue::Bool);
gen_protocol_type_functions!(byte, u8, ProtocolValue::Byte);
gen_protocol_type_functions!(short, u16, ProtocolValue::Short);
gen_protocol_type_functions!(int, u32, ProtocolValue::Integer);
gen_protocol_type_functions!(float, f32, ProtocolValue::Float);
gen_protocol_type_functions!(hashtable, HashMap<ProtocolValue<'a>, ProtocolValue<'a>>, ProtocolValue::Hashtable);
gen_protocol_type_functions!(array, Vec<ProtocolValue<'a>>, ProtocolValue::Array);

// helper method to read Either<u32, f32>
// not a macro (yet?), shh don't tell anyone
// in the future, I should probably stop using macros and try to use generic functions instead
pub fn get_protocol_int_or_float<'a>(
    map: &mut HashMap<ProtocolValue<'a>, ProtocolValue<'a>>,
    key: ProtocolValue<'static>,
) -> PacketReadResult<Either<u32, f32>> {
    match map.remove(&key) {
        Some(val) => match val {
            ProtocolValue::Integer(i) => Ok(Either::Left(i)),
            ProtocolValue::Float(i) => Ok(Either::Right(i)),
            _ => {
                debug!("Expected int or float, found {:?}", val);
                Err(PacketReadError::UnexpectedProtocolValue)
            }
        },
        None => {
            debug!("Couldn't find key {:?} in {:?}", key, map);
            Err(PacketReadError::CouldNotFindKeyProtocolValue(key))
        }
    }
}
