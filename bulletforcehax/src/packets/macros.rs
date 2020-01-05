use super::*;
use log::debug;
use std::collections::HashMap;

macro_rules! gen_protocol_type_functions {
    ($unwrap_fn_name:ident, $get_prot_fn_name:ident, $get_u8_fn_name:ident, $type:ty, $protocol_type:path) => {
        pub fn $unwrap_fn_name<'a>(protocol_type: ProtocolValue<'a>) -> PacketReadResult<$type> {
            match protocol_type {
                $protocol_type(i) => Ok(i),
                _ => Err(PacketReadError::UnexpectedProtocolValue),
            }
        }

        #[allow(dead_code)]
        pub fn $get_prot_fn_name<'a>(
            map: &mut HashMap<ProtocolValue<'a>, ProtocolValue<'a>>,
            key: ProtocolValue<'static>,
        ) -> PacketReadResult<$type> {
            match map.remove(&key) {
                Some(val) => Ok($unwrap_fn_name(val)?),
                None => {
                    debug!("Couldn't find key {:?} in {:?}", key, map);
                    Err(PacketReadError::CouldNotFindKeyProtocolValue(key))
                }
            }
        }

        #[allow(dead_code)]
        pub fn $get_u8_fn_name<'a>(map: &mut HashMap<u8, ProtocolValue<'a>>, param_code: u8) -> PacketReadResult<$type> {
            match map.remove(&param_code) {
                Some(val) => Ok($unwrap_fn_name(val)?),
                None => {
                    debug!("Couldn't find key {} in {:?}", param_code, map);
                    Err(PacketReadError::CouldNotFindKey(param_code))
                }
            }
        }
    };
}

gen_protocol_type_functions!(unwrap_protocol_string, get_u8_string, get_protocol_string, &'a str, ProtocolValue::String);
gen_protocol_type_functions!(unwrap_protocol_bool, get_u8_bool, get_protocol_bool, bool, ProtocolValue::Bool);
gen_protocol_type_functions!(unwrap_protocol_byte, get_u8_byte, get_protocol_byte, u8, ProtocolValue::Byte);
gen_protocol_type_functions!(unwrap_protocol_short, get_u8_short, get_protocol_short, u16, ProtocolValue::Short);
gen_protocol_type_functions!(unwrap_protocol_int, get_u8_int, get_protocol_int, u32, ProtocolValue::Integer);
gen_protocol_type_functions!(unwrap_protocol_float, get_u8_float, get_protocol_float, f32, ProtocolValue::Float);
gen_protocol_type_functions!(
    unwrap_protocol_hashtable,
    get_u8_hashtable,
    get_protocol_hashtable,
    HashMap<ProtocolValue<'a>, ProtocolValue<'a>>,
    ProtocolValue::Hashtable
);
gen_protocol_type_functions!(
    unwrap_protocol_array,
    get_u8_array,
    get_protocol_array,
    Vec<ProtocolValue<'a>>,
    ProtocolValue::Array
);
