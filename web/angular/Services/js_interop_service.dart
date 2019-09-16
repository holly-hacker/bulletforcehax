@JS()
library dart_main;

import 'dart:typed_data';

import 'package:js/js.dart';

import '../Models/game_instance.dart';

typedef List<ByteBuffer> webSocketSendHookCallback(ByteBuffer data);
typedef ByteBuffer webSocketRecvHookCallback(ByteBuffer data);

@JS("hookWebSock")
external void _hookWebSock(webSocketSendHookCallback cbSend, webSocketRecvHookCallback cbRecv);

@JS('gameInstance')
external set _gameInstance(GameInstance inst);

class JsInteropService {
  void hookWebSock(webSocketSendHookCallback cbSend, webSocketRecvHookCallback cbRecv) => _hookWebSock(allowInterop(cbSend), allowInterop(cbRecv));
  set gameInstance(GameInstance inst) => _gameInstance = inst;
}
