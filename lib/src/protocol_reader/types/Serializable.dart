import '../ProtocolWriter.dart';

abstract class Serializable {
  void writeType(ProtocolWriter writer);
  void writeValue(ProtocolWriter writer);
}