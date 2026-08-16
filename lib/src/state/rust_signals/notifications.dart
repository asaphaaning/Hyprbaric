import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<NotificationStatus> _notificationStatusStream() async* {
  final latest = NotificationStatus.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in NotificationStatus.rustSignalStream) {
    yield rustSignal.message;
  }
}

/// Live notification-center state emitted from Rust.
final notificationStatusProvider = StreamProvider<NotificationStatus>(
  (ref) => _notificationStatusStream(),
);
