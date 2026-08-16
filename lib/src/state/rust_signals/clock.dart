import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<ClockStatus> _clockStatusStream() async* {
  final latest = ClockStatus.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in ClockStatus.rustSignalStream) {
    yield rustSignal.message;
  }
}

/// Live clock and calendar state calculated by Rust.
final clockStatusProvider = StreamProvider<ClockStatus>(
  (ref) => _clockStatusStream(),
);
