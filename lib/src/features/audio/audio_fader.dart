import 'dart:math' as math;

import 'package:flutter/material.dart';

import '../../bindings/bindings.dart';
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
          width: 44,
          height: 168,
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
      width: 44,
      height: 168,
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
    final RRect well = RRect.fromRectAndRadius(
      Offset.zero & size,
      const Radius.circular(8),
    );
    canvas.drawRRect(
      well,
      Paint()
        ..shader = const LinearGradient(
          begin: Alignment.topCenter,
          end: Alignment.bottomCenter,
          colors: <Color>[Color(0x26121C25), Color(0x141E9BCF)],
        ).createShader(well.outerRect),
    );
    final Rect rail = Rect.fromLTRB(8, 12, size.width - 8, size.height - 10);
    final RRect railShape = RRect.fromRectAndRadius(
      rail,
      const Radius.circular(4),
    );
    canvas.drawRRect(railShape, Paint()..color = AudioMixerColors.rail);
    canvas.drawRRect(
      railShape.deflate(0.5),
      Paint()
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1
        ..color = AudioMixerColors.railBorder,
    );

    const int segments = 20;
    const double gap = 3;
    final double segmentHeight =
        (rail.height - gap * (segments - 1)) / segments;
    final int activeSegments = muted
        ? 0
        : (value.clamp(0, 1) * segments).round().clamp(0, segments);
    final Paint segmentPaint = Paint();
    for (int i = 0; i < segments; i += 1) {
      final bool active = i < activeSegments;
      final double top = rail.bottom - (i + 1) * segmentHeight - i * gap;
      final Rect segment = Rect.fromLTWH(
        rail.left + 4,
        top,
        rail.width - 8,
        segmentHeight,
      );
      segmentPaint.color = active
          ? accent.withValues(alpha: 0.88)
          : AudioMixerColors.slot;
      canvas.drawRRect(
        RRect.fromRectAndRadius(segment, const Radius.circular(1.5)),
        segmentPaint,
      );
      canvas.drawLine(
        Offset(segment.left, segment.bottom),
        Offset(segment.right, segment.bottom),
        Paint()
          ..color = AudioMixerColors.slotBorder.withValues(
            alpha: active ? 0.35 : 0.24,
          )
          ..strokeWidth = 0.5,
      );
    }

    final double handleCenterY = rail.bottom - value.clamp(0, 1) * rail.height;
    final Rect handleRect = Rect.fromCenter(
      center: Offset(size.width / 2, handleCenterY),
      width: 33,
      height: 17,
    ).translate(0, -1);
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
      handleRect.deflate(5).translate(0, -0.5),
      const Radius.circular(1.5),
    );
    canvas.drawRRect(handleFace, Paint()..color = AudioMixerColors.handleFace);
    canvas.drawRRect(
      handleFace.deflate(0.5),
      Paint()
        ..style = PaintingStyle.stroke
        ..strokeWidth = 0.6
        ..color = AudioMixerColors.handleLine,
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
