import 'package:angular/angular.dart';

import '../Models/packet_handler.dart';
import '../Services/js_interop_service.dart';
import 'game_component.dart';
import 'game_info_component.dart';

@Component(
  selector: 'game-container',
  template: '<game></game> <game-info [state]="handler.state"></game-info>',
  directives: [
    GameComponent,
    GameInfoComponent,
  ],
)
class GameContainerComponent implements OnInit {
  JsInteropService jsInterop;
  PacketHandler handler = PacketHandler();

  @ViewChild(GameComponent)
  GameComponent game;

  GameContainerComponent(this.jsInterop);

  void ngOnInit() {
    jsInterop.hookWebSock(handler.handleBufferSend, handler.handleBufferRecv);
    game.startGame();
  }
}
