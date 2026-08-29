import 'dart:math' as math;

import 'package:flutter/material.dart';

import '../../widgets/hypr_surface.dart';

class BrightnessKnobPainter extends CustomPainter {
  const BrightnessKnobPainter({
    required this.value,
    required this.enabled,
    required this.emphasized,
    this.console = false,
  });

  static const double _startAngle = -135;
  static const double _sweepAngle = 270;

  final double value;
  final bool enabled;
  final bool emphasized;
  final bool console;

  @override
  void paint(Canvas canvas, Size size) {
    final double progress = value.clamp(0.0, 1.0);
    final Offset center = size.center(Offset.zero);
    final double arcRadius = size.width * 48 / 92;
    final Rect arcRect = Rect.fromCircle(center: center, radius: arcRadius);
    final double start = _degreesToRadians(_startAngle - 90);
    final double sweep = _degreesToRadians(_sweepAngle);
    final double activeSweep = sweep * progress;
    final bool lit = enabled && progress > 0.001;

    if (console) {
      _paintConsoleKnob(canvas, center, size.width, progress, lit);
      return;
    }

    canvas.drawArc(
      arcRect,
      start,
      sweep,
      false,
      Paint()
        ..style = PaintingStyle.stroke
        ..strokeCap = StrokeCap.round
        ..strokeWidth = 7
        ..color = Colors.black.withValues(alpha: 0.5),
    );
    canvas.drawArc(
      arcRect,
      start,
      sweep,
      false,
      Paint()
        ..style = PaintingStyle.stroke
        ..strokeCap = StrokeCap.round
        ..strokeWidth = 4
        ..color = const Color(0xFF29333C),
    );

    if (lit) {
      final Paint glow = Paint()
        ..style = PaintingStyle.stroke
        ..strokeCap = StrokeCap.round
        ..strokeWidth = 5
        ..color = const Color(0x88F0D76C)
        ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 4);
      canvas.drawArc(arcRect, start, activeSweep, false, glow);

      final Paint active = Paint()
        ..style = PaintingStyle.stroke
        ..strokeCap = StrokeCap.round
        ..strokeWidth = 4
        ..shader = const LinearGradient(
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
          colors: <Color>[Color(0xFFFFE78B), Color(0xFFE39D30)],
        ).createShader(arcRect);
      canvas.drawArc(arcRect, start, activeSweep, false, active);
    }

    _paintTicks(canvas, center, size.width);
    _paintBezel(canvas, center, size.width, lit);
    _paintPointer(canvas, center, size.width, progress, lit);
  }

  void _paintConsoleKnob(
    Canvas canvas,
    Offset center,
    double width,
    double progress,
    bool lit,
  ) {
    _paintDotRing(canvas, center, width, progress, lit);

    final double bezelRadius = width * 0.35;
    final Rect bezelRect = Rect.fromCircle(center: center, radius: bezelRadius);

    canvas.drawCircle(
      center.translate(width * 0.055, width * 0.085),
      bezelRadius * 1.08,
      Paint()
        ..color = const Color(0xD9000000)
        ..maskFilter = MaskFilter.blur(BlurStyle.normal, width * 0.10),
    );
    canvas.drawCircle(
      center.translate(-width * 0.025, -width * 0.025),
      bezelRadius + 5,
      Paint()
        ..shader = const LinearGradient(
          begin: Alignment.topCenter,
          end: Alignment.bottomCenter,
          colors: <Color>[
            Color(0xFF55565A),
            Color(0xFF2D2E31),
            Color(0xFF17181A),
          ],
          stops: <double>[0, 0.38, 1],
        ).createShader(bezelRect.inflate(5)),
    );
    canvas.drawCircle(
      center,
      bezelRadius,
      Paint()
        ..shader = const RadialGradient(
          center: Alignment(-0.12, -0.22),
          radius: 0.96,
          colors: <Color>[
            Color(0xFF202125),
            Color(0xFF18191C),
            Color(0xFF0E0F12),
          ],
          stops: <double>[0, 0.62, 1],
        ).createShader(bezelRect),
    );
    canvas.drawCircle(
      center,
      bezelRadius - 0.5,
      Paint()
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1
        ..color = const Color(0xD9000000),
    );
    canvas.drawArc(
      bezelRect.deflate(1),
      _degreesToRadians(200),
      _degreesToRadians(135),
      false,
      Paint()
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1
        ..color = Colors.white.withValues(alpha: 0.10),
    );

    final double angle = _degreesToRadians(
      _startAngle + progress * _sweepAngle - 90,
    );
    final Offset direction = Offset(math.cos(angle), math.sin(angle));
    canvas.drawLine(
      center + direction * (bezelRadius * 0.48),
      center + direction * (bezelRadius * 0.88),
      Paint()
        ..strokeCap = StrokeCap.round
        ..strokeWidth = 2
        ..color = enabled
            ? const Color(0xFFF4F4F2)
            : HyprColors.textFaint.withValues(alpha: 0.55),
    );
  }

  void _paintDotRing(
    Canvas canvas,
    Offset center,
    double width,
    double progress,
    bool lit,
  ) {
    const int dots = 28;
    final int activeDots = lit ? (progress * (dots - 1)).round() + 1 : 0;
    final double radius = width * 0.567;

    for (int index = 0; index < dots; index += 1) {
      final double fraction = index / (dots - 1);
      final double angle = _degreesToRadians(
        _startAngle + fraction * _sweepAngle - 90,
      );
      final Offset dot =
          center + Offset(math.cos(angle), math.sin(angle)) * radius;
      final bool active = index < activeDots;

      if (active) {
        canvas.drawCircle(
          dot,
          3.2,
          Paint()
            ..color = const Color(0x88F4D86D)
            ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 4),
        );
      }

      canvas.drawCircle(
        dot,
        1.65,
        Paint()
          ..color = active ? const Color(0xFFFFE79A) : const Color(0xFF303236),
      );
    }
  }

  void _paintTicks(Canvas canvas, Offset center, double width) {
    final Paint paint = Paint()..strokeCap = StrokeCap.round;
    for (int i = 0; i < 5; i += 1) {
      final double t = i / 4;
      final double angle = _degreesToRadians(
        _startAngle + t * _sweepAngle - 90,
      );
      final bool major = i == 0 || i == 4;
      final double outer = width * 46 / 92;
      final double inner = outer - (major ? 6 : 4);
      final Offset direction = Offset(math.cos(angle), math.sin(angle));
      paint
        ..strokeWidth = 1
        ..color = (major ? HyprColors.textMuted : HyprColors.textFaint)
            .withValues(alpha: major ? 0.7 : 0.4);
      canvas.drawLine(
        center + direction * inner,
        center + direction * outer,
        paint,
      );
    }
  }

  void _paintBezel(Canvas canvas, Offset center, double width, bool lit) {
    final double radius = width * 30 / 92;
    final Rect rect = Rect.fromCircle(center: center, radius: radius);
    final Paint shadow = Paint()
      ..color = Colors.black.withValues(alpha: 0.6)
      ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 8);
    canvas.drawCircle(center.translate(0, 5), radius * 0.95, shadow);

    canvas.drawCircle(
      center,
      radius + 2,
      Paint()
        ..shader = const LinearGradient(
          begin: Alignment.topCenter,
          end: Alignment.bottomCenter,
          colors: <Color>[Color(0x663F4851), Color(0x00222A33)],
          stops: <double>[0, 0.55],
        ).createShader(rect.inflate(2)),
    );
    canvas.drawCircle(
      center,
      radius,
      Paint()
        ..shader = const RadialGradient(
          center: Alignment(-0.42, -0.50),
          radius: 0.92,
          colors: <Color>[
            Color(0xFF575E66),
            Color(0xFF262D36),
            Color(0xFF121820),
          ],
          stops: <double>[0, 0.60, 1],
        ).createShader(rect),
    );
    canvas.drawCircle(
      center,
      radius - 0.5,
      Paint()
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1
        ..color = Colors.black.withValues(alpha: 0.5),
    );
    canvas.drawArc(
      rect.deflate(0.75),
      _degreesToRadians(205),
      _degreesToRadians(130),
      false,
      Paint()
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1
        ..color = Colors.white.withValues(alpha: lit ? 0.13 : 0.09),
    );
    canvas.drawArc(
      rect.deflate(0.75),
      _degreesToRadians(35),
      _degreesToRadians(118),
      false,
      Paint()
        ..style = PaintingStyle.stroke
        ..strokeWidth = 1
        ..color = Colors.black.withValues(alpha: 0.45),
    );
  }

  void _paintPointer(
    Canvas canvas,
    Offset center,
    double width,
    double progress,
    bool lit,
  ) {
    final double angle = _degreesToRadians(
      _startAngle + progress * _sweepAngle - 90,
    );
    final Offset direction = Offset(math.cos(angle), math.sin(angle));
    final Offset tipCenter = center + direction * (width * 33 / 92);
    final Offset tangent = Offset(-direction.dy, direction.dx);
    final Path pointer = Path()
      ..moveTo((tipCenter + tangent * 1.5).dx, (tipCenter + tangent * 1.5).dy)
      ..lineTo((tipCenter - tangent * 1.5).dx, (tipCenter - tangent * 1.5).dy)
      ..lineTo(
        (tipCenter - direction * 9 - tangent * 1.5).dx,
        (tipCenter - direction * 9 - tangent * 1.5).dy,
      )
      ..lineTo(
        (tipCenter - direction * 9 + tangent * 1.5).dx,
        (tipCenter - direction * 9 + tangent * 1.5).dy,
      )
      ..close();

    if (lit) {
      canvas.drawPath(
        pointer,
        Paint()
          ..color = const Color(0xBCEFD96E)
          ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 4),
      );
    }
    canvas.drawPath(
      pointer,
      Paint()
        ..color = lit
            ? const Color(0xFFEFD96E)
            : HyprColors.textMuted.withValues(alpha: 0.55),
    );
  }

  static double _degreesToRadians(double degrees) => degrees * math.pi / 180;

  @override
  bool shouldRepaint(covariant BrightnessKnobPainter oldDelegate) {
    return value != oldDelegate.value ||
        enabled != oldDelegate.enabled ||
        emphasized != oldDelegate.emphasized ||
        console != oldDelegate.console;
  }
}
