import 'package:flutter/material.dart';

import '../../bindings/bindings.dart';
import '../../widgets/hypr_surface.dart';
import 'audio_chrome.dart';
import 'audio_fader.dart';

enum AudioMixerChannel {
  output('OUT', AudioMixerColors.output),
  input('MIC', AudioMixerColors.input);

  const AudioMixerChannel(this.label, this.accent);

  final String label;
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
        Text(
          widget.channel.label,
          style: HyprTypography.compactMonoStrong.copyWith(
            color: value == null || value.muted
                ? HyprColors.textFaint
                : AudioMixerColors.label,
            fontSize: HyprTypography.size(9),
            letterSpacing: 1.6,
          ),
        ),
        const SizedBox(height: 8),
        if (value == null)
          AudioDisabledFader(accent: widget.channel.accent)
        else
          AudioFader(
            endpoint: value,
            accent: widget.channel.accent,
            onPreviewVolume: (int volume) => _setPreviewVolume(value, volume),
            onSetVolume: widget.onSetVolume,
          ),
        const SizedBox(height: 8),
        AudioDbReadout(
          value: displayedVolume,
          muted: value?.muted ?? true,
          accent: widget.channel.accent,
        ),
        const SizedBox(height: 7),
        AudioMuteButton(
          muted: value?.muted ?? true,
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
      ],
    );
  }
}

/// Recessed decibel display for an audio endpoint.
class AudioDbReadout extends StatelessWidget {
  const AudioDbReadout({
    super.key,
    required this.value,
    required this.muted,
    required this.accent,
  });

  final int? value;
  final bool muted;
  final Color accent;

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 58,
      padding: const EdgeInsets.symmetric(vertical: 4),
      decoration: BoxDecoration(
        color: AudioMixerColors.well,
        borderRadius: BorderRadius.circular(5),
        border: Border.all(color: const Color(0x4D000000)),
        boxShadow: const <BoxShadow>[
          BoxShadow(
            color: Color(0x7A000000),
            blurRadius: 3,
            offset: Offset(0, 1),
            blurStyle: BlurStyle.inner,
          ),
        ],
      ),
      child: Text.rich(
        TextSpan(
          text: value == null
              ? '--'
              : audioDecibelReadout(value!, muted: muted),
          children: const <InlineSpan>[
            TextSpan(
              text: ' dB',
              style: TextStyle(color: HyprColors.textFaint, fontSize: 7),
            ),
          ],
        ),
        textAlign: TextAlign.center,
        style: HyprTypography.compactMonoStrong.copyWith(
          color: muted ? HyprColors.textFaint : accent,
          fontSize: HyprTypography.size(10.5),
          fontWeight: FontWeight.w500,
        ),
      ),
    );
  }
}

/// Compact mute control for a mixer channel.
class AudioMuteButton extends StatelessWidget {
  const AudioMuteButton({
    super.key,
    required this.muted,
    required this.enabled,
    required this.label,
    required this.onPressed,
  });

  final bool muted;
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
              color: muted ? const Color(0x2ED45146) : AudioMixerColors.well,
              shape: RoundedSuperellipseBorder(
                borderRadius: BorderRadius.circular(4),
                side: BorderSide(
                  color: muted
                      ? const Color(0x70D45146)
                      : const Color(0x24000000),
                ),
              ),
            ),
            child: SizedBox(
              width: 58,
              height: 21,
              child: Center(
                child: Text(
                  'M',
                  style: HyprTypography.compactMonoStrong.copyWith(
                    color: enabled
                        ? muted
                              ? AudioMixerColors.peak
                              : AudioMixerColors.quiet
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
