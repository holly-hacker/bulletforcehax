import 'dart:typed_data';

import 'package:bullet_force_hax/src/protocol_reader/ProtocolWriter.dart';

import 'CustomData.dart';

class UnimplementedCustomData extends CustomData {
  int get typeCode => _typeCode;
  int _typeCode;
  Uint8List data;

  UnimplementedCustomData(this._typeCode, this.data);

  void write(ProtocolWriter w) => w.write(data);
}