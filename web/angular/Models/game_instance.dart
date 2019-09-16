@JS()
library game_instance_js;

import 'dart:html';

import 'package:js/js.dart';

@JS()
class GameInstance {
  // for some reason I can't create a nested Module class :(
  @JS() Element container;
  @JS() String url;

  @JS() external void popup(String str, [List<PopupCallback> callbacks]);
  @JS() external void SetFullscreen();
}

@JS()
@anonymous
class PopupCallback {
  external String get text;
  external void Function(MouseEvent) get callback;

  external factory PopupCallback({String text, void Function(MouseEvent) callback});
}

