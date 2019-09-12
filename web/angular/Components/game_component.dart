import 'package:angular/angular.dart';
import 'package:angular_components/angular_components.dart';

import '../Services/js_interop_service.dart';

@Component(
  selector: 'game',
  templateUrl: 'game_component.html',
  directives: [
    MaterialButtonComponent,
    MaterialIconComponent
  ]
)
class GameComponent {
  var name = 'Bullet Force Hax';
  JsInteropService jsInterop;

  GameComponent(this.jsInterop);

  void startGame() {
    jsInterop.startGame();
  }
}
