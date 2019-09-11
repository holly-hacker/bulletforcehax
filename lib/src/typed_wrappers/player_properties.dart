import 'dart:typed_data';

import 'package:bullet_force_hax/bullet_force_hax.dart';
import 'package:collection/collection.dart';

class PlayerProperties {
  String name; // field 255
  int rank;
  int teamNumber;
  List<int> unlockedWeapons;
  int killStreak;
  Uint8List perks;
  int model;
  int characterCamo;

  PlayerProperties();

  PlayerProperties.initial() {
    name = "Player";
    rank = 1;
    teamNumber = 10;  // should be spectator
    unlockedWeapons = [0x14200, 0];
    killStreak = 0;
    perks = Uint8List(8);
    model = 1;
    characterCamo = 0;
  }

  PlayerProperties.fromMap(Map<Object, Object> map) {
    characterCamo = (map['characterCamo'] as SizedInt)?.value;
    unlockedWeapons = (map['unlockedweapons'] as ProtocolArray)?.data?.cast<SizedInt>()?.map((d) => d.value)?.toList();
    rank = (map['rank'] as SizedInt)?.value;
    killStreak = (map['killstreak'] as SizedInt)?.value;
    perks = map['perks'];
    teamNumber = (map['teamNumber'] as SizedInt)?.value;
    name = map[u8(255)];
    model = (map['model'] as SizedInt)?.value;
  }

  Map<Object, Object> toMap() {
    var map = Map<Object, Object>();
    map['characterCamo'] = u8(characterCamo);
    if (unlockedWeapons != null) map['unlockedweapons'] = ProtocolArray(DataType.Integer, unlockedWeapons.map((w) => s32(w)).toList());
    map['rank'] = u8(rank);
    map['killstreak'] = u8(killStreak);
    map['perks'] = perks;
    map['teamNumber'] = u8(teamNumber);
    map['model'] = u8(model);
    map[u8(255)] = name;
    return map;
  }

  int get hashCode {
    return name.hashCode ^ rank.hashCode ^ teamNumber.hashCode;
  }

  bool operator ==(other) {
    return identical(this, other)
            || other is PlayerProperties
            && other.name == name
            && other.rank == rank
            && other.teamNumber == teamNumber
            && ListEquality().equals(other.unlockedWeapons, unlockedWeapons)
            && other.killStreak == killStreak
            && ListEquality().equals(other.perks, perks)
            && other.model == model
            && other.characterCamo == characterCamo;
  }
}