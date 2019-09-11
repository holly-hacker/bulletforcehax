class ConnectionCredentials {
  String address;
  String secret;
  String roomId;
  
  String get host => address.split('://').last.split(':')[0];
  int get port => int.parse(address.split('://').last.split(':')[1]);
  bool get hasSecret => secret != null;
  bool get hasRoomId => roomId != null;

  ConnectionCredentials(this.address, [this.secret, this.roomId]);
}
