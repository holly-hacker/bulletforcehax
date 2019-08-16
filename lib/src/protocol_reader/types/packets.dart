import 'package:bullet_force_hax/src/protocol_reader/ProtocolReader.dart';
import 'package:bullet_force_hax/src/protocol_reader/ProtocolWriter.dart';
import 'package:bullet_force_hax/src/protocol_reader/constants.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/Serializable.dart';

abstract class PacketWithPayload implements Serializable {
  int code;
  Map<int, Object> params;
}

class OperationRequest extends PacketWithPayload {
  OperationRequest(int code, Map<int, Object> params) {
    super.code = code;
    super.params = params;
  }

  OperationRequest.read(ProtocolReader reader) {
    code = reader.readUint8();
    params = reader.readParameterTable();
  }

  void writeType(ProtocolWriter writer) => writer.writeUint8(PacketType.Operation);

  void writeValue(ProtocolWriter writer) {
    writer.writeUint8(code);
    writer.writeParameterTable(params);
  }

  String toString() => 'OperationRequest $code: $params';
}

class OperationResponse extends PacketWithPayload {
  int returnCode;
  String debugMessage;

  OperationResponse(int code, this.debugMessage, this.returnCode, Map<int, Object> params) {
    super.code = code;
    super.params = params;
  }

  OperationResponse.read(ProtocolReader reader) {
    code = reader.readUint8();
    returnCode = reader.readInt16();
    debugMessage = reader.readValue() as String;
    params = reader.readParameterTable();
  }

  void writeType(ProtocolWriter writer) => writer.writeUint8(PacketType.OperationResponse);

  void writeValue(ProtocolWriter writer) {
    writer.writeUint8(code);
    writer.writeInt16(returnCode);
    writer.writeValue(debugMessage);
    writer.writeParameterTable(params);
  }

  String toString() => 'OperationResponse $code (return=$returnCode, msg=$debugMessage): $params';
}

class Event extends PacketWithPayload {
  Event(int code, Map<int, Object> params) {
    super.code = code;
    super.params = params;
  }

  Event.read(ProtocolReader reader) {
    code = reader.readUint8();
    params = reader.readParameterTable();
  }

  void writeType(ProtocolWriter writer) => writer.writeUint8(PacketType.Event);

  void writeValue(ProtocolWriter writer) {
    writer.writeUint8(code);
    writer.writeParameterTable(params);
  }

  String toString() => 'Event $code: $params';
}

class InternalOperationRequest extends PacketWithPayload {
  InternalOperationRequest(int code, Map<int, Object> params) {
    super.code = code;
    super.params = params;
  }

  InternalOperationRequest.read(ProtocolReader reader) {
    code = reader.readUint8();
    params = reader.readParameterTable();
  }

  void writeType(ProtocolWriter writer) => writer.writeUint8(PacketType.InternalOperationRequest);

  void writeValue(ProtocolWriter writer) {
    writer.writeUint8(code);
    writer.writeParameterTable(params);
  }

  String toString() => 'InternalOperationRequest $code: $params';
}

class InternalOperationResponse extends PacketWithPayload {
  int returnCode;
  String debugMessage;

  InternalOperationResponse(int code, this.debugMessage, this.returnCode, Map<int, Object> params) {
    super.code = code;
    super.params = params;
  }

  InternalOperationResponse.read(ProtocolReader reader) {
    code = reader.readUint8();
    returnCode = reader.readInt16();
    debugMessage = reader.readValue() as String;
    params = reader.readParameterTable();
  }

  void writeType(ProtocolWriter writer) => writer.writeUint8(PacketType.InternalOperationResponse);

  void writeValue(ProtocolWriter writer) {
    writer.writeUint8(code);
    writer.writeInt16(returnCode);
    writer.writeValue(debugMessage);
    writer.writeParameterTable(params);
  }

  String toString() => 'InternalOperationResponse $code (return=$returnCode, msg=$debugMessage): $params';
}
