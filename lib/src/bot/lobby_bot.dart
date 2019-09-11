import 'dart:async';

import 'package:bullet_force_hax/bullet_force_hax.dart';
import 'package:synchronized/synchronized.dart';

import 'connection_details.dart';
import 'gamesocket.dart';

class LobbyBot {
  Map<String, GameListItem> games = Map();

  StreamController<GameListItem> _gamesController = StreamController<GameListItem>();
  Stream<GameListItem> get gamesStream => _gamesController.stream;  // TODO: implement pause/resume?
  StreamController<GameListItem> _newGamesController = StreamController<GameListItem>();
  Stream<GameListItem> get newGamesStream => _newGamesController.stream;  // TODO: implement pause/resume?

  GameSocket _lobbySocket;

  var _roomJoinLock = Lock();
  var _roomCreateLock = Lock();

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
        await _lobbySocket.add(OperationRequest(OperationCode.JoinLobby, {}));
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
    OperationResponse joinGamePacket;

    await _roomJoinLock.synchronized(() async {
      await _lobbySocket.add(OperationRequest(OperationCode.JoinGame, { ParameterCode.RoomName: roomId }));

      joinGamePacket = await _lobbySocket.packets.firstWhere((packet) => packet is OperationResponse && packet.code == OperationCode.JoinGame) as OperationResponse;
    });

    if (joinGamePacket.returnCode != 0) {
      throw Exception("Error during game join: ${joinGamePacket.debugMessage} (${joinGamePacket.returnCode})");
    }
    return ConnectionCredentials(joinGamePacket.params[ParameterCode.Address], joinGamePacket.params[ParameterCode.Secret], roomId);
  }

  Future<ConnectionCredentials> createMatch() async {
    OperationResponse joinGamePacket;

    await _roomCreateLock.synchronized(() async {
      await _lobbySocket.add(OperationRequest(OperationCode.CreateGame, {}));

      joinGamePacket = await _lobbySocket.packets.firstWhere((packet) => packet is OperationResponse && packet.code == OperationCode.CreateGame) as OperationResponse;
    });

    if (joinGamePacket.returnCode != 0) {
      throw Exception("Error during game join: ${joinGamePacket.debugMessage} (${joinGamePacket.returnCode})");
    }
    return ConnectionCredentials(joinGamePacket.params[ParameterCode.Address], joinGamePacket.params[ParameterCode.Secret], joinGamePacket.params[ParameterCode.RoomName]);
  }
}