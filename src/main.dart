import 'dart:typed_data';

import 'package:bullet_force_hax/bullet_force_hax.dart';

import 'websock_creator.dart';

const endpointHost = "ns.exitgames.com";
const httpPort = 9093;
const httpsPort = 19093;
const protocol = "GpBinaryV16";
const applicationId = "8c2cad3e-2e3f-4941-9044-b390ff2c4956";
const applicationVersion = "1.34_WebGL_1.73";
const region = "us";

Uint8List quickSerialize(PacketWithPayload pwp) => (ProtocolWriter()..writePacket(pwp)).toBytes();

Future main() async {
  print('Hello, world!');

  var firstWebSocket = await connectSocket(endpointHost, httpPort, protocol);
  firstWebSocket.handleError((e) {
    print('!!! Got error $e');
  });
  firstWebSocket.listen((data) {
    var parsed = ProtocolReader(data).readPacket();

    if (parsed is InitResponse) {
      print('sending bytes');
      firstWebSocket.add(quickSerialize(InternalOperationRequest(InternalOperationCode.Ping, {
        1: SizedInt.int(0)  // should be Environment.TickCount
      })));
      print('sent 1');

      firstWebSocket.add(quickSerialize(OperationRequest(OperationCode.Authenticate, {
        ParameterCode.AppVersion: applicationVersion,
        ParameterCode.ApplicationId: applicationId,
        ParameterCode.AzureNodeInfo: region,
      })));
      print('sent 2');
    }
    else if (parsed is InternalOperationResponse && parsed.code == InternalOperationCode.Ping) {
      // param 1 = sent time, param 2 = server time offset
      var num1 = parsed.params[2];
      print('server tickbase: $num1');
    } else if (parsed is OperationResponse && parsed.code == OperationCode.Authenticate) {
      var nickname = parsed.params[196];
      var address = parsed.params[ParameterCode.Address];
      var secret = parsed.params[ParameterCode.Secret];
      var userId = parsed.params[ParameterCode.UserId];
      print("Received address: $address (userid '$userId', nickname '$nickname', secret '$secret')");
      // TODO: use this data for next ws

      // close socket, we have what we need
      firstWebSocket.close();
    } else {
      assert(false);
      print('Received packet: $parsed');
    }
  });
  print('connected to socket');

  // See PeerBase.PrepareConnectData

  // >>> InternalOperationRequest 1: {1: int32 -226380891}
  // >>> OperationRequest 230: {220: 1.34_WebGL_1.73, 224: 8c2cad3e-2e3f-4941-9044-b390ff2c4956, 210: us}
  // <<< InternalOperationResponse 1 (return=0, msg=null): {1: int32 -226380891, 2: int32 1206606921}
  // <<< OperationResponse 230 (return=0, msg=null): {196: default, 230: wss://GCASH013.exitgames.com:19090, 221: AoQUdLbwoF4pN7MGxWJgmtXRMbF1eX5lv1kCLb/nalqEIXvCJ9jBtmDytG3amTMJOaSKrKuJI+oAac7h9iP1qWDf+9V9tjEbNWeR+NyrfC2GsVGih5m757WJWKewEyDMcv9zID+CTAa9kuMFyQGuBmDD0E561otSKsuhBGKnWCRuLwTM3r5oeer53b4cNwLZ+VPZwa1XUeda2wC7kpYckPCFV+K9N0rhdTyg4xwG6d+80v2nRgYzmxeo/RwRKF93wYdmofjQxeAKTQilChjNYe5RTwOF03cGTrME98cKRq4Toup3f2d/U08FmCD4wxl8uUu3qkh7zhVsmG8vXatzDXOszOU9VZTgh2v0raOIjyf/xCvwlE5sj/5iu5D30n/yEg==, 225: 9a2dd9f4-f644-45f2-be61-c5c7c02a2083}

  // OperationRequest 230: {220: 1.34_WebGL_1.73, 224: 8c2cad3e-2e3f-4941-9044-b390ff2c4956, 210: us}
  // <<< OperationResponse 230 (return=0, msg=null): {196: default, 230: wss://GCASH013.exitgames.com:19090, 221: ArmHRVMTWwgGOVg6uUI1d+vGbfaBLWBXTAPqHYyWSp5NQHdG5fWORWfHOyHeG9YXonEmzGrW6TwZN01MbL7yyDeQEBTdtG6FJnOh/f7YetdzjvZFtBvBOrY83ZJyBI9oSq6Ohl3VJypxMqkqfo3hRhhv2NVFJi4Zj6HAzZwzTTH794UftX74HeT0Wk3i5P6222mdvLpiE9vZQ83p+VAzNCEIdP6Cp8FSuoDDTCWAhi6ZUkEQjQGdhhp0RK4eXTqeBcY8BrttuRB0P/wTOIFPfyK4tLIJkP6Rqw+q5yNO7PLs4cVUBD9oxLEI9SA74RZxYeIXEgrbcDVvL0J9UDxHqEz1vacDkp9ZunRfT1+MzRvlx9urBLr41MVJDDWyJ5+Udw==, 225: 3177f04e-f26d-4797-b0c9-40798913caff}
}