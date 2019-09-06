import 'dart:typed_data';

import '../ProtocolReader.dart';
import '../ProtocolWriter.dart';
import '../constants.dart';
import 'Serializable.dart';

abstract class PacketWithPayload implements Serializable {
  int code;
  Map<int, Object> params;

  String toString() => '${this.runtimeType} $code: $params';
}

class InitPacket extends PacketWithPayload {
  static const protocolVersion = [1, 6];
  static const clientVersion = [4, 1, 2, 16];
  static const clientSdkId = 15;
  static const clientSdkIdShifted = clientSdkId << 1;

  Uint8List appID;  // GUID
  var isIpv6;

  InitPacket(this.appID, {this.isIpv6 = false});

  void writeType(ProtocolWriter writer) => writer.writeUint8(PacketType.Init);

  void writeValue(ProtocolWriter writer) {
    writer.writeUint8(protocolVersion[0]);
    writer.writeUint8(protocolVersion[1]);

    writer.writeUint8(clientSdkIdShifted);

    // first 2 parts of version are packet with [isIpv6]
    var versionBitField = ((clientVersion[0] << 4) | clientVersion[1]);
    if (isIpv6) {
      versionBitField |= 0x80;
    }
    else {
      versionBitField &= 0x7F;
    }

    writer.writeUint8(versionBitField);
    writer.writeUint8(clientVersion[2]);
    writer.writeUint8(clientVersion[3]);
    writer.writeUint8(0);

    for (int i = 0; i < 32; ++i) {
      if (i < appID.length) {
        writer.writeUint8(appID[i]);
      }
      else {
        writer.writeUint8(0);
      }
    }
  }
}

class InitResponse extends PacketWithPayload {
  InitResponse() {
    code = 0;
  }

  InitResponse.read(ProtocolReader protocolReader) {
    code = protocolReader.readInt8();
    assert(code == 0);
  }

  void writeType(ProtocolWriter writer) => writer.writeUint8(PacketType.InitResponse);
  void writeValue(ProtocolWriter writer) => writer.writeUint8(code);

  String toString() => 'InitResponse';
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
