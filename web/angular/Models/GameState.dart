import 'PlayerState.dart';

class GameState {
  int actorNumber;
  Map<int, PlayerState> _players = {};

  GameState(this.actorNumber);

  PlayerState getMe() {
    return getPlayer(actorNumber);
  }

  PlayerState getPlayer(int actorNumber) {
    if (!_players.containsKey(actorNumber)) {
      _players[actorNumber] = PlayerState(actorNumber);
    }

    return _players[actorNumber];
  }
}