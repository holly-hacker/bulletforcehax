import 'package:bullet_force_hax/src/protocol_reader/ProtocolReader.dart';

abstract class PacketWithPayload {
  int code;
  Map<int, Object> params;
}

class OperationRequest extends PacketWithPayload {
  OperationRequest.read(ProtocolReader reader) {
    code = reader.readUint8();
    params = reader.readParameterTable();
  }
}

class OperationResponse extends PacketWithPayload {
  int returnCode;
  String debugMessage;

  OperationResponse.read(ProtocolReader reader) {
    code = reader.readUint8();
    returnCode = reader.readInt16();
    debugMessage = reader.readValue() as String;
    params = reader.readParameterTable();
  }
}

class Event extends PacketWithPayload {
  Event.read(ProtocolReader reader) {
    code = reader.readUint8();
    params = reader.readParameterTable();
  }
}

class InternalOperationRequest extends PacketWithPayload {
  InternalOperationRequest.read(ProtocolReader reader) {
    code = reader.readUint8();
    params = reader.readParameterTable();
  }
}

class InternalOperationResponse extends PacketWithPayload {
  int returnCode;
  String debugMessage;

  InternalOperationResponse.read(ProtocolReader reader) {
    code = reader.readUint8();
    returnCode = reader.readInt16();
    debugMessage = reader.readValue() as String;
    params = reader.readParameterTable();
  }
}
