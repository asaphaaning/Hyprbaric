import 'dart:math' as math;

import 'package:flutter/material.dart';

import '../../bindings/bindings.dart';
import '../../widgets/hypr_surface.dart';
import '../../widgets/primitives/primitives.dart';
import 'audio_chrome.dart';

class AudioFader extends StatefulWidget {
  const AudioFader({
    super.key,
    required this.endpoint,
    required this.accent,
    required this.onPreviewVolume,
    required this.onSetVolume,
  });

  final AudioEndpoint endpoint;
  final Color accent;
  final ValueChanged<int> onPreviewVolume;
  final void Function(AudioEndpointKind kind, int volume) onSetVolume;

  @override
  State<AudioFader> createState() => AudioFaderState();
}

class AudioFaderState extends State<AudioFader> {
  late double _volume;
  late final HyprLiveValue _commits;
  bool _hovered = false;

  @override
  void initState() {
    super.initState();
    _volume = widget.endpoint.volume.toDouble();
    _commits = HyprLiveValue(
      initialValue: widget.endpoint.volume,
      commitInterval: const Duration(milliseconds: 75),
    );
  }

  @override
  void didUpdateWidget(covariant AudioFader oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (!_commits.active &&
        (oldWidget.endpoint.volume != widget.endpoint.volume ||
            oldWidget.endpoint.id != widget.endpoint.id)) {
      _volume = widget.endpoint.volume.toDouble();
      _commits.sync(widget.endpoint.volume);
    }
  }

  void _setFromLocalPosition(
    Offset position,
    Size size, {
    bool send = false,
    bool force = false,
  }) {
    final double next = (100 - (position.dy / size.height) * 100).clamp(0, 100);
    final int volume = next.round();
    setState(() => _volume = next);
    widget.onPreviewVolume(volume);
    if (send) {
      _sendVolume(force: force);
    }
  }

  void _sendVolume({bool force = false}) {
    final int volume = _volume.round();
    _commits.preview(volume);
    final int? next = _commits.commit(force: force);
    if (next != null) {
      widget.onSetVolume(widget.endpoint.kind, next);
    }
  }

  @override
  Widget build(BuildContext context) {
    final AudioEndpoint endpoint = widget.endpoint;
    return Semantics(
      slider: true,
      label: '${endpoint.name} volume',
      value: _volume.round().toString(),
      increasedValue: math.min(100, _volume.round() + 5).toString(),
      decreasedValue: math.max(0, _volume.round() - 5).toString(),
      onIncrease: () {
        setState(() => _volume = math.min(100, _volume + 5));
        widget.onPreviewVolume(_volume.round());
        _sendVolume(force: true);
      },
      onDecrease: () {
        setState(() => _volume = math.max(0, _volume - 5));
        widget.onPreviewVolume(_volume.round());
        _sendVolume(force: true);
      },
      child: MouseRegion(
        cursor: SystemMouseCursors.resizeUpDown,
        onEnter: (_) => setState(() => _hovered = true),
        onExit: (_) => setState(() => _hovered = false),
        child: SizedBox(
          width: 41,
          height: 152,
          child: LayoutBuilder(
            builder: (BuildContext context, BoxConstraints constraints) {
              final Size size = constraints.biggest;
              return GestureDetector(
                behavior: HitTestBehavior.opaque,
                onTapDown: (TapDownDetails details) {
                  _setFromLocalPosition(
                    details.localPosition,
                    size,
                    send: true,
                    force: true,
                  );
                },
                onTapUp: (_) => _sendVolume(force: true),
                onVerticalDragStart: (DragStartDetails details) {
                  setState(() => _commits.begin(_volume.round()));
                  _setFromLocalPosition(
                    details.localPosition,
                    size,
                    send: true,
                    force: true,
                  );
                },
                onVerticalDragUpdate: (DragUpdateDetails details) {
                  _setFromLocalPosition(
                    details.localPosition,
                    size,
                    send: true,
                  );
                },
                onVerticalDragEnd: (_) {
                  setState(() => _commits.end());
                  _sendVolume(force: true);
                },
                onVerticalDragCancel: () {
                  setState(() => _commits.end());
                  _sendVolume(force: true);
                },
                child: CustomPaint(
                  painter: AudioFaderPainter(
                    value: _volume / 100,
                    accent: widget.accent,
                    emphasized: _hovered || _commits.active,
                    muted: endpoint.muted,
                  ),
                ),
              );
            },
          ),
        ),
      ),
    );
  }
}

class AudioDisabledFader extends StatelessWidget {
  const AudioDisabledFader({super.key, required this.accent});

  final Color accent;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 41,
      height: 152,
      child: CustomPaint(
        painter: AudioFaderPainter(
          value: 0,
          accent: accent,
          emphasized: false,
          muted: true,
        ),
      ),
    );
  }
}

class AudioFaderPainter extends CustomPainter {
  const AudioFaderPainter({
    required this.value,
    required this.accent,
    required this.emphasized,
    required this.muted,
  });

  final double value;
  final Color accent;
  final bool emphasized;
  final bool muted;

  @override
  void paint(Canvas canvas, Size size) {
    final Rect meter = Rect.fromLTWH(0, 0, 6, size.height);
    final RRect meterWell = RRect.fromRectAndRadius(
      meter,
      const Radius.circular(3),
    );
    canvas.drawRRect(meterWell, Paint()..color = AudioMixerColors.rail);
    canvas.drawRRect(
      meterWell.deflate(0.5),
      Paint()
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1
        ..color = AudioMixerColors.railBorder,
    );

    const int segments = 24;
    const double gap = 1.5;
    final double segmentHeight =
        (meter.height - 4 - gap * (segments - 1)) / segments;
    final int activeSegments = muted
        ? 0
        : (value.clamp(0, 1) * segments).round().clamp(0, segments);
    final Paint segmentPaint = Paint();
    for (int i = 0; i < segments; i += 1) {
      final bool active = i < activeSegments;
      final double top = meter.bottom - 2 - (i + 1) * segmentHeight - i * gap;
      final Rect segment = Rect.fromLTWH(
        meter.left + 2,
        top,
        meter.width - 4,
        segmentHeight,
      );
      final double threshold = i / segments;
      final Color ladder = threshold >= 0.90
          ? AudioMixerColors.peak
          : threshold >= 0.76
          ? AudioMixerColors.warning
          : AudioMixerColors.output;
      segmentPaint.color = active ? ladder : AudioMixerColors.slot;
      canvas.drawRRect(
        RRect.fromRectAndRadius(segment, const Radius.circular(1.5)),
        segmentPaint,
      );
    }

    final Rect track = Rect.fromLTWH(15, 0, 26, size.height);
    final Rect slot = Rect.fromCenter(
      center: track.center,
      width: 6,
      height: track.height,
    );
    final RRect slotShape = RRect.fromRectAndRadius(
      slot,
      const Radius.circular(3),
    );
    canvas.drawRRect(
      slotShape,
      Paint()
        ..shader = const LinearGradient(
          begin: Alignment.topCenter,
          end: Alignment.bottomCenter,
          colors: <Color>[Color(0xFF15161A), Color(0xFF222329)],
        ).createShader(slot),
    );
    final double handleCenterY =
        7.5 + (1 - value.clamp(0, 1)) * (track.height - 15);
    final double fillTop = handleCenterY;
    final Rect fill = Rect.fromLTRB(
      slot.center.dx - 1,
      fillTop,
      slot.center.dx + 1,
      slot.bottom,
    );
    final Color levelColor = value >= .88
        ? AudioMixerColors.peak
        : value >= .72
        ? AudioMixerColors.warning
        : accent;
    if (!muted) {
      canvas.drawRRect(
        RRect.fromRectAndRadius(fill, const Radius.circular(1)),
        Paint()..color = levelColor,
      );
    }
    final Rect handleRect = Rect.fromCenter(
      center: Offset(track.center.dx, handleCenterY),
      width: 26,
      height: 15,
    );
    final RRect handle = RRect.fromRectAndRadius(
      handleRect,
      const Radius.circular(3.5),
    );
    canvas.drawRRect(
      handle,
      Paint()
        ..color = Colors.black.withValues(alpha: 0.42)
        ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 3),
    );
    canvas.drawRRect(handle, Paint()..color = AudioMixerColors.handle);
    canvas.drawRRect(
      handle.deflate(0.5),
      Paint()
        ..style = PaintingStyle.stroke
        ..strokeWidth = emphasized ? 1 : 0.75
        ..color = emphasized
            ? AudioMixerColors.accentBorder
            : AudioMixerColors.handleBorder,
    );
    final RRect handleFace = RRect.fromRectAndRadius(
      Rect.fromCenter(center: handleRect.center, width: 20, height: 1),
      const Radius.circular(1.5),
    );
    canvas.drawRRect(
      handleFace,
      Paint()..color = muted ? HyprColors.textFaint : levelColor,
    );
  }

  @override
  bool shouldRepaint(covariant AudioFaderPainter oldDelegate) {
    return value != oldDelegate.value ||
        accent != oldDelegate.accent ||
        emphasized != oldDelegate.emphasized ||
        muted != oldDelegate.muted;
  }
}
