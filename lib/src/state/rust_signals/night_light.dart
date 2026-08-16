import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<NightLightStatus> _nightLightStatusStream() async* {
  final latest = NightLightStatus.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in NightLightStatus.rustSignalStream) {
    yield rustSignal.message;
  }
}

Stream<NightLightCommandResult> _nightLightCommandResultStream() async* {
  final latest = NightLightCommandResult.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in NightLightCommandResult.rustSignalStream) {
    yield rustSignal.message;
  }
}

/// Live hyprsunset state emitted from Rust.
final nightLightStatusProvider = StreamProvider<NightLightStatus>(
  (ref) => _nightLightStatusStream(),
);

/// Results from night-light commands.
final nightLightCommandResultProvider = StreamProvider<NightLightCommandResult>(
  (ref) => _nightLightCommandResultStream(),
);
