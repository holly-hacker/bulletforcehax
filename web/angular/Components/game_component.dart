import 'package:angular/angular.dart';
import 'package:angular_components/angular_components.dart';

import '../Services/js_interop_service.dart';

@Component(
  selector: 'game',
  templateUrl: 'game_component.html',
  styleUrls: ['game_component.css'],
  directives: [
    MaterialButtonComponent,
    MaterialIconComponent
  ]
)
class GameComponent {
  JsInteropService jsInterop;

  GameComponent(this.jsInterop);

  void startGame() {
    jsInterop.startGame();
  }
}
