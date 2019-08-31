import 'package:angular/angular.dart';

@Component(
  selector: 'my-app',
  template: '''
    <div class="webgl-content">
        <span id="status" style="font-family: monospace"></span>
        <div id="gameContainer" style="width: 960px; height: 540px"></div>
        <div class="footer">
            <div class="webgl-logo"></div>
            <div class="fullscreen" onclick="gameInstance.SetFullscreen(1)"></div>
            <div class="title">{{name}}</div>
        </div>
    </div>
  ''',
)
class GameComponent {
  var name = 'Bullet Force WebGL';
}
