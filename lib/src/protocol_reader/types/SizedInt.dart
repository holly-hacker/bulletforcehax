import 'package:bullet_force_hax/src/protocol_reader/ProtocolReader.dart';
import 'package:bullet_force_hax/src/protocol_reader/ProtocolWriter.dart';
import 'package:bullet_force_hax/src/protocol_reader/constants.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/Serializable.dart';

class SizedInt implements Serializable {
  int value;
  int size;

  SizedInt(this.value, this.size) {
    _checkSize();
  }

  SizedInt.byte(this.value)  { size = 1; _checkSize(); }
  SizedInt.short(this.value) { size = 2; _checkSize(); }
  SizedInt.int(this.value)   { size = 4; _checkSize(); }
  SizedInt.long(this.value)  { size = 8; _checkSize(); }

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

  String toString() => 'int${size*8} $value';

  void _checkSize() {
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
