import 'dart:typed_data';

import 'package:bullet_force_hax/src/protocol_reader/types/Vector3.dart';
import 'package:convert/convert.dart';

import '../ProtocolReader.dart';
import '../ProtocolWriter.dart';
import '../constants.dart';
import 'Serializable.dart';
import 'UnimplementedCustomData.dart';

abstract class CustomData implements Serializable {
  int get typeCode;

  CustomData();

  factory CustomData.read(ProtocolReader reader) {
    var typeCode = reader.readUint8();
    var data = reader.read(reader.readUint16());
    var tempReader = ProtocolReader(data);

    switch(typeCode) {
      case Vector3.TypeCode: return Vector3.read(tempReader);
      default: return UnimplementedCustomData(typeCode, data);
    }
  }

  void writeType(ProtocolWriter writer) {
    writer.writeUint8(DataType.Custom);
  }

  void writeValue(ProtocolWriter writer) {
    writer.writeUint8(typeCode);
    var data = getBytes();
    writer.writeUint16(data.length);
    writer.write(data);
  }

  void write(ProtocolWriter w);

  Uint8List getBytes() {
    var w = ProtocolWriter();
    write(w);
    return w.toBytes();
  }

  String toString() {
    return 'CustomData $typeCode ${hex.encode(getBytes())}';
  }
}
