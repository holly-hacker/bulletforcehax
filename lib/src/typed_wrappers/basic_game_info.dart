import 'dart:core';

import 'package:bullet_force_hax/bullet_force_hax.dart';
import 'package:collection/collection.dart';

abstract class BasicGameInfo {  // GameProperties extends from this?
  String roomName;
  String password;
  String modeName;
  String mapName;
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
    field253 = map[u8(253)];
    modeName = map['modeName'];
    averageRank = (map['averagerank'] as SizedInt)?.value;
    switchingMap = map['switchingmap'];
    roomName = map['roomName'];
    allowedWeapons = (map['allowedweapons'] as ProtocolArray)?.data?.cast<SizedInt>()?.map((d) => d.value)?.toList();
    eventCode = (map['eventcode'] as SizedInt)?.value;
    dedicated = map['dedicated'];
    password = map['password'];
    mapName = map['mapName'];
    _field255 = (map[u8(255)] as SizedInt)?.value;
  }

  Map<Object, Object> toMap() {
    var map = Map<Object, Object>();
    map[u8(253)] = field253;
    map['modeName'] = modeName;
    map['averagerank'] = s32(averageRank);
    map['switchingmap'] = switchingMap;
    map['roomName'] = roomName;
    if (allowedWeapons != null) map['allowedweapons'] = ProtocolArray(DataType.Integer, allowedWeapons.map((i) => s32(i)).toList());
    map['eventcode'] = s32(eventCode);
    map['dedicated'] = dedicated;
    map['password'] = password;
    map['mapName'] = mapName;
    map[u8(255)] = u8(_field255);
    return map;
  }

  String toString() {
    return "Match $roomName ($modeName) on $mapName, max $maxPlayerCount players";
  }

  int get hashCode {
    return roomName.hashCode ^ password.hashCode ^ modeName.hashCode
    ^ mapName.hashCode ^  maxPlayerCount.hashCode;
  }

  bool operator ==(other) => this.equals(other);

  bool equals(other) {
    return identical(this, other)
        || other is BasicGameInfo
            && other.roomName == roomName
            && other.password == password
            && other.modeName == modeName
            && other.mapName == mapName
            && other._field255 == _field255
            && other.averageRank == averageRank
            && DeepCollectionEquality().equals(other.allowedWeapons, allowedWeapons)
            && other.switchingMap == switchingMap
            && other.dedicated == dedicated
            && other.eventCode == eventCode
            && other.field253 == field253;
  }
}