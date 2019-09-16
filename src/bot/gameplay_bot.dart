import 'package:bullet_force_hax/bullet_force_hax.dart';
import 'package:bullet_force_hax/src/typed_wrappers/game_properties.dart';
import 'package:bullet_force_hax/src/typed_wrappers/player_properties.dart';

import 'connection_details.dart';
import 'gamesocket.dart';

class GameplayBot {
  GameSocket _matchSocket;
  int _ourActorNr;  // int32
  int get ourActorId => _ourActorNr * 1000 + 1;
  PlayerProperties _ourPlayer = PlayerProperties.initial();
  Map<int, PlayerProperties> _otherPlayers;
  GameProperties _gameProps;

  Future connectMatch(ConnectionCredentials credentials, [GameProperties newGameProps]) async {
    assert(credentials.hasSecret);
    assert(credentials.hasRoomId);

    _matchSocket = GameSocket.fromCredentials(credentials);
    _matchSocket.packets.listen((parsed) async {
      if (parsed is OperationResponse && parsed.code == OperationCode.Authenticate) {
        print('auth');
        if (newGameProps != null) {
          await _matchSocket.add((OperationRequest(OperationCode.CreateGame, {
            ParameterCode.RoomName: credentials.roomId,
            ParameterCode.PlayerProperties: {u8(255): ''}, // what
            ParameterCode.Broadcast: true,
            ParameterCode.GameProperties: newGameProps.toMap(),
            ParameterCode.CleanupCacheOnLeave: true,
          })));
        }
        else {
          await _matchSocket.add(OperationRequest(OperationCode.JoinGame, {
            ParameterCode.RoomName: credentials.roomId,
            ParameterCode.Broadcast: true,
            ParameterCode.PlayerProperties: _ourPlayer.toMap(),
          }));
        }
      }
      else if (parsed is OperationResponse && parsed.code == OperationCode.CreateGame) {
        _ourActorNr = (parsed.params[ParameterCode.ActorNr] as SizedInt).value;

        var gameProps = parsed.params[ParameterCode.GameProperties] as Map;
        _gameProps = GameProperties.fromMap(gameProps);
      }
      else if (parsed is OperationResponse && parsed.code == OperationCode.JoinGame) {
        _ourActorNr = (parsed.params[ParameterCode.ActorNr] as SizedInt).value;

        var playerPropsMap = parsed.params[ParameterCode.PlayerProperties] as Map;
        _otherPlayers = Map<int, PlayerProperties>();
        _otherPlayers.addAll(playerPropsMap.map((k, v) => MapEntry((k as SizedInt).value, PlayerProperties.fromMap(v))));

        var gameProps = parsed.params[ParameterCode.GameProperties] as Map;
        _gameProps = GameProperties.fromMap(gameProps);

        // won't act on this packet, Event Join will join right after
      }
      else if (parsed is Event && parsed.code == EventCode.Join) {
        var myActorId = (parsed.params[ParameterCode.ActorNr] as SizedInt).value;
        await _matchSocket.add(OperationRequest(OperationCode.RaiseEvent, {
          ParameterCode.Code: u8(202),
          ParameterCode.Cache: u8(4),
          ParameterCode.Data: {
            u8(0): 'PlayerBody',
            u8(6): s32(-41875289),
            u8(7): s32(myActorId * 1000 + 1), // this value can crash other clients
          },
        }));
        await _matchSocket.add(OperationRequest(OperationCode.SetProperties, {
          ParameterCode.ActorNr: s32(myActorId),
          ParameterCode.Broadcast: true,
          ParameterCode.Properties: {
            u8(255): _ourPlayer.name,
          },
        }));
        await _matchSocket.add(OperationRequest(OperationCode.SetProperties, {
          ParameterCode.ActorNr: s32(myActorId),
          ParameterCode.Broadcast: true,
          ParameterCode.Properties: _ourPlayer.toMap(),
        }));
      }
    });
  }

  Future disconnectMatch() async => await _matchSocket.close();
}