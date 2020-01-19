use bitflags::bitflags;
use num_derive::FromPrimitive;
use std::collections::HashMap;

pub use photon_core::ProtocolValue;
use photon_core::*;
use photon_derive::PacketTypeImpl;

mod errors;
pub use errors::*;
mod macros;
mod packets_impl;
mod payloads_impl;
mod payloads_tests;
mod tests;

#[derive(Debug, PartialEq)]
pub enum Packet<'a> {
    OperationRequest(Operation<'a>),
    OperationResponse {
        parameters: Operation<'a>,
        return_code: i16,
        debug_string: Option<&'a str>,
        secret: Option<&'a str>, // note: not present in OperationRequest on purpose. See CreateGame
    },
    Event {
        parameters: Event<'a>,
        custom_data: Option<ProtocolValue<'a>>,
        sender: Option<i32>,
    },
    InternalOperationRequest(InternalOperation),
    InternalOperationResponse(InternalOperation, i16, Option<&'a str>),
}

#[derive(Debug, PartialEq, PacketTypeImpl)]
pub enum Event<'a> {
    /// Only when hosted with Azure, now obsolete
    #[packet_type(210)]
    AzureNodeInfo,
    /// Sent to update token
    #[packet_type(223)]
    AuthEvent,
    /// Contains list of lobbies with player and game counts
    #[packet_type(224)]
    LobbyStats,
    /// Stats such as game, peer and master peer count. Sent every minute by master server.
    #[packet_type(226)]
    AppStats {
        game_count: i32,
        peer_count: i32,
        master_peer_count: i32,
    },
    /// Unused
    #[packet_type(227)]
    Match,
    /// Unused
    #[packet_type(228)]
    QueueState,
    /// Initial game list. Contains all current games.
    #[packet_type(230)]
    GameList(HashMap<&'a str, RoomInfo<'a>>),
    /// Update to game list. Contains `Option`s which are `None` if the game was removed, and `Some` if it was added or updated.
    #[packet_type(229)]
    GameListUpdate(HashMap<&'a str, Option<RoomInfo<'a>>>),
    #[packet_type(250)]
    CacheSliceChanged,
    #[packet_type(251)]
    ErrorInfo,
    /// Used to update broadcasted properties
    #[packet_type(253)]
    PropertiesChanged,
    /// Player leaves the game
    #[packet_type(254)]
    Leave,
    /// Player joins the game. If `actor_nr` is 1, we may be creating the game.
    #[packet_type(255)]
    Join {
        player_properties: Player<'a>,
        /// All players currently in the room, if this event is ours
        actor_list: Option<Vec<i32>>,
    },
}

#[derive(Debug, PartialEq, PacketTypeImpl)]
pub enum Operation<'a> {
    /// Used to get game list with SQL filter
    #[packet_type(217)]
    GetGameList,
    /// Changes server settings
    #[packet_type(218)]
    ServerSettings,
    #[packet_type(219)]
    WebRpc,
    /// Gets a list of region servers
    #[packet_type(220)]
    GetRegions,
    #[packet_type(221)]
    GetLobbyStats,
    /// Request room and online status from friend by name
    #[packet_type(222)]
    FindFriends,
    #[packet_type(224)]
    CancelJoinRandom,
    #[packet_type(225)]
    JoinRandomGame,
    #[packet_type(226)]
    JoinGame,
    /// CreateGame on MasterServer
    #[packet_type(227)]
    CreateGameRequestMaster {
        room_name: Option<&'a str>,   // can be null, not present then
        lobby_name: Option<&'a str>,  // if lobby not null and not default
        lobby_type: Option<bool>,     // if lobby not null and not default
        expected_users: Vec<&'a str>, // not present if null or empty
    },
    /// CreateGame on GameServer, has extra options compared to `CreateGameRequestMaster`
    #[packet_type(227)]
    CreateGameRequestGame {
        room_name: Option<&'a str>,   // can be null, not present then
        lobby_name: Option<&'a str>,  // if lobby not null and not default
        lobby_type: Option<bool>,     // if lobby not null and not default
        expected_users: Vec<&'a str>, // not present if null or empty

        /// Player struct, but with only custom properties and nick
        player_properties: Option<Player<'a>>, // only present if not null and not empty?
        broadcast: Option<bool>, // present+true if playerproperties is present

        game_properties: RoomOptions<'a>, // always present, so can be used to differentiate master and game packet
        player_ttl: i32,                  // only present if ttl > 0 or == -1
        empty_room_ttl: i32,              // only present if ttl > 0
        plugins: Option<Vec<&'a str>>,    // not present if null

        room_option_flags: RoomOptionsFlags, // note: assuming always present,  so that other bool flags don't need to be present
    },
    /// CreateGame in MasterServer. The `Secret` variable is can be found in `OperationResponse`.
    #[packet_type(227)]
    CreateGameResponseMaster { room_name: Option<&'a str>, address: &'a str },
    /// CreateGame on GameServer
    #[packet_type(227)]
    CreateGameResponseGame {
        /// our actor number
        actor_nr: i32,
        actor_list: Option<Vec<i32>>,
        game_properties: RoomInfo<'a>,
        /// A list of all actors in the room.
        player_properties: HashMap<i32, Player<'a>>, // Should correspond to `actor_list`?
    },
    #[packet_type(228)]
    LeaveLobby,
    #[packet_type(229)]
    JoinLobby(),
    /// Full authentication request to request a token
    #[packet_type(230)]
    AuthenticateRequestNoToken {
        lobby_stats: bool, // not present if false
        app_version: &'a str,
        app_id: &'a str, // could be parsed as u128 since it's a guid
        region: Option<&'a str>,
        user_id: Option<&'a str>,
        client_auth_type: Option<u8>,
        /// Only if `client_auth_type` is not 255
        client_auth_params: Option<&'a str>, // not present if not null or empty
        client_auth_data: Option<&'a str>, // can be empty
    },
    /// Authenticate if we already have a token. Since this is sent, `secret` is a payload parameter.
    #[packet_type(230)]
    AuthenticateRequestToken {
        lobby_stats: bool, // not present if false
        secret: &'a str,
    },
    /// The Authenticate response on NameServer
    /// The `Secret` variable is can be found in `OperationResponse`.
    #[packet_type(230)]
    AuthenticateResponseName {
        user_id: Option<&'a str>,
        nickname: Option<&'a str>,
        encryption_data: Option<HashMap<u8, ProtocolValue<'a>>>, // probably not used in websocket connections
        custom_data: Option<HashMap<&'a str, ProtocolValue<'a>>>,

        // unique
        cluster: Option<&'a str>,
        address: &'a str,
    },
    /// The Authenticate response on MasterServer or GameServer
    /// The `Secret` variable is can be found in `OperationResponse`.
    #[packet_type(230)]
    AuthenticateResponseMasterOrGame {
        /// Only on MasterServer
        user_id: Option<&'a str>,
        /// Only on MasterServer
        nickname: Option<&'a str>,
        /// Only on MasterServer
        encryption_data: Option<HashMap<u8, ProtocolValue<'a>>>, // probably not used in websocket connections
        custom_data: Option<HashMap<&'a str, ProtocolValue<'a>>>,
        /// Unused field, here for completeness. Seems to be 0 (meaning no waitlist to join?) for MasterServer
        position: Option<i32>,
    },
    #[packet_type(231)]
    AuthenticateOnce,
    #[packet_type(248)]
    ChangeGroups,
    #[packet_type(250)]
    ExchangeKeysForEncryption,
    #[packet_type(251)]
    GetProperties,
    // send only
    // TODO: how to handle this? add fn to RoomInfo to apply this update?
    #[packet_type(252)]
    SetPropertiesGame {
        /// The added/changed properties of this room
        properties: HashMap<ProtocolValue<'a>, ProtocolValue<'a>>,
        expected_properties: Option<HashMap<ProtocolValue<'a>, ProtocolValue<'a>>>,
        broadcast: bool,     // always true
        event_forward: bool, // not present if false
    },
    // send only
    // TODO: how to handle this? add fn to Player to apply this update?
    #[packet_type(252)]
    SetPropertiesActor {
        /// The actor to update
        actor_nr: i32,
        /// The added/changed properties of this actor
        properties: HashMap<ProtocolValue<'a>, ProtocolValue<'a>>,
        expected_properties: Option<HashMap<ProtocolValue<'a>, ProtocolValue<'a>>>,
        broadcast: bool,     // always true
        event_forward: bool, // not present if false
    },
    /// Raise an event for other actors in the room
    #[packet_type(253)]
    RaiseEvent {
        /// The type of caching. If one of the following values, the other properties are `None`:
        /// - SliceSetIndex
        /// - SlicePurgeIndex
        /// - SlicePurgeUpToIndex
        /// - SliceIncreaseIndex
        /// - RemoveFromRoomCacheForActorsLeft
        ///
        /// if this is `TargetActors`, only the `actor_list`, `code` and `data` properties may be set
        cache: EventCaching, // not present if `DoNotCache`
        actor_list: Option<Vec<i32>>,
        /// The target group. 0 means everybody, other values require players to be subscribed to it.
        group: Option<u8>, // not present if 0
        receivers: Option<ReceiverGroup>, // not present if default
        event_forward: Option<bool>,      // not present if false
        /// The event code
        code: Option<u8>,
        /// Custom data associated to this event
        data: Option<ProtocolValue<'a>>,
    },
    #[packet_type(254)]
    Leave,
    #[packet_type(255)]
    Join,
}

#[derive(Debug, PartialEq, PacketTypeImpl)]
pub enum InternalOperation {
    #[packet_type(0)]
    InitEncryption, // TODO: has property public_key
    // TODO: server_time and local_time should be u32
    #[packet_type(1)]
    PingRequest { local_time: i32 },
    #[packet_type(1)]
    PingResponse { local_time: i32, server_time: i32 },
}

pub enum Direction {
    Send,
    Recv,
}

#[derive(Debug, PartialEq)]
// note: removed_from_list is not present here
// TODO: which of these are not always present for default values
/// A room containing players. You can receive a list by joining the lobby on the master server.
///
/// The name of this room is not included in this struct (not counting `custom_properties`).
pub struct RoomInfo<'a> {
    /// Max players that fit in this room. 0 for unlimited.
    max_players: u8,
    /// Allow other players to join
    is_open: bool, // defaults to true
    /// Does this room show in the lobby
    is_visible: bool, // defaults to true
    /// Current players in the room
    player_count: u8,
    cleanup_cache_on_leave: bool, // defaults to true
    master_client_id: Option<i32>,
    custom_properties_lobby: Vec<&'a str>,
    expected_users: Vec<&'a str>,
    empty_room_ttl: i32,
    player_ttl: i32,

    /// all other string-indexed properties
    custom_properties: HashMap<&'a str, ProtocolValue<'a>>,
}

// Used in CreateGame, JoinGame and JoinRandomGame
/// Info used when creating a room or when filtering. Very similar to `RoomInfo`
#[derive(Debug, PartialEq)]
pub struct RoomOptions<'a> {
    /// Max players that fit in this room. 0 for unlimited.
    max_players: u8,
    /// Allow other players to join
    is_open: bool,
    /// Does this room show in the lobby
    is_visible: bool,
    /// Should be the same as the parent!
    cleanup_cache_on_leave: bool, // included if false, but always included in parent
    custom_properties_lobby: Vec<&'a str>, // always present, even if empty

    /// all other string-indexed properties
    custom_properties: HashMap<&'a str, ProtocolValue<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Player<'a> {
    name: Option<&'a str>,
    user_id: Option<&'a str>,
    is_inactive: Option<bool>,

    /// all other string-indexed properties
    custom_properties: HashMap<&'a str, ProtocolValue<'a>>,
}

bitflags! {
    pub struct RoomOptionsFlags: u32 {
        const NONE = 0;
        /// toggles a check of the UserId when joining (enabling returning to a game)
        const CHECK_USER_ON_JOIN = 0x01;
        /// deletes cache on leave
        const DELETE_CACHE_ON_LEAVE = 0x02;
        /// suppresses all room events
        const SUPPRESS_ROOM_EVENTS = 0x04;
        /// signals that we should publish userId
        const PUBLISH_USER_ID = 0x08;
        /// signals that we should remove property if its value was set to null. see RoomOption to Delete Null Properties
        const DELETE_NULL_PROPS = 0x10;
        /// signals that we should send PropertyChanged event to all room players including initiator
        const BROADCAST_PROPS_CHANGE_TO_ALL = 0x20;
    }
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
    pub const Cluster: u8 = 196;
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

// TODO: check if enum is better
#[allow(dead_code, non_upper_case_globals, non_snake_case)]
pub mod GamePropertyKey {
    pub const MaxPlayers: u8 = 255;
    pub const IsVisible: u8 = 254;
    pub const IsOpen: u8 = 253;
    pub const PlayerCount: u8 = 252;
    pub const Removed: u8 = 251;
    pub const PropsListedInLobby: u8 = 250;
    pub const CleanupCacheOnLeave: u8 = 249;
    pub const MasterClientId: u8 = 248;
    pub const ExpectedUsers: u8 = 247;
    pub const PlayerTtl: u8 = 246;
    pub const EmptyRoomTtl: u8 = 245;
}

#[allow(dead_code, non_upper_case_globals, non_snake_case)]
pub mod ActorProperties {
    pub const PlayerName: u8 = 255;
    pub const IsInactive: u8 = 254;
    pub const UserId: u8 = 253;
}

#[allow(dead_code, non_snake_case)]
#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum EventCaching {
    DoNotCache = 0,
    /// obsolete
    MergeCache = 1,
    /// obsolete
    ReplaceCache = 2,
    /// obsolete
    RemoveCache = 3,
    AddToRoomCache = 4,
    AddToRoomCacheGlobal = 5,
    RemoveFromRoomCache = 6,
    RemoveFromRoomCacheForActorsLeft = 7,
    SliceIncreaseIndex = 10,
    SliceSetIndex = 11,
    SlicePurgeIndex = 12,
    SlicePurgeUpToIndex = 13,
}

#[allow(dead_code, non_snake_case)]
#[derive(Debug, PartialEq, FromPrimitive)]
pub enum ReceiverGroup {
    /// For everyone
    Others = 0,
    /// For everone including me
    All = 1,
    /// Only to master client (lowest actor nr)
    MasterClient = 2,
}
