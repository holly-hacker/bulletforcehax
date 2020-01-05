use super::macros::*;
use super::*;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use log::{debug, warn};
use maplit::hashmap;
use read_write::{read_parameter_table, read_string, write_parameter_table, write_value_of_type};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{Cursor, Write};

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
            255 => Ok(Event::Join {
                actor_list: get_protocol_array(&mut params, ParameterCode::ActorList).map(|protocol_array| {
                    protocol_array
                        .into_iter()
                        .map(|protocol_value| unwrap_protocol_int(protocol_value).expect("CreateGame response 2 had a non-int actor id"))
                        .collect()
                })?,
                actor_nr: get_protocol_int(&mut params, ParameterCode::ActorNr)?,
                player_properties: PlayerProperties::try_from(get_protocol_hashtable(&mut params, ParameterCode::PlayerProperties)?)?,
            }),
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
            Event::Join { .. } => 255,
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
                                Some(info) => info.into(),
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
                    .map(|info| (ProtocolValue::String(info.room_name), ProtocolValue::Hashtable(info.into())))
                    .collect())
            }),
            Event::CacheSliceChanged => err(Event::CacheSliceChanged),
            Event::ErrorInfo => err(Event::ErrorInfo),
            Event::SetProperties => err(Event::SetProperties),
            Event::PropertiesChanged => err(Event::PropertiesChanged),
            Event::Leave => err(Event::Leave),
            Event::Join {
                actor_list,
                actor_nr,
                player_properties,
            } => Ok(hashmap! {
                ParameterCode::ActorList => ProtocolValue::Array(actor_list.into_iter().map(|id| ProtocolValue::Integer(id)).collect()),
                ParameterCode::ActorNr => ProtocolValue::Integer(actor_nr),
                ParameterCode::PlayerProperties => ProtocolValue::Hashtable(player_properties.into()),
            }),
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
                    game_properties: GameProperties::try_from(get_protocol_hashtable(&mut params, ParameterCode::GameProperties)?)?,
                    player_properties: PlayerProperties::try_from(get_protocol_hashtable(&mut params, ParameterCode::PlayerProperties)?)?,
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
                    game_properties: GameProperties::try_from(get_protocol_hashtable(&mut params, ParameterCode::GameProperties)?)?,
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
            252 if params.len() == 0 => Ok(Operation::SetPropertiesEmpty()),
            252 if params.contains_key(&ParameterCode::ActorNr) => Ok(Operation::SetPropertiesActor {
                broadcast: get_protocol_bool(&mut params, ParameterCode::Broadcast)?,
                actor_nr: get_protocol_int(&mut params, ParameterCode::ActorNr)?,
                properties: get_protocol_hashtable(&mut params, ParameterCode::Properties)?,
            }),
            252 if params // TODO: super ugly!
                .get(&ParameterCode::Properties)
                .and_then(|p| match p {
                    ProtocolValue::Hashtable(t) => Some(t),
                    _ => None,
                })
                .and_then(|p| p.get(&ProtocolValue::String("roomName")))
                .is_some() =>
            {
                Ok(Operation::SetPropertiesGame {
                    broadcast: get_protocol_bool(&mut params, ParameterCode::Broadcast)?,
                    properties: GameProperties::try_from(get_protocol_hashtable(&mut params, ParameterCode::Properties)?)?,
                })
            }
            252 => Ok(Operation::SetPropertiesUnknown {
                broadcast: get_protocol_bool(&mut params, ParameterCode::Broadcast)?,
                properties: get_protocol_hashtable(&mut params, ParameterCode::Properties)?,
            }),
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
            Operation::SetPropertiesGame { .. } => 252,
            Operation::SetPropertiesActor { .. } => 252,
            Operation::SetPropertiesEmpty() => 252,
            Operation::SetPropertiesUnknown { .. } => 252,
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
                ParameterCode::GameProperties => ProtocolValue::Hashtable(game_properties.into()),
                ParameterCode::PlayerProperties => ProtocolValue::Hashtable(player_properties.into()),
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
                ParameterCode::GameProperties => ProtocolValue::Hashtable(game_properties.into()),
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
            Operation::SetPropertiesGame { broadcast, properties } => Ok(hashmap! {
                ParameterCode::Broadcast => ProtocolValue::Bool(broadcast),
                ParameterCode::Properties => ProtocolValue::Hashtable(GameProperties::into(properties)),
            }),
            Operation::SetPropertiesActor {
                broadcast,
                properties,
                actor_nr,
            } => Ok(hashmap! {
                ParameterCode::Broadcast => ProtocolValue::Bool(broadcast),
                ParameterCode::ActorNr => ProtocolValue::Integer(actor_nr),
                ParameterCode::Properties => ProtocolValue::Hashtable(properties),
            }),
            Operation::SetPropertiesEmpty() => Ok(hashmap!()),
            Operation::SetPropertiesUnknown { broadcast, properties } => Ok(hashmap! {
                ParameterCode::Broadcast => ProtocolValue::Bool(broadcast),
                ParameterCode::Properties => ProtocolValue::Hashtable(properties),
            }),
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
