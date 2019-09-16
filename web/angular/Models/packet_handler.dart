import 'dart:developer';
import 'dart:typed_data';

import 'package:bullet_force_hax/bullet_force_hax.dart';
import 'package:bullet_force_hax/src/typed_wrappers/player_properties.dart';

import '../Services/js_interop_service.dart';
import 'GameState.dart';
import 'PlayerState.dart';

class PacketHandler {
  GameState state;

  JsInteropService jsInterop;
  void Function(String s) get writeStatus => jsInterop.writeStatus;

  PacketHandler(this.jsInterop);

  List<ByteBuffer> handleBufferSend(ByteBuffer buffer) {
    var packet = ProtocolReader(buffer.asUint8List()).readPacket();
    var packets = handlePacketSend(packet);
    return packets.map((p) => (ProtocolWriter()..writePacket(p)).toBytes().buffer).toList();
  }

  ByteBuffer handleBufferRecv(ByteBuffer buffer) {
    var packet = ProtocolReader(buffer.asUint8List()).readPacket();
    var newPacket = handlePacketRecv(packet);
    return (ProtocolWriter()..writePacket(newPacket)).toBytes().buffer;
  }

  List<PacketWithPayload> handlePacketSend(PacketWithPayload packet) {
    if (packet is InternalOperationRequest) {
      // ignore
    }
    else if (packet is OperationRequest) {
      switch (packet.code) {
        case OperationCode.RaiseEvent: {
          var eventCode = packet.params[ParameterCode.Code];
          var eventData = packet.params[ParameterCode.CustomEventContent];

          if (eventCode is SizedInt && eventData is Map<Object, Object>) {
            switch (eventCode.value) {
              case 200:
                var code = eventData[u8(5)] as SizedInt;
                var data = eventData[u8(4)];
                if (code == null) {
                  // what
                }
                else if (code.value == 41) {
                  // shooting other 1
                  // var data2 = data as List<Object>;
                  // data2[1] = SizedFloat.float(13337);
                } else if (code.value == 10) {
                  // shooting other 2
                  // var data2 = data as List<Object>;
                  // data2[1] = SizedFloat.float(13337);
                  // data2[4] = SizedFloat.float(0); // health left?
                } else if (code.value == 26) {
                  // shooting other with RPG?
                  // var data2 = data as List<Object>;
                  // data2[1] = SizedFloat.float(13337);
                } else if (code.value == 25) {
                  // chat
                  var data2 = data as List<Object>;
                  data2[0] = '[hax] [Sandwich] [FuckYou] ' + data2[0].toString(); // author
                  // data[1] == message
                  data2[2] = s16(0xFF); // R
                  data2[3] = s16(0x69); // G
                  data2[4] = s16(0xB4); // B
                }
                print('>>> Event 200 code $code with data $data');
                return [packet];
              case 201:
                var data = eventData[s16(10)] as List<Object>;
                // buffer = (ProtocolWriter()..writePacket(packet)).toBytes().buffer;
                // writeStatus('Event 201: $data');
                print('>>> Event 201 Sending our player info $data');

                if (data is Map<Object, Object>) {
                  var player = state.getMe();
                  ApplyPacket201ToPlayer(player, data);
                }

                return [packet];
            }
          }
        }
      }
      print(packet);
    }
    else {
      debugger(message: 'packet shouldnt be here');
      print(packet);
    }

    return [packet];
  }

  PacketWithPayload handlePacketRecv(PacketWithPayload packet) {
    if (packet is InternalOperationResponse) {
      // ignore
    }
    else if (packet is Event) {
      switch (packet.code) {
        case EventCode.GameList:
        case EventCode.GameListUpdate:
          var gameList = packet.params[ParameterCode.GameList] as Map<Object, Object>;
          for (var value in gameList.keys) {
            var data = gameList[value] as Map<Object, Object>;
            if (data['password'] != '' && data['password'] != null && data['roomName'] != null) {
              print('Password-protected game "${data['roomName']}" has password "${data['password']}"');
              data['roomName'] = (data['roomName'] as String) + ' (password: ${data['password']})';
              data['password'] = null;
            }
          }
          return packet;
        case EventCode.AppStats:
          var masterPeerCount = packet.params[ParameterCode.MasterPeerCount];
          var gameCount = packet.params[ParameterCode.GameCount];
          var peerCount = packet.params[ParameterCode.PeerCount];
          print('Appstats: $gameCount games, $peerCount peers and $masterPeerCount master peers');
          break;
        case EventCode.Join:
          var actorNr = (packet.params[ParameterCode.ActorNr] as SizedInt).value;
          var player = packet.params[ParameterCode.PlayerProperties] as Map;
          var playerInfo = PlayerProperties.fromMap(player);  // this map could be empty!
          print('player joined: $playerInfo');
          state.getPlayer(actorNr); // just create for now
          break;
        case EventCode.Leave:
          var actorNr = (packet.params[ParameterCode.ActorNr] as SizedInt).value;
          // var actorList = (packet.params[ParameterCode.ActorList] as ProtocolArray).data.cast<SizedInt>();
          // actorList is likely the list of remaining players, including you
          state.removePlayer(actorNr);
          break;

        case 200:
          var actor = packet.params[ParameterCode.ActorNr];
          var eventData = packet.params[ParameterCode.CustomEventContent] as Map<Object, Object>;
          var code = (eventData[u8(5)] as SizedInt).value;
          var payload = eventData[u8(4)];  // can be null!
          print('<<< Event 200: actor $actor, code $code, payload $payload');

          if (payload is List<Object>) {
            switch (code) {
              case 10:
              // payload[1] = SizedFloat.float(0);
              // payload[4] = SizedFloat.float(50); // don't appear to receive damage
                break;
              case 24:
                if (state.actorNumber != null && payload[0] == s32(state.actorNumber)) {
                  // writeStatus("Fuck death");
                  // return OperationResponse(66, "", 0, {}); // When the server tell you that you died, just ignore it ;)
                }
                break;
            }
          }

          break;
        case 201:
          var actor = packet.params[ParameterCode.ActorNr] as SizedInt;
          var eventData = packet.params[ParameterCode.CustomEventContent] as Map<Object, Object>;
          var payload = eventData[s16(10)];  // can be null!
          print('<<< Event 201: actor $actor, payload $payload');

          if (payload is List<Object>) {
            var player = state.getPlayer(actor.value);
            ApplyPacket201ToPlayer(player, payload);
          }

          break;

        default:
          print(packet);
          break;
      }
    }
    else if (packet is OperationResponse) {
      print('<<< ' + packet.toString());

      if (packet.code == OperationCode.JoinGame && packet.params.containsKey(ParameterCode.ActorNr)) {
        // new game started
        var myActorNumber = (packet.params[ParameterCode.ActorNr] as SizedInt).value;
        state = GameState(myActorNumber);

        var players = packet.params[ParameterCode.PlayerProperties] as Map<Object, Object>;
        for (var playerId in players.keys.cast<SizedInt>()) {
          var player = state.getPlayer(playerId.value);
          var playerProps = players[playerId] as Map<Object, Object>;
          player.name = playerProps[u8(255)];
        }

        writeStatus('Our actor nr is ${state.actorNumber}');
      }
    }
    else {
      debugger(message: 'packet shouldnt be here');
      print(packet);
    }

    return packet;  // just return old value
  }

  void ApplyPacket201ToPlayer(PlayerState player, List<Object> payload) {
    player.secondaryId = (payload[0] as SizedInt).value;
    player.pitch = (payload[3] as SizedInt).value;
    player.yaw = (payload[4] as SizedInt).value;
    player.bodyYaw = (payload[5] as SizedInt).value;
    player.health = (payload[14] as SizedInt).value;
    player.position = (payload[21] as Vector3);
  }

}