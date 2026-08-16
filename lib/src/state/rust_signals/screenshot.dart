import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<ScreenshotCommandResult> _screenshotCommandResultStream() async* {
  final latest = ScreenshotCommandResult.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in ScreenshotCommandResult.rustSignalStream) {
    yield rustSignal.message;
  }
}

/// Results from screenshot capture commands.
final screenshotCommandResultProvider = StreamProvider<ScreenshotCommandResult>(
  (ref) => _screenshotCommandResultStream(),
);
