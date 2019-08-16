abstract class DataType {
  static const int NullValue = 42;
  static const int Dictionary = 68; // Map<Object, Object>, predefined types, C# type is IDictionary/Dictionary<T1, T2>
  static const int StringArray = 97;
  static const int Byte = 98;
  static const int Custom = 99;
  static const int Double = 100;
  static const int EventData = 101;
  static const int Float = 102;
  static const int Hashtable = 104; // Map<Object, Object>, random types, C# type is Hashtable/Dictionary<object, object>
  static const int Integer = 105;
  static const int Short = 107;
  static const int Long = 108;
  static const int IntegerArray = 110;
  static const int Bool = 111;
  static const int OperationResponse = 112;
  static const int OperationRequest = 113;
  static const int String = 115;
  static const int ByteArray = 120;
  static const int Array = 121;       // A List, predetermined type, C# type is Array
  static const int ObjectArray = 122; // A Set, random types, C# type is List<object>
}

abstract class PacketType {
  static const int Init = 0;
  static const int InitResponse = 1;
  static const int Operation = 2;
  static const int OperationResponse = 3;
  static const int Event = 4;
  static const int InternalOperationRequest = 6;
  static const int InternalOperationResponse = 7;
  static const int Message = 8;
  static const int RawMessage = 9;
}
