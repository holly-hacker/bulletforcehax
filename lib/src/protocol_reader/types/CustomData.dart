import 'dart:typed_data';

import '../ProtocolReader.dart';
import '../ProtocolWriter.dart';
import '../constants.dart';
import 'Serializable.dart';

class CustomData implements Serializable {
  Uint8List data;
  int typeCode;

  CustomData(this.typeCode, this.data);

  CustomData.read(ProtocolReader reader) {
    typeCode = reader.readUint8();
    data = reader.read(reader.readUint16());
  }

  void writeType(ProtocolWriter writer) {
    writer.writeUint8(DataType.Custom);
  }

  void writeValue(ProtocolWriter writer) {
    writer.writeUint8(typeCode);
    writer.writeUint16(data.length);
    writer.write(data);
  }
}
