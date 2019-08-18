@JS()
library dart_main;

import 'dart:typed_data';

import 'package:bullet_force_hax/src/protocol_reader/ProtocolReader.dart';
import 'package:bullet_force_hax/src/protocol_reader/ProtocolWriter.dart';
import 'package:bullet_force_hax/src/protocol_reader/constants.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/SizedInt.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/packets.dart';
import 'package:js/js.dart';

@JS()
external void startGame();

@JS()
external void hookWebSock(webSocketHookCallback cbSend, webSocketHookCallback cbRecv);
typedef ByteBuffer webSocketHookCallback(ByteBuffer data);

void main() {
  print('Hello, world!');
  hookWebSock(allowInterop(handlePacket), allowInterop(handlePacket));
  startGame();
}

ByteBuffer handlePacket(ByteBuffer buffer) {
  var reader = ProtocolReader(buffer.asUint8List());
  var packet = reader.readPacket();
  if (packet is InternalOperationRequest || packet is InternalOperationResponse) {
    // ignore
  } else if (packet is Event) {
    switch (packet.code) {
      case EventCode.GameList:
      case EventCode.GameListUpdate:
        var gameList = packet.params[ParameterCode.GameList] as Map<Object, Object>;
        for (var value in gameList.keys) {
          var data = gameList[value] as Map<Object, Object>;
          if (data['password'] != '' && data['password'] != null && data['roomName'] != null) {
            print('Password-protected game "${data['roomName']}" has password "${data['password']}"');
            data['roomName'] = (data['roomName'] as String) + ' (password: ${data['password']})';
          }
        }
        return (ProtocolWriter()..writePacket(packet)).toBytes().buffer;
      case EventCode.AppStats:
        var masterPeerCount = packet.params[ParameterCode.MasterPeerCount];
        var gameCount = packet.params[ParameterCode.GameCount];
        var peerCount = packet.params[ParameterCode.PeerCount];
        print('Appstats: $gameCount games, $peerCount peers and $masterPeerCount master peers');
        break;

      case 200:
        var actor = packet.params[ParameterCode.ActorNr];
        var eventData = packet.params[ParameterCode.CustomEventContent] as Map<Object, Object>;
        var code = (eventData[SizedInt.byte(5)] as SizedInt).value;
        var payload = eventData[SizedInt.byte(4)];  // can be null!
        print('<<< Event 200: actor $actor, code $code, payload $payload');
        break;
      case 201:
        var actor = packet.params[ParameterCode.ActorNr];
        var eventData = packet.params[ParameterCode.CustomEventContent] as Map<Object, Object>;
        var payload = eventData[SizedInt.short(10)];  // can be null!
        print('<<< Event 201: actor $actor, payload $payload');
        break;

      default:
        print(packet);
    }
  } else if (packet is OperationRequest) {
    if (packet.code == OperationCode.RaiseEvent) {
      var eventCode = packet.params[ParameterCode.Code];
      var eventData = packet.params[ParameterCode.CustomEventContent];

      if (eventCode is SizedInt && eventData is Map<Object, Object>) {
        switch (eventCode.value) {
          case 200:
            // custom game event. All in-game magic happens here
            var code = eventData[SizedInt.byte(5)];
            var data = eventData[SizedInt.byte(4)];
            print('>>> Event 200 code $code with data $data');
            return buffer;
          case 201:
            var data = eventData[SizedInt.short(10)] as List<Object>;
            // var data = data1 as List<Object>;
            // 11, 12, 13 are x, y, z
            print('>>> Event 201 Sending our player info $data');
            return buffer;
        }
      }
    }
    print(packet);
  } else {
    print(packet);
  }

  return buffer;  // just return old value
}
