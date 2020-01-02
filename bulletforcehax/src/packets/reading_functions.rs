use byteorder::{BigEndian, ReadBytesExt};
use log::{debug, error};
use std::collections::HashMap;
use std::io::Cursor;

use super::*;

/// Version of read_value that returns an error on a non-string
fn read_string<'a>(c: &mut Cursor<&'a [u8]>) -> PacketReadResult<Option<&'a str>> {
    match read_value(c)? {
        ProtocolValue::Null() => Ok(None),
        ProtocolValue::String(string) => Ok(Some(string)),
        _ => Err(PacketReadError::UnexpectedProtocolValue),
    }
}

fn read_value<'a>(c: &mut Cursor<&'a [u8]>) -> PacketReadResult<ProtocolValue<'a>> {
    let protocol_type = c.read_u8()?;
    read_value_of_type(c, protocol_type)
}

fn read_value_of_type<'a>(c: &mut Cursor<&'a [u8]>, protocol_type: u8) -> PacketReadResult<ProtocolValue<'a>> {
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
fn read_value_array_of_same_type<'a>(c: &mut Cursor<&'a [u8]>) -> PacketReadResult<Vec<ProtocolValue<'a>>> {
    let len = c.read_u16::<BigEndian>()?;
    let protocol_type = c.read_u8()?;
    let mut ret = Vec::new();
    for _i in 0..len {
        ret.push(read_value_of_type(c, protocol_type)?);
    }
    Ok(ret)
}

fn read_value_array<'a>(c: &mut Cursor<&'a [u8]>) -> PacketReadResult<Vec<ProtocolValue<'a>>> {
    let len = c.read_u16::<BigEndian>()?;
    let mut ret = Vec::new();
    for _i in 0..len {
        ret.push(read_value(c)?);
    }
    Ok(ret)
}

fn read_hash_table<'a>(c: &mut Cursor<&'a [u8]>) -> PacketReadResult<HashMap<ProtocolValue<'a>, ProtocolValue<'a>>> {
    let mut ret = HashMap::new();
    let len = c.read_u16::<BigEndian>()?;
    for _i in 0..len {
        ret.insert(read_value(c)?, read_value(c)?);
    }
    Ok(ret)
}

fn read_parameter_table<'a>(c: &mut Cursor<&'a [u8]>) -> PacketReadResult<HashMap<u8, ProtocolValue<'a>>> {
    let mut ret = HashMap::new();
    let len = c.read_u16::<BigEndian>()?;
    for _i in 0..len {
        ret.insert(c.read_u8()?, read_value(c)?);
    }
    Ok(ret)
}

impl Packet<'_> {
    pub fn read(data: &[u8], direction: Direction) -> PacketReadResult<Packet> {
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
    pub fn read(c: &mut Cursor<&[u8]>, direction: Direction) -> PacketReadResult<Event> {
        fn err(event: Event, params: HashMap<u8, ProtocolValue>) -> PacketReadResult<Event> {
            debug!("Unimplemented Event: {:?}, {:#?}", event, params);
            Err(PacketReadError::UnimplementedEventType(event))
        }

        let event_type = c.read_u8()?;
        let params = read_parameter_table(c)?;

        match event_type {
            210 => err(Event::AzureNodeInfo, params),
            223 => err(Event::AuthEvent, params),
            224 => err(Event::LobbyStats, params),
            226 => Ok(Event::AppStats {
                game_count: protocol_get_int(&params, ParameterCode::GameCount)?,
                peer_count: protocol_get_int(&params, ParameterCode::PeerCount)?,
                master_peer_count: protocol_get_int(&params, ParameterCode::MasterPeerCount)?,
            }),
            227 => err(Event::Match, params),
            228 => err(Event::QueueState, params),
            229 => err(Event::GameListUpdate, params),
            230 => err(Event::GameList, params),
            250 => err(Event::CacheSliceChanged, params),
            251 => err(Event::ErrorInfo, params),
            253 => match direction {
                Direction::Send => err(Event::SetProperties, params),
                Direction::Recv => err(Event::PropertiesChanged, params),
            },
            254 => err(Event::Leave, params),
            255 => err(Event::Join, params),
            _ => Err(PacketReadError::UnknownEventType(event_type)),
        }
    }
}

impl Operation<'_> {
    pub fn read<'a>(c: &mut Cursor<&'a [u8]>, direction: Direction) -> PacketReadResult<Operation<'a>> {
        let operation_type = c.read_u8()?;

        Operation::read_with_type(c, direction, operation_type)
    }
    pub fn read_with_type<'a>(c: &mut Cursor<&'a [u8]>, direction: Direction, operation_type: u8) -> PacketReadResult<Operation<'a>> {
        fn err<'a>(operation: Operation<'static>, params: HashMap<u8, ProtocolValue>) -> PacketReadResult<Operation<'a>> {
            debug!("Unimplemented Operation: {:?}, {:#?}", operation, params);
            Err(PacketReadError::UnimplementedOperationType(operation))
        }

        let params = read_parameter_table(c)?;

        match operation_type {
            217 => err(Operation::GetGameList, params),
            218 => err(Operation::ServerSettings, params),
            219 => err(Operation::WebRpc, params),
            220 => err(Operation::GetRegions, params),
            221 => err(Operation::GetLobbyStats, params),
            222 => err(Operation::FindFriends, params),
            224 => err(Operation::CancelJoinRandom, params),
            225 => err(Operation::JoinRandomGame, params),
            226 => err(Operation::JoinGame, params),
            227 => err(Operation::CreateGame, params),
            228 => err(Operation::LeaveLobby, params),
            229 => Ok(Operation::JoinLobby()),
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
            231 => err(Operation::AuthenticateOnce, params),
            248 => err(Operation::ChangeGroups, params),
            250 => err(Operation::ExchangeKeysForEncryption, params),
            251 => err(Operation::GetProperties, params),
            252 => err(Operation::SetProperties, params),
            253 => err(Operation::RaiseEvent, params),
            254 => err(Operation::Leave, params),
            255 => err(Operation::Join, params),
            _ => Err(PacketReadError::UnknownOperationType(operation_type)),
        }
    }
}

impl InternalOperation {
    pub fn read(c: &mut Cursor<&[u8]>, direction: Direction) -> PacketReadResult<InternalOperation> {
        let operation_type = c.read_u8()?;

        InternalOperation::read_with_type(c, direction, operation_type)
    }
    pub fn read_with_type(c: &mut Cursor<&[u8]>, direction: Direction, operation_type: u8) -> PacketReadResult<InternalOperation> {
        fn err<'a>(operation: InternalOperation, params: HashMap<u8, ProtocolValue>) -> PacketReadResult<InternalOperation> {
            debug!("Unimplemented InternalOperation: {:?}, {:#?}", operation, params);
            Err(PacketReadError::UnimplementedInternalOperationType(operation))
        }

        let params = read_parameter_table(c)?;

        match operation_type {
            0 => err(InternalOperation::InitEncryption, params),
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

fn protocol_get_int<'a>(map: &HashMap<u8, ProtocolValue<'a>>, param_code: u8) -> PacketReadResult<u32> {
    check_contains_key(map, param_code)?;

    match map[&param_code] {
        ProtocolValue::Integer(i) => Ok(i),
        _ => Err(PacketReadError::UnexpectedProtocolValue),
    }
}

fn protocol_get_str<'a>(map: &HashMap<u8, ProtocolValue<'a>>, param_code: u8) -> PacketReadResult<&'a str> {
    check_contains_key(map, param_code)?;

    match map[&param_code] {
        ProtocolValue::String(s) => Ok(s),
        _ => Err(PacketReadError::UnexpectedProtocolValue),
    }
}

fn check_contains_key<'a>(map: &HashMap<u8, ProtocolValue<'a>>, param_code: u8) -> PacketReadResult<()> {
    if !map.contains_key(&param_code) {
        error!("Couldn't find key {} in {:?}", param_code, map);
        return Err(PacketReadError::CouldNotFindKey(param_code));
    }

    Ok(())
}
