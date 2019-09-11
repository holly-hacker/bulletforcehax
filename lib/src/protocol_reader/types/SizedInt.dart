import '../ProtocolReader.dart';
import '../ProtocolWriter.dart';
import '../constants.dart';
import 'Serializable.dart';

SizedInt u8(int i) => i == null ? null : SizedInt(i, 1);
SizedInt s16(int i) => i == null ? null : SizedInt(i, 2);
SizedInt s32(int i) => i == null ? null : SizedInt(i, 4);
SizedInt s64(int i) => i == null ? null : SizedInt(i, 8);

class SizedInt implements Serializable {
  int value;
  int size;

  SizedInt(this.value, this.size) {
    _checkSize();
  }

  SizedInt.read(ProtocolReader reader, this.size) {
    switch(size) {
      case 1: value = reader.readUint8(); break;  // going to make this unsigned, it's more common
      case 2: value = reader.readInt16(); break;
      case 4: value = reader.readInt32(); break;
      case 8: value = reader.readInt64(); break;
    }
    _checkSize();
  }

  void writeType(ProtocolWriter writer) {
    switch(size) {
      case 1: writer.writeUint8(DataType.Byte); break;
      case 2: writer.writeUint8(DataType.Short); break;
      case 4: writer.writeUint8(DataType.Integer); break;
      case 8: writer.writeUint8(DataType.Long); break;
      default: throw Exception("Tried to write type of SizedInt with size $size. This should never happen");
    }
  }

  void writeValue(ProtocolWriter writer) {
    switch(size) {
      case 1: writer.writeUint8(value); break;
      case 2: writer.writeInt16(value); break;
      case 4: writer.writeInt32(value); break;
      case 8: writer.writeInt64(value); break;
      default: throw Exception("Tried to write SizedInt with size $size. This should never happen");
    }
  }

  bool operator ==(other) => size == other.size && value == other.value;
  int get hashCode => value.hashCode;
  String toString() => 'int${size*8} $value';

  void _checkSize() {
    if (value == null) {
      throw Exception("Tried to check size of null value");
    }
    if (size > 8) {
      throw Exception("Size is greater than 8");
    }
    if (size != 1 && size != 2 && size != 4 && size != 8) {
      throw Exception("Size $size is not a power of 2");
    }

    if (size == 1 && (value > 0xFF || value < 0x00)) {
      throw Exception("Value $value is out of range for a byte");
    }
    if (size == 2 && (value > 0x7FFF || value < -0x8000)) {
      throw Exception("Value $value is out of range for a short");
    }
    if (size == 4 && (value > 0x7FFFFFFF || value < -0x80000000)) {
      throw Exception("Value $value is out of range for an int");
    }
  }
}
