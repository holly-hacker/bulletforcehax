use byteorder::{BigEndian, ReadBytesExt};
use log::{debug, error};
use std::collections::HashMap;
use std::io::Cursor;

use super::*;

/// Version of read_value that returns an error on a non-string
fn read_string<'a>(c: &mut Cursor<&'a [u8]>) -> Result<Option<&'a str>, PacketReadError> {
    match read_value(c)? {
        ProtocolValue::Null() => Ok(None),
        ProtocolValue::String(string) => Ok(Some(string)),
        _ => Err(PacketReadError::UnexpectedProtocolValue),
    }
}

fn read_value<'a>(c: &mut Cursor<&'a [u8]>) -> Result<ProtocolValue<'a>, PacketReadError> {
    let protocol_type = c.read_u8()?;
    read_value_of_type(c, protocol_type)
}

fn read_value_of_type<'a>(c: &mut Cursor<&'a [u8]>, protocol_type: u8) -> Result<ProtocolValue<'a>, PacketReadError> {
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

// TODO: look into returning a slice here and in read_value_array
fn read_value_array_of_same_type<'a>(c: &mut Cursor<&'a [u8]>) -> Result<Vec<ProtocolValue<'a>>, PacketReadError> {
    let len = c.read_u16::<BigEndian>()?;
    let protocol_type = c.read_u8()?;
    let mut ret = Vec::new();
    for _i in 0..len {
        ret.push(read_value_of_type(c, protocol_type)?);
    }
    Ok(ret)
}

fn read_value_array<'a>(c: &mut Cursor<&'a [u8]>) -> Result<Vec<ProtocolValue<'a>>, PacketReadError> {
    let len = c.read_u16::<BigEndian>()?;
    let mut ret = Vec::new();
    for _i in 0..len {
        ret.push(read_value(c)?);
    }
    Ok(ret)
}

fn read_hash_table<'a>(c: &mut Cursor<&'a [u8]>) -> Result<HashMap<ProtocolValue<'a>, ProtocolValue<'a>>, PacketReadError> {
    let mut ret = HashMap::new();
    let len = c.read_u16::<BigEndian>()?;
    for _i in 0..len {
        ret.insert(read_value(c)?, read_value(c)?);
    }
    Ok(ret)
}

fn read_parameter_table<'a>(c: &mut Cursor<&'a [u8]>) -> Result<HashMap<u8, ProtocolValue<'a>>, PacketReadError> {
    let mut ret = HashMap::new();
    let len = c.read_u16::<BigEndian>()?;
    for _i in 0..len {
        ret.insert(c.read_u8()?, read_value(c)?);
    }
    debug!("Read param table: {:?}", ret);
    Ok(ret)
}

impl Packet<'_> {
    pub fn read(data: &[u8], direction: Direction) -> Result<Packet, PacketReadError> {
        let mut c = Cursor::new(data);

        let magic = c.read_u8()?;
        if magic != 0xF3 {
            return Err(PacketReadError::InvalidMagic(magic));
        }

        let packet_type: u8 = c.read_u8()?;

        match packet_type {
            0 => Ok(Packet::Init),
            1 => Ok(Packet::InitResponse),
            2 => Ok(Packet::OperationRequest(Operation::read(&mut c, direction)?)),
            3 => {
                let operation_type = c.read_u8()?;
                Ok(Packet::OperationResponse(
                    c.read_i16::<BigEndian>()?,
                    read_string(&mut c)?,
                    Operation::read_with_type(&mut c, direction, operation_type)?,
                ))
            }
            4 => Ok(Packet::Event(Event::read(&mut c, direction)?)),
            6 => Ok(Packet::InternalOperationRequest(InternalOperation::read(&mut c, direction)?)),
            7 => {
                let operation_type = c.read_u8()?;
                Ok(Packet::InternalOperationResponse(
                    c.read_i16::<BigEndian>()?,
                    read_string(&mut c)?,
                    InternalOperation::read_with_type(&mut c, direction, operation_type)?,
                ))
            }
            8 => Ok(Packet::Message),
            9 => Ok(Packet::RawMessage),
            _ => Err(PacketReadError::UnknownPacketType(packet_type)),
        }
    }
}

// TODO: how to check if I missed a field when deserializing?
impl Event {
    pub fn read(c: &mut Cursor<&[u8]>, direction: Direction) -> Result<Event, PacketReadError> {
        let event_type = c.read_u8()?;
        let _params = read_parameter_table(c)?;

        match event_type {
            210 => Ok(Event::AzureNodeInfo),
            223 => Ok(Event::AuthEvent),
            224 => Ok(Event::LobbyStats),
            226 => Ok(Event::AppStats),
            227 => Ok(Event::Match),
            228 => Ok(Event::QueueState),
            229 => Ok(Event::GameListUpdate),
            230 => Ok(Event::GameList),
            250 => Ok(Event::CacheSliceChanged),
            251 => Ok(Event::ErrorInfo),
            253 => match direction {
                Direction::Send => Ok(Event::SetProperties),
                Direction::Recv => Ok(Event::PropertiesChanged),
            },
            254 => Ok(Event::Leave),
            255 => Ok(Event::Join),
            _ => Err(PacketReadError::UnknownEventType(event_type)),
        }
    }
}

impl Operation<'_> {
    pub fn read<'a>(c: &mut Cursor<&'a [u8]>, direction: Direction) -> Result<Operation<'a>, PacketReadError> {
        let operation_type = c.read_u8()?;

        Operation::read_with_type(c, direction, operation_type)
    }
    pub fn read_with_type<'a>(c: &mut Cursor<&'a [u8]>, direction: Direction, operation_type: u8) -> Result<Operation<'a>, PacketReadError> {
        let params = read_parameter_table(c)?;

        match operation_type {
            217 => Ok(Operation::GetGameList),
            218 => Ok(Operation::ServerSettings),
            219 => Ok(Operation::WebRpc),
            220 => Ok(Operation::GetRegions),
            221 => Ok(Operation::GetLobbyStats),
            222 => Ok(Operation::FindFriends),
            224 => Ok(Operation::CancelJoinRandom),
            225 => Ok(Operation::JoinRandomGame),
            226 => Ok(Operation::JoinGame),
            227 => Ok(Operation::CreateGame),
            228 => Ok(Operation::LeaveLobby),
            229 => Ok(Operation::JoinLobby),
            230 => match direction {
                Direction::Send if params.contains_key(&(ParameterCode::Secret as u8)) => Ok(Operation::AuthenticateRequest2 {
                    secret: protocol_get_str(&params, ParameterCode::Secret)?,
                }),
                Direction::Send => Ok(Operation::AuthenticateRequest {
                    app_version: protocol_get_str(&params, ParameterCode::AppVersion)?,
                    application_id: protocol_get_str(&params, ParameterCode::ApplicationId)?,
                    region: protocol_get_str(&params, ParameterCode::Region)?,
                }),
                Direction::Recv if params.contains_key(&(ParameterCode::Position as u8)) => Ok(Operation::AuthenticateResponse2 {
                    secret: protocol_get_str(&params, ParameterCode::Secret)?,
                    position: protocol_get_int(&params, ParameterCode::Position)?,
                }),
                Direction::Recv => Ok(Operation::AuthenticateResponse {
                    unknown: protocol_get_str(&params, 196)?,
                    secret: protocol_get_str(&params, ParameterCode::Secret)?,
                    address: protocol_get_str(&params, ParameterCode::Address)?,
                    user_id: protocol_get_str(&params, ParameterCode::UserId)?,
                }),
            },
            231 => Ok(Operation::AuthenticateOnce),
            248 => Ok(Operation::ChangeGroups),
            250 => Ok(Operation::ExchangeKeysForEncryption),
            251 => Ok(Operation::GetProperties),
            252 => Ok(Operation::SetProperties),
            253 => Ok(Operation::RaiseEvent),
            254 => Ok(Operation::Leave),
            255 => Ok(Operation::Join),
            _ => Err(PacketReadError::UnknownOperationType(operation_type)),
        }
    }
}

impl InternalOperation {
    pub fn read(c: &mut Cursor<&[u8]>, direction: Direction) -> Result<InternalOperation, PacketReadError> {
        let operation_type = c.read_u8()?;

        InternalOperation::read_with_type(c, direction, operation_type)
    }
    pub fn read_with_type(c: &mut Cursor<&[u8]>, direction: Direction, operation_type: u8) -> Result<InternalOperation, PacketReadError> {
        let params = read_parameter_table(c)?;

        match operation_type {
            0 => Ok(InternalOperation::InitEncryption),
            1 => match direction {
                Direction::Send => Ok(InternalOperation::PingRequest {
                    local_time: protocol_get_int(&params, 1)?,
                }),
                Direction::Recv => Ok(InternalOperation::PingResponse {
                    local_time: protocol_get_int(&params, 1)?,
                    server_time: protocol_get_int(&params, 2)?,
                }),
            },
            _ => Err(PacketReadError::UnknownInternalOperationType(operation_type)),
        }
    }
}

fn protocol_get_int<'a>(map: &HashMap<u8, ProtocolValue<'a>>, param_code: u8) -> Result<u32, PacketReadError> {
    check_contains_key(map, param_code)?;

    match map[&param_code] {
        ProtocolValue::Integer(i) => Ok(i),
        _ => Err(PacketReadError::UnexpectedProtocolValue),
    }
}

fn protocol_get_str<'a>(map: &HashMap<u8, ProtocolValue<'a>>, param_code: u8) -> Result<&'a str, PacketReadError> {
    check_contains_key(map, param_code)?;

    match map[&param_code] {
        ProtocolValue::String(s) => Ok(s),
        _ => Err(PacketReadError::UnexpectedProtocolValue),
    }
}

fn check_contains_key<'a>(map: &HashMap<u8, ProtocolValue<'a>>, param_code: u8) -> Result<(), PacketReadError> {
    if !map.contains_key(&param_code) {
        error!("Couldn't find key {} in {:?}", param_code, map);
        return Err(PacketReadError::CouldNotFindKey(param_code));
    }

    Ok(())
}
