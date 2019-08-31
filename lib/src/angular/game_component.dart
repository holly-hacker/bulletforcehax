import 'package:angular/angular.dart';
import 'package:angular_components/angular_components.dart';

@Component(
  selector: 'my-app',
  templateUrl: 'game_component.html',
  directives: [
    MaterialButtonComponent,
    MaterialIconComponent
  ]
)
class GameComponent {
  var name = 'Bullet Force Hax';
}
