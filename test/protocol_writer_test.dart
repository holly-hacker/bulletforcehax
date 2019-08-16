import 'dart:typed_data';

import 'package:bullet_force_hax/src/protocol_reader/ProtocolWriter.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/CustomData.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/SizedFloat.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/SizedInt.dart';
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
    // TODO: Array test
    // TODO: ObjectArray test

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
}
