use super::*;
use std::collections::HashMap;

macro_rules! gen_protocol_type_functions {
    ($type_name:ident, $type:ty, $protocol_type:path) => {
        paste::item! {
            /// "Unwraps" a `ProtocolValue`, returning the internal type or an error if it doesn't match.
            #[allow(clippy::needless_lifetimes)]
            pub fn [<unwrap_protocol_ $type_name>]<'a>(protocol_type: ProtocolValue<'a>) -> PacketReadResult<$type> {
                match protocol_type {
                    $protocol_type(i) => Ok(i),
                    _ => Err(PacketReadError::UnexpectedProtocolValue),
                }
            }

            /// Extracts the type from `map` at key `param_code`, returning an None when not found.
            /// Returns an error if a value could not be unwrapped.
            #[allow(dead_code)]
            pub fn [<get_protocol_ $type_name _opt>]<'a>(
                map: &mut HashMap<ProtocolValue<'a>, ProtocolValue<'a>>,
                key: ProtocolValue<'static>,
            ) -> PacketReadResult<Option<$type>> {
                map.remove(&key).and_then(|x| Some([<unwrap_protocol_ $type_name>](x))).transpose()
            }

            /// Extracts the type from `map` at key `param_code`, returning an error when not found.
            /// Also returns an error if a value could not be unwrapped.
            #[allow(dead_code)]
            pub fn [<get_protocol_ $type_name>]<'a>(map: &mut HashMap<ProtocolValue<'a>, ProtocolValue<'a>>, key: ProtocolValue<'static>) -> PacketReadResult<$type> {
                // duplicated code because of lifetime issue. honestly can't be bothered to fix it.
                map.remove(&key).and_then(|x| Some([<unwrap_protocol_ $type_name>](x))).unwrap_or(Err(PacketReadError::CouldNotFindKeyProtocolValue(key)))
            }

            /// Extracts the type from `map` at key `param_code`, returning an None when not found.
            /// Returns an error if a value could not be unwrapped.
            #[allow(dead_code)]
            pub fn [<get_u8_ $type_name _opt>]<'a>(map: &mut HashMap<u8, ProtocolValue<'a>>, param_code: u8) -> PacketReadResult<Option<$type>> {
                map.remove(&param_code).and_then(|x| Some([<unwrap_protocol_ $type_name>](x))).transpose()
            }

            /// Extracts the type from `map` at key `param_code`, returning an error when not found.
            /// Also returns an error if a value could not be unwrapped.
            #[allow(dead_code)]
            pub fn [<get_u8_ $type_name>]<'a>(map: &mut HashMap<u8, ProtocolValue<'a>>, param_code: u8) -> PacketReadResult<$type> {
                [<get_u8_ $type_name _opt>](map, param_code).transpose().unwrap_or(Err(PacketReadError::CouldNotFindKey(param_code)))
            }
        }
    };
}

gen_protocol_type_functions!(string, &'a str, ProtocolValue::String);
gen_protocol_type_functions!(bool, bool, ProtocolValue::Bool);
gen_protocol_type_functions!(byte, u8, ProtocolValue::Byte);
gen_protocol_type_functions!(short, i16, ProtocolValue::Short);
gen_protocol_type_functions!(int, i32, ProtocolValue::Integer);
gen_protocol_type_functions!(float, f32, ProtocolValue::Float);
gen_protocol_type_functions!(hashtable, HashMap<ProtocolValue<'a>, ProtocolValue<'a>>, ProtocolValue::Hashtable);
gen_protocol_type_functions!(array, Vec<ProtocolValue<'a>>, ProtocolValue::Array);

/// Extracts a `ProtocolValue::Array` from `map` at key `param_code`, unwraps its items using `unwrap_fn`, and returns it (or None when not present).
/// Returns an error if a value could not be unwrapped.
pub fn get_u8_array_or_none<T, F>(map: &mut HashMap<u8, ProtocolValue>, param_code: u8, unwrap_fn: F) -> PacketReadResult<Option<Vec<T>>>
where
    F: Fn(ProtocolValue) -> PacketReadResult<T>,
{
    get_u8_array_opt(map, param_code)?
        .map(|arr| arr.into_iter().map(unwrap_fn).collect())
        .transpose()
}

/// Extracts a `ProtocolValue::Array` from `map` at key `param_code`, unwraps its items using `unwrap_fn`, and returns it (or an empty `Vec<T>` when not present).
/// Returns an error if a value could not be unwrapped.
#[allow(dead_code)]
pub fn get_u8_array_or_empty<T, F>(map: &mut HashMap<u8, ProtocolValue>, param_code: u8, unwrap_fn: F) -> PacketReadResult<Vec<T>>
where
    F: Fn(ProtocolValue) -> PacketReadResult<T>,
{
    get_u8_array_opt(map, param_code)?.map_or(Ok(Vec::new()), |arr| arr.into_iter().map(unwrap_fn).collect())
}
