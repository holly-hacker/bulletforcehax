@JS()
library dart_main;

import 'dart:typed_data';

import 'package:js/js.dart';
import 'package:angular/angular.dart';
import 'package:bullet_force_hax/src/angular/game_component.template.dart' as ng;

import 'package:bullet_force_hax/bullet_force_hax.dart';

typedef List<ByteBuffer> webSocketSendHookCallback(ByteBuffer data);
typedef ByteBuffer webSocketRecvHookCallback(ByteBuffer data);

@JS()
external void writeStatus(String s);

@JS()
external void hookWebSock(webSocketSendHookCallback cbSend, webSocketRecvHookCallback cbRecv);

@JS()
external void startGame();

void main() {
  print('Hello, world!');
  doHook();
  runApp(ng.GameComponentNgFactory);
  startGame();
}

void doHook() {
  var handler = PacketHandler(writeStatus);
  hookWebSock(allowInterop(handler.handleBufferSend), allowInterop(handler.handleBufferRecv));
}
