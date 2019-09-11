Bullet Force protocol
===

## Networking library
Bullet Force uses [Photon](https://www.photonengine.com/en/PUN) for networking. Download it from the Unity Asset Store and throw Photon3Unity3D.dll in [dnSpy](https://github.com/0xd4d/dnSpy/) to see how the protocol is implemented. It is located in ExitGames.Client.Photon.Protocol16.

## Packets
Packets are generally instances of Event, OperationRequest or OperationResponse. They have an OperationCode/EventCode (byte) and parameter hashmap containing the sent or received data. The parameter always has ParameterCode (byte) as keys and other serialized data as values.

Other packet types are Init, InitResponse, InternalOperationRequest, InternalOperationResponse, Message and RawMessage

### InternalOperationRequest

#### 1: Ping
Sent params are:
- 1: Your current time. This just get echo'd back along with the server time.

### InternalOperationResponse

#### 1: Ping
Received params are:
- 1: Original parameter 1 from request
- 2: The time of the server

### OperationRequest
Sent from client to server, usually expects a OperationResponse or Event (in the case of RaiseEvent) in return.

#### 226: JoinGame

`OperationRequest 226: {255: 7328cad7-9945-4ce8-a95f-f75f10af5097, 249: {teamNumber: int8 0, rank: int8 5, killstreak: int8 0, characterCamo: int8 0, unlockedweapons: ProtocolArray 105: [int32 82432, int32 0], model: int8 1, perks: [0, 0, 0, 0, 0, 0, 0, 0], int8 255: (H) SandwichHax}, 250: true}`
```
OperationRequest 226: {
    255: 07fac636-8cfc-4535-9f71-b28014571407,
    249: {
        teamNumber: int8 0,
        rank: int8 6,
        killstreak: int8 0,
        characterCamo: int8 0,
        unlockedweapons: ProtocolArray 105: [int32 82432, int32 0],
        model: int8 1,
        perks: [0, 0, 0, 0, 0, 0, 0, 0],
        int8 255: (H) SandwichHax
    },
    250: true
}
```

#### 227: CreateGame

Fields for full request:
- RoomName (255): 7f4c664b-1b85-4ca6-a368-da2a7d6335be
- PlayerProperties (249):
  - teamNumber: int8 0
  - rank: int8 5
  - killstreak: int8 0
  - characterCamo: int8 0
  - unlockedweapons: ProtocolArray 105: [int32 82432, int32 0]
  - model: int8 0
  - perks: [0, 0, 0, 0, 0, 0, 0, 0]
  - int8 255: (H) YourName
- Broadcast (250): true
- GameProperties (248):
  - int8 253: true
  - int8 254: false
  - int8 250: ProtocolArray 115: [roomName, mapName, modeName, password, dedicated, switchingmap, allowedweapons, eventcode, averagerank]
  - roomName: DefaultMatch
  - mapName: City
  - modeName: Conquest
  - password: sdfgdfh
  - roundStarted: false
  - maxPing: int16 700
  - timeScale: float32 1
  - dedicated: false
  - scorelimit: int32 200
  - gunGamePreset: int32 0
  - matchCountdownTime: float32 0
  - matchStarted: false
  - switchingmap: false
  - allowedweapons: ProtocolArray 105: [int32 0xFFFFFFFF, int32 0xFFFFFFFF]
  - bannedweaponmessage: This message should never appear!
  - eventcode: int32 0
  - averagerank: int32 5
  - int8 255: int8 9
  - int8 249: true
- CleanupCacheOnLeave (241): true


#### 230: Authenticate
Params for load balancing server:
- 220: AppVersion, eg. `1.34_WebGL_1.73`
- 224: ApplicationId, application-unique guid
- 210: AzureNodeInfo, seems to be region?

#### 252: SetProperties
On match join: `OperationRequest 252: {251: {matchCountdownTime: float32 0, eventcode: int32 0, timeScale: float32 1, password: sdfgdfh, modeName: Conquest, averagerank: int32 0, roundStarted: false, roomName: DefaultMatch, mapName: City, maxPing: int16 700, bannedweaponmessage: This message should never appear!, dedicated: false, matchStarted: false, gunGamePreset: int32 0, allowedweapons: ProtocolArray 105: [int32 -1, int32 -1], scorelimit: int32 200, switchingmap: false}, 250: true}`

Launch UAV killstreak: `OperationRequest 252: {251: {killstreak: int8 1}, 254: int32 8, 250: true}` (Parameters: 251=Properties, 254=ActorNr, 150=Broadcast)

Fields:
- 251: (data HashMap)
- 250: true

Killstream options:
- 1: uav
- 2: super soldier 
- 3: counter-uav (works!)
- 4: a-uav 
- 5: haste
- 6: Nuke, does not show

#### 253: RaiseEvent
Used during gameplay to send user updates to the server. Most commonly sent packet. Has a parameter `Code` (244) which specifies an EventCode, though this is usually a custom one. Also has a parameter `CustomEventContent` (245) which contains the event payload. May have `Cache` (247) parameter in some cases.

See Gameplay Events for information about gameplay events.

### OperationResponse
Sent from server to client, in response to OperationRequest.

#### 226: JoinGame

`OperationResponse 226 (return=0, msg=null): {254: int32 2, 249: {int32 1: {characterCamo: int8 0, unlockedweapons: ProtocolArray 105: [int32 82432, int32 0], rank: int8 1, killstreak: int8 0, perks: [0, 0, 0, 0, 0, 0, 0, 0], teamNumber: int8 0, int8 255: (H) JustM3, model: int8 0}}, 248: {matchCountdownTime: float32 20, int8 249: true, int8 255: int8 9, int8 254: true, int8 253: true, eventcode: int32 0, int8 250: ProtocolArray 115: [roomName, mapName, modeName, password, dedicated, switchingmap, allowedweapons, eventcode, averagerank], timeScale: float32 1, int8 248: int32 1, password: xfbfgfg, modeName: Conquest, averagerank: int32 1, roundStarted: false, roomName: DefaultMatch, mapName: Urban, maxPing: int16 700, bannedweaponmessage: This message should never appear!, dedicated: false, matchStarted: false, gunGamePreset: int32 0, allowedweapons: ProtocolArray 105: [int32 -1, int32 -1], scorelimit: int32 200, switchingmap: false}, 252: ProtocolArray 105: [int32 1, int32 2]}`

Fields on full request:
- ActorNr (254): int32 2
- PlayerProperties (249):
  - int32 1:
    - characterCamo: int8 0
    - unlockedweapons: ProtocolArray 105: [int32 82432, int32 0]
    - rank: int8 1
    - killstreak: int8 0
    - perks: [0, 0, 0, 0, 0, 0, 0, 0]
    - teamNumber: int8 0
    - int8 255: (H) JustM3
    - model: int8 0
- GameProperties (248):
  - matchCountdownTime: float32 20
  - int8 249: true
  - int8 255: int8 9
  - int8 254: true
  - int8 253: true
  - eventcode: int32 0
  - int8 250: ProtocolArray 115: [roomName, mapName, modeName, password, dedicated, switchingmap, allowedweapons, eventcode, averagerank]
  - timeScale: float32 1
  - int8 248: int32 1
  - password: xfbfgfg
  - modeName: Conquest
  - averagerank: int32 1
  - roundStarted: false
  - roomName: DefaultMatch
  - mapName: Urban
  - maxPing: int16 700
  - bannedweaponmessage: This message should never appear!
  - dedicated: false
  - matchStarted: false
  - gunGamePreset: int32 0
  - allowedweapons: ProtocolArray 105: [int32 -1, int32 -1]
  - scorelimit: int32 200
  - switchingmap: false
- ActorList (252): ProtocolArray 105: [int32 1, int32 2]

#### 227: CreateGame

Fields on full request:
- ActorNr (254): int32 1
- GameProperties (248):
  - matchCountdownTime: float32 0
  - int8 249: true
  - int8 255: int8 9
  - int8 254: false
  - int8 253: true
  - eventcode: int32 0
  - int8 250: ProtocolArray 115: [roomName, mapName, modeName, password, dedicated, switchingmap, allowedweapons, eventcode, averagerank]
  - timeScale: float32 1
  - int8 248: int32 1
  - password: sdfgdfh
  - modeName: Conquest
  - averagerank: int32 5
  - roundStarted: false
  - roomName: DefaultMatch
  - mapName: City
  - maxPing: int16 700
  - bannedweaponmessage: This message should never appear!
  - dedicated: false
  - matchStarted: false
  - gunGamePreset: int32 0
  - allowedweapons: ProtocolArray 105: [int32 0xFFFFFFFF, int32 0xFFFFFFFF]
  - scorelimit: int32 200
  - switchingmap: false
- ActorList (252): ProtocolArray 105: [int32 1]


#### 230: Authenticate
Has Address (230), Secret (221), UserId (225), Nickname (196)

UserId is a guid. When connected to load balancing server, Secret is used to authenticate on the game server.

#### 255: Join
Uses parameters ActorNr, ActorList, PlayerProperties and GameProperties.

TODO: correlation with event 255?

### Event
Sent from server to client, in response to OperationRequest 253 (RaiseEvent).

See Gameplay Events for information about gameplay events.

#### 202
Contains tickbase

`Event 202: {245: {int8 6: int32 1999486474, int8 7: int32 1001, int8 0: PlayerBody}, 254: int32 1}`

#### 230: GameList
#### 229: GameListUpdate
Contains a list of lobbies with information such as lobby name, lobby password (!), allowed weapons, map, average rank, amount of players, etc. The list of lobbies is a hashmap with the lobby's GUID as key.



GameList contains a list of all lobbies, while GameListUpdate only contains changed lobbies.

Example:
```
Event 230: {
    222: {
        eb66ecc5-1277-45e3-8096-8d4e31e44da5: {
            int8 253: true,
            int8 252: int8 1,
            modeName: Gun Game,
            averagerank: int32 73,
            switchingmap: false,
            roomName: outlaws,
            allowedweapons: ProtocolArray 105: [int32 -1, int32 -1],
            eventcode: int32 0,
            dedicated: false,
            password: stinky,
            mapName: Woods,
            int8 255: int8 4
        },
        ca3d05f5-a9ef-4e89-b07f-c8b2d750a4db: { ... },
        22cb83db-631b-4f13-8fc2-d742f8f715e3: { ... },
        ...
    }
}
```

#### 255: Join

Fields:
- PlayerProperties (249):
  - characterCamo: int8 0
  - unlockedweapons: ProtocolArray 105: [int32 0x14200, int32 0]
  - rank: int8 5
  - killstreak: int8 0
  - perks: [0, 0, 0, 0, 0, 0, 0, 0]
  - teamNumber: int8 0
  - int8 255: (H) YourName
  - model: int8 0
- ActorList (252): ProtocolArray 105: [int32 1]
- ActorNr (254): int32 1

## Gameplay Events
TODO

### During gameplay init
Has some weird packets

Example of data during init:
- `OperationRequest 253: {244: int8 202, 247: int8 6, 252: ProtocolArray 105: [int32 1]}`
- `OperationRequest 253: {244: int8 200, 247: int8 6, 252: ProtocolArray 105: [int32 1]}`

### 200
Seems to carry single actions performed by the player (TODO: or other player?).

Fields:
- 0: actorNr*1000+1
- 2: tickbase, see PhotonPeer.ServerTimeInMilliSeconds
- 4: Code
- 5: Data


Example of normal fields:
- `{int8 0: int32 6001, int8 2: int32 0xA7027D7C, int8 5: int8 25, int8 4: ['author', 'message', int16 255, int16 105, int16 180]}`
- `{int8 0: int32 7001, int8 2: int32 0xA70520DF, int8 5: int8 25, int8 4: ['author', 'test', int16 255, int16 105, int16 180]}`
- `{int8 0: int32 2001, int8 2: int32 0xA1C74864, int8 5: int8 25, int8 4: ['author', 'sad', int16 255, int16 105, int16 180]}`
- `{int8 0: int32 2001, int8 2: int32 0xA1C77177, int8 5: int8 25, int8 4: ['author', 'a', int16 255, int16 105, int16 180]}`
- `{int8 0: int32 2001, int8 2: int32 0xA1C774FF, int8 5: int8 25, int8 4: ['author', 'a', int16 255, int16 105, int16 180]}`
- `{int8 0: int32 2001, int8 2: int32 0xA1C77885, int8 5: int8 25, int8 4: ['author', 'ab', int16 255, int16 105, int16 180]}`

#### Code 5: CaptureFlag
`CaptureFlag(int32 flagId, float progress)`

Progress is a value between -1 and 1.

#### Code 7: Health?
`Health(float health)`

Health is between 0 and 100?

#### Code 10: ShootOther? TakenDamage?
`ShootOther10(int32 targetId, float damageGiven, Vector3 damageLocation?, byte gunType, float damageLeft?)`

Example send:
- [int32 1, float32 26.5, some Vector3, int8 14, float32 73.5]

Example receive:
- [int32 1, float32 56.445003509521484, Vector3(-0.22093963623046875,0.0686492919921875,0.204864501953125), int8 14, float32 43.554996490478516]
- [int32 1, float32 56.445003509521484, Vector3(-0.20749282836914062,0.10479736328125,0.19452285766601562), int8 14, float32 -12.890007019042969]

#### Code 14

Event14(true)


#### Code 15: Knife?
No parameter

#### Code 24: PlayerDied
`YouDied(int32 targetId, int32 sourceId, int8 ?, int8 gunType)`

TODO: can you send this? :)

Example receive:
- [int32 3, int32 1, int8 100, int8 14]

#### Code 25: Chat
`Chat(string author, string message, short r, short g, short b)`

Author is not checked serverside. For some reason, color is 16 bits per component instead of 8, is top half discarded?

#### Code 26: ShootOtherRPG?
`ShootOtherRPG(id?, damageGiven?, ?, ?, ?, ?, ?)`

#### Code 27
Sent at end of match, parameters are old and new map

`Event27(string map1, string map2)`

#### Code 34: TimeLeft?
`TimeLeft(float seconds)`

I have no clue why the client sends this to the server. Sent every 10s

#### Code 36: ChangeGun
`ChangeGun(byte id)`

#### Code 36: ChangeThrowable?
`ChangeThrowable(byte id)`

#### Code 40
Sent after 27, using its second parameter. Likely ChangeMap or something.

`Event40(string map)`

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
19. int8, weapon id?
20. int8, Animation flags
21. int32, ?
22. Vector3, position

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

### 202
- 0: string, what to init?
- 6: int, when to init, servertime
- 7: int, actor id/actor nr?

On match creation:
- `OperationRequest 253: {244: int8 202, 245: {int8 0: PlayerBody, int8 6: int32 -1862770206, int8 7: int32 1001}, 247: int8 4}`
- `OperationRequest 253: {244: int8 202, 245: {int8 0: Match Manager, int8 6: int32 -1862770198, int8 7: int32 1}, 247: int8 5}`

### 206
Version of 200 for spectators?

### 207
Version of 201 for spectators?

Examples:
- Sent during init: `OperationRequest 253: {244: int8 207, 245: {int8 0: int32 1}}`

## Custom Data
Photon allows sending/receiving arbitrary data. They will get wrapped in a CustomData object and be given a type code.

### 86: Vector3
A set of 3 floats.