@JS()
library dart_main;

import 'package:js/js.dart';

import 'package:bullet_force_hax/bullet_force_hax.dart';

@JS()
external void startGame();

void main() {
  print('Hello, world!');
  doHook();
  startGame();
}
