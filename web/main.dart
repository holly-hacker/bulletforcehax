@JS()
library dart_main;

import 'dart:typed_data';

import 'package:bullet_force_hax/src/protocol_reader/ProtocolReader.dart';
import 'package:js/js.dart';

@JS()
external void startGame();

@JS()
external void hookWebSock(webSocketHookCallback cbSend, webSocketHookCallback cbRecv);
typedef void webSocketHookCallback(ByteBuffer data);

void main() {
  print('Hello, world!');
  hookWebSock(handlePacket, handlePacket);
  // startGame();
}

void handlePacket(ByteBuffer buffer) {
  var reader = ProtocolReader(buffer.asUint8List());
  var packet = reader.readPacket();
  print(packet);
}
