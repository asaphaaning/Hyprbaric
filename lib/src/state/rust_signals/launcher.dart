import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<AppLauncherResults> _appLauncherResultsStream() async* {
  final latest = AppLauncherResults.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in AppLauncherResults.rustSignalStream) {
    yield rustSignal.message;
  }
}

Stream<AppLaunchResult> _appLaunchResultStream() async* {
  final latest = AppLaunchResult.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in AppLaunchResult.rustSignalStream) {
    yield rustSignal.message;
  }
}

/// Live app-launcher results emitted from Rust.
final appLauncherResultsProvider = StreamProvider<AppLauncherResults>(
  (ref) => _appLauncherResultsStream(),
);

/// Launch results for app-launcher activation requests.
final appLaunchResultProvider = StreamProvider<AppLaunchResult>(
  (ref) => _appLaunchResultStream(),
);
