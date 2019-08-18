Bullet Force protocol
===

## Networking library
Bullet Force uses [Photon](https://www.photonengine.com/en/PUN) for networking. Download it from the Unity Asset Store and throw Photon3Unity3D.dll in [dnSpy](https://github.com/0xd4d/dnSpy/) to see how the protocol is implemented. It is located in ExitGames.Client.Photon.Protocol16.

## Packets
Packets are generally instances of Event, OperationRequest or OperationResponse. They have an OperationCode/EventCode (byte) and parameter hashmap containing the sent or received data. The parameter always has ParameterCode (byte) as keys and other serialized data as values.

Other packet types are Init, InitResponse, InternalOperationRequest, InternalOperationResponse, Message and RawMessage

### OperationRequest
Sent from client to server, usually expects a OperationResponse or Event (in the case of RaiseEvent) in return.

#### 253: RaiseEvent
Used during gameplay to send user updates to the server. Most commonly sent packet. Has a parameter `Code` (244) which specifies an EventCode, though this is usually a custom one. Also has a parameter `CustomEventContent` (245) which contains the event payload.

See Gameplay Events for information about gameplay events.

### OperationResponse
Sent from server to client, in response to OperationRequest.

#### 255: JoinGame
Uses parameters ActorNr, ActorList, PlayerProperties and GameProperties.

TODO: correlation with event 255?

### Event
Sent from server to client, in response to OperationRequest 253 (RaiseEvent).

See Gameplay Events for information about gameplay events.

#### 230: GameList
#### 229: GameListUpdate
Contains a list of lobbies with information such as lobby name, lobby password (!), allowed weapons, map, average rank, amount of players, etc. The list of lobbies is a hashmap with the lobby's GUID as key.

GameList contains a list of all lobbies, while GameListUpdate only contains changed lobbies.

#### 255: Join
Contains ActorList, ActorNr, PlayerProperties

## Gameplay Events
TODO

### 200
Seems to carry single actions performed by the player (TODO: or other player?).

Fields:
- 0: Based on player id?
- 2: ?
- 4: Code
- 5: Data

#### Code 5: CaptureFlag
`CaptureFlag(int32 flagId, float progress)`

Progress is a value between -1 and 1.

#### Code 7: Health?
`Health(float health)`

Health is between 0 and 100?

#### Code 10: ShootOther?
`ShootOther10(int32 targetId, float damageGiven, CustomData ?, byte gunType?, float damageLeft?)`

Example data:
- [int32 1, float32 26.5, Instance of 'CustomData', int8 14, float32 73.5]

#### Code 25: Chat
`Chat(string author, string message, short r, short g, short b)`

Author is not checked serverside. For some reason, color is 16 bits per component instead of 8, is top half discarded?

#### Code 26: ShootOtherRPG?
`ShootOtherRPG(id?, damageGiven?, ?, ?, ?, ?, ?)`

#### Code 34: TimeLeft?
`TimeLeft(float seconds)`

I have no clue why the client sends this to the server.

#### Code 36: ChangeGun
`ChangeGun(byte id)`

#### Code 36: ChangeThrowable?
`ChangeThrowable(byte id)`

#### Code 41: ShootOther?
`ShootOther41(int32 targetId, float damageGiven, CustomData ?, byte ?, CustomData ?, CustomData ?)`

Example data:
- [int32 9, float32 24.513553619384766, Instance of 'CustomData', int8 1, Instance of 'CustomData', Instance of 'CustomData']

#### Code 58: ?
No parameter

#### Code 64: ?
No parameter

### 201
Seems to carry continuous status updates from the player, including velocity, pitch, yaw.

Comes with an ActorList when from server?

Different arguments if from server.

Examples of parameters from client:
- [int32 19001, false, null, int16 2850, int16 530, int16 534, int16 0, int16 0, int16 324, int16 0, int16 0, int16 0, int16 19056, int16 0, int16 10000, int8 1, int8 0, int8 0, int8 0, int8 0, int32 999, Instance of 'CustomData']
- [int32 19001, false, null, int16 2850, int16 530, int16 534, int16 0, int16 0, int16 324, int16 0, int16 0, int16 0, int16 17716, int16 0, int16 10000, int8 1, int8 0, int8 0, int8 0, int8 0, int32 999, Instance of 'CustomData']
- [int32 19001, false, null, int16 2850, int16 530, int16 534, int16 0, int16 0, int16 324, int16 0, int16 0, int16 0, int16 16376, int16 0, int16 10000, int8 1, int8 0, int8 0, int8 0, int8 0, int32 999, Instance of 'CustomData']
- [int32 3001, false, null, int16 2840, int16 530, int16 539, int16 0, int16 0, int16 204, int16 0, int16 0, int16 0, int16 -10185, int16 0, int16 10000, int8 1, int8 0, int8 1, int8 0, int8 0, int32 999, Instance of 'CustomData']

Examples of parameters from server:
- [int32 13002, false, null, int16 150, int16 2790, int16 3600, int16 17, int16 9, int16 250, int16 -7, int16 16, int16 -2361, int16 -4897, int16 401, int16 10000, int8 0, int8 0, int8 0, int8 28, int8 0, int32 15, Instance of 'CustomData']
- [int32 1001, false, null, int16 100, int16 1480, int16 3, int16 0, int16 0, int16 222, int16 0, int16 0, int16 993, int16 15, int16 -1589, int16 10000, int8 1, int8 0, int8 1, int8 0, int8 8, int32 999, Instance of 'CustomData']

### 206
Version of 200 for spectators?

### 207
Version of 201 for spectators?[]()
