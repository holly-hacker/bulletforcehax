@JS()
library dart_main;

import 'package:js/js.dart';

import 'PacketHandler.dart';

@JS()
external void startGame();

void main() {
  print('Hello, world!');
  doHook();
  startGame();
}
