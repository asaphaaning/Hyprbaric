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
      padding: const EdgeInsets.fromLTRB(16, 11, 16, 10),
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
    final double illumination = brightnessStatus?.isAvailable ?? false
        ? (brightnessStatus!.displayValue / 100).clamp(0, 1).toDouble()
        : 0;

    return SizedBox(
      height: 457,
      child: Stack(
        clipBehavior: Clip.none,
        children: <Widget>[
          Positioned(
            top: 0,
            left: 16,
            right: 16,
            child: _BrightnessDeck(illumination: illumination),
          ),
          Positioned(
            top: 117,
            left: 16,
            right: 16,
            height: 340,
            child: ClipPath(
              clipper: const _ConsoleNotchClipper(),
              child: ColoredBox(
                color: AudioMixerColors.console,
                child: Padding(
                  padding: const EdgeInsets.fromLTRB(14, 74, 14, 12),
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
  const _BrightnessDeck({required this.illumination});

  final double illumination;

  @override
  Widget build(BuildContext context) {
    return CustomPaint(
      painter: _BrightnessDeckPainter(illumination: illumination),
      child: const SizedBox(height: 151),
    );
  }
}

class _BrightnessDeckPainter extends CustomPainter {
  const _BrightnessDeckPainter({required this.illumination});

  final double illumination;

  @override
  void paint(Canvas canvas, Size size) {
    final Rect bounds = Offset.zero & size;
    final Paint deckPaint = Paint()
      ..shader = const LinearGradient(
        begin: Alignment.topCenter,
        end: Alignment.bottomCenter,
        colors: <Color>[
          AudioMixerColors.deckTop,
          AudioMixerColors.deckMiddle,
          AudioMixerColors.deckBottom,
        ],
        stops: <double>[0, 0.48, 1],
      ).createShader(bounds);
    final RRect deck = RRect.fromRectAndCorners(
      Rect.fromLTWH(0, 0, size.width, 131),
      topLeft: const Radius.circular(16),
      topRight: const Radius.circular(16),
    );
    final Path silhouette = Path()
      ..addRRect(deck)
      ..addOval(
        Rect.fromCircle(center: Offset(size.width / 2, 122), radius: 74.5),
      );

    canvas.save();
    canvas.clipPath(silhouette);
    canvas.drawRect(bounds, deckPaint);
    canvas.drawRect(
      bounds,
      Paint()
        ..shader = const RadialGradient(
          center: Alignment(-0.42, -0.92),
          radius: 1.1,
          colors: <Color>[
            Color(0x16FFFFFF),
            Color(0x08FFFFFF),
            Colors.transparent,
          ],
          stops: <double>[0, 0.42, 1],
        ).createShader(bounds),
    );

    if (illumination > 0) {
      final Offset lampCenter = Offset(size.width / 2 - 10, 114);
      final Rect lampBounds = Rect.fromCircle(center: lampCenter, radius: 96);
      canvas.drawCircle(
        lampCenter,
        96,
        Paint()
          ..shader = RadialGradient(
            colors: <Color>[
              const Color(0x18F2D77A).withValues(alpha: 0.09 * illumination),
              const Color(0x0CF2D77A).withValues(alpha: 0.045 * illumination),
              Colors.transparent,
            ],
            stops: const <double>[0, 0.48, 1],
          ).createShader(lampBounds)
          ..blendMode = BlendMode.plus,
      );
    }

    canvas.restore();
    canvas.drawLine(
      const Offset(12, 0),
      Offset(size.width - 12, 0),
      Paint()
        ..color = Colors.white.withValues(alpha: 0.07)
        ..strokeWidth = 1,
    );
  }

  @override
  bool shouldRepaint(_BrightnessDeckPainter oldDelegate) =>
      illumination != oldDelegate.illumination;
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
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 9),
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
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 10),
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
