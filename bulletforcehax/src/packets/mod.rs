mod errors;
pub use errors::PacketReadError;
mod reading_functions;

#[derive(Debug)]
pub enum Packet<'a> {
    Init,
    InitResponse,
    OperationRequest(Operation),
    OperationResponse(i16, Option<&'a str>, Operation),
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
    AppStats,
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
pub enum Operation {
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
    JoinLobby,
    Authenticate,
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
    Ping,
}

#[derive(Debug)]
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
    Array, // An array of predetermined type, C# type is Array.
    ObjectArray,
    ByteArray,
    StringArray,
    IntegerArray,
    Dictionary, // Map<Object, Object>, predefined types, C# type is IDictionary/Dictionary<T1, T2>, TODO
    Hashtable,  // Map<Object, Object>, random types, C# type is Hashtable/Dictionary<object, object>
    Custom,
}

pub enum Direction {
    Send,
    Recv,
}
