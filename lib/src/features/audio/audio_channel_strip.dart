import 'package:flutter/material.dart';

import '../../bindings/bindings.dart';
import '../../widgets/hypr_surface.dart';
import 'audio_chrome.dart';
import 'audio_fader.dart';

enum AudioMixerChannel {
  output('OUT', 'SPEAKERS', AudioMixerColors.output),
  input('IN', 'MIC', AudioMixerColors.input);

  const AudioMixerChannel(this.shortLabel, this.detailLabel, this.accent);

  final String shortLabel;
  final String detailLabel;
  final Color accent;
}

class AudioChannelStrip extends StatefulWidget {
  const AudioChannelStrip({
    super.key,
    required this.channel,
    required this.endpoint,
    required this.fallbackName,
    required this.onSetVolume,
    required this.onSetMuted,
  });

  final AudioMixerChannel channel;
  final AudioEndpoint? endpoint;
  final String fallbackName;
  final void Function(AudioEndpointKind kind, int volume) onSetVolume;
  final void Function(AudioEndpointKind kind, {required bool muted}) onSetMuted;

  @override
  State<AudioChannelStrip> createState() => AudioChannelStripState();
}

class AudioChannelStripState extends State<AudioChannelStrip> {
  String? _previewEndpointId;
  int? _previewVolume;

  @override
  void didUpdateWidget(covariant AudioChannelStrip oldWidget) {
    super.didUpdateWidget(oldWidget);
    final AudioEndpoint? value = widget.endpoint;
    if (value == null || value.id != _previewEndpointId) {
      _previewEndpointId = null;
      _previewVolume = null;
      return;
    }
    if (_previewVolume == value.volume) {
      _previewEndpointId = null;
      _previewVolume = null;
    }
  }

  void _setPreviewVolume(AudioEndpoint endpoint, int volume) {
    setState(() {
      _previewEndpointId = endpoint.id;
      _previewVolume = volume.clamp(0, 100);
    });
  }

  @override
  Widget build(BuildContext context) {
    final AudioEndpoint? value = widget.endpoint;
    final int? displayedVolume = value == null
        ? null
        : _previewEndpointId == value.id
        ? _previewVolume
        : value.volume;
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.center,
      children: <Widget>[
        AudioToggleChip(
          active: value != null && !value.muted,
          enabled: value != null,
          label: value == null
              ? widget.fallbackName
              : value.muted
              ? 'Unmute ${value.name}'
              : 'Mute ${value.name}',
          onPressed: value == null
              ? null
              : () => widget.onSetMuted(value.kind, muted: !value.muted),
        ),
        const SizedBox(height: 8),
        Text(
          displayedVolume?.toString() ?? '--',
          style: HyprTypography.compactMonoStrong.copyWith(
            color: AudioMixerColors.value,
            fontSize: HyprTypography.size(12),
            fontWeight: FontWeight.w500,
          ),
        ),
        const SizedBox(height: 13),
        if (value == null)
          AudioDisabledFader(accent: widget.channel.accent)
        else
          AudioFader(
            endpoint: value,
            accent: widget.channel.accent,
            onPreviewVolume: (int volume) => _setPreviewVolume(value, volume),
            onSetVolume: widget.onSetVolume,
          ),
        const SizedBox(height: 10),
        Text(
          '${widget.channel.shortLabel}\n${widget.channel.detailLabel}',
          textAlign: TextAlign.center,
          style: HyprTypography.compactMonoStrong.copyWith(
            color: AudioMixerColors.label,
            fontSize: HyprTypography.size(9),
            height: 1.2,
          ),
        ),
      ],
    );
  }
}

class AudioToggleChip extends StatelessWidget {
  const AudioToggleChip({
    super.key,
    required this.active,
    required this.enabled,
    required this.label,
    required this.onPressed,
  });

  final bool active;
  final bool enabled;
  final String label;
  final VoidCallback? onPressed;

  @override
  Widget build(BuildContext context) {
    return Semantics(
      container: true,
      button: true,
      label: label,
      child: ExcludeSemantics(
        child: InkWell(
          onTap: onPressed,
          customBorder: RoundedSuperellipseBorder(
            borderRadius: BorderRadius.circular(4),
          ),
          hoverColor: enabled ? const Color(0x12FFFFFF) : Colors.transparent,
          splashColor: Colors.transparent,
          highlightColor: enabled
              ? const Color(0x10FFFFFF)
              : Colors.transparent,
          child: DecoratedBox(
            decoration: ShapeDecoration(
              color: active ? const Color(0x1200B8C9) : Colors.transparent,
              shape: RoundedSuperellipseBorder(
                borderRadius: BorderRadius.circular(4),
                side: BorderSide(
                  color: active
                      ? const Color(0x303BCB7C)
                      : const Color(0x26AAB5C0),
                ),
              ),
            ),
            child: SizedBox(
              width: 34,
              height: 20,
              child: Center(
                child: Text(
                  active ? 'ON' : 'OFF',
                  style: HyprTypography.compactMonoStrong.copyWith(
                    color: enabled
                        ? AudioMixerColors.quiet
                        : HyprColors.textFaint,
                    fontSize: HyprTypography.size(8.5),
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
