import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<PowerStatus> _powerStatusStream() async* {
  final latest = PowerStatus.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in PowerStatus.rustSignalStream) {
    yield rustSignal.message;
  }
}

Stream<PowerCommandResult> _powerCommandResultStream() async* {
  final latest = PowerCommandResult.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in PowerCommandResult.rustSignalStream) {
    yield rustSignal.message;
  }
}

/// Live battery and power-profile state emitted from Rust.
final powerStatusProvider = StreamProvider<PowerStatus>(
  (ref) => _powerStatusStream(),
);

/// Results from power-profile commands.
final powerCommandResultProvider = StreamProvider<PowerCommandResult>(
  (ref) => _powerCommandResultStream(),
);
