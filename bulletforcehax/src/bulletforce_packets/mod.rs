use std::collections::HashMap;

pub use super::photon::ProtocolValue;
use super::photon::*;

mod errors;
pub use errors::*;
mod macros;
mod packets_impl;
mod payloads_impl;
mod payloads_tests;

#[derive(Debug)]
pub enum Packet<'a> {
    OperationRequest(Operation<'a>),
    OperationResponse(Operation<'a>, i16, Option<&'a str>),
    Event(Event<'a>),
    InternalOperationRequest(InternalOperation),
    InternalOperationResponse(InternalOperation, i16, Option<&'a str>),
}

#[derive(Debug)]
pub enum Event<'a> {
    /// Only when hosted with Azure, now obsolete
    AzureNodeInfo,
    /// Sent to update token
    AuthEvent,
    /// Contains list of lobbies with player and game counts
    LobbyStats,
    /// Stats such as game, peer and master peer count
    AppStats {
        game_count: i32,
        peer_count: i32,
        master_peer_count: i32,
    },
    /// Unused
    Match,
    /// Unused
    QueueState,
    /// Update to game list
    GameListUpdate(HashMap<&'a str, Option<GameInfo<'a>>>),
    /// Initial game list
    GameList(Vec<GameInfo<'a>>),
    CacheSliceChanged,
    ErrorInfo,
    /// Used to update broadcasted properties
    PropertiesChanged,
    /// Player leaves the game
    Leave,
    /// Player joins the game
    Join {
        actor_nr: i32,
        player_properties: PlayerProperties<'a>,
        actor_list: Vec<i32>,
    },
}

#[derive(Debug)]
pub enum Operation<'a> {
    /// Used to get game list with SQL filter
    GetGameList,
    /// Changes server settings
    ServerSettings,
    WebRpc,
    /// Gets a list of region servers
    GetRegions,
    GetLobbyStats,
    /// Request room and online status from friend by name
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
        room_option_flags: i32,
        cleanup_cache_on_leave: bool,
        check_user_on_join: bool,
    },
    CreateGameResponse2 {
        actor_list: Vec<i32>,
        actor_nr: i32,
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
        position: i32,
    },
    AuthenticateResponseEmpty(),
    AuthenticateOnce,
    ChangeGroups,
    ExchangeKeysForEncryption,
    GetProperties,
    SetPropertiesGame {
        broadcast: bool,
        properties: GameProperties<'a>,
    },
    SetPropertiesActor {
        broadcast: bool,
        /// updates select properties of an actor
        properties: HashMap<ProtocolValue<'a>, ProtocolValue<'a>>,
        actor_nr: i32,
    },
    SetPropertiesEmpty(),
    /// when props only contains `is_visible: bool`
    SetPropertiesUnknown {
        broadcast: bool,
        properties: HashMap<ProtocolValue<'a>, ProtocolValue<'a>>,
    },
    /// Raise an event for other actors in the room
    RaiseEventActors {
        cache: u8,
        actor_list: Vec<i32>,
        code: u8,
    },
    RaiseEventSelf {
        cache: Option<u8>,
        code: u8,
        data: HashMap<ProtocolValue<'a>, ProtocolValue<'a>>,
    },
    RaiseEventEmpty(),
    Leave,
    Join,
}

#[derive(Debug)]
pub enum InternalOperation {
    InitEncryption,
    PingRequest { local_time: i32 },
    PingResponse { local_time: i32, server_time: i32 },
}

pub enum Direction {
    Send,
    Recv,
}

#[derive(Debug, PartialEq)]
pub struct GameInfo<'a> {
    // shared with GameProperties
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
    mean_rank: either::Either<i32, f32>,
    mean_kd: f32,
    average_rank: i32,
    event_code: i32,

    // byte_251: bool, is_removed, either this or all other fields are present
    /// Current players in the room
    player_count: u8, // 252
    /// Allow other players to join
    is_open: bool, // 253
    /// Max players that fit in this room. 0 for unlimited.
    max_players: u8, // 255
}

#[derive(Debug, PartialEq)]
pub struct GameProperties<'a> {
    // shared with GameInfo, names are contained in props_listed_in_lobby
    game_id: &'a str,
    room_id: &'a str,
    store_id: &'a str,
    room_name: &'a str,
    mode_name: &'a str,
    password: &'a str,
    map_name: &'a str,
    match_started: bool,
    switching_map: bool,
    room_type: u8,
    dedicated: bool,
    hardcore: bool,
    allowed_weapons: u64,
    mean_rank: either::Either<i32, f32>,
    mean_kd: f32,
    average_rank: i32,
    event_code: i32,

    // exclusive for GameProperties
    spectate_for_mods_only: bool,
    max_ping: i16,
    banned_weapon_message: &'a str,
    time_scale: f32,
    match_countdown_time: f32,
    round_started: bool,
    score_limit: i32,
    gun_game_preset: i32,

    // options below not present in Operation::SetProperties
    cleanup_cache_on_leave: Option<bool>, // 249
    /// List of some fields that are present in this struct, which don't seem to be present in GameInfo
    props_listed_in_lobby: Option<Vec<&'a str>>, // 250
    is_open: Option<bool>,                // 253
    /// Is room visible in lobby
    is_visible: Option<bool>, // 254
    max_players: Option<u8>,              // 255
    /// Only present in Operation::CreateGame* response
    master_client_id: Option<i32>, // 248
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
    /// Also called `CustomEventContent`, which has the same purpose.
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
