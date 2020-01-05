use std::collections::HashMap;
use std::hash::{Hash, Hasher};
mod errors;
pub use errors::*;
mod packets_impl;
mod packets_payload_tests;
mod read_write;

#[derive(Debug)]
pub enum Packet<'a> {
    Init,
    InitResponse,
    OperationRequest(Operation<'a>),
    OperationResponse(i16, Option<&'a str>, Operation<'a>),
    Event(Event<'a>),
    InternalOperationRequest(InternalOperation),
    InternalOperationResponse(i16, Option<&'a str>, InternalOperation),
    Message,
    RawMessage,
}

#[derive(Debug)]
pub enum Event<'a> {
    AzureNodeInfo,
    AuthEvent,
    LobbyStats,
    AppStats {
        game_count: u32,
        peer_count: u32,
        master_peer_count: u32,
    },
    Match,
    QueueState,
    GameListUpdate(HashMap<&'a str, Option<GameInfo<'a>>>),
    GameList(Vec<GameInfo<'a>>),
    CacheSliceChanged,
    ErrorInfo,
    PropertiesChanged,
    SetProperties,
    Leave,
    Join,
}

#[derive(Debug)]
pub enum Operation<'a> {
    GetGameList,
    ServerSettings,
    WebRpc,
    GetRegions,
    GetLobbyStats,
    FindFriends,
    CancelJoinRandom,
    JoinRandomGame,
    JoinGame,
    CreateGameRequest {
        room_name: &'a str,
    },
    CreateGameResponse {
        room_name: &'a str,
        address: &'a str,
        secret: &'a str,
    },
    CreateGameRequest2 {
        broadcast: bool,
        room_name: &'a str,
        game_properties: GameProperties<'a>,
        player_properties: PlayerProperties<'a>,
        room_option_flags: u32,
        cleanup_cache_on_leave: bool,
        check_user_on_join: bool,
    },
    CreateGameResponse2 {
        actor_list: Vec<u32>,
        actor_nr: u32,
        game_properties: GameProperties<'a>,
    },
    LeaveLobby,
    JoinLobby(),
    AuthenticateRequest {
        region: &'a str,
        application_id: &'a str, // could be parsed as u128
        app_version: &'a str,
    },
    AuthenticateRequest2 {
        secret: &'a str,
    },
    AuthenticateResponse {
        unknown: &'a str,
        secret: &'a str,
        address: &'a str,
        user_id: &'a str,
    },
    AuthenticateResponse2 {
        secret: &'a str,
        position: u32,
    },
    AuthenticateOnce,
    ChangeGroups,
    ExchangeKeysForEncryption,
    GetProperties,
    SetProperties,
    RaiseEvent,
    Leave,
    Join,
}

#[derive(Debug)]
pub enum InternalOperation {
    InitEncryption,
    PingRequest { local_time: u32 },
    PingResponse { local_time: u32, server_time: u32 },
}

#[derive(Debug, PartialEq)]
pub enum ProtocolValue<'a> {
    Null(),
    Bool(bool),
    Byte(u8),
    Short(u16),
    Integer(u32),
    Long(u64),
    Float(f32),
    Double(f64),
    String(&'a str),
    OperationRequest,
    OperationResponse,
    EventData,
    Array(Vec<ProtocolValue<'a>>), // An array of predetermined type, C# type is Array.
    ObjectArray(Vec<ProtocolValue<'a>>),
    ByteArray,
    StringArray,
    IntegerArray,
    Dictionary, // Map<Object, Object>, predefined types, C# type is IDictionary/Dictionary<T1, T2>, TODO
    Hashtable(HashMap<ProtocolValue<'a>, ProtocolValue<'a>>), // Map<Object, Object>, random types, C# type is Hashtable/Dictionary<object, object>
    Custom,
}

pub enum Direction {
    Send,
    Recv,
}

#[derive(Debug, PartialEq)]
pub struct GameInfo<'a> {
    game_id: &'a str,
    room_id: &'a str,
    store_id: &'a str,
    room_name: &'a str,
    mode_name: &'a str,
    password: &'a str,
    map_name: &'a str,
    match_started: bool,
    switching_map: bool,
    room_type: u8, // could become c-style enum
    dedicated: bool,
    hardcore: bool,
    allowed_weapons: u64,
    mean_rank: f32,
    mean_kd: f32,
    average_rank: u32,
    event_code: u32,
    // byte_251: bool, mentions whether this game got removed. either this or the other fields are present
    byte_252: u8,
    byte_253: bool,
    byte_255: u8,
}

#[derive(Debug, PartialEq)]
pub struct GameProperties<'a> {
    spectate_for_mods_only: bool,
    max_ping: u16,
    banned_weapon_message: &'a str,
    time_scale: f32,
    match_countdown_time: f32,
    round_started: bool,
    score_limit: u32,
    gun_game_preset: u32,
    byte_249: bool,
    /// List of some fields that are present in this struct, which don't seem to be present in GameInfo
    byte_250: Vec<&'a str>,
    byte_253: bool,
    byte_254: bool,
    byte_255: u8,
    /// Only present in response
    byte_248: Option<u32>,

    // fields contained in byte_250
    room_name: &'a str,
    map_name: &'a str,
    mode_name: &'a str,
    password: &'a str,
    hardcore: bool,
    dedicated: bool,
    match_started: bool,
    mean_kd: f32,
    mean_rank: u32,
    room_type: u8,
    switching_map: bool,
    allowed_weapons: u64,
    event_code: u32,
    average_rank: u32,
    game_id: &'a str,
    room_id: &'a str,
    store_id: &'a str,
}

#[derive(Debug, PartialEq)]
pub enum PlayerProperties<'a> {
    NameOnly(&'a str),
}

// This would be an enum, but Rust does not allow multiple enum members with the same value
#[allow(dead_code, non_upper_case_globals, non_snake_case)]
pub mod ParameterCode {
    pub const FindFriendsResponseOnlineList: u8 = 1;
    pub const FindFriendsRequestList: u8 = 1;
    pub const FindFriendsResponseRoomIdList: u8 = 2;
    pub const FindFriendsOptions: u8 = 2;
    pub const RoomOptionFlags: u8 = 191;
    pub const EncryptionData: u8 = 192;
    pub const EncryptionMode: u8 = 193;
    pub const CustomInitData: u8 = 194;
    pub const ExpectedProtocol: u8 = 195;
    pub const PluginVersion: u8 = 200;
    pub const PluginName: u8 = 201;
    pub const NickName: u8 = 202;
    pub const MasterClientId: u8 = 203;
    pub const Plugins: u8 = 204;
    pub const CacheSliceIndex: u8 = 205;
    pub const WebRpcReturnMessage: u8 = 206;
    pub const WebRpcReturnCode: u8 = 207;
    pub const AzureMasterNodeId: u8 = 208;
    pub const WebRpcParameters: u8 = 208;
    pub const AzureLocalNodeId: u8 = 209;
    pub const UriPath: u8 = 209;
    pub const AzureNodeInfo: u8 = 210;
    pub const Region: u8 = 210;
    pub const LobbyStats: u8 = 211;
    pub const LobbyType: u8 = 212;
    pub const LobbyName: u8 = 213;
    pub const ClientAuthenticationData: u8 = 214;
    pub const CreateIfNotExists: u8 = 215;
    pub const JoinMode: u8 = 215;
    pub const ClientAuthenticationParams: u8 = 216;
    pub const ClientAuthenticationType: u8 = 217;
    pub const Info: u8 = 218;
    pub const AppVersion: u8 = 220;
    pub const Secret: u8 = 221;
    pub const GameList: u8 = 222;
    pub const Position: u8 = 223;
    pub const MatchMakingType: u8 = 223;
    pub const ApplicationId: u8 = 224;
    pub const UserId: u8 = 225;
    pub const MasterPeerCount: u8 = 227;
    pub const GameCount: u8 = 228;
    pub const PeerCount: u8 = 229;
    pub const Address: u8 = 230;
    pub const ExpectedValues: u8 = 231;
    pub const CheckUserOnJoin: u8 = 232;
    pub const IsInactive: u8 = 233;
    pub const IsComingBack: u8 = 233;
    pub const EventForward: u8 = 234;
    pub const PlayerTTL: u8 = 235;
    pub const EmptyRoomTTL: u8 = 236;
    pub const SuppressRoomEvents: u8 = 237;
    pub const Add: u8 = 238;
    pub const Remove: u8 = 239;
    pub const PublishUserId: u8 = 239;
    pub const Group: u8 = 240;
    pub const CleanupCacheOnLeave: u8 = 241;
    pub const Code: u8 = 244;
    pub const CustomEventContent: u8 = 245;
    pub const Data: u8 = 245;
    pub const ReceiverGroup: u8 = 246;
    pub const Cache: u8 = 247;
    pub const GameProperties: u8 = 248;
    pub const PlayerProperties: u8 = 249;
    pub const Broadcast: u8 = 250;
    pub const Properties: u8 = 251;
    pub const ActorList: u8 = 252;
    pub const TargetActorNr: u8 = 253;
    pub const ActorNr: u8 = 254;
    pub const RoomName: u8 = 255;
}

// this may not work, I'm not sure yet
impl Eq for ProtocolValue<'_> {}

// really annoying and ugly hack, but I wouldn't know how to fix it
impl Hash for ProtocolValue<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            ProtocolValue::Null() => 0.hash(state),
            ProtocolValue::Bool(x) => x.hash(state),
            ProtocolValue::Byte(x) => x.hash(state),
            ProtocolValue::Short(x) => x.hash(state),
            ProtocolValue::Integer(x) => x.hash(state),
            ProtocolValue::Long(x) => x.hash(state),
            ProtocolValue::Float(x) => unsafe { std::mem::transmute::<f32, u32>(*x) }.hash(state),
            ProtocolValue::Double(x) => unsafe { std::mem::transmute::<f64, u64>(*x) }.hash(state),
            ProtocolValue::String(x) => x.hash(state),
            _ => panic!("Tried to hash {:?}", self),
        }
    }
}
