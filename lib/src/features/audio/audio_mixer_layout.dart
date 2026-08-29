import 'package:flutter/material.dart';

import '../../bindings/bindings.dart';
import '../../widgets/hypr_surface.dart';
import 'audio_channel_strip.dart';
import 'audio_chrome.dart';
import 'brightness_control.dart';

/// Mixer title and the currently selected output endpoint.
class AudioMixerHeader extends StatelessWidget {
  const AudioMixerHeader({super.key, required this.output});

  final AudioEndpoint? output;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(14, 11, 14, 10),
      child: Row(
        children: <Widget>[
          Text(
            'MIXER',
            style: HyprTypography.compactMonoStrong.copyWith(
              color: AudioMixerColors.label,
              fontSize: HyprTypography.size(10),
              letterSpacing: 2.2,
            ),
          ),
          const Spacer(),
          Flexible(
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 176),
              child: Container(
                height: 26,
                padding: const EdgeInsets.symmetric(horizontal: 9),
                decoration: BoxDecoration(
                  color: AudioMixerColors.well,
                  borderRadius: BorderRadius.circular(6),
                  border: Border.all(color: const Color(0x52000000)),
                  boxShadow: const <BoxShadow>[
                    BoxShadow(
                      color: Color(0x80000000),
                      blurRadius: 3,
                      offset: Offset(0, 1),
                      blurStyle: BlurStyle.inner,
                    ),
                  ],
                ),
                child: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: <Widget>[
                    Flexible(
                      child: Text(
                        output?.name ?? 'No output device',
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                        style: HyprTypography.compactMonoStrong.copyWith(
                          color: output == null
                              ? HyprColors.textFaint
                              : const Color(0xFFD1EEF0),
                          fontSize: HyprTypography.size(9.5),
                          fontWeight: FontWeight.w500,
                          letterSpacing: 0.4,
                        ),
                      ),
                    ),
                    const SizedBox(width: 7),
                    Text(
                      '▾',
                      style: HyprTypography.compactMonoStrong.copyWith(
                        color: HyprColors.textFaint,
                        fontSize: HyprTypography.size(8),
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

/// Layered brightness deck and two-channel audio console.
class AudioMixerStage extends StatelessWidget {
  const AudioMixerStage({
    super.key,
    required this.output,
    required this.input,
    required this.brightnessStatus,
    required this.brightnessLoading,
    required this.onSetVolume,
    required this.onSetMuted,
    required this.onSetBrightness,
  });

  final AudioEndpoint? output;
  final AudioEndpoint? input;
  final BrightnessStatus? brightnessStatus;
  final bool brightnessLoading;
  final void Function(AudioEndpointKind kind, int volume) onSetVolume;
  final void Function(AudioEndpointKind kind, {required bool muted}) onSetMuted;
  final ValueChanged<int> onSetBrightness;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 457,
      child: Stack(
        clipBehavior: Clip.none,
        children: <Widget>[
          const Positioned(
            top: 0,
            left: 12,
            right: 12,
            child: _BrightnessDeck(),
          ),
          Positioned(
            top: 117,
            left: 12,
            right: 12,
            height: 340,
            child: ClipPath(
              clipper: const _ConsoleNotchClipper(),
              child: ColoredBox(
                color: AudioMixerColors.console,
                child: Padding(
                  padding: const EdgeInsets.fromLTRB(12, 74, 12, 12),
                  child: Row(
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
                      const SizedBox(width: 10),
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
                ),
              ),
            ),
          ),
          Positioned(
            top: 13,
            left: 0,
            right: 0,
            child: BrightnessControl(
              status: brightnessStatus,
              loading: brightnessLoading,
              presentation: BrightnessControlPresentation.console,
              onSetBrightness: onSetBrightness,
            ),
          ),
        ],
      ),
    );
  }
}

class _BrightnessDeck extends StatelessWidget {
  const _BrightnessDeck();

  @override
  Widget build(BuildContext context) {
    return const CustomPaint(
      painter: _BrightnessDeckPainter(),
      child: SizedBox(height: 151),
    );
  }
}

class _BrightnessDeckPainter extends CustomPainter {
  const _BrightnessDeckPainter();

  @override
  void paint(Canvas canvas, Size size) {
    final Paint paint = Paint()
      ..shader = const LinearGradient(
        begin: Alignment.topCenter,
        end: Alignment.bottomCenter,
        colors: <Color>[
          AudioMixerColors.deckTop,
          AudioMixerColors.deckMiddle,
          AudioMixerColors.deckBottom,
        ],
        stops: <double>[0, 0.48, 1],
      ).createShader(Offset.zero & size);
    final RRect deck = RRect.fromRectAndCorners(
      Rect.fromLTWH(0, 0, size.width, 131),
      topLeft: const Radius.circular(16),
      topRight: const Radius.circular(16),
    );
    canvas.drawRRect(deck, paint);
    canvas.drawCircle(Offset(size.width / 2, 122), 74.5, paint);
    canvas.drawLine(
      const Offset(12, 0),
      Offset(size.width - 12, 0),
      Paint()
        ..color = Colors.white.withValues(alpha: 0.07)
        ..strokeWidth = 1,
    );
  }

  @override
  bool shouldRepaint(_BrightnessDeckPainter oldDelegate) => false;
}

class _ConsoleNotchClipper extends CustomClipper<Path> {
  const _ConsoleNotchClipper();

  @override
  Path getClip(Size size) {
    final Path console = Path()
      ..addRRect(
        RRect.fromRectAndCorners(
          Offset.zero & size,
          bottomLeft: const Radius.circular(16),
          bottomRight: const Radius.circular(16),
        ),
      );
    final Path clearance = Path()
      ..addOval(
        Rect.fromCircle(center: Offset(size.width / 2, -6), radius: 74),
      );
    return Path.combine(PathOperation.difference, console, clearance);
  }

  @override
  bool shouldReclip(_ConsoleNotchClipper oldClipper) => false;
}

/// Compact aggregate meter for the output endpoint.
class AudioMasterRail extends StatelessWidget {
  const AudioMasterRail({super.key, required this.output});

  final AudioEndpoint? output;

  @override
  Widget build(BuildContext context) {
    final int volume = output?.volume ?? 0;
    final bool muted = output?.muted ?? true;
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 5),
      decoration: const BoxDecoration(
        border: Border(
          top: BorderSide(color: Color(0x82000000)),
          bottom: BorderSide(color: Color(0x24000000)),
        ),
      ),
      child: Row(
        children: <Widget>[
          Text(
            'MASTER',
            style: HyprTypography.compactMonoStrong.copyWith(
              color: HyprColors.textFaint,
              fontSize: HyprTypography.size(8),
              letterSpacing: 1.6,
            ),
          ),
          const SizedBox(width: 10),
          Expanded(
            child: SizedBox(
              height: 8,
              child: CustomPaint(
                painter: AudioMasterMeterPainter(
                  value: muted ? 0 : volume / 100,
                ),
              ),
            ),
          ),
          const SizedBox(width: 10),
          SizedBox(
            width: 54,
            child: Text.rich(
              TextSpan(
                text: audioDecibelReadout(volume, muted: muted),
                children: const <InlineSpan>[
                  TextSpan(
                    text: ' dB',
                    style: TextStyle(color: HyprColors.textFaint, fontSize: 7),
                  ),
                ],
              ),
              textAlign: TextAlign.right,
              style: HyprTypography.compactMonoStrong.copyWith(
                color: AudioMixerColors.value,
                fontSize: HyprTypography.size(10.5),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

/// Paints the segmented output level used by [AudioMasterRail].
class AudioMasterMeterPainter extends CustomPainter {
  const AudioMasterMeterPainter({required this.value});

  final double value;

  @override
  void paint(Canvas canvas, Size size) {
    final RRect well = RRect.fromRectAndRadius(
      Offset.zero & size,
      const Radius.circular(3),
    );
    canvas.drawRRect(well, Paint()..color = AudioMixerColors.rail);
    const int segments = 32;
    const double gap = 1.5;
    final double width = (size.width - 4 - gap * (segments - 1)) / segments;
    final int active = (value.clamp(0, 1) * segments).round();
    for (int index = 0; index < segments; index += 1) {
      final double threshold = index / segments;
      final Color tone = threshold >= .90
          ? AudioMixerColors.peak
          : threshold >= .76
          ? AudioMixerColors.warning
          : AudioMixerColors.output;
      final Rect segment = Rect.fromLTWH(
        2 + index * (width + gap),
        2,
        width,
        size.height - 4,
      );
      canvas.drawRRect(
        RRect.fromRectAndRadius(segment, const Radius.circular(1)),
        Paint()..color = index < active ? tone : AudioMixerColors.slot,
      );
    }
  }

  @override
  bool shouldRepaint(AudioMasterMeterPainter oldDelegate) =>
      value != oldDelegate.value;
}

/// Reference audio format and external mixer affordance.
class AudioMixerFooter extends StatelessWidget {
  const AudioMixerFooter({super.key, required this.onOpenMixer});

  final VoidCallback onOpenMixer;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(14, 4, 14, 5),
      child: Row(
        children: <Widget>[
          Expanded(
            child: FittedBox(
              fit: BoxFit.scaleDown,
              alignment: Alignment.centerLeft,
              child: Text(
                '3 CH · 48 KHZ · 24-BIT',
                style: HyprTypography.compactMonoStrong.copyWith(
                  color: HyprColors.textFaint,
                  fontSize: HyprTypography.size(8.5),
                  letterSpacing: 0.8,
                ),
              ),
            ),
          ),
          const SizedBox(width: 18),
          Flexible(
            child: Semantics(
              container: true,
              button: true,
              label: 'Open pavucontrol',
              child: InkWell(
                onTap: onOpenMixer,
                borderRadius: BorderRadius.circular(4),
                hoverColor: const Color(0x0FFFFFFF),
                splashColor: Colors.transparent,
                child: Padding(
                  padding: const EdgeInsets.symmetric(vertical: 2),
                  child: FittedBox(
                    fit: BoxFit.scaleDown,
                    alignment: Alignment.centerRight,
                    child: Text(
                      'PAVUCONTROL →',
                      style: HyprTypography.compactMonoStrong.copyWith(
                        color: AudioMixerColors.label,
                        fontSize: HyprTypography.size(8.5),
                        letterSpacing: 1.2,
                      ),
                    ),
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
