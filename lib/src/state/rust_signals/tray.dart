import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<TrayStatus> _trayStatusStream() async* {
  final latest = TrayStatus.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in TrayStatus.rustSignalStream) {
    yield rustSignal.message;
  }
}

Stream<TrayMenuStatus> _trayMenuStatusStream() async* {
  final latest = TrayMenuStatus.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in TrayMenuStatus.rustSignalStream) {
    yield rustSignal.message;
  }
}

/// Live StatusNotifier/AppIndicator tray state emitted from Rust.
final trayStatusProvider = StreamProvider<TrayStatus>(
  (ref) => _trayStatusStream(),
);

/// Latest DBusMenu opened for a tray item.
final trayMenuStatusProvider = StreamProvider<TrayMenuStatus>(
  (ref) => _trayMenuStatusStream(),
);
