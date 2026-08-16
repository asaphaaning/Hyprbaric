import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<ScheduleStatus> _scheduleStatusStream() async* {
  final latest = ScheduleStatus.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in ScheduleStatus.rustSignalStream) {
    yield rustSignal.message;
  }
}

Stream<ScheduleCommandResult> _scheduleCommandResultStream() async* {
  final latest = ScheduleCommandResult.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in ScheduleCommandResult.rustSignalStream) {
    yield rustSignal.message;
  }
}

/// Latest daily schedule state published by Rust.
final scheduleStatusProvider = StreamProvider<ScheduleStatus>(
  (ref) => _scheduleStatusStream(),
);

/// Results from daily schedule commands.
final scheduleCommandResultProvider = StreamProvider<ScheduleCommandResult>(
  (ref) => _scheduleCommandResultStream(),
);
