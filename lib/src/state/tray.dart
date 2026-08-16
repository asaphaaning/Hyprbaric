import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../bindings/bindings.dart';
import 'rust_signals.dart';

/// Raw tray updates emitted from Rust via RINF.
final currentTraySignalProvider = trayStatusProvider;

/// The latest available tray snapshot for the bar, or `null` before Rust emits one.
final currentTrayStatusProvider = Provider<TrayStatus?>((ref) {
  final AsyncValue<TrayStatus> status = ref.watch(currentTraySignalProvider);
  return status.asData?.value;
});

/// The latest opened tray menu, or `null` before a tray item opens one.
final currentTrayMenuStatusProvider = Provider<TrayMenuStatus?>((ref) {
  final AsyncValue<TrayMenuStatus> status = ref.watch(trayMenuStatusProvider);
  return status.asData?.value;
});
