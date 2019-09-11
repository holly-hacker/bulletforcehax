import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import 'package:bullet_force_hax/bullet_force_hax_cli.dart';

Future main(List<String> arguments) async {
  if (arguments.isNotEmpty && arguments[0] == "decodeb64") {
    String input;
    if (arguments.length == 2) {
      input = arguments[1];
    } else {
      stdout.write("Enter base64 to decode: ");
      input = stdin.readLineSync();
    }

    Uint8List inputBytes = base64.decode(input);

    var parsed = ProtocolReader(inputBytes).readPacket();
    print(parsed);
  }
  else if (arguments.isNotEmpty) {
    await doBot(arguments[0]);
  }
  else {
    print('no arguments specified, will connect to test match');
    await doBot();
  }
}

Future doBot([String action = 'jointestmatch']) async {
  print('initializing');
  var lobbyBot = LobbyBot();

  print('establishing connection to lobby');
  await lobbyBot.connectLobby();

  switch(action) {
    case 'jointestmatch':
    case 'creatematch':
      ConnectionCredentials credentials;
      GameProperties gameProps;
      int waitTime;
      if (action == 'jointestmatch') {
        waitTime = 10;

        print('Finding match to join');
        var game = await lobbyBot.gamesStream.firstWhere((match) => match.roomName == "HoLLyTest");

        print('getting room credentials');
        credentials = await lobbyBot.getRoomCredentials(game.roomId);
      }
      else if (action == 'creatematch') {
        waitTime = 60;
        gameProps = GameProperties.initial()
          ..roomName = "HoLLyTest";

        print('Requesting match creation');
        credentials = await lobbyBot.createMatch();
      }

      print('disconnecting from lobby');
      await lobbyBot.disconnectLobby();

      print('connecting to match');
      var gameplayBot = GameplayBot();
      await gameplayBot.connectMatch(credentials, gameProps);

      print('waiting $waitTime seconds');
      await Future.delayed(Duration(seconds: waitTime));

      print('disconnecting from match');
      await gameplayBot.disconnectMatch();
      break;
    default:
      print("unknown action '$action'");
      break;
  }

  print('done');
}
