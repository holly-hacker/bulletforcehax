use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use log::{debug, warn};
use maplit::hashmap;
use std::collections::HashMap;
use std::io::{Cursor, Write};

use read_write::{read_parameter_table, read_string, write_parameter_table, write_value_of_type};

use super::*;

impl Packet<'_> {
    pub fn read<'a>(data: &'a [u8], direction: Direction) -> PacketReadResult<Packet<'a>> {
        let ref mut c = Cursor::new(data);
        let magic = c.read_u8()?;
        if magic != 0xF3 {
            return Err(PacketReadError::InvalidMagic(magic));
        }

        fn err<'a>(packet: Packet<'static>) -> PacketReadResult<Packet<'a>> {
            Err(PacketReadError::UnimplementedPacketType(packet))
        }

        let packet_type: u8 = c.read_u8()?;
        match packet_type {
            0 => err(Packet::Init),
            1 => err(Packet::InitResponse),
            2 => Ok(Packet::OperationRequest(Operation::read(c, direction)?)),
            3 => {
                let operation_type = c.read_u8()?;
                Ok(Packet::OperationResponse(
                    c.read_i16::<BigEndian>()?,
                    read_string(c)?,
                    Operation::read_with_type(c, direction, operation_type)?,
                ))
            }
            4 => Ok(Packet::Event(Event::read(c, direction)?)),
            6 => Ok(Packet::InternalOperationRequest(InternalOperation::read(c, direction)?)),
            7 => {
                let operation_type = c.read_u8()?;
                Ok(Packet::InternalOperationResponse(
                    c.read_i16::<BigEndian>()?,
                    read_string(c)?,
                    InternalOperation::read_with_type(c, direction, operation_type)?,
                ))
            }
            8 => err(Packet::Message),
            9 => err(Packet::RawMessage),
            _ => Err(PacketReadError::UnknownPacketType(packet_type)),
        }
    }

    fn get_type(&self) -> u8 {
        match self {
            Packet::Init => 0,
            Packet::InitResponse => 1,
            Packet::OperationRequest(_) => 2,
            Packet::OperationResponse(_, _, _) => 3,
            Packet::Event(_) => 4,
            Packet::InternalOperationRequest(_) => 6,
            Packet::InternalOperationResponse(_, _, _) => 7,
            Packet::Message => 8,
            Packet::RawMessage => 9,
        }
    }

    pub fn into_vec(self) -> PacketWriteResult<Vec<u8>> {
        let mut vec = Vec::new();
        let ref mut writer = vec;

        writer.write_u8(0xF3)?;
        writer.write_u8(self.get_type())?;

        fn err<'a>(packet: Packet<'static>) -> PacketWriteResult<()> {
            Err(PacketWriteError::UnimplementedPacketType(packet))
        }

        match self {
            Packet::Init => err(Packet::Init),
            Packet::InitResponse => err(Packet::InitResponse),
            Packet::OperationRequest(operation) => operation.write(writer),
            Packet::OperationResponse(return_value, debug_string, operation) => {
                writer.write_u8(operation.get_type())?;
                writer.write_i16::<BigEndian>(return_value)?;
                match debug_string {
                    Some(x) => write_value_of_type(writer, ProtocolValue::String(x))?,
                    None => write_value_of_type(writer, ProtocolValue::Null())?,
                }
                operation.write_without_type(writer)?;
                Ok(())
            }
            Packet::Event(event) => event.write(writer),
            Packet::InternalOperationRequest(operation) => operation.write(writer),
            Packet::InternalOperationResponse(return_value, debug_string, operation) => {
                writer.write_u8(operation.get_type())?;
                writer.write_i16::<BigEndian>(return_value)?;
                match debug_string {
                    Some(x) => write_value_of_type(writer, ProtocolValue::String(x))?,
                    None => write_value_of_type(writer, ProtocolValue::Null())?,
                }
                operation.write_without_type(writer)?;
                Ok(())
            }
            Packet::Message => err(Packet::Message),
            Packet::RawMessage => err(Packet::RawMessage),
        }?;

        Ok(vec)
    }
}

impl<'s> Event<'s> {
    pub fn read<'a>(c: &mut Cursor<&'a [u8]>, direction: Direction) -> PacketReadResult<Event<'a>> {
        fn err<'a>(event: Event<'static>, params: &HashMap<u8, ProtocolValue>) -> PacketReadResult<Event<'a>> {
            debug!("Unimplemented Event: {:?}, {:#?}", event, params);
            Err(PacketReadError::UnimplementedEventType(event))
        }

        let event_type = c.read_u8()?;
        let mut params = read_parameter_table(c)?;

        let ret = match event_type {
            210 => err(Event::AzureNodeInfo, &params),
            223 => err(Event::AuthEvent, &params),
            224 => err(Event::LobbyStats, &params),
            226 => Ok(Event::AppStats {
                game_count: get_protocol_int(&mut params, ParameterCode::GameCount)?,
                peer_count: get_protocol_int(&mut params, ParameterCode::PeerCount)?,
                master_peer_count: get_protocol_int(&mut params, ParameterCode::MasterPeerCount)?,
            }),
            227 => err(Event::Match, &params),
            228 => err(Event::QueueState, &params),
            229 => Ok(Event::GameListUpdate(GameInfo::new_from_hashtable_table(get_protocol_hashtable(
                &mut params,
                ParameterCode::GameList,
            )?)?)),
            230 => Ok(Event::GameList(
                GameInfo::new_from_hashtable_table(get_protocol_hashtable(&mut params, ParameterCode::GameList)?).map(|mut table| {
                    table
                        .drain()
                        .map(|(_key, value)| value.expect("GameList packet contained removed game"))
                        .into_iter()
                        .collect()
                })?,
            )),
            250 => err(Event::CacheSliceChanged, &params),
            251 => err(Event::ErrorInfo, &params),
            253 => match direction {
                Direction::Send => err(Event::SetProperties, &params),
                Direction::Recv => err(Event::PropertiesChanged, &params),
            },
            254 => err(Event::Leave, &params),
            255 => err(Event::Join, &params),
            _ => Err(PacketReadError::UnknownEventType(event_type)),
        };

        if ret.is_ok() && params.len() > 0 {
            warn!("Missed event parameters: {:#?}, obj is {:#?}", params, ret);
        }

        ret
    }

    pub fn write(self, c: &mut dyn Write) -> PacketWriteResult<()> {
        c.write_u8(self.get_type())?;
        write_parameter_table(c, self.get_param_map()?)?;
        Ok(())
    }

    fn get_type(&self) -> u8 {
        match self {
            Event::AzureNodeInfo => 210,
            Event::AuthEvent => 223,
            Event::LobbyStats => 224,
            Event::AppStats { .. } => 226,
            Event::Match => 227,
            Event::QueueState => 228,
            Event::GameListUpdate(_) => 229,
            Event::GameList(_) => 230,
            Event::CacheSliceChanged => 250,
            Event::ErrorInfo => 251,
            Event::SetProperties => 253,
            Event::PropertiesChanged => 253,
            Event::Leave => 254,
            Event::Join => 255,
        }
    }

    fn get_param_map(self) -> PacketWriteResult<HashMap<u8, ProtocolValue<'s>>> {
        fn err<'a>(event: Event<'static>) -> PacketWriteResult<HashMap<u8, ProtocolValue>> {
            Err(PacketWriteError::UnimplementedEventType(event))
        }

        match self {
            Event::AzureNodeInfo => err(Event::AzureNodeInfo),
            Event::AuthEvent => err(Event::AuthEvent),
            Event::LobbyStats => err(Event::LobbyStats),
            Event::AppStats {
                game_count,
                peer_count,
                master_peer_count,
            } => Ok(hashmap! {
                ParameterCode::GameCount => ProtocolValue::Integer(game_count),
                ParameterCode::PeerCount => ProtocolValue::Integer(peer_count),
                ParameterCode::MasterPeerCount => ProtocolValue::Integer(master_peer_count),
            }),
            Event::Match => err(Event::Match),
            Event::QueueState => err(Event::QueueState),
            Event::GameListUpdate(info) => Ok(hashmap! {
                ParameterCode::GameList => ProtocolValue::Hashtable(info
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            ProtocolValue::String(k),
                            ProtocolValue::Hashtable(match v {
                                Some(info) => info.into_hashtable(),
                                None => hashmap! {
                                    ProtocolValue::Byte(251) => ProtocolValue::Bool(true),
                                },
                            }),
                        )
                    })
                    .collect())
            }),
            Event::GameList(info) => Ok(hashmap! {
                ParameterCode::GameList => ProtocolValue::Hashtable(info
                    .into_iter()
                    .map(|info| (ProtocolValue::String(info.room_name), ProtocolValue::Hashtable(info.into_hashtable())))
                    .collect())
            }),
            Event::CacheSliceChanged => err(Event::CacheSliceChanged),
            Event::ErrorInfo => err(Event::ErrorInfo),
            Event::SetProperties => err(Event::SetProperties),
            Event::PropertiesChanged => err(Event::PropertiesChanged),
            Event::Leave => err(Event::Leave),
            Event::Join => err(Event::Join),
        }
    }
}

impl<'s> Operation<'s> {
    pub fn read<'a>(c: &mut Cursor<&'a [u8]>, direction: Direction) -> PacketReadResult<Operation<'a>> {
        let operation_type = c.read_u8()?;

        Operation::read_with_type(c, direction, operation_type)
    }
    pub fn read_with_type<'a>(c: &mut Cursor<&'a [u8]>, direction: Direction, operation_type: u8) -> PacketReadResult<Operation<'a>> {
        fn err<'a>(operation: Operation<'static>, params: &HashMap<u8, ProtocolValue>) -> PacketReadResult<Operation<'a>> {
            debug!("Unimplemented Operation: {:?}, {:#?}", operation, params);
            Err(PacketReadError::UnimplementedOperationType(operation))
        }

        let mut params = read_parameter_table(c)?;

        let ret = match operation_type {
            217 => err(Operation::GetGameList, &params),
            218 => err(Operation::ServerSettings, &params),
            219 => err(Operation::WebRpc, &params),
            220 => err(Operation::GetRegions, &params),
            221 => err(Operation::GetLobbyStats, &params),
            222 => err(Operation::FindFriends, &params),
            224 => err(Operation::CancelJoinRandom, &params),
            225 => err(Operation::JoinRandomGame, &params),
            226 => err(Operation::JoinGame, &params),
            227 => match direction {
                Direction::Send if !params.contains_key(&ParameterCode::GameProperties) => Ok(Operation::CreateGameRequest {
                    room_name: get_protocol_string(&mut params, ParameterCode::RoomName)?,
                }),
                Direction::Send => Ok(Operation::CreateGameRequest2 {
                    broadcast: get_protocol_bool(&mut params, ParameterCode::Broadcast)?,
                    room_name: get_protocol_string(&mut params, ParameterCode::RoomName)?,
                    game_properties: GameProperties::new_from_hashtable(get_protocol_hashtable(&mut params, ParameterCode::GameProperties)?)?,
                    player_properties: PlayerProperties::new_from_hashtable(get_protocol_hashtable(&mut params, ParameterCode::PlayerProperties)?)?,
                    room_option_flags: get_protocol_int(&mut params, ParameterCode::RoomOptionFlags)?,
                    cleanup_cache_on_leave: get_protocol_bool(&mut params, ParameterCode::CleanupCacheOnLeave)?,
                    check_user_on_join: get_protocol_bool(&mut params, ParameterCode::CheckUserOnJoin)?,
                }),
                Direction::Recv if !params.contains_key(&ParameterCode::GameProperties) => Ok(Operation::CreateGameResponse {
                    room_name: get_protocol_string(&mut params, ParameterCode::RoomName)?,
                    secret: get_protocol_string(&mut params, ParameterCode::Secret)?,
                    address: get_protocol_string(&mut params, ParameterCode::Address)?,
                }),
                Direction::Recv => Ok(Operation::CreateGameResponse2 {
                    actor_list: get_protocol_array(&mut params, ParameterCode::ActorList).map(|protocol_array| {
                        protocol_array
                            .into_iter()
                            .map(|protocol_value| unwrap_protocol_int(protocol_value).expect("CreateGame response 2 had a non-int actor id"))
                            .collect()
                    })?,
                    actor_nr: get_protocol_int(&mut params, ParameterCode::ActorNr)?,
                    game_properties: GameProperties::new_from_hashtable(get_protocol_hashtable(&mut params, ParameterCode::GameProperties)?)?,
                }),
            },
            228 => err(Operation::LeaveLobby, &params),
            229 => Ok(Operation::JoinLobby()),
            230 => match direction {
                Direction::Send if params.contains_key(&ParameterCode::Secret) => Ok(Operation::AuthenticateRequest2 {
                    secret: get_protocol_string(&mut params, ParameterCode::Secret)?,
                }),
                Direction::Send => Ok(Operation::AuthenticateRequest {
                    app_version: get_protocol_string(&mut params, ParameterCode::AppVersion)?,
                    application_id: get_protocol_string(&mut params, ParameterCode::ApplicationId)?,
                    region: get_protocol_string(&mut params, ParameterCode::Region)?,
                }),
                Direction::Recv if params.contains_key(&ParameterCode::Position) => Ok(Operation::AuthenticateResponse2 {
                    secret: get_protocol_string(&mut params, ParameterCode::Secret)?,
                    position: get_protocol_int(&mut params, ParameterCode::Position)?,
                }),
                Direction::Recv => Ok(Operation::AuthenticateResponse {
                    unknown: get_protocol_string(&mut params, 196)?, // TODO: [243, 3, 230, 0, 0, 42, 0, 0]
                    secret: get_protocol_string(&mut params, ParameterCode::Secret)?,
                    address: get_protocol_string(&mut params, ParameterCode::Address)?,
                    user_id: get_protocol_string(&mut params, ParameterCode::UserId)?,
                }),
            },
            231 => err(Operation::AuthenticateOnce, &params),
            248 => err(Operation::ChangeGroups, &params),
            250 => err(Operation::ExchangeKeysForEncryption, &params),
            251 => err(Operation::GetProperties, &params),
            252 => err(Operation::SetProperties, &params),
            253 => err(Operation::RaiseEvent, &params),
            254 => err(Operation::Leave, &params),
            255 => err(Operation::Join, &params),
            _ => Err(PacketReadError::UnknownOperationType(operation_type)),
        };

        if ret.is_ok() && params.len() > 0 {
            warn!("Missed operation parameters: {:#?}, obj is {:#?}", params, ret);
        }

        ret
    }

    fn write(self, c: &mut dyn Write) -> PacketWriteResult<()> {
        c.write_u8(self.get_type())?;
        self.write_without_type(c)
    }

    pub fn write_without_type(self, c: &mut dyn Write) -> PacketWriteResult<()> {
        write_parameter_table(c, self.get_param_map()?)?;
        Ok(())
    }

    pub fn get_type(&self) -> u8 {
        match self {
            Operation::GetGameList => 217,
            Operation::ServerSettings => 218,
            Operation::WebRpc => 219,
            Operation::GetRegions => 220,
            Operation::GetLobbyStats => 221,
            Operation::FindFriends => 222,
            Operation::CancelJoinRandom => 224,
            Operation::JoinRandomGame => 225,
            Operation::JoinGame => 226,
            Operation::CreateGameRequest { .. } => 227,
            Operation::CreateGameResponse { .. } => 227,
            Operation::CreateGameRequest2 { .. } => 227,
            Operation::CreateGameResponse2 { .. } => 227,
            Operation::LeaveLobby => 228,
            Operation::JoinLobby() => 229,
            Operation::AuthenticateRequest { .. } => 230,
            Operation::AuthenticateResponse { .. } => 230,
            Operation::AuthenticateRequest2 { .. } => 230,
            Operation::AuthenticateResponse2 { .. } => 230,
            Operation::AuthenticateOnce => 231,
            Operation::ChangeGroups => 248,
            Operation::ExchangeKeysForEncryption => 250,
            Operation::GetProperties => 251,
            Operation::SetProperties => 252,
            Operation::RaiseEvent => 253,
            Operation::Leave => 254,
            Operation::Join => 255,
        }
    }

    fn get_param_map(self) -> PacketWriteResult<HashMap<u8, ProtocolValue<'s>>> {
        fn err<'a>(operation: Operation<'static>) -> PacketWriteResult<HashMap<u8, ProtocolValue>> {
            Err(PacketWriteError::UnimplementedOperationType(operation))
        }

        match self {
            Operation::GetGameList => err(Operation::GetGameList),
            Operation::ServerSettings => err(Operation::ServerSettings),
            Operation::WebRpc => err(Operation::WebRpc),
            Operation::GetRegions => err(Operation::GetRegions),
            Operation::GetLobbyStats => err(Operation::GetLobbyStats),
            Operation::FindFriends => err(Operation::FindFriends),
            Operation::CancelJoinRandom => err(Operation::CancelJoinRandom),
            Operation::JoinRandomGame => err(Operation::JoinRandomGame),
            Operation::JoinGame => err(Operation::JoinGame),
            Operation::CreateGameRequest { room_name } => Ok(hashmap! {
                ParameterCode::RoomName => ProtocolValue::String(room_name),
            }),
            Operation::CreateGameResponse { room_name, address, secret } => Ok(hashmap! {
                ParameterCode::RoomName => ProtocolValue::String(room_name),
                ParameterCode::Address => ProtocolValue::String(address),
                ParameterCode::Secret => ProtocolValue::String(secret),
            }),
            Operation::CreateGameRequest2 {
                broadcast,
                room_name,
                game_properties,
                player_properties,
                room_option_flags,
                cleanup_cache_on_leave,
                check_user_on_join,
            } => Ok(hashmap! {
                ParameterCode::Broadcast => ProtocolValue::Bool(broadcast),
                ParameterCode::RoomName => ProtocolValue::String(room_name),
                ParameterCode::GameProperties => ProtocolValue::Hashtable(game_properties.into_hashtable()),
                ParameterCode::PlayerProperties => ProtocolValue::Hashtable(player_properties.into_hashtable()),
                ParameterCode::RoomOptionFlags => ProtocolValue::Integer(room_option_flags),
                ParameterCode::CleanupCacheOnLeave => ProtocolValue::Bool(cleanup_cache_on_leave),
                ParameterCode::CheckUserOnJoin => ProtocolValue::Bool(check_user_on_join),
            }),
            Operation::CreateGameResponse2 {
                actor_list,
                actor_nr,
                game_properties,
            } => Ok(hashmap! {
                ParameterCode::ActorList => ProtocolValue::Array(actor_list.into_iter().map(|id| ProtocolValue::Integer(id)).collect()),
                ParameterCode::ActorNr => ProtocolValue::Integer(actor_nr),
                ParameterCode::GameProperties => ProtocolValue::Hashtable(game_properties.into_hashtable()),
            }),
            Operation::LeaveLobby => err(Operation::LeaveLobby),
            Operation::JoinLobby() => Ok(hashmap!()),
            Operation::AuthenticateRequest {
                region,
                application_id,
                app_version,
            } => Ok(hashmap! {
                ParameterCode::Region => ProtocolValue::String(region),
                ParameterCode::ApplicationId => ProtocolValue::String(application_id),
                ParameterCode::AppVersion => ProtocolValue::String(app_version),
            }),
            Operation::AuthenticateResponse {
                unknown,
                secret,
                address,
                user_id,
            } => Ok(hashmap! {
                196 => ProtocolValue::String(unknown),
                ParameterCode::Secret => ProtocolValue::String(secret),
                ParameterCode::Address => ProtocolValue::String(address),
                ParameterCode::UserId => ProtocolValue::String(user_id),

            }),
            Operation::AuthenticateRequest2 { secret } => Ok(hashmap!(ParameterCode::Secret => ProtocolValue::String(secret))),
            Operation::AuthenticateResponse2 { secret, position } => Ok(hashmap! {
                ParameterCode::Secret => ProtocolValue::String(secret),
                ParameterCode::Position => ProtocolValue::Integer(position),
            }),
            Operation::AuthenticateOnce => err(Operation::AuthenticateOnce),
            Operation::ChangeGroups => err(Operation::ChangeGroups),
            Operation::ExchangeKeysForEncryption => err(Operation::ExchangeKeysForEncryption),
            Operation::GetProperties => err(Operation::GetProperties),
            Operation::SetProperties => err(Operation::SetProperties),
            Operation::RaiseEvent => err(Operation::RaiseEvent),
            Operation::Leave => err(Operation::Leave),
            Operation::Join => err(Operation::Join),
        }
    }
}

impl<'s> InternalOperation {
    pub fn read<'a>(c: &mut Cursor<&'a [u8]>, direction: Direction) -> PacketReadResult<InternalOperation> {
        let operation_type = c.read_u8()?;

        InternalOperation::read_with_type(c, direction, operation_type)
    }
    pub fn read_with_type<'a>(c: &mut Cursor<&'a [u8]>, direction: Direction, operation_type: u8) -> PacketReadResult<InternalOperation> {
        fn err<'a>(operation: InternalOperation, params: &HashMap<u8, ProtocolValue>) -> PacketReadResult<InternalOperation> {
            debug!("Unimplemented InternalOperation: {:?}, {:#?}", operation, params);
            Err(PacketReadError::UnimplementedInternalOperationType(operation))
        }

        let mut params = read_parameter_table(c)?;

        let ret = match operation_type {
            0 => err(InternalOperation::InitEncryption, &params),
            1 => match direction {
                Direction::Send => Ok(InternalOperation::PingRequest {
                    local_time: get_protocol_int(&mut params, 1)?,
                }),
                Direction::Recv => Ok(InternalOperation::PingResponse {
                    local_time: get_protocol_int(&mut params, 1)?,
                    server_time: get_protocol_int(&mut params, 2)?,
                }),
            },
            _ => Err(PacketReadError::UnknownInternalOperationType(operation_type)),
        };

        if ret.is_ok() && params.len() > 0 {
            warn!("Missed operation parameters: {:#?}, obj is {:#?}", params, ret);
        }

        ret
    }

    pub fn write(self, c: &mut dyn Write) -> PacketWriteResult<()> {
        c.write_u8(self.get_type())?;
        self.write_without_type(c)
    }

    pub fn write_without_type(self, c: &mut dyn Write) -> PacketWriteResult<()> {
        write_parameter_table(c, self.get_param_map()?)?;
        Ok(())
    }

    pub fn get_type(&self) -> u8 {
        match self {
            InternalOperation::InitEncryption => 0,
            InternalOperation::PingRequest { .. } => 1,
            InternalOperation::PingResponse { .. } => 1,
        }
    }

    fn get_param_map(self) -> PacketWriteResult<HashMap<u8, ProtocolValue<'s>>> {
        fn err<'a>(operation: InternalOperation) -> PacketWriteResult<HashMap<u8, ProtocolValue<'a>>> {
            Err(PacketWriteError::UnimplementedInternalOperationType(operation))
        }

        match self {
            InternalOperation::InitEncryption => err(InternalOperation::InitEncryption),
            InternalOperation::PingRequest { local_time } => Ok(hashmap!(1 => ProtocolValue::Integer(local_time))),
            InternalOperation::PingResponse { local_time, server_time } => Ok(hashmap! {
                1 => ProtocolValue::Integer(local_time),
                2 => ProtocolValue::Integer(server_time),
            }),
        }
    }
}

impl<'s> GameInfo<'s> {
    pub fn new_from_hashtable_table<'a>(
        big_table: HashMap<ProtocolValue<'a>, ProtocolValue<'a>>,
    ) -> PacketReadResult<HashMap<&'a str, Option<GameInfo<'a>>>> {
        let mut map: HashMap<&'a str, Option<GameInfo<'a>>> = HashMap::new();
        for (key, value) in big_table {
            // could look into getting map past the borrow checker
            let ht = unwrap_protocol_hashtable(value)?;
            let val = GameInfo::new_from_hashtable(ht)?;
            map.insert(unwrap_protocol_string(key)?, val);
        }

        Ok(map)
    }
    pub fn new_from_hashtable<'a>(mut table: HashMap<ProtocolValue<'a>, ProtocolValue<'a>>) -> PacketReadResult<Option<GameInfo<'a>>> {
        if table.contains_key(&ProtocolValue::Byte(251)) {
            // got removed
            return Ok(None);
        }

        let ret = Ok(Some(GameInfo {
            game_id: get_u8_string(&mut table, ProtocolValue::String("gameID"))?,
            room_id: get_u8_string(&mut table, ProtocolValue::String("roomID"))?,
            store_id: get_u8_string(&mut table, ProtocolValue::String("storeID"))?,
            room_name: get_u8_string(&mut table, ProtocolValue::String("roomName"))?,
            mode_name: get_u8_string(&mut table, ProtocolValue::String("modeName"))?,
            password: get_u8_string(&mut table, ProtocolValue::String("password"))?,
            map_name: get_u8_string(&mut table, ProtocolValue::String("mapName"))?,
            match_started: get_u8_bool(&mut table, ProtocolValue::String("matchStarted"))?,
            switching_map: get_u8_bool(&mut table, ProtocolValue::String("switchingmap"))?,
            room_type: get_u8_byte(&mut table, ProtocolValue::String("roomType"))?,
            dedicated: get_u8_bool(&mut table, ProtocolValue::String("dedicated"))?,
            hardcore: get_u8_bool(&mut table, ProtocolValue::String("hardcore"))?,
            allowed_weapons: {
                // this is an array of 2 u32s, but we save this as a u64 because it makes more sense
                let mut arr = get_u8_array(&mut table, ProtocolValue::String("allowedweapons"))?;
                if arr.len() != 2 {
                    return Err(PacketReadError::Other(format!("allowedweapons array was not 2 long, but {}", arr.len())));
                }
                let int2 = unwrap_protocol_int(arr.remove(1))? as u64;
                let int1 = unwrap_protocol_int(arr.remove(0))? as u64;
                int1 | (int2 << 32)
            },
            mean_rank: get_u8_float(&mut table, ProtocolValue::String("meanRank"))?,
            mean_kd: get_u8_float(&mut table, ProtocolValue::String("meanKD"))?,
            average_rank: get_u8_int(&mut table, ProtocolValue::String("averagerank"))?,
            event_code: get_u8_int(&mut table, ProtocolValue::String("eventcode"))?,
            byte_252: get_u8_byte(&mut table, ProtocolValue::Byte(252))?,
            byte_253: get_u8_bool(&mut table, ProtocolValue::Byte(253))?,
            byte_255: get_u8_byte(&mut table, ProtocolValue::Byte(255))?,
        }));

        if ret.is_ok() && table.len() > 0 {
            warn!("Missed GameInfo parameters: {:#?}, obj is {:#?}", table, ret);
        }

        ret
    }
    pub fn into_hashtable(self) -> HashMap<ProtocolValue<'s>, ProtocolValue<'s>> {
        let mut map = HashMap::new();
        map.insert(ProtocolValue::String("gameID"), ProtocolValue::String(self.game_id));
        map.insert(ProtocolValue::String("roomID"), ProtocolValue::String(self.room_id));
        map.insert(ProtocolValue::String("storeID"), ProtocolValue::String(self.store_id));
        map.insert(ProtocolValue::String("roomName"), ProtocolValue::String(self.room_name));
        map.insert(ProtocolValue::String("modeName"), ProtocolValue::String(self.mode_name));
        map.insert(ProtocolValue::String("password"), ProtocolValue::String(self.password));
        map.insert(ProtocolValue::String("mapName"), ProtocolValue::String(self.map_name));
        map.insert(ProtocolValue::String("matchStarted"), ProtocolValue::Bool(self.match_started));
        map.insert(ProtocolValue::String("switchingmap"), ProtocolValue::Bool(self.switching_map));
        map.insert(ProtocolValue::String("roomType"), ProtocolValue::Byte(self.room_type));
        map.insert(ProtocolValue::String("dedicated"), ProtocolValue::Bool(self.dedicated));
        map.insert(ProtocolValue::String("hardcore"), ProtocolValue::Bool(self.hardcore));
        map.insert(
            ProtocolValue::String("allowedweapons"),
            ProtocolValue::Array(vec![
                ProtocolValue::Integer((self.allowed_weapons & 0xFFFFFFFF) as u32),
                ProtocolValue::Integer((self.allowed_weapons >> 32) as u32),
            ]),
        );
        map.insert(ProtocolValue::String("meanRank"), ProtocolValue::Float(self.mean_rank));
        map.insert(ProtocolValue::String("meanKD"), ProtocolValue::Float(self.mean_kd));
        map.insert(ProtocolValue::String("averagerank"), ProtocolValue::Integer(self.average_rank));
        map.insert(ProtocolValue::String("eventcode"), ProtocolValue::Integer(self.event_code));
        map.insert(ProtocolValue::Byte(252), ProtocolValue::Byte(self.byte_252));
        map.insert(ProtocolValue::Byte(253), ProtocolValue::Bool(self.byte_253));
        map.insert(ProtocolValue::Byte(255), ProtocolValue::Byte(self.byte_255));
        map
    }
}

impl<'s> GameProperties<'s> {
    pub fn new_from_hashtable<'a>(mut table: HashMap<ProtocolValue<'a>, ProtocolValue<'a>>) -> PacketReadResult<GameProperties<'a>> {
        let ret = Ok(GameProperties {
            spectate_for_mods_only: get_u8_bool(&mut table, ProtocolValue::String("spectateForModsOnly"))?,
            max_ping: get_u8_short(&mut table, ProtocolValue::String("maxPing"))?,
            banned_weapon_message: get_u8_string(&mut table, ProtocolValue::String("bannedweaponmessage"))?,
            time_scale: get_u8_float(&mut table, ProtocolValue::String("timeScale"))?,
            match_countdown_time: get_u8_float(&mut table, ProtocolValue::String("matchCountdownTime"))?,
            round_started: get_u8_bool(&mut table, ProtocolValue::String("roundStarted"))?,
            score_limit: get_u8_int(&mut table, ProtocolValue::String("scorelimit"))?,
            gun_game_preset: get_u8_int(&mut table, ProtocolValue::String("gunGamePreset"))?,
            byte_249: get_u8_bool(&mut table, ProtocolValue::Byte(249))?,
            byte_250: get_u8_array(&mut table, ProtocolValue::Byte(250))?
                .into_iter()
                .map(|protocol_val| unwrap_protocol_string(protocol_val).expect("Found non-string type in GameProperties::byte_250"))
                .collect(),
            byte_253: get_u8_bool(&mut table, ProtocolValue::Byte(253))?,
            byte_254: get_u8_bool(&mut table, ProtocolValue::Byte(254))?,
            byte_255: get_u8_byte(&mut table, ProtocolValue::Byte(255))?,
            byte_248: get_u8_int(&mut table, ProtocolValue::Byte(248)).ok(), // could use direction to conditionally check for this
            room_name: get_u8_string(&mut table, ProtocolValue::String("roomName"))?,
            map_name: get_u8_string(&mut table, ProtocolValue::String("mapName"))?,
            mode_name: get_u8_string(&mut table, ProtocolValue::String("modeName"))?,
            password: get_u8_string(&mut table, ProtocolValue::String("password"))?,
            hardcore: get_u8_bool(&mut table, ProtocolValue::String("hardcore"))?,
            dedicated: get_u8_bool(&mut table, ProtocolValue::String("dedicated"))?,
            match_started: get_u8_bool(&mut table, ProtocolValue::String("matchStarted"))?,
            mean_kd: get_u8_float(&mut table, ProtocolValue::String("meanKD"))?,
            mean_rank: get_u8_int(&mut table, ProtocolValue::String("meanRank"))?,
            room_type: get_u8_byte(&mut table, ProtocolValue::String("roomType"))?,
            switching_map: get_u8_bool(&mut table, ProtocolValue::String("switchingmap"))?,
            allowed_weapons: {
                // this is an array of 2 u32s, but we save this as a u64 because it makes more sense
                let mut arr = get_u8_array(&mut table, ProtocolValue::String("allowedweapons"))?;
                if arr.len() != 2 {
                    return Err(PacketReadError::Other(format!("allowedweapons array was not 2 long, but {}", arr.len())));
                }
                let int2 = unwrap_protocol_int(arr.remove(1))? as u64;
                let int1 = unwrap_protocol_int(arr.remove(0))? as u64;
                int1 | (int2 << 32)
            },
            event_code: get_u8_int(&mut table, ProtocolValue::String("eventcode"))?,
            average_rank: get_u8_int(&mut table, ProtocolValue::String("averagerank"))?,
            game_id: get_u8_string(&mut table, ProtocolValue::String("gameID"))?,
            room_id: get_u8_string(&mut table, ProtocolValue::String("roomID"))?,
            store_id: get_u8_string(&mut table, ProtocolValue::String("storeID"))?,
        });

        if ret.is_ok() && table.len() > 0 {
            warn!("Missed GameProperties parameters: {:#?}, obj is {:#?}", table, ret);
        }

        ret
    }

    pub fn into_hashtable(self) -> HashMap<ProtocolValue<'s>, ProtocolValue<'s>> {
        let mut map = HashMap::new();
        map.insert(
            ProtocolValue::String("spectateForModsOnly"),
            ProtocolValue::Bool(self.spectate_for_mods_only),
        );
        map.insert(ProtocolValue::String("maxPing"), ProtocolValue::Short(self.max_ping));
        map.insert(
            ProtocolValue::String("bannedweaponmessage"),
            ProtocolValue::String(self.banned_weapon_message),
        );
        map.insert(ProtocolValue::String("timeScale"), ProtocolValue::Float(self.time_scale));
        map.insert(
            ProtocolValue::String("matchCountdownTime"),
            ProtocolValue::Float(self.match_countdown_time),
        );
        map.insert(ProtocolValue::String("roundStarted"), ProtocolValue::Bool(self.round_started));
        map.insert(ProtocolValue::String("scorelimit"), ProtocolValue::Integer(self.score_limit));
        map.insert(ProtocolValue::String("gunGamePreset"), ProtocolValue::Integer(self.gun_game_preset));
        map.insert(ProtocolValue::Byte(249), ProtocolValue::Bool(self.byte_249));
        map.insert(
            ProtocolValue::Byte(250),
            ProtocolValue::Array(self.byte_250.into_iter().map(|s| ProtocolValue::String(s)).collect()),
        );
        map.insert(ProtocolValue::Byte(253), ProtocolValue::Bool(self.byte_253));
        map.insert(ProtocolValue::Byte(254), ProtocolValue::Bool(self.byte_254));
        map.insert(ProtocolValue::Byte(255), ProtocolValue::Byte(self.byte_255));
        if let Some(b) = self.byte_248 {
            map.insert(ProtocolValue::Byte(248), ProtocolValue::Integer(b));
        }
        map.insert(ProtocolValue::String("roomName"), ProtocolValue::String(self.room_name));
        map.insert(ProtocolValue::String("mapName"), ProtocolValue::String(self.map_name));
        map.insert(ProtocolValue::String("modeName"), ProtocolValue::String(self.mode_name));
        map.insert(ProtocolValue::String("password"), ProtocolValue::String(self.password));
        map.insert(ProtocolValue::String("hardcore"), ProtocolValue::Bool(self.hardcore));
        map.insert(ProtocolValue::String("dedicated"), ProtocolValue::Bool(self.dedicated));
        map.insert(ProtocolValue::String("matchStarted"), ProtocolValue::Bool(self.match_started));
        map.insert(ProtocolValue::String("meanKD"), ProtocolValue::Float(self.mean_kd));
        map.insert(ProtocolValue::String("meanRank"), ProtocolValue::Integer(self.mean_rank));
        map.insert(ProtocolValue::String("roomType"), ProtocolValue::Byte(self.room_type));
        map.insert(ProtocolValue::String("switchingmap"), ProtocolValue::Bool(self.switching_map));
        map.insert(
            ProtocolValue::String("allowedweapons"),
            ProtocolValue::Array(vec![
                ProtocolValue::Integer((self.allowed_weapons & 0xFFFFFFFF) as u32),
                ProtocolValue::Integer((self.allowed_weapons >> 32) as u32),
            ]),
        );
        map.insert(ProtocolValue::String("eventcode"), ProtocolValue::Integer(self.event_code));
        map.insert(ProtocolValue::String("averagerank"), ProtocolValue::Integer(self.average_rank));
        map.insert(ProtocolValue::String("gameID"), ProtocolValue::String(self.game_id));
        map.insert(ProtocolValue::String("roomID"), ProtocolValue::String(self.room_id));
        map.insert(ProtocolValue::String("storeID"), ProtocolValue::String(self.store_id));
        map
    }
}

impl<'s> PlayerProperties<'s> {
    pub fn new_from_hashtable<'a>(mut table: HashMap<ProtocolValue<'a>, ProtocolValue<'a>>) -> PacketReadResult<PlayerProperties<'a>> {
        if table.len() != 1 || !table.contains_key(&ProtocolValue::Byte(255)) {
            return Err(PacketReadError::Other("Full PlayerProperties not yet implemented!".to_string()));
        }
        let ret = Ok(PlayerProperties::NameOnly(get_u8_string(&mut table, ProtocolValue::Byte(255))?));

        // can't be hit, actually
        if ret.is_ok() && table.len() > 0 {
            warn!("Missed PlayerProperties parameters: {:#?}, obj is {:#?}", table, ret);
        }

        ret
    }

    pub fn into_hashtable(self) -> HashMap<ProtocolValue<'s>, ProtocolValue<'s>> {
        let mut map = HashMap::new();
        match self {
            PlayerProperties::NameOnly(name) => map.insert(ProtocolValue::Byte(255), ProtocolValue::String(name)),
        };
        map
    }
}

macro_rules! gen_protocol_type_functions {
    ($unwrap_fn_name:ident, $get_prot_fn_name:ident, $get_u8_fn_name:ident, $type:ty, $protocol_type:path) => {
        fn $unwrap_fn_name<'a>(protocol_type: ProtocolValue<'a>) -> PacketReadResult<$type> {
            match protocol_type {
                $protocol_type(i) => Ok(i),
                _ => Err(PacketReadError::UnexpectedProtocolValue),
            }
        }

        #[allow(dead_code)]
        fn $get_prot_fn_name<'a>(map: &mut HashMap<ProtocolValue<'a>, ProtocolValue<'a>>, key: ProtocolValue<'static>) -> PacketReadResult<$type> {
            match map.remove(&key) {
                Some(val) => Ok($unwrap_fn_name(val)?),
                None => {
                    debug!("Couldn't find key {:?} in {:?}", key, map);
                    Err(PacketReadError::CouldNotFindKeyProtocolValue(key))
                }
            }
        }

        #[allow(dead_code)]
        fn $get_u8_fn_name<'a>(map: &mut HashMap<u8, ProtocolValue<'a>>, param_code: u8) -> PacketReadResult<$type> {
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
