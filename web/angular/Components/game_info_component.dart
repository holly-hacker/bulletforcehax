import 'package:angular/angular.dart';

import '../Models/GameState.dart';

@Component(
  selector: 'game-info',
  template: """
  <div *ngIf="state != null">
    {{ state.players.length }} players:
    <ul>
      <li *ngFor='let player of state.players.values'>{{ player }}</li>
    </ul>
  </div>
  """,
  directives: [
    NgIf,
    NgFor,
  ],
)
class GameInfoComponent {
  @Input() GameState state;
}