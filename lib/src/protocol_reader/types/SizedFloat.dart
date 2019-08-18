import '../ProtocolReader.dart';
import '../ProtocolWriter.dart';
import '../constants.dart';
import 'Serializable.dart';

class SizedFloat implements Serializable {
  double value;
  int size;

  SizedFloat(this.value, this.size) {
    _checkSize();
  }

  SizedFloat.float(this.value)  { size = 4; _checkSize(); }
  SizedFloat.double(this.value) { size = 8; _checkSize(); }

  SizedFloat.read(ProtocolReader reader, this.size) {
    switch(size) {
      case 4: value = reader.readFloat32(); break;
      case 8: value = reader.readFloat64(); break;
    }
    _checkSize();
  }

  void writeType(ProtocolWriter writer) {
    switch(size) {
      case 4: writer.writeUint8(DataType.Float); break;
      case 8: writer.writeUint8(DataType.Double); break;
      default: throw Exception("Tried to writetype of SizedFloat with size $size. This should never happen");
    }
  }

  void writeValue(ProtocolWriter writer) {
    switch(size) {
      case 4: writer.writeFloat32(value); break;
      case 8: writer.writeFloat64(value); break;
      default: throw Exception("Tried to write SizedFloat with size $size. This should never happen");
    }
  }

  bool operator ==(other) => size == other.size && value == other.value;
  int get hashCode => value.hashCode;
  String toString() => 'float${size*8} $value';

  void _checkSize() {
    if (size > 8) {
      throw Exception("Size is greater than 8");
    }
    if (size != 4 && size != 8) {
      throw Exception("Size $size is not 4 or 8");
    }
  }
}
