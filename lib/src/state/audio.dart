import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../bindings/bindings.dart';
import 'rust_signals.dart';

/// Raw audio updates emitted from Rust via RINF.
final currentAudioSignalProvider = audioStatusProvider;

/// The latest available audio snapshot for the bar, or `null` when unavailable.
final currentAudioStatusProvider = Provider<AudioStatusAvailable?>((ref) {
  final AsyncValue<AudioStatus> status = ref.watch(currentAudioSignalProvider);
  return status.when(
    data: (AudioStatus value) => switch (value) {
      AudioStatusAvailable() => value,
      AudioStatusUnavailable() => null,
      _ => null,
    },
    loading: () => null,
    error: (_, _) => null,
  );
});
