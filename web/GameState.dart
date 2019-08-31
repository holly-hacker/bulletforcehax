import 'package:bullet_force_hax/src/protocol_reader/types/Vector3.dart';

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

class PlayerState {
  int actorNumber;
  int secondaryId;

  String name;
  int health;
  Vector3 position;

  int pitch;
  int yaw;
  int bodyYaw;

  double get x => position.f1;
  double get y => position.f2;
  double get z => position.f3;

  PlayerState(this.actorNumber);

  String toString() {
    return "Player $actorNumber '$name' at position {$x;$y;$z}";
  }
}
