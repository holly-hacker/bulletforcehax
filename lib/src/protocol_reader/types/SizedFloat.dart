import 'package:bullet_force_hax/src/protocol_reader/ProtocolReader.dart';

class SizedFloat {
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

  void _checkSize() {
    if (size > 8) {
      throw Exception("Size is greater than 8");
    }
    if (size != 4 && size != 8) {
      throw Exception("Size $size is not 4 or 8");
    }
  }
}
