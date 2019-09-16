import 'package:bullet_force_hax/bullet_force_hax.dart';

class PlayerState {
  int actorNumber;
  int secondaryId;

  String name;
  int health;
  Vector3 position;

  int pitch;
  int yaw;
  int bodyYaw;

  double get x => position?.f1;
  double get y => position?.f2;
  double get z => position?.f3;

  PlayerState(this.actorNumber);

  String toString() {
    return "Player $actorNumber '$name' at position {$x;$y;$z}";
  }
}
