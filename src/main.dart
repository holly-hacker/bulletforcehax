import 'package:bullet_force_hax/bullet_force_hax_cli.dart';

Future main() async {
  print('Hello, world!');

  print('initializing');
  var bot = Bot();

  print('connecting to first endpoint');
  await bot.connectInitial();

  print('done');
}
