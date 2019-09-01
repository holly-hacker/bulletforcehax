import 'package:js/js.dart';
import 'package:angular/angular.dart';

import 'package:bullet_force_hax/bullet_force_hax.dart';
import 'angular/game_component.template.dart' as ng;
import 'js_interop.dart';

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
