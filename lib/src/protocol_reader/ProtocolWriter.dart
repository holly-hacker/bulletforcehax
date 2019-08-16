import 'dart:typed_data';

import 'package:buffer/buffer.dart';
import 'package:bullet_force_hax/src/protocol_reader/constants.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/Serializable.dart';

class ProtocolWriter extends ByteDataWriter {
  ProtocolWriter() : super(endian: Endian.big);

  writeValue(Object s) {
    if (s == null) {
      writeUint8(DataType.NullValue);
    }
    // TODO: Dictionary
    else if (s is List<String>) { // needs to be checked before List<Object>
      writeUint8(DataType.StringArray);
      writeStringArray(s);
    }
    else if (s is Serializable) { // handles integers, floats, CustomData
      s..writeType(this)
        ..writeValue(this);
    }
    // TODO: EventData
    else if (s is Map<Object, Object>) {
      writeUint8(DataType.Hashtable);
      writeUint16(s.length);
      for (var key in s.keys) {
        writeValue(key);
        writeValue(s[key]);
      }
    }
    else if (s is Int32List) {
      writeUint8(DataType.IntegerArray);
      writeInt32(s.length);
      for (int i = 0; i < s.length; ++i) {
        writeInt32(s[i]);
      }
    }
    else if (s is bool) {
      writeUint8(DataType.Bool);
      writeUint8(s ? 1 : 0);
    }
    // TODO: OperationResponse
    // TODO: OperationRequest
    else if (s is String) {
      writeUint8(DataType.String);
      writeString(s);
    }
    else if (s is Uint8List) {
      writeUint8(DataType.ByteArray);
      writeInt32(s.length);
      write(s);
    }
    /*
    else if (s is List<Object>) {
      writeUint8(DataType.Array);
      writeInt16(s.length);
      // now write element type code
      // TODO: List<Object> (Array)
      throw UnimplementedError();
    }
     */
    else if (s is List<Object>) {
      writeUint8(DataType.ObjectArray);
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
    var bytes = s.codeUnits;  // TODO: this only works for ASCII text!
    writeUint16(bytes.length);
    write(bytes);
  }
}