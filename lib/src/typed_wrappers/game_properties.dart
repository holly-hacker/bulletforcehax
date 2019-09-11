import 'package:bullet_force_hax/bullet_force_hax.dart';
import 'package:bullet_force_hax/src/typed_wrappers/basic_game_info.dart';
import 'package:collection/collection.dart';

class GameProperties extends BasicGameInfo {
  String bannedWeaponMessage;
  int gunGamePreset;
  int hostId; // field 248
  bool field249;
  List<String> field250;
  bool field254;
  double matchCountdownTime;
  bool matchStarted;
  int maxPing;
  bool roundStarted;
  int scoreLimit;
  double timeScale;

  GameProperties();

  GameProperties.initial() {
    field253 = true;
    field254 = false;
    field250 = ['roomName', 'mapName', 'modeName', 'password', 'dedicated', 'switchingmap', 'allowedweapons', 'eventcode', 'averagerank'];
    roomName = "My Room Name";
    mapName = "Urban";
    modeName = "Conquest";
    password = "My Password";
    roundStarted = false;
    maxPing = 700;
    timeScale = 1;
    dedicated = false;
    scoreLimit = 200;
    gunGamePreset = 0;
    matchCountdownTime = 0;
    matchStarted = false;
    switchingMap = false;
    allowedWeapons = [-1, -1];
    bannedWeaponMessage = "This message should never appear!";
    eventCode = 0;
    averageRank = 1;
    maxPlayerCount = 10;
    field249 = true;
    // NOTE: hostId (field 248) is not set on match creation, so we're not setting it here
  }

  GameProperties.fromMap(Map<Object, Object> map) : super.fromMap(map) {
    bannedWeaponMessage = map['bannedweaponmessage'];
    gunGamePreset = (map['gunGamePreset'] as SizedInt)?.value;
    hostId = (map[SizedInt.byte(248)] as SizedInt)?.value;
    field249 = map[SizedInt.byte(249)];
    field250 = (map[SizedInt.byte(250)] as ProtocolArray)?.data?.cast<String>();
    field254 = map[SizedInt.byte(254)];
    matchCountdownTime = (map['matchCountdownTime'] as SizedFloat)?.value;
    matchStarted = map['matchStarted'];
    maxPing = (map['maxPing'] as SizedInt)?.value;
    roundStarted = map['roundStarted'];
    scoreLimit = (map['scorelimit'] as SizedInt)?.value;
    timeScale = (map['timeScale'] as SizedFloat)?.value;
  }

  Map<Object, Object> toMap() {
    var map = super.toMap();
    map['bannedweaponmessage'] = bannedWeaponMessage;
    map['gunGamePreset'] = SizedInt.int(gunGamePreset);
    if (hostId != null) map[SizedInt.byte(248)] = SizedInt.int(hostId); // TODO: use i32 function
    map[SizedInt.byte(249)] = field249;
    map[SizedInt.byte(250)] = ProtocolArray(DataType.String, field250);
    map[SizedInt.byte(254)] = field254;
    map['matchCountdownTime'] = SizedFloat.float(matchCountdownTime);
    map['matchStarted'] = matchStarted;
    map['maxPing'] = SizedInt.short(maxPing);
    map['roundStarted'] = roundStarted;
    map['scorelimit'] = SizedInt.int(scoreLimit);
    map['timeScale'] = SizedFloat.float(timeScale);
    return map;
  }

  bool operator ==(other) => equals(other);

  bool equals(other) {
    return identical(this, other)
        || other is GameProperties
        && super.equals(other)
        && other.bannedWeaponMessage == bannedWeaponMessage
        && other.gunGamePreset == gunGamePreset
        && other.hostId == hostId
        && other.field249 == field249
        && DeepCollectionEquality().equals(other.field250, field250)
        && other.field254 == field254
        && other.matchCountdownTime == matchCountdownTime
        && other.matchStarted == matchStarted
        && other.maxPing == maxPing
        && other.roundStarted == roundStarted
        && other.scoreLimit == scoreLimit
        && other.timeScale == timeScale;
  }
}