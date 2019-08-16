import 'dart:typed_data';

import 'package:bullet_force_hax/src/protocol_reader/ProtocolReader.dart';

class CustomData {
  Uint8List data;
  int typeCode;

  CustomData.read(ProtocolReader reader) {
    typeCode = reader.readUint8();
    data = reader.read(reader.readUint16());
  }
}
