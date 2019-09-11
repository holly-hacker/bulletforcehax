import 'package:bullet_force_hax/src/typed_wrappers/basic_game_info.dart';

import '../../bullet_force_hax.dart';

class ListedGameInfo extends BasicGameInfo {
  int playerCount;    // field 252

  ListedGameInfo();

  ListedGameInfo.fromMap(Map<Object, Object> map) : super.fromMap(map) {
    playerCount = (map[u8(252)] as SizedInt)?.value;
  }

  Map<Object, Object> toMap() {
    var map = super.toMap();
    map[u8(252)] = u8(playerCount);
    return map;
  }

  String toString() {
    return "Match $roomName ($modeName) on $mapName, $playerCount/$maxPlayerCount players";
  }

  int get hashCode {
    return super.hashCode ^ playerCount;
  }

  bool operator ==(other) => equals(other);

  bool equals(other) {
    return identical(this, other)
        || other is ListedGameInfo
        && super.equals(other)
        && other.playerCount == playerCount;
  }
}