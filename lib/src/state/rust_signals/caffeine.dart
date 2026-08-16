import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<CaffeineStatus> _caffeineStatusStream() async* {
  final latest = CaffeineStatus.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in CaffeineStatus.rustSignalStream) {
    yield rustSignal.message;
  }
}

Stream<CaffeineCommandResult> _caffeineCommandResultStream() async* {
  final latest = CaffeineCommandResult.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in CaffeineCommandResult.rustSignalStream) {
    yield rustSignal.message;
  }
}

/// Live login1 Caffeine inhibitor state emitted from Rust.
final caffeineStatusProvider = StreamProvider<CaffeineStatus>(
  (ref) => _caffeineStatusStream(),
);

/// Results from Caffeine commands.
final caffeineCommandResultProvider = StreamProvider<CaffeineCommandResult>(
  (ref) => _caffeineCommandResultStream(),
);
