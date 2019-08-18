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
### 201
### 206
### 207