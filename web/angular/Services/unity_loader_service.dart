@JS('UnityLoader')
library unity_loader;

import 'package:js/js.dart';

import '../Models/game_instance.dart';

// returns gameInstance
@JS("instantiate")
external GameInstance _instantiate(String container, String url, InstantiateOptions options);

typedef _onProgressDefinition = void Function(GameInstance, double);

@JS()
@anonymous
class InstantiateOptions {
  external _onProgressDefinition get onProgress;

  external factory InstantiateOptions({Function onProgress});
}

class UnityLoaderService {
  GameInstance instantiate(String container, String url, _onProgressDefinition onProgress)
    => _instantiate(container, url, InstantiateOptions(onProgress: allowInterop(onProgress)));
}
