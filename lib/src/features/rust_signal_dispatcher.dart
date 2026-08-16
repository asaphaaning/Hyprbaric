import 'package:flutter/foundation.dart';

const bool _rustSignalLogEnabled = bool.fromEnvironment('HYPRBARIC_SIGNAL_LOG');

/// Sends a RINF signal while keeping development/test failures non-fatal.
void sendRustSignal(void Function() send, {String? debugLabel}) {
  try {
    send();
    if (_rustSignalLogEnabled && debugLabel != null) {
      debugPrint('Sent Rust signal: $debugLabel');
    }
  } catch (error, stackTrace) {
    debugPrint('Skipping Rust command (${debugLabel ?? 'unknown'}): $error');
    if (_rustSignalLogEnabled) {
      debugPrint('$stackTrace');
    }
  }
}
