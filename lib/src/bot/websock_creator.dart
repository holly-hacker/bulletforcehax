/// A bunch of hacky code, needed because the Dart SDK lowercases all header names

import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'dart:math';
import 'dart:typed_data';

Future<WebSocket> connectSocket(String host, int port, String protocol) async {
  Random r = Random();
  String key = base64.encode(List<int>.generate(16, (_) => r.nextInt(255)));

  Socket socket = await Socket.connect(host, port);

  // send request
  const crlf = "\r\n";
  var reqString =
      "GET / HTTP/1.1"
      "$crlf" "Host: $host:$port"
      "$crlf" "Upgrade: websocket"
      "$crlf" "Connection: Upgrade"
      "$crlf" "Sec-WebSocket-Version: 13"
      "$crlf" "Sec-WebSocket-Key: $key"
      "$crlf" "Origin: ws://$host:$port"
      "$crlf" "Sec-WebSocket-Protocol: $protocol" // this is the only case-sensitive header name
      "$crlf$crlf";
  socket.add(utf8.encode(reqString));

  // Completer to convert reactive callback to future, because
  var c = Completer<Uint8List>();
  var done = false;
  var sub = socket.listen((data) {
    if (done) return;

    // Find end of http packet, leave the rest to the websocket
    var pattern = ascii.encode("\r\n" * 2);
    var idx = findPattern(data, pattern);
    assert(idx != -1);
    var newData = data.sublist(idx + pattern.length);

    c.complete(newData);
    done = true;
  });

  var remainingData = await c.future;

  return WebSocket.fromUpgradedSocket(
    _DetachedSocket(socket, _HttpDetachedIncoming(sub, remainingData)),
    protocol: protocol,
    serverSide: false,
  );
}

// could be bayer-moore but who cares
int findPattern<T>(List<T> haystack, List<T> needle) {
  assert(haystack.length >= needle.length);
  for(int i = 0; i < haystack.length - needle.length + 1; ++i) {
    bool ok = true;
    for(int j = 0; j < needle.length; ++j) {
      if (haystack[i + j] != needle[j]) {
        ok = false;
        break;
      }
    }
    if (ok) {
      return i;
    }
  }

  return -1;
}

// The following code is taken and adapted from the dart SDK
// See:
// - https://github.com/dart-lang/sdk/blob/master/sdk/lib/_http/http_parser.dart
// - https://github.com/dart-lang/sdk/blob/master/sdk/lib/_http/http_impl.dart

/*
Copyright 2012, the Dart project authors.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are
met:
    * Redistributions of source code must retain the above copyright
      notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above
      copyright notice, this list of conditions and the following
      disclaimer in the documentation and/or other materials provided
      with the distribution.
    * Neither the name of Google Inc. nor the names of its
      contributors may be used to endorse or promote products derived
      from this software without specific prior written permission.
THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

/// The _HttpDetachedStreamSubscription takes a subscription and some extra data,
/// and makes it possible to "inject" the data in from of other data events
/// from the subscription.
///
/// It does so by overriding pause/resume, so that once the
/// _HttpDetachedStreamSubscription is resumed, it'll deliver the data before
/// resuming the underlying subscription.
class _HttpDetachedStreamSubscription implements StreamSubscription<Uint8List> {
  StreamSubscription<Uint8List> _subscription;
  Uint8List _injectData;
  bool _isCanceled = false;
  int _pauseCount = 1;
  Function _userOnData;
  bool _scheduled = false;

  _HttpDetachedStreamSubscription(
      this._subscription, this._injectData, this._userOnData);

  bool get isPaused => _subscription.isPaused;

  Future<T> asFuture<T>([T futureValue]) =>
      _subscription.asFuture<T>(futureValue);

  Future cancel() {
    _isCanceled = true;
    _injectData = null;
    return _subscription.cancel();
  }

  void onData(void handleData(Uint8List data)) {
    _userOnData = handleData;
    _subscription.onData(handleData);
  }

  void onDone(void handleDone()) {
    _subscription.onDone(handleDone);
  }

  void onError(Function handleError) {
    _subscription.onError(handleError);
  }

  void pause([Future resumeSignal]) {
    if (_injectData == null) {
      _subscription.pause(resumeSignal);
    } else {
      _pauseCount++;
      if (resumeSignal != null) {
        resumeSignal.whenComplete(resume);
      }
    }
  }

  void resume() {
    if (_injectData == null) {
      _subscription.resume();
    } else {
      _pauseCount--;
      _maybeScheduleData();
    }
  }

  void _maybeScheduleData() {
    if (_scheduled) return;
    if (_pauseCount != 0) return;
    _scheduled = true;
    scheduleMicrotask(() {
      _scheduled = false;
      if (_pauseCount > 0 || _isCanceled) return;
      var data = _injectData;
      _injectData = null;
      // To ensure that 'subscription.isPaused' is false, we resume the
      // subscription here. This is fine as potential events are delayed.
      _subscription.resume();
      if (_userOnData != null) {
        _userOnData(data);
      }
    });
  }
}

class _HttpDetachedIncoming extends Stream<Uint8List> {
  final StreamSubscription<Uint8List> subscription;
  final Uint8List bufferedData;

  _HttpDetachedIncoming(this.subscription, this.bufferedData);

  StreamSubscription<Uint8List> listen(void onData(Uint8List event),
      {Function onError, void onDone(), bool cancelOnError}) {
    if (subscription != null) {
      subscription
        ..onData(onData)
        ..onError(onError)
        ..onDone(onDone);
      if (bufferedData == null) {
        return subscription..resume();
      }
      return _HttpDetachedStreamSubscription(
          subscription, bufferedData, onData)
        ..resume();
    } else {
      return Stream<Uint8List>.fromIterable([bufferedData]).listen(onData,
          onError: onError, onDone: onDone, cancelOnError: cancelOnError);
    }
  }
}

class _DetachedSocket extends Stream<Uint8List> implements Socket {
  final Stream<Uint8List> _incoming;
  final Socket _socket;

  _DetachedSocket(this._socket, this._incoming);


  StreamSubscription<Uint8List> listen(void onData(Uint8List event),
      {Function onError, void onDone(), bool cancelOnError}) {
    return _incoming.listen(onData,
        onError: onError, onDone: onDone, cancelOnError: cancelOnError);
  }

  Encoding get encoding => _socket.encoding;

  set encoding(Encoding value) {
    _socket.encoding = value;
  }

  void write(Object obj) {
    _socket.write(obj);
  }

  void writeln([Object obj = ""]) {
    _socket.writeln(obj);
  }

  void writeCharCode(int charCode) {
    _socket.writeCharCode(charCode);
  }

  void writeAll(Iterable objects, [String separator = ""]) {
    _socket.writeAll(objects, separator);
  }

  void add(List<int> bytes) {
    _socket.add(bytes);
  }

  void addError(error, [StackTrace stackTrace]) =>
      _socket.addError(error, stackTrace);

  Future addStream(Stream<List<int>> stream) {
    return _socket.addStream(stream);
  }

  void destroy() {
    _socket.destroy();
  }

  Future flush() => _socket.flush();

  Future close() => _socket.close();

  Future get done => _socket.done;

  int get port => _socket.port;

  InternetAddress get address => _socket.address;

  InternetAddress get remoteAddress => _socket.remoteAddress;

  int get remotePort => _socket.remotePort;

  bool setOption(SocketOption option, bool enabled) {
    return _socket.setOption(option, enabled);
  }

  Uint8List getRawOption(RawSocketOption option) {
    return _socket.getRawOption(option);
  }

  void setRawOption(RawSocketOption option) {
    _socket.setRawOption(option);
  }

  Map _toJSON(bool ref) {
    return (_socket as dynamic)._toJSON(ref);
  }
}
