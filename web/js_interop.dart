@JS()
library dart_main;

import 'dart:typed_data';

import 'package:js/js.dart';

typedef List<ByteBuffer> webSocketSendHookCallback(ByteBuffer data);
typedef ByteBuffer webSocketRecvHookCallback(ByteBuffer data);

@JS()
external void writeStatus(String s);

@JS()
external void hookWebSock(webSocketSendHookCallback cbSend, webSocketRecvHookCallback cbRecv);

@JS()
external void startGame();