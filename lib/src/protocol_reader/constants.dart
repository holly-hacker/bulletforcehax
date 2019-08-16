abstract class DataType {
  static const int NullValue = 42;
  static const int Dictionary = 68; // Map<Object, Object>, predefined types, C# type is IDictionary/Dictionary<T1, T2>
  static const int StringArray = 97;
  static const int Byte = 98;
  static const int Custom = 99;
  static const int Double = 100;
  static const int EventData = 101;
  static const int Float = 102;
  static const int Hashtable = 104; // Map<Object, Object>, random types, C# type is Hashtable/Dictionary<object, object>
  static const int Integer = 105;
  static const int Short = 107;
  static const int Long = 108;
  static const int IntegerArray = 110;
  static const int Bool = 111;
  static const int OperationResponse = 112;
  static const int OperationRequest = 113;
  static const int String = 115;
  static const int ByteArray = 120;
  static const int Array = 121;       // A List, predetermined type, C# type is Array
  static const int ObjectArray = 122; // A Set, random types, C# type is List<object>
}

abstract class PacketType {
  static const int Init = 0;
  static const int InitResponse = 1;
  static const int Operation = 2;
  static const int OperationResponse = 3;
  static const int Event = 4;
  static const int InternalOperationRequest = 6;
  static const int InternalOperationResponse = 7;
  static const int Message = 8;
  static const int RawMessage = 9;
}

abstract class OperationCode {
  static const int GetGameList = 217;
  static const int ServerSettings = 218;
  static const int WebRpc = 219;
  static const int GetRegions = 220;
  static const int GetLobbyStats = 221;
  static const int FindFriends = 222;
  static const int CancelJoinRandom = 224;
  static const int JoinRandomGame = 225;
  static const int JoinGame = 226;
  static const int CreateGame = 227;
  static const int LeaveLobby = 228;
  static const int JoinLobby = 229;
  static const int Authenticate = 230;
  static const int AuthenticateOnce = 231;
  static const int ChangeGroups = 248;
  static const int ExchangeKeysForEncryption = 250;
  static const int GetProperties = 251;
  static const int SetProperties = 252;
  static const int RaiseEvent = 253;
  static const int Leave = 254;
  static const int Join = 255;
}

abstract class EventCode {
  static const int AzureNodeInfo = 210;
  static const int AuthEvent = 223;
  static const int LobbyStats = 224;
  static const int AppStats = 226;
  static const int Match = 227;
  static const int QueueState = 228;
  static const int GameListUpdate = 229;
  static const int GameList = 230;
  static const int CacheSliceChanged = 250;
  static const int ErrorInfo = 251;
  static const int PropertiesChanged = 253;
  static const int SetProperties = 253;
  static const int Leave = 254;
  static const int Join = 255;

  // custom
  // static const int GameList = 229;
  // static const int GameUpdate = 230;
}

abstract class ParameterCode {
  static const int FindFriendsRequestList = 1;
  static const int FindFriendsResponseOnlineList = 1;
  static const int FindFriendsOptions = 2;
  static const int FindFriendsResponseRoomIdList = 2;
  static const int RoomOptionFlags = 191;
  static const int EncryptionData = 192;
  static const int EncryptionMode = 193;
  static const int CustomInitData = 194;
  static const int ExpectedProtocol = 195;
  static const int PluginVersion = 200;
  static const int PluginName = 201;
  static const int NickName = 202;
  static const int MasterClientId = 203;
  static const int Plugins = 204;
  static const int CacheSliceIndex = 205;
  static const int WebRpcReturnMessage = 206;
  static const int WebRpcReturnCode = 207;
  static const int AzureMasterNodeId = 208;
  static const int WebRpcParameters = 208;
  static const int AzureLocalNodeId = 209;
  static const int UriPath = 209;
  static const int AzureNodeInfo = 210;
  static const int Region = 210;
  static const int LobbyStats = 211;
  static const int LobbyType = 212;
  static const int LobbyName = 213;
  static const int ClientAuthenticationData = 214;
  static const int CreateIfNotExists = 215;
  static const int JoinMode = 215;
  static const int ClientAuthenticationParams = 216;
  static const int ClientAuthenticationType = 217;
  static const int Info = 218;
  static const int AppVersion = 220;
  static const int Secret = 221;
  static const int GameList = 222;
  static const int MatchMakingType = 223;
  static const int Position = 223;
  static const int ApplicationId = 224;
  static const int UserId = 225;
  static const int MasterPeerCount = 227;
  static const int GameCount = 228;
  static const int PeerCount = 229;
  static const int Address = 230;
  static const int ExpectedValues = 231;
  static const int CheckUserOnJoin = 232;
  static const int IsComingBack = 233;
  static const int IsInactive = 233;
  static const int EventForward = 234;
  static const int PlayerTTL = 235;
  static const int EmptyRoomTTL = 236;
  static const int SuppressRoomEvents = 237;
  static const int Add = 238;
  static const int PublishUserId = 239;
  static const int Remove = 239;
  static const int Group = 240;
  static const int CleanupCacheOnLeave = 241;
  static const int Code = 244;
  static const int CustomEventContent = 245;
  static const int Data = 245;
  static const int ReceiverGroup = 246;
  static const int Cache = 247;
  static const int GameProperties = 248;
  static const int PlayerProperties = 249;
  static const int Broadcast = 250;
  static const int Properties = 251;
  static const int ActorList = 252;
  static const int TargetActorNr = 253;
  static const int ActorNr = 254;
  static const int RoomName = 255;
}
