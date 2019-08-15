import 'package:bullet_force_hax/src/protocol_reader/ProtocolReader.dart';

class SizedInt {
  int value;
  int size;

  SizedInt(this.value, this.size) {
    _checkSize(size);
  }

  SizedInt.read(ProtocolReader reader, this.size) {
    _checkSize(size);
    switch(size) {
      case 1: value = reader.readUint8(); break;  // going to make this unsigned, it's more common
      case 2: value = reader.readInt16(); break;
      case 4: value = reader.readInt32(); break;
      case 8: value = reader.readInt64(); break;
    }
  }

  static void _checkSize(int size) {
    if (size > 8) {
      throw Exception("Size is greater than 8");
    }
    if (size != 1 && size != 2 && size != 4 && size != 8) {
      throw Exception("Size $size is not a power of 2");
    }
  }
}