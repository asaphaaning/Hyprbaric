import 'dart:math' as math;

import 'package:flutter/material.dart';

import '../../bindings/bindings.dart';
import '../../widgets/hypr_surface.dart';
import '../../widgets/primitives/primitives.dart';
import 'network_chrome.dart';
import 'network_formatting.dart';

class NetworkSpectrumPanel extends StatelessWidget {
  const NetworkSpectrumPanel({
    super.key,
    required this.traffic,
    required this.uploadHistory,
    required this.downloadHistory,
  });

  final NetworkTraffic traffic;
  final List<double> uploadHistory;
  final List<double> downloadHistory;

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: const Color(0x96384D5A)),
        gradient: const LinearGradient(
          begin: Alignment.topCenter,
          end: Alignment.bottomCenter,
          colors: <Color>[
            Color(0xFF04080D),
            Color(0xFF07111B),
            Color(0xFF0C1824),
          ],
          stops: <double>[0, 0.56, 1],
        ),
        boxShadow: const <BoxShadow>[
          BoxShadow(color: Color(0x99000000), blurRadius: 2),
          BoxShadow(
            color: Color(0x2200B8FF),
            blurRadius: 18,
            spreadRadius: -10,
          ),
        ],
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(8),
        child: SizedBox(
          height: 120,
          child: Stack(
            children: <Widget>[
              Positioned.fill(
                child: CustomPaint(
                  painter: _NetworkSpectrumPainter(
                    uploadHistory: uploadHistory,
                    downloadHistory: downloadHistory,
                  ),
                ),
              ),
              Positioned(
                top: 8,
                left: 10,
                right: 10,
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: <Widget>[
                    _NetworkSpectrumPill(
                      label: '↑ UP',
                      transfer: traffic.upload,
                      suffix: 'tx',
                    ),
                    _NetworkSpectrumPill(
                      label: '↓ DOWN',
                      transfer: traffic.download,
                      suffix: 'rx',
                      alignEnd: true,
                    ),
                  ],
                ),
              ),
              Positioned(
                left: 8,
                right: 8,
                bottom: 4,
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: <Widget>[
                    for (final String label in const <String>[
                      '-20s',
                      '-15s',
                      '-10s',
                      '-5s',
                      'now',
                    ])
                      Text(
                        label,
                        style: HyprTypography.compactMono.copyWith(
                          color: NetworkMenuColors.fg3.withValues(alpha: 0.5),
                          fontSize: HyprTypography.size(8),
                          letterSpacing: 0.32,
                        ),
                      ),
                  ],
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _NetworkSpectrumPill extends StatelessWidget {
  const _NetworkSpectrumPill({
    required this.label,
    required this.transfer,
    required this.suffix,
    this.alignEnd = false,
  });

  final String label;
  final NetworkTransfer transfer;
  final String suffix;
  final bool alignEnd;

  @override
  Widget build(BuildContext context) {
    return HyprMetricCard(
      label: label,
      value: formatTransferRateValue(transfer.bytesPerSecond),
      unit: formatTransferRateUnit(transfer.bytesPerSecond),
      detail: '${formatBytes(transfer.totalBytes)} $suffix',
      alignEnd: alignEnd,
      borderColor: NetworkMenuColors.cardBorder,
      labelColor: NetworkMenuColors.fg3,
      unitColor: NetworkMenuColors.fg3,
      detailColor: NetworkMenuColors.fg3,
    );
  }
}

class _NetworkSpectrumPainter extends CustomPainter {
  const _NetworkSpectrumPainter({
    required this.uploadHistory,
    required this.downloadHistory,
  });

  final List<double> uploadHistory;
  final List<double> downloadHistory;

  @override
  void paint(Canvas canvas, Size size) {
    final Rect rect = Offset.zero & size;
    canvas.drawRect(
      rect,
      Paint()
        ..shader = const LinearGradient(
          begin: Alignment.topCenter,
          end: Alignment.bottomCenter,
          colors: <Color>[
            Color(0xFF02060A),
            Color(0xFF07111B),
            Color(0xFF102334),
          ],
          stops: <double>[0, 0.58, 1],
        ).createShader(rect),
    );

    final Paint lowerGlow = Paint()
      ..shader = RadialGradient(
        center: const Alignment(0, 1.15),
        radius: 1.05,
        colors: <Color>[
          HyprColors.accent.withValues(alpha: 0.22),
          const Color(0xFF245B7A).withValues(alpha: 0.10),
          Colors.transparent,
        ],
        stops: const <double>[0, 0.38, 1],
      ).createShader(rect);
    canvas.drawRect(rect, lowerGlow);

    final Paint topVignette = Paint()
      ..shader = RadialGradient(
        center: const Alignment(0, -1.05),
        radius: 1.25,
        colors: <Color>[
          Colors.black.withValues(alpha: 0.78),
          Colors.transparent,
        ],
      ).createShader(rect);
    canvas.drawRect(rect, topVignette);

    final Paint grid = Paint()
      ..color = Colors.white.withValues(alpha: 0.035)
      ..strokeWidth = 1;
    for (int index = 1; index < 10; index += 1) {
      final double x = size.width * index / 10;
      canvas.drawLine(Offset(x, 0), Offset(x, size.height), grid);
    }
    for (int index = 1; index < 4; index += 1) {
      final double y = size.height * index / 4;
      canvas.drawLine(Offset(0, y), Offset(size.width, y), grid);
    }

    final Paint fineGrid = Paint()
      ..color = HyprColors.accent.withValues(alpha: 0.018)
      ..strokeWidth = 1;
    for (int index = 1; index < 20; index += 1) {
      final double x = size.width * index / 20;
      canvas.drawLine(Offset(x, 0), Offset(x, size.height), fineGrid);
    }

    final double midY = size.height / 2;
    final Paint center = Paint()
      ..color = const Color(0x805F6A78)
      ..strokeWidth = 1;
    for (double x = 0; x < size.width; x += 8) {
      canvas.drawLine(
        Offset(x, midY),
        Offset(math.min(x + 4, size.width), midY),
        center,
      );
    }

    _drawHistory(
      canvas,
      size,
      samples: uploadHistory,
      color: HyprColors.accent,
      up: true,
    );
    _drawHistory(
      canvas,
      size,
      samples: downloadHistory,
      color: NetworkMenuColors.good,
      up: false,
    );
  }

  void _drawHistory(
    Canvas canvas,
    Size size, {
    required List<double> samples,
    required Color color,
    required bool up,
  }) {
    if (samples.length < 2) {
      return;
    }
    const double pad = 6;
    final double midY = size.height / 2;
    final double height = midY - pad;
    final double maxValue = math.max(0.1, samples.reduce(math.max));
    final List<Offset> points = _historyPoints(
      size: size,
      samples: samples,
      maxValue: maxValue,
      midY: midY,
      height: height,
      up: up,
    );
    final Path line = _spline(points);
    final Path fill = Path()
      ..moveTo(points.first.dx, midY)
      ..lineTo(points.first.dx, points.first.dy);
    _appendSpline(fill, points);
    fill.lineTo(size.width, midY);
    fill.close();

    canvas.drawPath(
      fill,
      Paint()
        ..shader = LinearGradient(
          begin: up ? Alignment.topCenter : Alignment.bottomCenter,
          end: Alignment.center,
          colors: <Color>[
            color.withValues(alpha: up ? 0.24 : 0.20),
            color.withValues(alpha: 0.025),
          ],
        ).createShader(Offset.zero & size)
        ..style = PaintingStyle.fill,
    );

    canvas.drawPath(
      line,
      Paint()
        ..color = color.withValues(alpha: 0.26)
        ..strokeWidth = 9
        ..strokeCap = StrokeCap.round
        ..strokeJoin = StrokeJoin.round
        ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 8)
        ..style = PaintingStyle.stroke,
    );
    canvas.drawPath(
      line,
      Paint()
        ..color = color.withValues(alpha: 0.44)
        ..strokeWidth = 4.4
        ..strokeCap = StrokeCap.round
        ..strokeJoin = StrokeJoin.round
        ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 3)
        ..style = PaintingStyle.stroke,
    );
    canvas.drawPath(
      line,
      Paint()
        ..shader = LinearGradient(
          begin: Alignment.centerLeft,
          end: Alignment.centerRight,
          colors: <Color>[
            color.withValues(alpha: 0.66),
            color.withValues(alpha: 1),
          ],
        ).createShader(Offset.zero & size)
        ..strokeWidth = 1.55
        ..strokeCap = StrokeCap.round
        ..strokeJoin = StrokeJoin.round
        ..style = PaintingStyle.stroke,
    );
  }

  List<Offset> _historyPoints({
    required Size size,
    required List<double> samples,
    required double maxValue,
    required double midY,
    required double height,
    required bool up,
  }) {
    return <Offset>[
      for (int index = 0; index < samples.length; index += 1)
        Offset(
          size.width * index / (samples.length - 1),
          up
              ? midY - (samples[index] / maxValue).clamp(0.0, 1.0) * height
              : midY + (samples[index] / maxValue).clamp(0.0, 1.0) * height,
        ),
    ];
  }

  Path _spline(List<Offset> points) {
    final Path path = Path()..moveTo(points.first.dx, points.first.dy);
    _appendSpline(path, points);
    return path;
  }

  void _appendSpline(Path path, List<Offset> points) {
    if (points.length < 2) {
      return;
    }
    for (int index = 0; index < points.length - 1; index += 1) {
      final Offset p0 = points[math.max(0, index - 1)];
      final Offset p1 = points[index];
      final Offset p2 = points[index + 1];
      final Offset p3 = points[math.min(points.length - 1, index + 2)];
      final Offset c1 = p1 + (p2 - p0) * (1 / 6);
      final Offset c2 = p2 - (p3 - p1) * (1 / 6);
      path.cubicTo(c1.dx, c1.dy, c2.dx, c2.dy, p2.dx, p2.dy);
    }
  }

  @override
  bool shouldRepaint(covariant _NetworkSpectrumPainter oldDelegate) {
    return true;
  }
}

class NetworkPingRow extends StatelessWidget {
  const NetworkPingRow({super.key, required this.pingMs});

  final int? pingMs;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(8, 4, 8, 10),
      child: Row(
        children: <Widget>[
          Text(
            'Ping',
            style: HyprTypography.popRow.copyWith(
              color: NetworkMenuColors.fg2,
              fontSize: HyprTypography.size(11.5),
            ),
          ),
          const Spacer(),
          Text(
            pingMs == null ? '-' : '$pingMs ms',
            style: HyprTypography.compactMonoStrong.copyWith(
              color: _pingColor(pingMs),
              fontSize: HyprTypography.size(11),
            ),
          ),
        ],
      ),
    );
  }

  Color _pingColor(int? pingMs) {
    if (pingMs == null) {
      return NetworkMenuColors.fg3;
    }
    if (pingMs < 20) {
      return NetworkMenuColors.good;
    }
    if (pingMs < 50) {
      return NetworkMenuColors.warning;
    }
    return HyprColors.danger;
  }
}
