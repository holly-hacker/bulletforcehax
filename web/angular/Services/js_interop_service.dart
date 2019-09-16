@JS()
library dart_main;

import 'dart:typed_data';

import 'package:js/js.dart';

typedef List<ByteBuffer> webSocketSendHookCallback(ByteBuffer data);
typedef ByteBuffer webSocketRecvHookCallback(ByteBuffer data);

@JS("hookWebSock")
external void _hookWebSock(webSocketSendHookCallback cbSend, webSocketRecvHookCallback cbRecv);

@JS("startGame")
external void _startGame();

class JsInteropService {
  void hookWebSock(webSocketSendHookCallback cbSend, webSocketRecvHookCallback cbRecv) => _hookWebSock(allowInterop(cbSend), allowInterop(cbRecv));

  void startGame() => _startGame();
}
