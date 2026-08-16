import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<SessionActionAvailability> _sessionAvailabilityStream() async* {
  final latest = SessionActionAvailability.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in SessionActionAvailability.rustSignalStream) {
    yield rustSignal.message;
  }
}

Stream<SessionCommandResult> _sessionCommandResultStream() async* {
  final latest = SessionCommandResult.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in SessionCommandResult.rustSignalStream) {
    yield rustSignal.message;
  }
}

/// Availability flags for optional session-launcher actions.
final sessionActionAvailabilityProvider =
    StreamProvider<SessionActionAvailability>(
      (ref) => _sessionAvailabilityStream(),
    );

/// Result stream for session actions executed by Rust.
final sessionCommandResultProvider = StreamProvider<SessionCommandResult>(
  (ref) => _sessionCommandResultStream(),
);
