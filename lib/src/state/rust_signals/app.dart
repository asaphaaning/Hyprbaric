import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<AppStatus> _appStatusStream() async* {
  final latest = AppStatus.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in AppStatus.rustSignalStream) {
    yield rustSignal.message;
  }
}

/// Watches immutable Hyprbaric runtime metadata forwarded from Rust.
final appStatusProvider = StreamProvider<AppStatus>(
  (ref) => _appStatusStream(),
);
