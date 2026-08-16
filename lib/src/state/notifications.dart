import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../bindings/bindings.dart';
import 'rust_signals.dart';

/// Raw notification-center updates emitted from Rust via RINF.
final currentNotificationSignalProvider = notificationStatusProvider;

/// The latest available notification snapshot for the bar.
final currentNotificationStatusProvider = Provider<NotificationStatus?>((ref) {
  final AsyncValue<NotificationStatus> status = ref.watch(
    currentNotificationSignalProvider,
  );
  return status.asData?.value;
});
