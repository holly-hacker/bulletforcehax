@JS()
library dart_main;

import 'dart:typed_data';

import 'package:bullet_force_hax/src/protocol_reader/ProtocolReader.dart';
import 'package:bullet_force_hax/src/protocol_reader/ProtocolWriter.dart';
import 'package:bullet_force_hax/src/protocol_reader/constants.dart';
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
      default: print(packet);
    }
  } else {
    print(packet);
  }

  return buffer;  // just return old value
}
