import 'package:riverpod_annotation/riverpod_annotation.dart';

import '../../bindings/bindings.dart';
import '../../state/providers.dart';
import '../rust_commands.dart';

part 'audio_controller.g.dart';

@Riverpod(keepAlive: true)
class AudioController extends _$AudioController {
  @override
  void build() {}

  void setVolume(AudioEndpointKind kind, int volume) {
    final int clampedVolume = volume.clamp(0, 100).toInt();
    if (kind == AudioEndpointKind.output) {
      ref
          .read(transientOverlayProvider.notifier)
          .showVolumeOsd(value: clampedVolume, muted: false);
    }
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(AudioIntent.setVolume(kind: kind, volume: clampedVolume));
  }

  void setMuted(AudioEndpointKind kind, {required bool muted}) {
    if (kind == AudioEndpointKind.output) {
      final int volume =
          ref.read(currentAudioStatusProvider)?.output?.volume ?? 0;
      ref
          .read(transientOverlayProvider.notifier)
          .showVolumeOsd(value: volume, muted: muted);
    }
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(AudioIntent.setMuted(kind: kind, muted: muted));
  }

  void setBrightness(int value) {
    final int clampedValue = value.clamp(0, 100).toInt();
    ref
        .read(transientOverlayProvider.notifier)
        .showBrightnessOsd(value: clampedValue);
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(BrightnessIntent.setLevel(value: clampedValue));
  }
}
