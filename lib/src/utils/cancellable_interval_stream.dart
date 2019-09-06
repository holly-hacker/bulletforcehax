/// Abstraction over a while loop with a sleep function
class CancellableIntervalStream {
  static Stream<Function()> run([int msInterval = 100]) async* {
    bool enabled = true;
    var disableFunction = () => { enabled = false };
    while (enabled) {
      await Future.delayed(Duration(milliseconds: msInterval));
      yield disableFunction;
    }
  }
}
