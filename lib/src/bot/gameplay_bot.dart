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

  Future connectMatch(String roomId, ConnectionCredentials credentials) async {
    _matchSocket = GameSocket.fromCredentials(credentials);
    _matchSocket.packets.listen((parsed) async {
      if (parsed is OperationResponse && parsed.code == OperationCode.Authenticate) {
        print('auth');
        await _matchSocket.add((OperationRequest(OperationCode.JoinGame, {
          ParameterCode.RoomName: roomId,
          ParameterCode.Broadcast: true,
          ParameterCode.PlayerProperties: _ourPlayer.toMap(),
        })));
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
          ParameterCode.Code: SizedInt.byte(202),
          ParameterCode.Cache: SizedInt.byte(4),
          ParameterCode.Data: {
            SizedInt.byte(0): 'PlayerBody',
            SizedInt.byte(6): SizedInt.int(-41875289),
            SizedInt.byte(7): SizedInt.int(myActorId * 1000 + 1), // this value can crash other clients
          },
        }));
        await _matchSocket.add(OperationRequest(OperationCode.SetProperties, {
          ParameterCode.ActorNr: SizedInt.int(myActorId),
          ParameterCode.Broadcast: true,
          ParameterCode.Properties: {
            SizedInt.byte(255): _ourPlayer.name,
          },
        }));
        await _matchSocket.add(OperationRequest(OperationCode.SetProperties, {
          ParameterCode.ActorNr: SizedInt.int(myActorId),
          ParameterCode.Broadcast: true,
          ParameterCode.Properties: _ourPlayer.toMap(),
        }));
      }
    });
  }

  Future disconnectMatch() async => await _matchSocket.close();
}