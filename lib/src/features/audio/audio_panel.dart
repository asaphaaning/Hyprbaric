import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';
import '../../widgets/hypr_surface.dart';
import '../../widgets/primitives/primitives.dart';
import 'audio_channel_strip.dart';
import 'audio_chrome.dart';
import 'brightness_control.dart';

class AudioPanel extends StatelessWidget {
  const AudioPanel({
    super.key,
    required this.borderRadius,
    required this.status,
    required this.brightnessStatus,
    required this.onSetVolume,
    required this.onSetMuted,
    required this.onSetBrightness,
  });

  final BorderRadius borderRadius;
  final AsyncValue<AudioStatus> status;
  final AsyncValue<BrightnessStatus> brightnessStatus;
  final void Function(AudioEndpointKind kind, int volume) onSetVolume;
  final void Function(AudioEndpointKind kind, {required bool muted}) onSetMuted;
  final ValueChanged<int> onSetBrightness;

  @override
  Widget build(BuildContext context) {
    final AudioStatus? snapshot = status.asData?.value;
    final AudioEndpoint? output = snapshot?.output;
    final AudioEndpoint? input = snapshot?.input;
    final bool unavailable =
        snapshot != null &&
        (!snapshot.isAvailable || (output == null && input == null));

    return HyprPopoverPanel(
      borderRadius: borderRadius,
      constraints: const BoxConstraints(minWidth: 288, maxWidth: 288),
      padding: const EdgeInsets.fromLTRB(18, 14, 18, 16),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: <Widget>[
          HyprSectionLabel(
            'Audio & Display',
            color: AudioMixerColors.label,
            fontSize: HyprTypography.size(10),
            fontWeight: FontWeight.w700,
            letterSpacing: 0,
          ),
          const SizedBox(height: 10),
          BrightnessControl(
            status: brightnessStatus.asData?.value,
            loading: brightnessStatus.isLoading,
            onSetBrightness: onSetBrightness,
          ),
          const HyprSectionBreak(before: 13, after: 12),
          HyprSectionLabel(
            'Audio Mixer',
            color: AudioMixerColors.label,
            fontSize: HyprTypography.size(10),
            fontWeight: FontWeight.w700,
            letterSpacing: 0,
          ),
          const SizedBox(height: 17),
          Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: <Widget>[
              Expanded(
                child: AudioChannelStrip(
                  channel: AudioMixerChannel.output,
                  endpoint: output,
                  fallbackName: 'No output device',
                  onSetVolume: onSetVolume,
                  onSetMuted: onSetMuted,
                ),
              ),
              const SizedBox(width: 16),
              const SizedBox(height: 230, child: AudioMixerDivider()),
              const SizedBox(width: 16),
              Expanded(
                child: AudioChannelStrip(
                  channel: AudioMixerChannel.input,
                  endpoint: input,
                  fallbackName: 'No input device',
                  onSetVolume: onSetVolume,
                  onSetMuted: onSetMuted,
                ),
              ),
            ],
          ),
          if (status.isLoading) ...<Widget>[
            const SizedBox(height: 14),
            const AudioMessage(message: 'Reading audio devices...'),
          ] else if (unavailable) ...<Widget>[
            const SizedBox(height: 14),
            AudioMessage(
              message: snapshot.message ?? 'Audio controls are unavailable.',
            ),
          ],
        ],
      ),
    );
  }
}
