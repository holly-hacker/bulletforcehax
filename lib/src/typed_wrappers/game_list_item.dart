import 'package:bullet_force_hax/bullet_force_hax.dart';

class GameListItem {  // TODO: GameProperties extends from this?
  String roomName;
  String password;
  String modeName;
  String mapName;
  int playerCount;    // field 252
  int get maxPlayerCount => _field255 + 1; // field 255
  int _field255;
  int averageRank;
  List<int> allowedWeapons; // 64bit bit field
  bool switchingMap;
  bool dedicated;
  int eventCode;
  bool field253;  // TODO: what is this?

  GameListItem.fromMap(Map<Object, Object> map) {
    field253 = map[SizedInt.byte(253)];
    if ((map[SizedInt.byte(252)] as SizedInt) == null) {
      print('what');
    }
    playerCount = (map[SizedInt.byte(252)] as SizedInt).value;
    modeName = map['modeName'];
    averageRank = (map['averagerank'] as SizedInt).value;
    switchingMap = map['switchingmap'];
    roomName = map['roomName'];
    allowedWeapons = (map['allowedweapons'] as ProtocolArray).data.cast<SizedInt>().map((d) => d.value).toList();
    eventCode = (map['eventcode'] as SizedInt).value;
    dedicated = map['dedicated'];
    password = map['password'];
    mapName = map['mapName'];
    _field255 = (map[SizedInt.byte(255)] as SizedInt).value;
  }

  String toString() {
    return "Match $roomName ($modeName) on $mapName, $playerCount/$maxPlayerCount players";
  }
}