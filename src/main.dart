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
  var bot = Bot();

  print('connecting to first endpoint');
  await bot.connectInitial();

  print('establishing connection to main endpoint');
  await bot.connectMain();

  print('done');
}
