import 'package:angular/angular.dart';
import 'package:angular_components/angular_components.dart';

import '../Services/js_interop_service.dart';
import '../Services/packet_handler_service.dart';

@Component(
  selector: 'game',
  templateUrl: 'game_component.html',
  directives: [
    MaterialButtonComponent,
    MaterialIconComponent
  ]
)
class GameComponent implements OnInit {
  var name = 'Bullet Force Hax';

  JsInteropService jsInterop;
  PacketHandlerService handler;

  GameComponent(this.jsInterop, this.handler);

  void ngOnInit() {
    jsInterop.hookWebSock(handler.handleBufferSend, handler.handleBufferRecv);
    jsInterop.startGame();
  }
}
