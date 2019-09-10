import 'dart:async';
import 'dart:core';
import 'dart:io';

import 'package:bullet_force_hax/bullet_force_hax.dart';

import '../utils/cancellable_interval_stream.dart';
import 'connection_details.dart';
import 'websock_creator.dart';

class GameSocket {
  static const endpointHost = "ns.exitgames.com";
  static const httpPort = 9093;
  static const httpsPort = 19093;
  static const applicationId = "8c2cad3e-2e3f-4941-9044-b390ff2c4956";
  static const applicationVersion = "1.34_WebGL_1.73";
  static const region = "us";
  static const protocol = "GpBinaryV16";
  static const pingInterval = 1000;

  DateTime _startTime = DateTime.now();
  DateTime _lastPing = DateTime.fromMillisecondsSinceEpoch(0);
  int _serverTickOffset;
  int get _tickCount => DateTime.now().difference(_startTime).inMilliseconds;
  int get serverTime => _tickCount + _serverTickOffset;

  Stream<PacketWithPayload> get packets => _packetStream ?? (_packetStream = _packetController.stream.asBroadcastStream().cast<PacketWithPayload>());
  Stream<PacketWithPayload> _packetStream;  // to allow lazy initialization of broadcast stream
  StreamController<PacketWithPayload> _packetController;

  ConnectionCredentials _credentials;
  WebSocket _socket;
  StreamSubscription _listenSub;
  StreamSubscription<void> _pingSub;
  
  bool _opened = false;
  bool _closed = false;

  GameSocket.initial() : this.fromCredentials(ConnectionCredentials("ws://$endpointHost:$httpPort"));

  GameSocket.fromCredentials(this._credentials) {
    _packetController = StreamController<PacketWithPayload>(
      onListen: connect,  // when listening starts, automatically connect
      // don't implement pause or resume, a websocket w/o keepalive gets timed out
      onCancel: close,
    );
  }

  Future connect() async {
    if (_opened) {
      return;
    }

    _opened = true;
    _socket = await connectSocket(_credentials.host, _credentials.port, protocol);
    _socket.handleError((error) {
      print('encountered an error! $error');
    });

    _listenSub = _socket.map((data) => ProtocolReader(data).readPacket()).listen((data) {
      if (data is InitResponse) {
        add(_getPing());

        if (_credentials.hasSecret) {
          add(OperationRequest(OperationCode.Authenticate, {
            ParameterCode.Secret: _credentials.secret,
          }));
        }
        else {
          add(OperationRequest(OperationCode.Authenticate, {
            ParameterCode.AppVersion: applicationVersion,
            ParameterCode.ApplicationId: applicationId,
            ParameterCode.AzureNodeInfo: region,
          }));
        }
      }
      else if (data is InternalOperationResponse && data.code == InternalOperationCode.Ping) {
        // param 1 = sent time, param 2 = server time
        _serverTickOffset = (data.params[2] as SizedInt).value - _tickCount;  // TODO: check, should prob use lerp(_ticks, _tickWhenSent)
      }
      else {
        // we don't handle this packet, pass it to the consumer
        _packetController.add(data);
      }
    });

    _pingSub = getIntervalStream().listen((_) {
      if (DateTime.now().difference(_lastPing).inMilliseconds > pingInterval) {
        add(_getPing());
        _lastPing = DateTime.now();
      }
    });
  }

  Future close() async {
    if (!_closed) {
      await _pingSub.cancel();
      await _listenSub.cancel();
      await _socket.close();
      _closed = true;
    }
  }

  Future add(PacketWithPayload pwp) async {
    await connect();
    _socket.add((ProtocolWriter()..writePacket(pwp)).toBytes());
  }

  PacketWithPayload _getPing() => InternalOperationRequest(InternalOperationCode.Ping, {1: SizedInt.int(_tickCount)});
}