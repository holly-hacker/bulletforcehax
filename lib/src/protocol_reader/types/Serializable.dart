import 'package:bullet_force_hax/src/protocol_reader/ProtocolWriter.dart';

abstract class Serializable {
  void writeType(ProtocolWriter writer);
  void writeValue(ProtocolWriter writer);
}