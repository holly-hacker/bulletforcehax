import 'dart:typed_data';

import 'package:bullet_force_hax/src/protocol_reader/ProtocolReader.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/SizedInt.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/packets.dart';
import 'package:test/test.dart';

void main() {
  group('reading protocol types', () {
    test('can read null', () {
      var reader = ProtocolReader(Uint8List.fromList([0x2a]));
      var t = reader.readValue();

      expect(t, null);
    });

    test('can read strings', () {
      var reader = ProtocolReader(Uint8List.fromList([0x73, 0x00, 0x03, 0x61, 0x62, 0x63]));
      var t = reader.readValue();

      expect(t is String, isTrue);
      expect(t, "abc");
    });
  });

  group('reading basic packets', () {
    test('packet 0x02: OperationRequest', () {
      var reader = ProtocolReader(Uint8List.fromList([0xf3, 0x02, 0xe5, 0x00, 0x00]));
      var t = reader.readPacket();

      expect(t is OperationRequest, isTrue);
      expect(t.code, 229);  // JoinLobby
      expect(t.params.length, 0);
    });
    test('packet 0x03: OperationResponse', () {
      var reader = ProtocolReader(Uint8List.fromList([0xf3, 0x03, 0xe5, 0x00, 0x00, 0x2a, 0x00, 0x00]));
      var t = reader.readPacket();

      expect(t is OperationResponse, isTrue);
      if (t is OperationResponse) {
        expect(t.code, 229);  // JoinLobby
        expect(t.debugMessage, null);
        expect(t.returnCode, 0);
        expect(t.params.length, 0);
      }
    });
    test('packet 0x04: Event', () {
      var reader = ProtocolReader(Uint8List.fromList([0xf3, 0x04, 0xe2, 0x00, 0x03, 0xe3, 0x69, 0x00, 0x00, 0x00, 0xdf, 0xe5, 0x69, 0x00, 0x00, 0x01, 0x6c, 0xe4, 0x69, 0x00, 0x00, 0x00, 0x46]));
      var t = reader.readPacket();

      expect(t is Event, isTrue);
      expect(t.code, 226);  // AppStats
      expect(t.params.length, 3);
      expect((t.params[0xE3] as SizedInt).value, 223);  // MASTER_PEER_COUNT
      expect((t.params[0xE5] as SizedInt).value, 364);  // PEER_COUNT
      expect((t.params[0xE4] as SizedInt).value, 70);   // GAME_COUNT
    });
    test('packet 0x06: InternalOperationRequest', () {
      var reader = ProtocolReader(Uint8List.fromList([0xf3, 0x06, 0x01, 0x00, 0x01, 0x01, 0x69, 0x4c, 0xb9, 0x9b, 0x22]));
      var t = reader.readPacket();

      expect(t is InternalOperationRequest, isTrue);
      expect(t.code, 1);  // JoinLobby
      expect(t.params.length, 1);
      expect((t.params[1] as SizedInt).value, 0x4CB99B22);  // FIND_FRIEND_REQUEST_LIST
    });
    test('packet 0x07: InternalOperationResponse', () {
      var reader = ProtocolReader(Uint8List.fromList([0xf3, 0x07, 0x01, 0x00, 0x00, 0x2a, 0x00, 0x02, 0x01, 0x69, 0x4c, 0xb9, 0x9b, 0x22, 0x02, 0x69, 0x9f, 0x13, 0x6e, 0x5d]));
      var t = reader.readPacket();

      expect(t is InternalOperationResponse, isTrue);
      if (t is InternalOperationResponse) {
        expect(t.code, 1);  // JoinLobby
        expect(t.debugMessage, null);
        expect(t.returnCode, 0);
        expect(t.params.length, 2);
        expect((t.params[1] as SizedInt).value, 0x4CB99B22);  // FIND_FRIEND_REQUEST_LIST
        expect((t.params[2] as SizedInt).value, -0x60EC91A3);  // FIND_FRIENDS_OPTIONS
      }
    });
  });
}
