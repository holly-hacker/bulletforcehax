class ConnectionCredentials {
  String address;
  String secret;
  
  String get host => address.split('://').last.split(':')[0];
  int get port => int.parse(address.split('://').last.split(':')[1]);

  ConnectionCredentials(this.address, [this.secret]);

  bool get hasSecret => secret != null;
}
