import '../ProtocolReader.dart';
import '../ProtocolWriter.dart';
import 'CustomData.dart';

class Vector3 extends CustomData {
  static const TypeCode = 86;

  int get typeCode => TypeCode;
  double f1, f2, f3;

  Vector3(this.f1, this.f2, this.f3);

  Vector3.read(ProtocolReader r) {
    f1 = r.readFloat32();
    f2 = r.readFloat32();
    f3 = r.readFloat32();
  }

  void write(ProtocolWriter w) {
    w.writeFloat32(f1);
    w.writeFloat32(f2);
    w.writeFloat32(f3);
  }

  String toString() => 'Vector3($f1,$f2,$f3)';
}