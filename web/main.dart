@JS()
library dart_main;

import 'dart:typed_data';

import 'package:bullet_force_hax/bullet_force_hax.dart';
import 'package:js/js.dart';

@JS()
external void startGame();

@JS()
external void hookWebSock(webSocketHookCallback cbSend, webSocketHookCallback cbRecv);
typedef ByteBuffer webSocketHookCallback(ByteBuffer data);

@JS()
external void writeStatus(String s);

void main() {
  print('Hello, world!');
  writeStatus('Hooking');
  hookWebSock(allowInterop(handlePacket), allowInterop(handlePacket));
  writeStatus('Starting game');
  startGame();
  writeStatus('Done');
}

ByteBuffer handlePacket(ByteBuffer buffer) {
  var reader = ProtocolReader(buffer.asUint8List());
  var packet = reader.readPacket();
  if (packet is InternalOperationRequest || packet is InternalOperationResponse) {
    // ignore
  } else if (packet is Event) {
    switch (packet.code) {
      case EventCode.GameList:
      case EventCode.GameListUpdate:
        var gameList = packet.params[ParameterCode.GameList] as Map<Object, Object>;
        for (var value in gameList.keys) {
          var data = gameList[value] as Map<Object, Object>;
          if (data['password'] != '' && data['password'] != null && data['roomName'] != null) {
            print('Password-protected game "${data['roomName']}" has password "${data['password']}"');
            data['roomName'] = (data['roomName'] as String) + ' (password: ${data['password']})';
          }
        }
        return (ProtocolWriter()..writePacket(packet)).toBytes().buffer;
      case EventCode.AppStats:
        var masterPeerCount = packet.params[ParameterCode.MasterPeerCount];
        var gameCount = packet.params[ParameterCode.GameCount];
        var peerCount = packet.params[ParameterCode.PeerCount];
        print('Appstats: $gameCount games, $peerCount peers and $masterPeerCount master peers');
        break;

      case 200:
        var actor = packet.params[ParameterCode.ActorNr];
        var eventData = packet.params[ParameterCode.CustomEventContent] as Map<Object, Object>;
        var code = (eventData[SizedInt.byte(5)] as SizedInt).value;
        var payload = eventData[SizedInt.byte(4)];  // can be null!
        print('<<< Event 200: actor $actor, code $code, payload $payload');
        break;
      case 201:
        var actor = packet.params[ParameterCode.ActorNr];
        var eventData = packet.params[ParameterCode.CustomEventContent] as Map<Object, Object>;
        var payload = eventData[SizedInt.short(10)];  // can be null!
        print('<<< Event 201: actor $actor, payload $payload');
        break;

      default:
        print(packet);
        break;
    }
  } else if (packet is OperationRequest) {
    switch (packet.code) {
      case OperationCode.RaiseEvent: {
        var eventCode = packet.params[ParameterCode.Code];
        var eventData = packet.params[ParameterCode.CustomEventContent];

        if (eventCode is SizedInt && eventData is Map<Object, Object>) {
          switch (eventCode.value) {
            case 200:
              var code = eventData[SizedInt.byte(5)] as SizedInt;
              var data = eventData[SizedInt.byte(4)];
              if (code == null) {
                // what
              }
              else if (code.value == 41) {
                // shooting other 1
                var data2 = data as List<Object>;
                data2[1] = SizedFloat.float(13337);
                buffer = (ProtocolWriter()..writePacket(packet)).toBytes().buffer;
              } else if (code.value == 10) {
                // shooting other 2
                var data2 = data as List<Object>;
                data2[1] = SizedFloat.float(13337);
                data2[4] = SizedFloat.float(0); // health left?
                buffer = (ProtocolWriter()..writePacket(packet)).toBytes().buffer;
              } else if (code.value == 26) {
                // shooting other with RPG?
                var data2 = data as List<Object>;
                data2[1] = SizedFloat.float(13337);
                buffer = (ProtocolWriter()..writePacket(packet)).toBytes().buffer;
              } else if (code.value == 25) {
                // chat
                var data2 = data as List<Object>;
                data2[0] = '[hax] [Sandwich] [FuckYou] ' + data2[0].toString(); // author
                // data[1] == message
                data2[2] = SizedInt.short(0xFF); // R
                data2[3] = SizedInt.short(0x69); // G
                data2[4] = SizedInt.short(0xB4); // B
                buffer = (ProtocolWriter()..writePacket(packet)).toBytes().buffer;
              }
              print('>>> Event 200 code $code with data $data');
              writeStatus('>>> Event 200 code $code with data $data');
              return buffer;
            case 201:
              var data = eventData[SizedInt.short(10)] as List<Object>;
              // buffer = (ProtocolWriter()..writePacket(packet)).toBytes().buffer;
              // writeStatus('Event 201: $data');
              print('>>> Event 201 Sending our player info $data');
              return buffer;
          }
        }
      }
    }
    print(packet);
  } else {
    print(packet);
  }

  return buffer;  // just return old value
}
