import 'dart:convert';
import 'dart:typed_data';

import 'package:buffer/buffer.dart';

import 'constants.dart';
import 'types/Array.dart';
import 'types/CustomData.dart';
import 'types/SizedFloat.dart';
import 'types/SizedInt.dart';
import 'types/packets.dart';

class ProtocolReader extends ByteDataReader {
  ProtocolReader(Uint8List buffer) : super(endian: Endian.big) {
    add(buffer);
  }

  Object readValue([int type]) {
    type ??= readUint8();

    switch (type) {
      case DataType.NullValue: return null;
      case DataType.Dictionary: throw UnimplementedError('Unimplemented data type $type (Dictionary)');
      case DataType.StringArray: return readStringArray();
      case DataType.Byte: return SizedInt.read(this, 1);
      case DataType.Custom: return CustomData.read(this);
      case DataType.Double: return SizedFloat.read(this, 8);
      case DataType.EventData: throw UnimplementedError('Unimplemented data type $type (EventData)');
      case DataType.Float: return SizedFloat.read(this, 4);
      case DataType.Hashtable: return readHashTable();
      case DataType.Integer: return SizedInt.read(this, 4);
      case DataType.Short: return SizedInt.read(this, 2);
      case DataType.Long: return SizedInt.read(this, 8);
      case DataType.IntegerArray: return readIntArray();
      case DataType.Bool: return readUint8() != 0;
      case DataType.OperationResponse: throw UnimplementedError('Unimplemented data type $type (OperationResponse)');
      case DataType.OperationRequest: throw UnimplementedError('Unimplemented data type $type (OperationRequest)');
      case DataType.String: return readString();
      case DataType.ByteArray: return readByteArray();
      case DataType.Array: return ProtocolArray.read(this);
      case DataType.ObjectArray: return readObjectArray();
    }

    throw UnsupportedError('Unknown data type $type');
  }

  PacketWithPayload readPacket() {
    var magic = readUint8();
    assert(magic == 0xF3);

    var type = readUint8();

    switch (type) {
      case PacketType.Init: break;
      case PacketType.InitResponse: return InitResponse.read(this);
      case PacketType.Operation: return OperationRequest.read(this);
      case PacketType.OperationResponse: return OperationResponse.read(this);
      case PacketType.Event: return Event.read(this);
      case PacketType.InternalOperationRequest: return InternalOperationRequest.read(this);
      case PacketType.InternalOperationResponse: return InternalOperationResponse.read(this);
      case PacketType.Message: break;
      case PacketType.RawMessage: break;
    }

    throw UnimplementedError('Unimplemented packet type $type');
  }

  String readString() {
    var len = readUint16();
    if (len == 0) return '';
    return utf8.decode(read(len));
  }

  Uint8List readByteArray() => read(readInt32());

  Int32List readIntArray() {
    var len = readInt32();
    var list = Int32List(len);
    for (int i = 0; i < len; i++) {
      list[i] = readInt32();
    }
    return list;
  }

  List<String> readStringArray() {
    var len = readInt16();
    var list = List<String>(len);
    for (int i = 0; i < len; i++) {
      list[i] = readString();
    }
    return list;
  }

  List<Object> readObjectArray() {
    var len = readUint16();
    var set = List<Object>();
    for (int i = 0; i < len; i++) {
      set.add(readValue());
    }
    return set;
  }

  Map<Object, Object> readHashTable() {
    Map<Object, Object> value = {};
    var len = readInt16();
    for (int i = 0; i < len; i++) {
      var key = readValue();
      var val = readValue();
      value[key] = val;
    }
    return value;
  }

  Map<int, Object> readParameterTable() {
    Map<int, Object> value = {};
    var len = readInt16();
    for (int i = 0; i < len; i++) {
      var key = readUint8();
      var val = readValue();
      value[key] = val;
    }
    return value;
  }
}