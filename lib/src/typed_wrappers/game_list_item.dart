import 'listed_game_info.dart';

class GameListItem extends ListedGameInfo {
  String roomId;

  GameListItem() : super();

  GameListItem.fromMap(this.roomId, Map<Object, Object> map) : super.fromMap(map);
}