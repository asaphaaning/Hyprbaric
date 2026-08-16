import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<ShortcutSettingsSnapshot> _shortcutSettingsSnapshotStream() async* {
  final latest = ShortcutSettingsSnapshot.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in ShortcutSettingsSnapshot.rustSignalStream) {
    yield rustSignal.message;
  }
}

Stream<ShortcutSettingsCommandResult>
_shortcutSettingsCommandResultStream() async* {
  await for (final rustSignal
      in ShortcutSettingsCommandResult.rustSignalStream) {
    yield rustSignal.message;
  }
}

/// Current keybinding settings snapshot from Rust.
final shortcutSettingsSnapshotProvider =
    StreamProvider<ShortcutSettingsSnapshot>(
      (ref) => _shortcutSettingsSnapshotStream(),
    );

/// Results from keybinding settings commands.
final shortcutSettingsCommandResultProvider =
    StreamProvider<ShortcutSettingsCommandResult>(
      (ref) => _shortcutSettingsCommandResultStream(),
    );
