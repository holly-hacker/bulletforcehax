import 'dart:async';
import 'dart:core';
import 'dart:typed_data';

import 'package:bullet_force_hax/bullet_force_hax.dart';
import 'package:bullet_force_hax/src/utils/cancellable_interval_stream.dart';

import 'websock_creator.dart';

class Bot {
  static const endpointHost = "ns.exitgames.com";
  static const httpPort = 9093;
  static const httpsPort = 19093;
  static const protocol = "GpBinaryV16";
  static const applicationId = "8c2cad3e-2e3f-4941-9044-b390ff2c4956";
  static const applicationVersion = "1.34_WebGL_1.73";
  static const region = "us";

  String _address0, _address2;
  String _userId;
  String _secret0, _secret1, _secret2;
  String _roomName;

  DateTime _startTime = DateTime.now();
  DateTime _lastPing = DateTime.fromMillisecondsSinceEpoch(0);
  int get _tickCount => DateTime.now().difference(_startTime).inMilliseconds;
  int _serverTickOffset;

  Map<String, GameListItem> games = Map();

  Future connectInitial() async {
    var completer = Completer<void>();

    var firstWebSocket = await connectSocket(endpointHost, httpPort, protocol);
    firstWebSocket.handleError((e) {
      print('!!! Got error on first ws: $e');
    });
    firstWebSocket.map((data) => ProtocolReader(data).readPacket()).listen((parsed) async {
      if (parsed is InitResponse) {
        firstWebSocket.add(_quickSerialize(InternalOperationRequest(InternalOperationCode.Ping, {
          1: SizedInt.int(0)  // should be Environment.TickCount
        })));

        firstWebSocket.add(_quickSerialize(OperationRequest(OperationCode.Authenticate, {
          ParameterCode.AppVersion: applicationVersion,
          ParameterCode.ApplicationId: applicationId,
          ParameterCode.AzureNodeInfo: region,
        })));
      }
      else if (parsed is InternalOperationResponse && parsed.code == InternalOperationCode.Ping) {
        // param 1 = sent time, param 2 = server time offset
        var num1 = parsed.params[2];
        print('server0 tickbase: $num1');
      } else if (parsed is OperationResponse && parsed.code == OperationCode.Authenticate) {
        // var nickname = parsed.params[196];
        _address0 = parsed.params[ParameterCode.Address];
        _secret0 = parsed.params[ParameterCode.Secret];
        _userId = parsed.params[ParameterCode.UserId];

        // close socket, we have what we need
        await firstWebSocket.close();
        completer.complete();
      } else {
        assert(false);
        print('Received unknown packet on first ws: $parsed');
      }
    });
    return completer.future;
  }

  Future connectMain(bool Function(GameListItem) matcher) async {
    var split = _address0.split('://').last.split(':'); // format is ws://host:port
    var host = split[0];
    var port = int.parse(split[1]);
    var ws = await connectSocket(host, port, protocol);
    ws.handleError((e) {
      print('!!! Got error on ws: $e');
    });
    ws.map((data) => ProtocolReader(data).readPacket()).listen((parsed) async {
      if (ws.closeCode != null) {
        return;
      }

      if (parsed is InitResponse) {
        ws.add(_quickSerialize(_getPing()));

        ws.add(_quickSerialize(OperationRequest(OperationCode.Authenticate, {
          ParameterCode.Secret: _secret0,
        })));

      } else if (parsed is InternalOperationResponse && parsed.code == InternalOperationCode.Ping) {
        // param 1 = sent time, param 2 = server time offset
        var num1 = parsed.params[2] as SizedInt;
        _serverTickOffset = num1.value - _tickCount;
      } else if (parsed is OperationResponse && parsed.code == OperationCode.Authenticate) {
        // get more auth stuff?
        _secret1 = parsed.params[ParameterCode.Secret];

        ws.add(_quickSerialize(OperationRequest(OperationCode.JoinLobby, {})));
      } else if (parsed is Event && (parsed.code == EventCode.GameList || parsed.code == EventCode.GameListUpdate)) {
        var map = parsed.params[ParameterCode.GameList] as Map<Object, Object>;
        map.forEach((key, value) {
          assert(key is String);
          assert(value is Map);
          if ((value as Map).containsKey(SizedInt.byte(251))) {
            games.remove(key);
            print('removed game $key');
          } else {
            games[key] = GameListItem.fromMap(value);
          }
        });

        print('lobby update for ${map.length}/${games.length} games');

        if (games.values.any(matcher)) {
          // TODO: prevent this from triggering twice, if I care
          // TODO: prevent duplicate execution of matcher on 1 packet
          _roomName = games.entries.firstWhere((kvp) => matcher(kvp.value)).key;
          ws.add(_quickSerialize(OperationRequest(OperationCode.JoinGame, { ParameterCode.RoomName: _roomName })));
        }
      } else if (parsed is OperationResponse && parsed.code == OperationCode.JoinGame) {
        _secret2 = parsed.params[ParameterCode.Secret];
        _address2 = parsed.params[ParameterCode.Address];
        // there is also parameter 219 but idk if you need it
        await ws.close();
      } else {
        // assert(false);
        print('Received unhandled packet: $parsed');
      }
    });

    // start a loop so we can send packets at arbitrary times
    await for (var stop in CancellableIntervalStream.run(100)) {
      // exit when we found our match
      if (_address2 != null && _secret2 != null) {
        stop();
      }

      if (DateTime.now().difference(_lastPing).inMilliseconds > 1000) {
        ws.add(_quickSerialize(_getPing()));
        _lastPing = DateTime.now();
      }
    }

    await ws.close();
    return;
  }

  Future connectMatch() async {
    var split = _address2.split('://').last.split(':'); // format is ws://host:port
    var host = split[0];
    var port = int.parse(split[1]);
    var matchWebSocket = await connectSocket(host, port, protocol);
    var completer = Completer<void>();

    matchWebSocket.handleError((e) {
      print('!!! Got error on match ws: $e');
    });
    matchWebSocket.map((data) => ProtocolReader(data).readPacket()).listen((parsed) {
      if (parsed is InitResponse) {
        matchWebSocket.add(_quickSerialize(InternalOperationRequest(InternalOperationCode.Ping, {
          1: SizedInt.int(0)  // should be Environment.TickCount
        })));

        matchWebSocket.add(_quickSerialize(OperationRequest(OperationCode.Authenticate, {
          ParameterCode.Secret: _secret2
        })));
      }
      else if (parsed is OperationResponse && parsed.code == OperationCode.Authenticate) {
        print('auth');
        matchWebSocket.add(_quickSerialize(OperationRequest(OperationCode.JoinGame, {
          ParameterCode.RoomName: _roomName,
          ParameterCode.Broadcast: true,
          ParameterCode.PlayerProperties: {
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
        })));
      }
      else if (parsed is Event && parsed.code == EventCode.Join) {
        var myActorId = (parsed.params[ParameterCode.ActorNr] as SizedInt).value;
        matchWebSocket.add(_quickSerialize(OperationRequest(OperationCode.RaiseEvent, {
          ParameterCode.Code: SizedInt.byte(202),
          ParameterCode.Cache: SizedInt.byte(4),
          ParameterCode.Data: {
            SizedInt.byte(0): 'PlayerBody',
            SizedInt.byte(6): SizedInt.int(-41875289),
            SizedInt.byte(7): SizedInt.int(11001),
          },
        })));
        matchWebSocket.add(_quickSerialize(OperationRequest(OperationCode.SetProperties, {
          ParameterCode.ActorNr: SizedInt.byte(myActorId),
          ParameterCode.Broadcast: true,
          ParameterCode.Properties: {
            SizedInt.byte(255): 'MyPlayerName',
          },
        })));
        matchWebSocket.add(_quickSerialize(OperationRequest(OperationCode.SetProperties, {
          ParameterCode.ActorNr: SizedInt.byte(myActorId),
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
        })));
      }
      else {
        print('Received unknown packet on last ws: $parsed');
      }
    });

    // start a loop so we can send packets at arbitrary times
    await for (var stop in CancellableIntervalStream.run(100)) {
      if (DateTime.now().difference(_lastPing).inMilliseconds > 1000) {
        matchWebSocket.add(_quickSerialize(_getPing()));
        _lastPing = DateTime.now();
      }
    }

    return completer.future;
  }

  static Uint8List _quickSerialize(PacketWithPayload pwp) => (ProtocolWriter()..writePacket(pwp)).toBytes();
  PacketWithPayload _getPing() => InternalOperationRequest(InternalOperationCode.Ping, {1: SizedInt.int(_tickCount)});
}