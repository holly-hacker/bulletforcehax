use super::macros::*;
use super::*;
use log::{debug, warn};
use maplit::hashmap;
use num_traits::cast::FromPrimitive;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

type ParameterTable<'a> = HashMap<u8, ProtocolValue<'a>>;

impl Packet<'_> {
    pub fn read<'a>(data: &'a [u8], direction: Direction) -> PacketReadResult<Packet<'a>> {
        let photon_packet = PhotonPacket::try_from(data)?;

        match photon_packet {
            PhotonPacket::OperationRequest(packet_type, params) => Ok(Packet::OperationRequest(Operation::read(packet_type, params, direction)?)),
            PhotonPacket::OperationResponse(packet_type, mut params, return_code, debug_string) => Ok(Packet::OperationResponse {
                return_code,
                debug_string,
                secret: params.remove(&ParameterCode::Secret).map(unwrap_protocol_string).transpose()?,
                parameters: Operation::read(packet_type, params, direction)?,
            }),
            PhotonPacket::Event(packet_type, mut params) => Ok(Packet::Event {
                sender: params.remove(&ParameterCode::ActorNr).map(unwrap_protocol_int).transpose()?,
                custom_data: params.remove(&ParameterCode::Data),
                parameters: Event::read(packet_type, params)?,
            }),
            PhotonPacket::InternalOperationRequest(packet_type, params) => {
                Ok(Packet::InternalOperationRequest(InternalOperation::read(packet_type, params, direction)?))
            }
            PhotonPacket::InternalOperationResponse(packet_type, params, return_code, debug_string) => Ok(Packet::InternalOperationResponse(
                InternalOperation::read(packet_type, params, direction)?,
                return_code,
                debug_string,
            )),
        }
    }

    pub fn into_vec(self) -> PacketWriteResult<Vec<u8>> {
        let photon_packet: PhotonPacket = match self {
            Packet::OperationRequest(operation) => PhotonPacket::OperationRequest(operation.get_type(), operation.get_param_map()?),
            Packet::OperationResponse {
                parameters,
                return_code,
                debug_string,
                secret,
            } => {
                let packet_type = parameters.get_type();
                let mut map = parameters.get_param_map()?;
                secret.and_then(|s| map.insert(ParameterCode::Secret, ProtocolValue::String(s)));
                PhotonPacket::OperationResponse(packet_type, map, return_code, debug_string)
            }
            Packet::Event {
                sender,
                custom_data,
                parameters,
            } => {
                let packet_type = parameters.get_type();
                let mut param_map = parameters.get_param_map()?;
                sender.and_then(|s| param_map.insert(ParameterCode::ActorNr, ProtocolValue::Integer(s)));
                custom_data.and_then(|d| param_map.insert(ParameterCode::Data, d));
                PhotonPacket::Event(packet_type, param_map)
            }
            Packet::InternalOperationRequest(operation) => PhotonPacket::InternalOperationRequest(operation.get_type(), operation.get_param_map()?),
            Packet::InternalOperationResponse(operation, return_code, debug_string) => {
                (PhotonPacket::InternalOperationResponse(operation.get_type(), operation.get_param_map()?, return_code, debug_string))
            }
        };

        Ok(photon_packet.try_into()?)
    }
}

impl<'s> Event<'s> {
    pub fn read<'a>(event_type: u8, mut params: ParameterTable<'a>) -> PacketReadResult<Event<'a>> {
        fn err<'a>(event: Event<'static>, params: &HashMap<u8, ProtocolValue>) -> PacketReadResult<Event<'a>> {
            debug!("Unimplemented Event: {:?}, {:#?}", event, params);
            Err(PacketReadError::UnimplementedEventType(event))
        }

        let ret = match event_type {
            210 => err(Event::AzureNodeInfo, &params),
            223 => err(Event::AuthEvent, &params),
            224 => err(Event::LobbyStats, &params),
            226 => Ok(Event::AppStats {
                game_count: get_u8_int(&mut params, ParameterCode::GameCount)?,
                peer_count: get_u8_int(&mut params, ParameterCode::PeerCount)?,
                master_peer_count: get_u8_int(&mut params, ParameterCode::MasterPeerCount)?,
            }),
            227 => err(Event::Match, &params),
            228 => err(Event::QueueState, &params),
            229 => Ok(Event::GameListUpdate(RoomInfo::try_from_hashtable_table(get_u8_hashtable(
                &mut params,
                ParameterCode::GameList,
            )?)?)),
            230 => Ok(Event::GameList(
                RoomInfo::try_from_hashtable_table(get_u8_hashtable(&mut params, ParameterCode::GameList)?).map(|mut table| {
                    table
                        .drain()
                        .map(|(key, value)| (key, value.expect("GameList packet contained removed game")))
                        .collect()
                })?,
            )),
            250 => err(Event::CacheSliceChanged, &params),
            251 => err(Event::ErrorInfo, &params),
            253 => err(Event::PropertiesChanged, &params),
            254 => err(Event::Leave, &params),
            255 => Ok(Event::Join {
                actor_list: get_u8_array_or_none(&mut params, ParameterCode::ActorList, unwrap_protocol_int)?,
                player_properties: Player::try_from(get_u8_hashtable(&mut params, ParameterCode::PlayerProperties)?)?,
            }),
            _ => Err(PacketReadError::UnknownEventType(event_type)),
        };

        if ret.is_ok() && !params.is_empty() {
            warn!("Missed event parameters: {:#?}, obj is {:#?}", params, ret);
        }

        ret
    }

    pub fn get_param_map(self) -> PacketWriteResult<HashMap<u8, ProtocolValue<'s>>> {
        fn err(event: Event<'static>) -> PacketWriteResult<HashMap<u8, ProtocolValue>> {
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
                    .map(|(k, v)| (ProtocolValue::String(k), ProtocolValue::Hashtable(match v {
                        Some(m) => m.into(),
                        None => hashmap! (ProtocolValue::Byte(GamePropertyKey::Removed) => ProtocolValue::Bool(true)),
                    })))
                    .collect())
            }),
            Event::GameList(info) => Ok(hashmap! {
                ParameterCode::GameList => ProtocolValue::Hashtable(info
                    .into_iter()
                    .map(|(k, v)| (ProtocolValue::String(k), ProtocolValue::Hashtable(v.into())))
                    .collect())
            }),
            Event::CacheSliceChanged => err(Event::CacheSliceChanged),
            Event::ErrorInfo => err(Event::ErrorInfo),
            Event::PropertiesChanged => err(Event::PropertiesChanged),
            Event::Leave => err(Event::Leave),
            Event::Join {
                actor_list,
                player_properties,
            } => {
                let mut map = hashmap! { ParameterCode::PlayerProperties => ProtocolValue::Hashtable(player_properties.into()) };
                actor_list.and_then(|list| {
                    map.insert(
                        ParameterCode::ActorList,
                        ProtocolValue::Array(list.into_iter().map(ProtocolValue::Integer).collect()),
                    )
                });
                Ok(map)
            }
        }
    }
}

impl<'s> Operation<'s> {
    pub fn read<'a>(operation_type: u8, mut params: ParameterTable<'a>, direction: Direction) -> PacketReadResult<Operation<'a>> {
        fn err<'a>(operation: Operation<'static>, params: &HashMap<u8, ProtocolValue>) -> PacketReadResult<Operation<'a>> {
            debug!("Unimplemented Operation: {:?}, {:#?}", operation, params);
            Err(PacketReadError::UnimplementedOperationType(operation))
        }

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
                Direction::Send if !params.contains_key(&ParameterCode::GameProperties) => Ok(Operation::CreateGameRequestMaster {
                    room_name: get_u8_string_opt(&mut params, ParameterCode::RoomName)?,
                    lobby_name: get_u8_string_opt(&mut params, ParameterCode::LobbyName)?,
                    lobby_type: get_u8_bool_opt(&mut params, ParameterCode::LobbyType)?,
                    expected_users: get_u8_array_opt(&mut params, ParameterCode::Add)?
                        .map_or(Ok(Vec::new()), |arr| arr.into_iter().map(unwrap_protocol_string).collect())?,
                }),
                Direction::Send => Ok(Operation::CreateGameRequestGame {
                    room_name: get_u8_string_opt(&mut params, ParameterCode::RoomName)?,
                    lobby_name: get_u8_string_opt(&mut params, ParameterCode::LobbyName)?,
                    lobby_type: get_u8_bool_opt(&mut params, ParameterCode::LobbyType)?,
                    expected_users: get_u8_array_opt(&mut params, ParameterCode::Add)?
                        .map_or(Ok(Vec::new()), |arr| arr.into_iter().map(unwrap_protocol_string).collect())?,
                    // more
                    player_properties: {
                        let map = get_u8_hashtable_opt(&mut params, ParameterCode::PlayerProperties)?;
                        if let Some(inner_map) = map {
                            if !inner_map.is_empty() {
                                Some(Player::try_from(inner_map)?)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    },
                    // present+true if playerproperties is present, so could derive this
                    broadcast: get_u8_bool_opt(&mut params, ParameterCode::Broadcast)?,

                    game_properties: RoomOptions::try_from(get_u8_hashtable(&mut params, ParameterCode::GameProperties)?)?,
                    player_ttl: get_u8_int_opt(&mut params, ParameterCode::PlayerTTL)?.unwrap_or(0), // could also be any neg number under -1
                    empty_room_ttl: get_u8_int_opt(&mut params, ParameterCode::EmptyRoomTTL)?.unwrap_or(0), // could also be any neg number
                    plugins: get_u8_array_opt(&mut params, ParameterCode::Plugins)?
                        .map(|arr| arr.into_iter().map(unwrap_protocol_string).collect())
                        .transpose()?,
                    room_option_flags: RoomOptionsFlags::from_bits(get_u8_int(&mut params, ParameterCode::RoomOptionFlags)? as u32)
                        .expect("Received invalid RoomOptionsFlags"), // possibly not present if value is 0
                }),
                Direction::Recv if !params.contains_key(&ParameterCode::GameProperties) => Ok(Operation::CreateGameResponseMaster {
                    room_name: get_u8_string_opt(&mut params, ParameterCode::RoomName)?,
                    address: get_u8_string(&mut params, ParameterCode::Address)?,
                }),
                Direction::Recv => Ok(Operation::CreateGameResponseGame {
                    actor_nr: get_u8_int(&mut params, ParameterCode::ActorNr)?,
                    actor_list: get_u8_array_or_none(&mut params, ParameterCode::ActorList, unwrap_protocol_int)?,
                    game_properties: RoomInfo::try_from(get_u8_hashtable(&mut params, ParameterCode::GameProperties)?)?,
                    player_properties: {
                        get_u8_hashtable(&mut params, ParameterCode::PlayerProperties)?
                            .into_iter()
                            .map(|(k, v)| {
                                let actor_nr = unwrap_protocol_int(k).unwrap();
                                let props = unwrap_protocol_hashtable(v).unwrap();
                                let actor = Player::try_from(props).unwrap();
                                (actor_nr, actor)
                            })
                            .collect()
                    },
                }),
            },
            228 => err(Operation::LeaveLobby, &params),
            229 => Ok(Operation::JoinLobby()),
            230 => match direction {
                Direction::Send if params.contains_key(&ParameterCode::Secret) => Ok(Operation::AuthenticateRequestToken {
                    lobby_stats: get_u8_bool_opt(&mut params, ParameterCode::LobbyStats)?.unwrap_or(false),
                    secret: get_u8_string(&mut params, ParameterCode::Secret)?,
                }),
                Direction::Send => Ok(Operation::AuthenticateRequestNoToken {
                    lobby_stats: get_u8_bool_opt(&mut params, ParameterCode::LobbyStats)?.unwrap_or(false),
                    app_version: get_u8_string(&mut params, ParameterCode::AppVersion)?,
                    app_id: get_u8_string(&mut params, ParameterCode::ApplicationId)?,
                    region: get_u8_string_opt(&mut params, ParameterCode::Region)?,
                    user_id: get_u8_string_opt(&mut params, ParameterCode::UserId)?,
                    client_auth_type: get_u8_byte_opt(&mut params, ParameterCode::ClientAuthenticationType)?,
                    client_auth_params: get_u8_string_opt(&mut params, ParameterCode::ClientAuthenticationParams)?,
                    client_auth_data: get_u8_string_opt(&mut params, ParameterCode::ClientAuthenticationData)?,
                }),
                Direction::Recv if params.contains_key(&ParameterCode::Address) => Ok(Operation::AuthenticateResponseName {
                    user_id: get_u8_string_opt(&mut params, ParameterCode::UserId)?,
                    nickname: get_u8_string_opt(&mut params, ParameterCode::NickName)?,
                    encryption_data: get_u8_hashtable_opt(&mut params, ParameterCode::EncryptionData)?.map(|map| {
                        map.into_iter()
                            .map(|(k, v)| (unwrap_protocol_byte(k).expect("expected u8 keys in encryption_data"), v))
                            .collect()
                    }),
                    custom_data: get_u8_hashtable_opt(&mut params, ParameterCode::CustomInitData)?.map(|map| {
                        map.into_iter()
                            .map(|(k, v)| (unwrap_protocol_string(k).expect("expected string keys in custom_data"), v))
                            .collect()
                    }),

                    cluster: get_u8_string_opt(&mut params, ParameterCode::Cluster)?,
                    address: get_u8_string(&mut params, ParameterCode::Address)?,
                }),
                Direction::Recv => Ok(Operation::AuthenticateResponseMasterOrGame {
                    user_id: get_u8_string_opt(&mut params, ParameterCode::UserId)?,
                    nickname: get_u8_string_opt(&mut params, ParameterCode::NickName)?,
                    encryption_data: get_u8_hashtable_opt(&mut params, ParameterCode::EncryptionData)?.map(|map| {
                        map.into_iter()
                            .map(|(k, v)| (unwrap_protocol_byte(k).expect("expected u8 keys in encryption_data"), v))
                            .collect()
                    }),
                    custom_data: get_u8_hashtable_opt(&mut params, ParameterCode::CustomInitData)?.map(|map| {
                        map.into_iter()
                            .map(|(k, v)| (unwrap_protocol_string(k).expect("expected string keys in custom_data"), v))
                            .collect()
                    }),
                    position: get_u8_int_opt(&mut params, ParameterCode::Position)?,
                }),
            },
            231 => err(Operation::AuthenticateOnce, &params),
            248 => err(Operation::ChangeGroups, &params),
            250 => err(Operation::ExchangeKeysForEncryption, &params),
            251 => err(Operation::GetProperties, &params),
            252 if params.contains_key(&ParameterCode::ActorNr) => Ok(Operation::SetPropertiesActor {
                broadcast: get_u8_bool(&mut params, ParameterCode::Broadcast)?,
                event_forward: get_u8_bool_opt(&mut params, ParameterCode::EventForward)?.unwrap_or(false),
                actor_nr: get_u8_int(&mut params, ParameterCode::ActorNr)?,
                properties: get_u8_hashtable(&mut params, ParameterCode::Properties)?,
                expected_properties: get_u8_hashtable_opt(&mut params, ParameterCode::ExpectedValues)?,
            }),
            252 => Ok(Operation::SetPropertiesGame {
                broadcast: get_u8_bool(&mut params, ParameterCode::Broadcast)?,
                event_forward: get_u8_bool_opt(&mut params, ParameterCode::EventForward)?.unwrap_or(false),
                properties: get_u8_hashtable(&mut params, ParameterCode::Properties)?,
                expected_properties: get_u8_hashtable_opt(&mut params, ParameterCode::ExpectedValues)?,
            }),
            253 => Ok(Operation::RaiseEvent {
                cache: EventCaching::from_u8(get_u8_byte_opt(&mut params, ParameterCode::Cache)?.unwrap_or(0)).expect("Invalid EventCaching"),
                actor_list: get_u8_array_or_none(&mut params, ParameterCode::ActorList, unwrap_protocol_int)?,
                group: get_u8_byte_opt(&mut params, ParameterCode::Group)?,
                receivers: get_u8_byte_opt(&mut params, ParameterCode::ReceiverGroup)?
                    .map(|byte| ReceiverGroup::from_u8(byte).expect("Invalid ReceiverGroup")),
                event_forward: get_u8_bool_opt(&mut params, ParameterCode::EventForward)?,
                code: get_u8_byte_opt(&mut params, ParameterCode::Code)?,
                data: params.remove(&ParameterCode::Data),
            }),
            254 => err(Operation::Leave, &params),
            255 => err(Operation::Join, &params),
            _ => Err(PacketReadError::UnknownOperationType(operation_type)),
        };

        if ret.is_ok() && !params.is_empty() {
            warn!("Missed operation parameters: {:#?}, obj is {:#?}", params, ret);
        }

        ret
    }

    pub fn get_param_map(self) -> PacketWriteResult<HashMap<u8, ProtocolValue<'s>>> {
        fn err(operation: Operation<'static>) -> PacketWriteResult<HashMap<u8, ProtocolValue>> {
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
            Operation::CreateGameRequestMaster {
                room_name,
                lobby_name,
                lobby_type,
                expected_users,
            } => Ok({
                let mut map = hashmap!();

                room_name
                    .filter(|n| !n.is_empty())
                    .and_then(|n| map.insert(ParameterCode::RoomName, ProtocolValue::String(n)));
                lobby_name.and_then(|n| map.insert(ParameterCode::LobbyName, ProtocolValue::String(n)));
                lobby_type.and_then(|n| map.insert(ParameterCode::LobbyType, ProtocolValue::Bool(n)));
                if !expected_users.is_empty() {
                    map.insert(
                        ParameterCode::Add,
                        ProtocolValue::Array(expected_users.into_iter().map(ProtocolValue::String).collect()),
                    );
                }

                map
            }),
            Operation::CreateGameRequestGame {
                room_name,
                lobby_name,
                lobby_type,
                expected_users,

                player_properties,
                broadcast,
                game_properties,
                player_ttl,
                empty_room_ttl,
                plugins,
                room_option_flags,
            } => Ok({
                let mut map = hashmap!();

                room_name
                    .filter(|n| !n.is_empty())
                    .and_then(|n| map.insert(ParameterCode::RoomName, ProtocolValue::String(n)));
                lobby_name.and_then(|n| map.insert(ParameterCode::LobbyName, ProtocolValue::String(n)));
                lobby_type.and_then(|n| map.insert(ParameterCode::LobbyType, ProtocolValue::Bool(n)));
                if !expected_users.is_empty() {
                    map.insert(
                        ParameterCode::Add,
                        ProtocolValue::Array(expected_users.into_iter().map(ProtocolValue::String).collect()),
                    );
                }

                if let Some(pp) = player_properties {
                    let pp_map: HashMap<ProtocolValue, ProtocolValue> = pp.into();
                    if !pp_map.is_empty() {
                        map.insert(ParameterCode::PlayerProperties, ProtocolValue::Hashtable(pp_map));
                    }
                }
                broadcast
                    .filter(|b| *b)
                    .and_then(|_| map.insert(ParameterCode::Broadcast, ProtocolValue::Bool(true)));
                if broadcast == Some(true) {
                    map.insert(ParameterCode::Broadcast, ProtocolValue::Bool(true));
                }

                map.insert(ParameterCode::GameProperties, ProtocolValue::Hashtable(game_properties.into()));
                if player_ttl > 0 || player_ttl == -1 {
                    map.insert(ParameterCode::PlayerTTL, ProtocolValue::Integer(player_ttl));
                }
                if empty_room_ttl > 0 {
                    map.insert(ParameterCode::EmptyRoomTTL, ProtocolValue::Integer(empty_room_ttl));
                }
                plugins.and_then(|p| {
                    map.insert(
                        ParameterCode::Plugins,
                        ProtocolValue::Array(p.into_iter().map(ProtocolValue::String).collect()),
                    )
                });

                if room_option_flags.contains(RoomOptionsFlags::SUPPRESS_ROOM_EVENTS) {
                    map.insert(ParameterCode::SuppressRoomEvents, ProtocolValue::Bool(true));
                }
                map.insert(
                    ParameterCode::CleanupCacheOnLeave,
                    ProtocolValue::Bool(room_option_flags.contains(RoomOptionsFlags::DELETE_CACHE_ON_LEAVE)),
                );
                map.insert(
                    ParameterCode::CheckUserOnJoin,
                    ProtocolValue::Bool(room_option_flags.contains(RoomOptionsFlags::CHECK_USER_ON_JOIN)),
                );
                if room_option_flags.contains(RoomOptionsFlags::PUBLISH_USER_ID) {
                    map.insert(ParameterCode::PublishUserId, ProtocolValue::Bool(true));
                }
                map.insert(ParameterCode::RoomOptionFlags, ProtocolValue::Integer(room_option_flags.bits() as i32)); // sketchy cast, not sure if this works in rust

                map
            }),
            Operation::CreateGameResponseMaster { room_name, address } => Ok({
                let mut map = hashmap! {
                    ParameterCode::Address => ProtocolValue::String(address),
                };

                room_name
                    .filter(|r| !r.is_empty())
                    .and_then(|r| map.insert(ParameterCode::RoomName, ProtocolValue::String(r)));

                map
            }),
            Operation::CreateGameResponseGame {
                actor_nr,
                actor_list,
                game_properties,
                player_properties,
            } => Ok({
                let mut map = hashmap! {
                    ParameterCode::ActorNr => ProtocolValue::Integer(actor_nr),
                    ParameterCode::GameProperties => ProtocolValue::Hashtable(game_properties.into()),
                    ParameterCode::PlayerProperties => ProtocolValue::Hashtable(player_properties.into_iter().map(|(k, v)| (ProtocolValue::Integer(k), ProtocolValue::Hashtable(v.into()))).collect()),
                };

                actor_list.and_then(|l| {
                    map.insert(
                        ParameterCode::ActorList,
                        ProtocolValue::Array(l.into_iter().map(ProtocolValue::Integer).collect()),
                    )
                });

                map
            }),
            Operation::LeaveLobby => err(Operation::LeaveLobby),
            Operation::JoinLobby() => Ok(hashmap!()),
            Operation::AuthenticateRequestNoToken {
                lobby_stats,
                app_version,
                app_id,
                region,
                user_id,
                client_auth_type,
                client_auth_params,
                client_auth_data,
            } => Ok({
                let mut map = hashmap! {
                    ParameterCode::AppVersion => ProtocolValue::String(app_version),
                    ParameterCode::ApplicationId => ProtocolValue::String(app_id),
                };

                if lobby_stats {
                    map.insert(ParameterCode::LobbyStats, ProtocolValue::Bool(lobby_stats));
                }

                region
                    .filter(|r| !r.is_empty())
                    .and_then(|r| map.insert(ParameterCode::Region, ProtocolValue::String(r)));
                user_id
                    .filter(|u| !u.is_empty())
                    .and_then(|u| map.insert(ParameterCode::UserId, ProtocolValue::String(u)));

                client_auth_type.filter(|t| t != &255).and_then(|t| {
                    map.insert(ParameterCode::ClientAuthenticationType, ProtocolValue::Byte(t));
                    client_auth_params
                        .filter(|s| !s.is_empty())
                        .and_then(|s| map.insert(ParameterCode::ClientAuthenticationParams, ProtocolValue::String(s)));
                    client_auth_data
                        .filter(|s| !s.is_empty())
                        .and_then(|s| map.insert(ParameterCode::ClientAuthenticationData, ProtocolValue::String(s)));
                    Some(())
                });

                map
            }),
            Operation::AuthenticateRequestToken {
                lobby_stats, // not present if false
                secret,
            } => Ok({
                let mut map = hashmap! {
                    ParameterCode::Secret => ProtocolValue::String(secret),
                };

                if lobby_stats {
                    map.insert(ParameterCode::LobbyStats, ProtocolValue::Bool(lobby_stats));
                }

                map
            }),
            Operation::AuthenticateResponseName {
                user_id,
                nickname,
                encryption_data,
                custom_data,
                cluster,
                address,
            } => Ok({
                let mut map = hashmap!(ParameterCode::Address => ProtocolValue::String(address));
                user_id.and_then(|d| map.insert(ParameterCode::UserId, ProtocolValue::String(d)));
                nickname.and_then(|d| map.insert(ParameterCode::NickName, ProtocolValue::String(d)));
                encryption_data.and_then(|d| {
                    map.insert(
                        ParameterCode::EncryptionData,
                        ProtocolValue::Hashtable(d.into_iter().map(|(k, v)| (ProtocolValue::Byte(k), v)).collect()),
                    )
                });
                custom_data.and_then(|d| {
                    map.insert(
                        ParameterCode::CustomInitData,
                        ProtocolValue::Hashtable(d.into_iter().map(|(k, v)| (ProtocolValue::String(k), v)).collect()),
                    )
                });
                cluster.and_then(|d| map.insert(ParameterCode::Cluster, ProtocolValue::String(d)));
                map
            }),
            Operation::AuthenticateResponseMasterOrGame {
                user_id,
                nickname,
                encryption_data,
                custom_data,
                position,
            } => Ok({
                let mut map = hashmap!();
                user_id.and_then(|d| map.insert(ParameterCode::UserId, ProtocolValue::String(d)));
                nickname.and_then(|d| map.insert(ParameterCode::NickName, ProtocolValue::String(d)));
                encryption_data.and_then(|d| {
                    map.insert(
                        ParameterCode::EncryptionData,
                        ProtocolValue::Hashtable(d.into_iter().map(|(k, v)| (ProtocolValue::Byte(k), v)).collect()),
                    )
                });
                custom_data.and_then(|d| {
                    map.insert(
                        ParameterCode::CustomInitData,
                        ProtocolValue::Hashtable(d.into_iter().map(|(k, v)| (ProtocolValue::String(k), v)).collect()),
                    )
                });
                position.and_then(|p| map.insert(ParameterCode::Position, ProtocolValue::Integer(p)));
                map
            }),
            Operation::AuthenticateOnce => err(Operation::AuthenticateOnce),
            Operation::ChangeGroups => err(Operation::ChangeGroups),
            Operation::ExchangeKeysForEncryption => err(Operation::ExchangeKeysForEncryption),
            Operation::GetProperties => err(Operation::GetProperties),
            Operation::SetPropertiesGame {
                broadcast,
                properties,
                expected_properties,
                event_forward,
            } => Ok({
                let mut map = hashmap! {
                    ParameterCode::Broadcast => ProtocolValue::Bool(broadcast),
                    ParameterCode::Properties => ProtocolValue::Hashtable(properties),
                };

                expected_properties.and_then(|p| map.insert(ParameterCode::ExpectedValues, ProtocolValue::Hashtable(p)));
                if event_forward {
                    map.insert(ParameterCode::EventForward, ProtocolValue::Bool(event_forward));
                }

                map
            }),
            Operation::SetPropertiesActor {
                broadcast,
                properties,
                actor_nr,
                expected_properties,
                event_forward,
            } => Ok({
                let mut map = hashmap! {
                    ParameterCode::Broadcast => ProtocolValue::Bool(broadcast),
                    ParameterCode::ActorNr => ProtocolValue::Integer(actor_nr),
                    ParameterCode::Properties => ProtocolValue::Hashtable(properties),
                };

                expected_properties.and_then(|p| map.insert(ParameterCode::ExpectedValues, ProtocolValue::Hashtable(p)));
                if event_forward {
                    map.insert(ParameterCode::EventForward, ProtocolValue::Bool(event_forward));
                }

                map
            }),
            Operation::RaiseEvent {
                cache,
                actor_list,
                group,
                receivers,
                event_forward,
                code,
                data,
            } => Ok({
                let mut map = hashmap!();

                if cache != EventCaching::DoNotCache {
                    map.insert(ParameterCode::Cache, ProtocolValue::Byte(cache as u8));
                };

                match cache {
                    EventCaching::SliceSetIndex
                    | EventCaching::SlicePurgeIndex
                    | EventCaching::SlicePurgeUpToIndex
                    | EventCaching::SliceIncreaseIndex
                    | EventCaching::RemoveFromRoomCacheForActorsLeft => {
                        // nothing
                    }
                    EventCaching::RemoveFromRoomCache => {
                        actor_list.and_then(|l| {
                            map.insert(
                                ParameterCode::ActorList,
                                ProtocolValue::Array(l.into_iter().map(ProtocolValue::Integer).collect()),
                            )
                        });
                    }
                    _ => {
                        actor_list.and_then(|l| {
                            map.insert(
                                ParameterCode::ActorList,
                                ProtocolValue::Array(l.into_iter().map(ProtocolValue::Integer).collect()),
                            )
                        });
                        group.and_then(|b| map.insert(ParameterCode::Group, ProtocolValue::Byte(b)));
                        receivers
                            .filter(|b| b != &ReceiverGroup::Others)
                            .and_then(|b| map.insert(ParameterCode::ReceiverGroup, ProtocolValue::Byte(b as u8)));
                        event_forward
                            .filter(|b| *b)
                            .and_then(|b| map.insert(ParameterCode::EventForward, ProtocolValue::Bool(b)));
                        code.and_then(|b| map.insert(ParameterCode::Code, ProtocolValue::Byte(b)));
                        data.and_then(|d| map.insert(ParameterCode::Data, d));
                    }
                }

                map
            }),
            Operation::Leave => err(Operation::Leave),
            Operation::Join => err(Operation::Join),
        }
    }
}

impl<'s> InternalOperation {
    pub fn read(operation_type: u8, mut params: ParameterTable<'_>, direction: Direction) -> PacketReadResult<InternalOperation> {
        fn err(operation: InternalOperation, params: &HashMap<u8, ProtocolValue>) -> PacketReadResult<InternalOperation> {
            debug!("Unimplemented InternalOperation: {:?}, {:#?}", operation, params);
            Err(PacketReadError::UnimplementedInternalOperationType(operation))
        }

        let ret = match operation_type {
            0 => err(InternalOperation::InitEncryption, &params),
            1 => match direction {
                Direction::Send => Ok(InternalOperation::PingRequest {
                    local_time: get_u8_int(&mut params, 1)?,
                }),
                Direction::Recv => Ok(InternalOperation::PingResponse {
                    local_time: get_u8_int(&mut params, 1)?,
                    server_time: get_u8_int(&mut params, 2)?,
                }),
            },
            _ => Err(PacketReadError::UnknownInternalOperationType(operation_type)),
        };

        if ret.is_ok() && !params.is_empty() {
            warn!("Missed operation parameters: {:#?}, obj is {:#?}", params, ret);
        }

        ret
    }

    pub fn get_param_map(self) -> PacketWriteResult<HashMap<u8, ProtocolValue<'s>>> {
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
