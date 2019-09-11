import 'dart:convert';
import 'dart:typed_data';

import 'package:buffer/buffer.dart';

import 'constants.dart';
import 'types/Serializable.dart';
import 'types/packets.dart';

class ProtocolWriter extends ByteDataWriter {
  ProtocolWriter() : super(endian: Endian.big);

  writePacket(PacketWithPayload packet) {
    writeUint8(0xF3);
    writeValue(packet);
  }

  writeValue(Object s, [bool writeType = true]) {
    if (s == null) {
      if (writeType) {
        writeUint8(DataType.NullValue);
      }
    }
    // TODO: Dictionary
    else if (s is List<String>) { // needs to be checked before List<Object>
      if (writeType) {
        writeUint8(DataType.StringArray);
      }
      writeStringArray(s);
    }
    else if (s is Serializable) { // handles integers, floats, CustomData, Array, packets w/o 0xF3 prefix
      if (writeType) {
        s..writeType(this);
      }
      s.writeValue(this);
    }
    // TODO: EventData
    else if (s is Map<Object, Object>) {
      if (writeType) {
        writeUint8(DataType.Hashtable);
      }
      writeUint16(s.length);
      for (var key in s.keys) {
        writeValue(key);
        writeValue(s[key]);
      }
    }
    else if (s is Int32List) {
      if (writeType) {
        writeUint8(DataType.IntegerArray);
      }
      writeInt32(s.length);
      for (int i = 0; i < s.length; ++i) {
        writeInt32(s[i]);
      }
    }
    else if (s is bool) {
      if (writeType) {
        writeUint8(DataType.Bool);
      }
      writeUint8(s ? 1 : 0);
    }
    // TODO: OperationResponse
    // TODO: OperationRequest
    else if (s is String) {
      if (writeType) {
      writeUint8(DataType.String);
      }
      writeString(s);
    }
    else if (s is Uint8List) {
      if (writeType) {
        writeUint8(DataType.ByteArray);
      }
      writeInt32(s.length);
      write(s);
    }
    else if (s is List<Object>) {
      if (writeType) {
        writeUint8(DataType.ObjectArray);
      }
      writeInt16(s.length);
      for (var o in s) {
        writeValue(o);
      }
    }
    else {
      throw UnsupportedError("Cannot serialize '$s' (of type ${s.runtimeType})");
    }
  }

  writeStringArray(List<String> strings) {
    writeInt16(strings.length);
    for (var s in strings) {
      writeString(s);
    }
  }

  writeString(String s) {
    var bytes = utf8.encode(s);
    writeUint16(bytes.length);
    write(bytes);
  }

  writeParameterTable(Map<int, Object> params) {
    writeUint16(params.length);
    for (var key in params.keys) {
      writeUint8(key);
      writeValue(params[key]);
    }
  }
}