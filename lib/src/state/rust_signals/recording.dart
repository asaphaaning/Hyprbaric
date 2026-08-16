import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<RecordingStatus> _recordingStatusStream() async* {
  final latest = RecordingStatus.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in RecordingStatus.rustSignalStream) {
    yield rustSignal.message;
  }
}

Stream<RecordingCommandResult> _recordingCommandResultStream() async* {
  final latest = RecordingCommandResult.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in RecordingCommandResult.rustSignalStream) {
    yield rustSignal.message;
  }
}

/// Live screen recording state emitted from Rust.
final recordingStatusProvider = StreamProvider<RecordingStatus>(
  (ref) => _recordingStatusStream(),
);

/// Results from screen recording commands.
final recordingCommandResultProvider = StreamProvider<RecordingCommandResult>(
  (ref) => _recordingCommandResultStream(),
);
