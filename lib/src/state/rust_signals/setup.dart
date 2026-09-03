import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<SetupStatus> _setupStatusStream() async* {
  final latest = SetupStatus.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in SetupStatus.rustSignalStream) {
    yield rustSignal.message;
  }
}

Stream<SetupCommandResult> _setupCommandResultStream() async* {
  final latest = SetupCommandResult.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in SetupCommandResult.rustSignalStream) {
    yield rustSignal.message;
  }
}

/// Persisted first-run setup state.
final setupStatusProvider = StreamProvider<SetupStatus>(
  (ref) => _setupStatusStream(),
);

/// Results from setup completion persistence.
final setupCommandResultProvider = StreamProvider<SetupCommandResult>(
  (ref) => _setupCommandResultStream(),
);
