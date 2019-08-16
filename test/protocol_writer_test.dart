import 'dart:typed_data';

import 'package:bullet_force_hax/src/protocol_reader/ProtocolWriter.dart';
import 'package:bullet_force_hax/src/protocol_reader/constants.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/Array.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/CustomData.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/SizedFloat.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/SizedInt.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/packets.dart';
import 'package:test/test.dart';

void main() {
  group('writing protocol types', () {
    test('can write null', () {
      var writer = ProtocolWriter()
        ..writeValue(null);
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([0x2a]));
    });

    // TODO: Dictionary test
    // TODO: EventData test
    // TODO: OperationResponse test
    // TODO: OperationRequest test

    test('can write bool', () {
      var writer1 = ProtocolWriter()..writeValue(false);
      var writer2 = ProtocolWriter()..writeValue(true);
      var buffer1 = writer1.toBytes();
      var buffer2 = writer2.toBytes();

      expect(buffer1, Uint8List.fromList([0x6f, 0x00]));
      expect(buffer2, Uint8List.fromList([0x6f, 0x01]));
    });

    test('can write u8', () {
      var writer = ProtocolWriter()
        ..writeValue(SizedInt.byte(0x90));
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([0x62, 0x90]));
    });

    test('can write s16', () {
      var writer = ProtocolWriter()
        ..writeValue(SizedInt.short(-1337));
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([0x6b, 0xFA, 0xC7]));
    });

    test('can write s32', () {
      var writer = ProtocolWriter()
        ..writeValue(SizedInt.int(-559038737));
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([0x69, 0xDE, 0xAD, 0xBE, 0xEF]));
    });

    test('can write s64', () {
      var writer = ProtocolWriter()
        ..writeValue(SizedInt.long(-3886136854700967234));
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([0x6c, 0xCA, 0x11, 0xAB, 0x1E, 0xCA, 0xFE, 0xBA, 0xBE]));
    });

    test('can write f32', () {
      var writer = ProtocolWriter()
        ..writeValue(SizedFloat.float(42));
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([0x66, 0x42, 0x28, 0x00, 0x00]));
    });

    test('can write f64', () {
      var writer = ProtocolWriter()
        ..writeValue(SizedFloat.double(13.37));
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([0x64, 0x40, 0x2a, 0xbd, 0x70, 0xa3, 0xd7, 0x0a, 0x3d]));
    });

    // TODO: add unicode test
    test('can write strings', () {
      var writer = ProtocolWriter()
        ..writeValue('abc');
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([0x73, 0x00, 0x03, 0x61, 0x62, 0x63]));
    });

    test('can write byte[]', () {
      var writer = ProtocolWriter()
        ..writeValue(Uint8List.fromList([0xDE, 0xAD, 0xBE, 0xEF]));
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([120, 0, 0, 0, 4, 0xDE, 0xAD, 0xBE, 0xEF]));
    });

    test('can write int[]', () {
      var writer = ProtocolWriter()
        ..writeValue(Int32List.fromList([-559038737, -889275714]));
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([110, 0, 0, 0, 2, 0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE]));
    });

    test('can read string[]', () {
      var writer = ProtocolWriter()
        ..writeValue(['abc', '']);
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([97, 0, 2, 0, 3, 0x61, 0x62, 0x63, 0, 0]));
    });

    test('can read Array', () {
      var writer = ProtocolWriter()
        ..writeValue(ProtocolArray(DataType.Bool, [true, false, true]));
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([121, 0, 3, 111, 1, 0, 1]));
    });

    test('can read ObjectArray', () {
      var writer = ProtocolWriter()
        ..writeValue(['abc', null, SizedInt.short(0x123)]);
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([122, 0, 3, 115, 0, 3, 0x61, 0x62, 0x63, 42, 107, 0x01, 0x23]));
    });

    test('can write hashtable', () {
      var writer = ProtocolWriter()
        ..writeValue({SizedInt.byte(0xFF): null, 'abc': true});
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([0x68, 0x00, 0x02, 98, 0xFF, 42, 115, 0x00, 0x03, 0x61, 0x62, 0x63, 111, 0x01]));
    });

    test('can write custom data', () {
      var writer = ProtocolWriter()
        ..writeValue(CustomData(42, Uint8List.fromList([0xDE, 0xAD, 0xBE, 0xEF])));
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([99, 42, 0, 4, 0xDE, 0xAD, 0xBE, 0xEF]));
    });
  });

  group('reading packets', () {
    test('packet 0x02: OperationRequest', () {
      var writer = ProtocolWriter()
        ..writePacket(OperationRequest(229, {}));
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([0xf3, 0x02, 0xe5, 0x00, 0x00]));
    });
    test('packet 0x03: OperationResponse', () {
      var writer = ProtocolWriter()
        ..writePacket(OperationResponse(229, null, 0, {}));
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([0xf3, 0x03, 0xe5, 0x00, 0x00, 0x2a, 0x00, 0x00]));
    });
    test('packet 0x04: Event', () {
      var writer = ProtocolWriter()
        ..writePacket(Event(226, {
          0xE3: SizedInt.int(223),
          0xE4: SizedInt.int(70),
          0xE5: SizedInt.int(364),
        }));
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([0xf3, 0x04, 0xe2, 0x00, 0x03,
        0xe3, 0x69, 0x00, 0x00, 0x00, 0xdf,
        0xe4, 0x69, 0x00, 0x00, 0x00, 0x46,
        0xe5, 0x69, 0x00, 0x00, 0x01, 0x6c,
      ]));
    });
    test('packet 0x06: InternalOperationRequest', () {
      var writer = ProtocolWriter()
        ..writePacket(InternalOperationRequest(1, {1: SizedInt.int(0x4CB99B22)}));
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([0xf3, 0x06, 0x01, 0x00, 0x01, 0x01, 0x69, 0x4c, 0xb9, 0x9b, 0x22]));
    });
    test('packet 0x07: InternalOperationResponse', () {
      var writer = ProtocolWriter()
        ..writePacket(InternalOperationResponse(1, null, 0, {
          1: SizedInt.int(0x4CB99B22),
          2: SizedInt.int(-0x60EC91A3),
        }));
      var buffer = writer.toBytes();

      expect(buffer, Uint8List.fromList([0xf3, 0x07, 0x01, 0x00, 0x00, 0x2a, 0x00, 0x02, 0x01, 0x69, 0x4c, 0xb9, 0x9b, 0x22, 0x02, 0x69, 0x9f, 0x13, 0x6e, 0x5d]));
    });
  });
}
