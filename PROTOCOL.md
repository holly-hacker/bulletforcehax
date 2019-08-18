Bullet Force protocol
===

## Networking library
Bullet Force uses [Photon](https://www.photonengine.com/en/PUN) for networking. Download it from the Unity Asset Store and throw Photon3Unity3D.dll in [dnSpy](https://github.com/0xd4d/dnSpy/) to see how the protocol is implemented. It is located in ExitGames.Client.Photon.Protocol16.

## Packets
Packets are generally instances of Event, OperationRequest or OperationResponse. They have an OperationCode/EventCode (byte) and parameter hashmap containing the sent or received data. The parameter always has ParameterCode (byte) as keys and other serialized data as values.

Other packet types are Init, InitResponse, InternalOperationRequest, InternalOperationResponse, Message and RawMessage

### OperationRequest
Sent from client to server, usually expects a OperationResponse or Event (in the case of RaiseEvent) in return.

#### 252: SetProperties
Launch UAV killstreak?: `OperationRequest 252: {251: {killstreak: int8 1}, 254: int32 8, 250: true}` (Parameters: 251=Properties, 254=ActorNr, 150=Broadcast)

- 1: uav
- 2: super soldier 
- 3: counter-uav (works!)
- 4: a-uav 
- 5: haste
- 6: Nuke, does not show

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

#### Code 15: Knife?
No parameter

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
`ShootOther41(int32 targetId, float damageGiven, Vector3 bulletDirection?, byte weaponId, Vector3 targetLocation?, Vector3 shooterPos)`

Example data:
- Foot: `[int32 4, float32 24.645000457763672, Vector3(0.4758644104003906,-1.3645477294921875,-0.22233200073242188), int8 14, Vector3(14.226451873779297,49.95946502685547,-6.848456859588623), Vector3(17.378496170043945,51.19110107421875,-6.529361724853516)]`
- Stomach: `[int32 4, float32 26.5, Vector3(0.08257770538330078,-0.5146980285644531,0.0635824203491211), int8 14, Vector3(13.833165168762207,50.8093147277832,-6.56254243850708), Vector3(17.397846221923828,51.192405700683594,-6.52968692779541)]`
- Head: `[int32 4, float32 56.445003509521484, Vector3(0.29883384704589844,0.0554046630859375,-0.027111530303955078), int8 14, Vector3(14.049421310424805,51.379417419433594,-6.653236389160156), Vector3(17.378286361694336,51.18973922729492,-6.529699802398682)]`

#### Code 58: ?
No parameter

#### Code 64: ?
No parameter

### 201
Seems to carry continuous status updates from the player, including velocity, pitch, yaw.

Values:
1. int, based on actorId
2. bool, ?
3. null, ?
4. int16, camera pitch
5. int16, camera yaw
6. int16, walking direction
7. int16, ?
8. int16, spawn invulnerability?
9. int16, ?
10. int16, ?
11. int16, ?
12. int16, velocity x
13. int16, velocity y
14. int16, velocity z
15. int16 health?
16. int8, ?
17. int8, ?
18. int8, ?
19. int8, ?
20. int8, Animation flags
21. [if S->C] int8, ?
21. int32, always 999 for client?
22. Vector3, likely position

Comes with an ActorList when from server

#### Animation flags
- 0x01: ?
- 0x02: ?
- 0x04: ? 
- 0x08: on land?
- 0x10: throwing
- 0x20: reload
- 0x40: shooting
- 0x80: crouch

Examples of parameters from client:
- [int32 19001, false, null, int16 2850, int16 530, int16 534, int16 0, int16 0, int16 324, int16 0, int16 0, int16 0, int16 19056, int16 0, int16 10000, int8 1, int8 0, int8 0, int8 0, int8 0, int32 999, Instance of 'CustomData']
- [int32 19001, false, null, int16 2850, int16 530, int16 534, int16 0, int16 0, int16 324, int16 0, int16 0, int16 0, int16 17716, int16 0, int16 10000, int8 1, int8 0, int8 0, int8 0, int8 0, int32 999, Instance of 'CustomData']
- [int32 19001, false, null, int16 2850, int16 530, int16 534, int16 0, int16 0, int16 324, int16 0, int16 0, int16 0, int16 16376, int16 0, int16 10000, int8 1, int8 0, int8 0, int8 0, int8 0, int32 999, Instance of 'CustomData']
- [int32 3001, false, null, int16 2840, int16 530, int16 539, int16 0, int16 0, int16 204, int16 0, int16 0, int16 0, int16 -10185, int16 0, int16 10000, int8 1, int8 0, int8 1, int8 0, int8 0, int32 999, Instance of 'CustomData']

Examples of parameters from server:
- [int32 13002, false, null, int16 150, int16 2790, int16 3600, int16 17, int16 9, int16 250, int16 -7, int16 16, int16 -2361, int16 -4897, int16 401, int16 10000, int8 0, int8 0, int8 0, int8 28, int8 0, int32 15, Instance of 'CustomData']
- [int32 1001, false, null, int16 100, int16 1480, int16 3, int16 0, int16 0, int16 222, int16 0, int16 0, int16 993, int16 15, int16 -1589, int16 10000, int8 1, int8 0, int8 1, int8 0, int8 8, int32 999, Instance of 'CustomData']
- [int32 1001, false, null, int16 80, int16 3560, int16 3567, int16 0, int16 0, int16 193, int16 0, int16 0, int16 0, int16 0, int16 0, int16 10000, int8 0, int8 0, int8 0, int8 0, int8 8, int32 999, Vector3(16.558441162109375,31.58450698852539,45.3120002746582)]

### 206
Version of 200 for spectators?

### 207
Version of 201 for spectators?

## Custom Data
Photon allows sending/receiving arbitrary data. They will get wrapped in a CustomData object and be given a type code.

### 86: Vector3
A set of 3 floats.