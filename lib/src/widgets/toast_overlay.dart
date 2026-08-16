import 'package:flutter/material.dart';

import '../bindings/bindings.dart';
import '../state/transient_overlays.dart';
import 'hypr_surface.dart';
import 'primitives/primitives.dart';

class ToastPill extends StatelessWidget {
  const ToastPill({super.key, required this.entry, required this.onPressed});

  final ToastEntry entry;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return TweenAnimationBuilder<double>(
      tween: Tween<double>(begin: 0, end: 1),
      duration: HyprMotion.toast,
      curve: HyprMotion.toastCurve,
      builder: (BuildContext context, double value, Widget? child) {
        return Opacity(
          opacity: value,
          child: Transform.translate(
            offset: Offset(0, (1 - value) * -10),
            child: Transform.scale(scale: 0.96 + (0.04 * value), child: child),
          ),
        );
      },
      child: Semantics(
        button: true,
        label: '${entry.app}: ${entry.message}',
        child: GestureDetector(
          behavior: HitTestBehavior.opaque,
          onTap: onPressed,
          child: HyprSurface(
            borderRadius: BorderRadius.circular(4),
            color: const Color(0xF012151D),
            borderColor: entry.urgency == NotificationUrgency.critical
                ? const Color(0x66E16658)
                : const Color(0x80505A64),
            blur: 28,
            frame: HyprSurfaceFrame.popover,
            child: Stack(
              children: <Widget>[
                const Positioned.fill(child: ToastCornerBrackets()),
                ConstrainedBox(
                  constraints: const BoxConstraints(maxWidth: 480),
                  child: Padding(
                    padding: const EdgeInsets.fromLTRB(14, 11, 16, 11),
                    child: Row(
                      mainAxisSize: MainAxisSize.min,
                      children: <Widget>[
                        ToastAppTag(app: entry.app, accent: entry.color),
                        const SizedBox(width: 13),
                        Flexible(
                          child: Text(
                            entry.message,
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                            style: HyprTypography.compactMonoStrong.copyWith(
                              color: HyprColors.text,
                              fontSize: HyprTypography.size(12),
                              fontWeight: FontWeight.w500,
                              letterSpacing: 0.12,
                            ),
                          ),
                        ),
                        const SizedBox(width: 14),
                        Text(
                          'now',
                          style: HyprTypography.compactMono.copyWith(
                            color: HyprColors.textFaint,
                            fontSize: HyprTypography.size(9),
                            letterSpacing: 0.72,
                            fontFeatures: HyprTypography.tabularNumbers,
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class ToastAppTag extends StatelessWidget {
  const ToastAppTag({super.key, required this.app, required this.accent});

  final String app;
  final Color accent;

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: ShapeDecoration(
        color: Colors.black.withValues(alpha: 0.34),
        shape: RoundedSuperellipseBorder(
          borderRadius: BorderRadius.circular(2),
          side: BorderSide(color: accent.withValues(alpha: 0.18)),
        ),
      ),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: <Widget>[
            HyprBracketedTag(
              label: app,
              accent: accent,
              textColor: HyprColors.textMuted,
            ),
          ],
        ),
      ),
    );
  }
}

class ToastCornerBrackets extends StatelessWidget {
  const ToastCornerBrackets({super.key});

  @override
  Widget build(BuildContext context) {
    return CustomPaint(painter: ToastCornerBracketPainter());
  }
}

class ToastCornerBracketPainter extends CustomPainter {
  @override
  void paint(Canvas canvas, Size size) {
    const double length = 8;
    final Paint paint = Paint()
      ..color = Colors.white.withValues(alpha: 0.08)
      ..strokeWidth = 1
      ..style = PaintingStyle.stroke;

    canvas.drawLine(Offset.zero, const Offset(length, 0), paint);
    canvas.drawLine(Offset.zero, const Offset(0, length), paint);
    canvas.drawLine(
      Offset(size.width, 0),
      Offset(size.width - length, 0),
      paint,
    );
    canvas.drawLine(Offset(size.width, 0), Offset(size.width, length), paint);
    canvas.drawLine(Offset(0, size.height), Offset(length, size.height), paint);
    canvas.drawLine(
      Offset(0, size.height),
      Offset(0, size.height - length),
      paint,
    );
    canvas.drawLine(
      Offset(size.width, size.height),
      Offset(size.width - length, size.height),
      paint,
    );
    canvas.drawLine(
      Offset(size.width, size.height),
      Offset(size.width, size.height - length),
      paint,
    );
  }

  @override
  bool shouldRepaint(covariant ToastCornerBracketPainter oldDelegate) => false;
}
