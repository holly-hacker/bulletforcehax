@JS()
library dart_main;

import 'package:js/js.dart';
import 'package:angular/angular.dart';
import 'package:bullet_force_hax/src/angular/game_component.template.dart' as ng;

import 'PacketHandler.dart';

@JS()
external void startGame();

void main() {
  print('Hello, world!');
  doHook();
  runApp(ng.GameComponentNgFactory);
  startGame();
}
