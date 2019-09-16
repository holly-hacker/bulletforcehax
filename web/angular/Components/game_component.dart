import 'package:angular/angular.dart';
import 'package:angular_components/angular_components.dart';

import '../Models/game_instance.dart';
import '../Services/js_interop_service.dart';
import '../Services/unity_loader_service.dart';

@Component(
  selector: 'game',
  templateUrl: 'game_component.html',
  styleUrls: ['game_component.css'],
  providers: [ClassProvider(UnityLoaderService)],
  directives: [
    MaterialButtonComponent,
    MaterialIconComponent
  ]
)
class GameComponent {
  UnityLoaderService unityLoader;
  JsInteropService jsInterop;
  GameInstance instance;

  GameComponent(this.unityLoader, this.jsInterop);

  void startGame([bool useCrazyGamesBuild = false]) {
    instance = unityLoader.instantiate("gameContainer", _getUrl(useCrazyGamesBuild), (instance, progress) {
      print('progress update: $progress');
    });

    // allow for easier debugging in browser devtools
    jsInterop.gameInstance = instance;
  }

  String _getUrl(bool useCrazyGamesBuild) {
    if (useCrazyGamesBuild) {
      return "https://files.crazygames.com/bullet-force/9/Build/01650350c4b8491fe275c2ded00c5af5.json";
    }
    else {
      return "https://server2.blayzegames.com/BulletForceWebGL/Build/BulletForceWebGL_New.json";
    }
  }
}
