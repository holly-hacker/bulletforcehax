use std::collections::HashMap;
use std::hash::{Hash, Hasher};
mod errors;
pub use errors::{PacketReadError, PacketReadResult};
mod reading_functions;

#[derive(Debug)]
pub enum Packet<'a> {
    Init,
    InitResponse,
    OperationRequest(Operation<'a>),
    OperationResponse(i16, Option<&'a str>, Operation<'a>),
    Event(Event),
    InternalOperationRequest(InternalOperation),
    InternalOperationResponse(i16, Option<&'a str>, InternalOperation),
    Message,
    RawMessage,
}

#[derive(Debug)]
pub enum Event {
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
    GameListUpdate,
    GameList,
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
    CreateGame,
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
