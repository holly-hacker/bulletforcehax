import 'package:angular/angular.dart';

import '../Services/js_interop_service.dart';
import 'game_component.dart';

@Component(
    selector: 'my-app',
    template: '<game></game>',
    directives: [
      GameComponent
    ],
    providers: [ClassProvider(JsInteropService)]
)
class AppComponent {
}
