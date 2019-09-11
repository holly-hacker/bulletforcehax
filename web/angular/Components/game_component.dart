import 'package:angular/angular.dart';
import 'package:angular_components/angular_components.dart';

import '../Models/packet_handler.dart';
import '../Services/js_interop_service.dart';

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
  PacketHandler handler;

  GameComponent(this.jsInterop) {
    handler = PacketHandler(jsInterop);
  }

  void ngOnInit() {
    jsInterop.hookWebSock(handler.handleBufferSend, handler.handleBufferRecv);
    jsInterop.startGame();
  }
}
