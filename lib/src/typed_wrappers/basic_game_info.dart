import 'dart:core';

import 'package:bullet_force_hax/bullet_force_hax.dart';
import 'package:collection/collection.dart';

class BasicGameInfo {  // GameProperties extends from this?
  String roomName;
  String password;
  String modeName;
  String mapName;
  int playerCount;    // field 252, TODO: not present in GameProperties
  int get maxPlayerCount => _field255 + 1;
  set maxPlayerCount(int val) => _field255 = val - 1;
  int _field255; // field 255
  int averageRank;
  List<int> allowedWeapons; // 64bit bit field
  bool switchingMap;
  bool dedicated;
  int eventCode;
  bool field253;  // TODO: what is this?

  BasicGameInfo();

  BasicGameInfo.fromMap(Map<Object, Object> map) {
    field253 = map[SizedInt.byte(253)];
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

  Map<Object, Object> toMap() {
    var map = Map<Object, Object>();
    map[SizedInt.byte(253)] = field253;
    map[SizedInt.byte(252)] = SizedInt.byte(playerCount);
    map['modeName'] = modeName;
    map['averagerank'] = SizedInt.int(averageRank);
    map['switchingmap'] = switchingMap;
    map['roomName'] = roomName;
    map['allowedweapons'] = ProtocolArray(DataType.Integer, allowedWeapons.map((i) => SizedInt.int(i)).toList());
    map['eventcode'] = SizedInt.int(eventCode);
    map['dedicated'] = dedicated;
    map['password'] = password;
    map['mapName'] = mapName;
    map[SizedInt.byte(255)] = SizedInt.byte(_field255);
    return map;
  }

  String toString() {
    return "Match $roomName ($modeName) on $mapName, $playerCount/$maxPlayerCount players";
  }

  int get hashCode {
    return roomName.hashCode ^ password.hashCode ^ modeName.hashCode
    ^ mapName.hashCode ^ playerCount.hashCode ^ maxPlayerCount.hashCode;
  }

  @override
  bool operator ==(other) {
    return identical(this, other)
        || other is BasicGameInfo
        && other.roomName == roomName
        && other.password == password
        && other.modeName == modeName
        && other.mapName == mapName
        && other.playerCount == playerCount
        && other._field255 == _field255
        && other.averageRank == averageRank
        && DeepCollectionEquality().equals(other.allowedWeapons, allowedWeapons)
        && other.switchingMap == switchingMap
        && other.dedicated == dedicated
        && other.eventCode == eventCode
        && other.field253 == field253;
  }
}