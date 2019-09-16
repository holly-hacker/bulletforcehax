@JS('UnityLoader')
library unity_loader;

import 'package:js/js.dart';

// returns gameInstance
@JS("instantiate")
external Object _instantiate(String container, String url, InstantiateOptions options);

typedef _onProgressDefinition = void Function(Object, double);

@JS()
@anonymous
class InstantiateOptions {
  external _onProgressDefinition get onProgress;

  external factory InstantiateOptions({Function onProgress});
}

class UnityLoaderService {
  Object instantiate(String container, String url, _onProgressDefinition onProgress)
    => _instantiate(container, url, InstantiateOptions(onProgress: allowInterop(onProgress)));
}
