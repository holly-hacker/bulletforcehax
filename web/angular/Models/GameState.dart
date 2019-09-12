import 'PlayerState.dart';

class GameState {
  int actorNumber;
  Map<int, PlayerState> players = {};
  PlayerState get me => getMe();

  GameState(this.actorNumber);

  PlayerState getMe() {
    return getPlayer(actorNumber);
  }

  PlayerState getPlayer(int actorNumber) {
    if (!players.containsKey(actorNumber)) {
      players[actorNumber] = PlayerState(actorNumber);
    }

    return players[actorNumber];
  }
}