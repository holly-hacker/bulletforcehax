import 'package:angular/angular.dart';

import '../Services/js_interop_service.dart';
import 'game_container_component.dart';

@Component(
    selector: 'my-app',
    template: '<game-container></game-container>',
    directives: [
      GameContainerComponent
    ],
    providers: [ClassProvider(JsInteropService)]
)
class AppComponent {
}
