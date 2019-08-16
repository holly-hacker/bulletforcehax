import 'dart:typed_data';

import 'package:buffer/buffer.dart';
import 'package:bullet_force_hax/src/protocol_reader/constants.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/CustomData.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/SizedFloat.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/SizedInt.dart';
import 'package:bullet_force_hax/src/protocol_reader/types/packets.dart';

class ProtocolReader extends ByteDataReader {
  int get length => _buffer.length;

  Uint8List _buffer;

  ProtocolReader(this._buffer) : super(endian: Endian.big) {
    add(_buffer);
  }

  Object readValue() {
    var type = readUint8();

    switch (type) {
      case DataType.NullValue: return null;
      case DataType.Dictionary: break;
      case DataType.StringArray: break;
      case DataType.Byte: return SizedInt.read(this, 1);
      case DataType.Custom: return CustomData.read(this);
      case DataType.Double: return SizedFloat.read(this, 8);
      case DataType.EventData: break;
      case DataType.Float: return SizedFloat.read(this, 4);
      case DataType.Hashtable: return readHashTable();
      case DataType.Integer: return SizedInt.read(this, 4);
      case DataType.Short: return SizedInt.read(this, 2);
      case DataType.Long: return SizedInt.read(this, 8);
      case DataType.IntegerArray: return readIntArray();
      case DataType.Bool: return readUint8() != 0;
      case DataType.OperationResponse: break;
      case DataType.OperationRequest: break;
      case DataType.String: return readString();
      case DataType.ByteArray: return readByteArray();
      case DataType.Array: break;
      case DataType.ObjectArray: break;
    }

    throw Exception('Unimplemented data type $type');
  }

  PacketWithPayload readPacket() {
    var magic = readUint8();
    assert(magic == 0xF3);

    var type = readUint8();

    switch (type) {
      case PacketType.Init: break;
      case PacketType.InitResponse: break;
      case PacketType.Operation: return OperationRequest.read(this);
      case PacketType.OperationResponse: return OperationResponse.read(this);
      case PacketType.Event: return Event.read(this);
      case PacketType.InternalOperationRequest: return InternalOperationRequest.read(this);
      case PacketType.InternalOperationResponse: return InternalOperationResponse.read(this);
      case PacketType.Message: break;
      case PacketType.RawMessage: break;
    }

    throw Exception('Unimplemented packet type $type');
  }

  String readString() => String.fromCharCodes(read(readUint16()));

  Uint8List readByteArray() => read(readInt32());

  Int32List readIntArray() {
    var len = readInt32();
    var list = Int32List(len);
    for (int i = 0; i < len; i++) {
      list[i] = readInt32();
    }
    return list;
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