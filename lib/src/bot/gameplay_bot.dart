import 'package:bullet_force_hax/bullet_force_hax.dart';

import 'connection_details.dart';
import 'gamesocket.dart';

class GameplayBot {
  GameSocket _matchSocket;

  Future connectMatch(String roomId, ConnectionCredentials credentials) async {
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
            SizedInt.byte(7): SizedInt.int(myActorId * 1000 + 1), // this value can crash other clients
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