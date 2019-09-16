import 'package:angular/angular.dart';
import 'package:angular_components/angular_components.dart';

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

  GameComponent(this.unityLoader);

  void startGame() {
    const urlOriginal = "https://server2.blayzegames.com/BulletForceWebGL/Build/BulletForceWebGL_New.json";
    const urlCrazyGames = "https://files.crazygames.com/bullet-force/9/Build/01650350c4b8491fe275c2ded00c5af5.json";
    var gameInstance = unityLoader.instantiate("gameContainer", urlOriginal, (x, progress) {
      print('progress update: $progress');
    });
  }
}
