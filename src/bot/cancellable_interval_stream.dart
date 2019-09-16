Stream<Function()> getCancellableIntervalStream([int msInterval = 100]) async* {
  bool enabled = true;
  var disableFunction = () => { enabled = false };
  while (enabled) {
    await Future.delayed(Duration(milliseconds: msInterval));
    yield disableFunction;
  }
}

Stream<void> getIntervalStream([int msInterval = 100]) async* {
  while (true) {
    await Future.delayed(Duration(milliseconds: msInterval));
    yield null;
  }
}
