import 'package:bullet_force_hax/src/protocol_reader/ProtocolReader.dart';
import 'package:bullet_force_hax/src/protocol_reader/ProtocolWriter.dart';
import 'package:bullet_force_hax/src/protocol_reader/constants.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/Serializable.dart';

class ProtocolArray implements Serializable {
  int innerDataType;
  List<Object> data;

  ProtocolArray(this.innerDataType, this.data);

  ProtocolArray.read(ProtocolReader reader) {
    var len = reader.readUint16();
    innerDataType = reader.readUint8();

    data = List<Object>(len);
    for (int i = 0; i < len; ++i) {
      data[i] = reader.readValue(innerDataType);
    }
  }

  void writeType(ProtocolWriter writer) {
    writer.writeUint8(DataType.Array);
  }

  void writeValue(ProtocolWriter writer) {
    writer.writeUint16(data.length);
    writer.writeUint8(innerDataType);
    for (var obj in data) {
      writer.writeValue(obj, false);
    }
  }
}