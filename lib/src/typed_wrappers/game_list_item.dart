import 'package:bullet_force_hax/src/typed_wrappers/basic_game_info.dart';

class GameListItem extends BasicGameInfo {
  String roomId;

  GameListItem() : super();

  GameListItem.fromMap(this.roomId, Map<Object, Object> map) : super.fromMap(map);
}