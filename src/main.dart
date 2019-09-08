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
  } else {
    await doBot();
  }
}

Future doBot() async {
  print('initializing');
  var lobbyBot = LobbyBot();

  print('establishing connection to lobby');
  await lobbyBot.connectLobby();

  print('Finding match to join');
  var game = await lobbyBot.gamesStream.firstWhere((match) => match.roomName == "HoLLyTest");

  print('getting room credentials');
  var credentials = await lobbyBot.getRoomCredentials(game.roomId);

  print('disconnecting from lobby');
  await lobbyBot.disconnectLobby();

  print('connecting to match');
  var gameplayBot = GameplayBot();
  await gameplayBot.connectMatch(game.roomId, credentials);

  print('waiting 10 seconds');
  await Future.delayed(Duration(seconds: 10));

  print('done');
}
