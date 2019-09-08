import 'dart:async';
import 'dart:core';

import 'package:bullet_force_hax/bullet_force_hax.dart';
import 'package:bullet_force_hax/src/bot/gamesocket.dart';

import 'connection_details.dart';

class Bot {
  Map<String, GameListItem> games = Map();

  StreamController<GameListItem> _gamesController = StreamController<GameListItem>();
  Stream<GameListItem> get gamesStream => _gamesController.stream;  // TODO: implement pause/resume?
  StreamController<GameListItem> _newGamesController = StreamController<GameListItem>();
  Stream<GameListItem> get newGamesStream => _newGamesController.stream;  // TODO: implement pause/resume?

  GameSocket _lobbySocket;
  GameSocket _matchSocket;

  Completer<void> _getRoomCompleter;

  Future connectLobby() async {
    var credentials = await _getLobbyCredentials();
    await _connectLobby(credentials);
  }

  Future<ConnectionCredentials> _getLobbyCredentials() async {
    var socket = GameSocket.initial();
    var authPacket = await socket.packets.firstWhere((p) => p is OperationResponse && p.code == OperationCode.Authenticate) as OperationResponse;
    var credentials = ConnectionCredentials(authPacket.params[ParameterCode.Address], authPacket.params[ParameterCode.Secret]);

    // clean up
    socket.close(); // ignore: unawaited_futures

    return credentials;
  }

  Future _connectLobby(ConnectionCredentials credentials) async {
    _lobbySocket = GameSocket.fromCredentials(credentials);
    _lobbySocket.packets.listen((parsed) async {
      if (parsed is OperationResponse && parsed.code == OperationCode.Authenticate) {
        // we're authenticated, join the lobby
        _lobbySocket.add(OperationRequest(OperationCode.JoinLobby, {}));
      }
      else if (parsed is Event && (parsed.code == EventCode.GameList || parsed.code == EventCode.GameListUpdate)) {
        var map = parsed.params[ParameterCode.GameList] as Map<Object, Object>;
        map.forEach((key, value) {
          assert(key is String);
          assert(value is Map);
          if ((value as Map).containsKey(SizedInt.byte(251))) {
            games.remove(key);
          } else {
            var item = GameListItem.fromMap(key, value);
            if (!games.containsKey(key)) _newGamesController.add(item);
            _gamesController.add(item);

            games[key] = item;
          }
        });
      }
    });
  }

  Future disconnectLobby() async => await _lobbySocket.close();
  
  Future<ConnectionCredentials> getRoomCredentials(String roomId) async {
    // wait until previous join request is done
    while (_getRoomCompleter != null) {
      await _getRoomCompleter.future;
    }

    _getRoomCompleter = Completer();
    _lobbySocket.add(OperationRequest(OperationCode.JoinGame, { ParameterCode.RoomName: roomId }));

    var joinGamePacket = await _lobbySocket.packets.firstWhere((packet) => packet is OperationResponse && packet.code == OperationCode.JoinGame) as OperationResponse;
    _getRoomCompleter.complete();
    _getRoomCompleter = null;
    if (joinGamePacket.returnCode != 0) {
      throw Exception("Error during game join: ${joinGamePacket.debugMessage} (${joinGamePacket.returnCode})");
    }
    return ConnectionCredentials(joinGamePacket.params[ParameterCode.Address], joinGamePacket.params[ParameterCode.Secret]);
  }

  Future connectMatch(String roomId, ConnectionCredentials credentials) async {
    if (credentials == null) {
      if (_lobbySocket == null) {
        throw Exception("Tried to connect to a match without credentials, and without means of getting it.");
      }

      credentials = await getRoomCredentials(roomId);
    }

    _matchSocket = GameSocket.fromCredentials(credentials);
    _matchSocket.packets.listen((parsed) {
      if (parsed is OperationResponse && parsed.code == OperationCode.Authenticate) {
        print('auth');
        _matchSocket.add((OperationRequest(OperationCode.JoinGame, {
          ParameterCode.RoomName: roomId,
          ParameterCode.Broadcast: true,
          ParameterCode.PlayerProperties: {
            'teamNumber': SizedInt.byte(0),
            'rank': SizedInt.byte(100),
            'killstreak': SizedInt.byte(0),
            'characterCamo': SizedInt.byte(0),
            'unlockedweapons': ProtocolArray(DataType.Integer, [SizedInt.int(0x14200), SizedInt.int(0)]),
            'model': SizedInt.byte(1),
            'perks': List.generate(8, (i) => SizedInt.int(0)).toList(),
            // SizedInt.byte(255): Random.secure().nextInt(1000).toString(),
            SizedInt.byte(255): 'MyPlayerName',
          },
        })));
      }
      else if (parsed is Event && parsed.code == EventCode.Join) {
        var myActorId = (parsed.params[ParameterCode.ActorNr] as SizedInt).value;
        _matchSocket.add(OperationRequest(OperationCode.RaiseEvent, {
          ParameterCode.Code: SizedInt.byte(202),
          ParameterCode.Cache: SizedInt.byte(4),
          ParameterCode.Data: {
            SizedInt.byte(0): 'PlayerBody',
            SizedInt.byte(6): SizedInt.int(-41875289),
            SizedInt.byte(7): SizedInt.int(11001),
          },
        }));
        _matchSocket.add(OperationRequest(OperationCode.SetProperties, {
          ParameterCode.ActorNr: SizedInt.int(myActorId),
          ParameterCode.Broadcast: true,
          ParameterCode.Properties: {
            SizedInt.byte(255): 'MyPlayerName',
          },
        }));
        _matchSocket.add(OperationRequest(OperationCode.SetProperties, {
          ParameterCode.ActorNr: SizedInt.int(myActorId),
          ParameterCode.Broadcast: true,
          ParameterCode.Properties: {
            'teamNumber': SizedInt.byte(0),
            'rank': SizedInt.byte(100),
            'killstreak': SizedInt.byte(0),
            'characterCamo': SizedInt.byte(0),
            'unlockedweapons': ProtocolArray(DataType.Integer, [SizedInt.int(82432), SizedInt.int(0)]), //  ProtocolArray 105: [int32 82432, int32 0],
            'model': SizedInt.byte(1),
            'perks': List.generate(8, (i) => SizedInt.int(0)).toList(),
            // SizedInt.byte(255): Random.secure().nextInt(1000).toString(),
            SizedInt.byte(255): 'MyPlayerName',
          },
        }));
      }
    });
  }

  Future disconnectMatch() async => await _matchSocket.close();
}
