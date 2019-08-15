import 'package:bullet_force_hax/bullet_force_hax.dart';
import 'package:test/test.dart';

void main() {
  group('A group of tests', () {
    MyClass answer;

    setUp(() {
      answer = MyClass();
    });

    test('Test Test', () {
      expect(answer.theAnwser, 42);
    });
  });
}
