import 'package:flutter/material.dart';

import '../bindings/bindings.dart';
import 'hypr_surface.dart';
import 'notification_panel_style.dart';
import 'primitives/primitives.dart';

class NotificationRow extends StatelessWidget {
  const NotificationRow({
    super.key,
    required this.entry,
    required this.onDismiss,
  });

  final NotificationEntry entry;
  final VoidCallback onDismiss;

  @override
  Widget build(BuildContext context) {
    final Color accent = notificationAccent(entry);
    return HyprInteractionRegion(
      builder: (BuildContext context, HyprInteractionState state) {
        final bool hovered = state.hovered;
        return AnimatedContainer(
          duration: HyprMotion.hover,
          curve: HyprMotion.hoverCurve,
          transform: Matrix4.translationValues(hovered ? 2 : 0, 0, 0),
          constraints: const BoxConstraints(minHeight: 51),
          decoration: BoxDecoration(
            color: hovered ? const Color(0xD0141A22) : const Color(0x8F0C1118),
          ),
          clipBehavior: Clip.antiAlias,
          child: Stack(
            children: <Widget>[
              Positioned.fill(
                child: IgnorePointer(
                  child: DecoratedBox(
                    decoration: BoxDecoration(
                      color: accent.withValues(alpha: hovered ? 0.07 : 0.04),
                    ),
                  ),
                ),
              ),
              Positioned(
                left: 0,
                top: 10,
                bottom: 10,
                child: DecoratedBox(
                  decoration: BoxDecoration(
                    color: accent,
                    borderRadius: const BorderRadius.horizontal(
                      right: Radius.circular(2),
                    ),
                    boxShadow: <BoxShadow>[
                      BoxShadow(
                        color: accent.withValues(alpha: 0.80),
                        blurRadius: 8,
                      ),
                    ],
                  ),
                  child: const SizedBox(width: 3),
                ),
              ),
              Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: <Widget>[
                  const SizedBox(width: 14),
                  Expanded(
                    child: Padding(
                      padding: const EdgeInsets.symmetric(vertical: 12),
                      child: Row(
                        children: <Widget>[
                          _NotificationAppChip(app: entry.app, accent: accent),
                          const SizedBox(width: 10),
                          Expanded(
                            child: Text(
                              entry.message,
                              maxLines: 1,
                              overflow: TextOverflow.ellipsis,
                              style: HyprTypography.notificationText.copyWith(
                                color: NotificationPalette.fg1,
                                fontSize: HyprTypography.size(12.5),
                                height: 1.2,
                              ),
                            ),
                          ),
                          const SizedBox(width: 10),
                          _NotificationTimePill(
                            label: notificationAgeLabel(entry.createdAtMs),
                          ),
                        ],
                      ),
                    ),
                  ),
                  SizedBox(
                    width: 28,
                    child: IgnorePointer(
                      ignoring: !hovered,
                      child: AnimatedOpacity(
                        duration: HyprMotion.hover,
                        opacity: hovered ? 1 : 0,
                        child: Center(
                          child: _NotificationDismissButton(
                            onPressed: onDismiss,
                          ),
                        ),
                      ),
                    ),
                  ),
                ],
              ),
            ],
          ),
        );
      },
    );
  }
}

class _NotificationAppChip extends StatelessWidget {
  const _NotificationAppChip({required this.app, required this.accent});

  final String app;
  final Color accent;

  @override
  Widget build(BuildContext context) {
    return HyprInlineTag(
      label: app,
      color: accent.withValues(alpha: 0.12),
      textColor: accent,
      padding: const EdgeInsets.symmetric(horizontal: 5, vertical: 1),
      borderRadius: BorderRadius.circular(3),
      style: HyprTypography.compactMonoStrong.copyWith(
        fontSize: HyprTypography.size(9),
        letterSpacing: 2.0,
        height: 1,
      ),
    );
  }
}

class _NotificationDismissButton extends StatelessWidget {
  const _NotificationDismissButton({required this.onPressed});

  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return IconButton(
      onPressed: onPressed,
      style: ButtonStyle(
        visualDensity: VisualDensity.compact,
        tapTargetSize: MaterialTapTargetSize.shrinkWrap,
        minimumSize: const WidgetStatePropertyAll<Size>(Size(20, 20)),
        fixedSize: const WidgetStatePropertyAll<Size>(Size(20, 20)),
        padding: const WidgetStatePropertyAll<EdgeInsetsGeometry>(
          EdgeInsets.zero,
        ),
        shape: WidgetStatePropertyAll<OutlinedBorder>(
          RoundedSuperellipseBorder(borderRadius: BorderRadius.circular(5)),
        ),
        overlayColor: const WidgetStatePropertyAll<Color>(Colors.transparent),
        backgroundColor: WidgetStateProperty.resolveWith<Color?>((states) {
          if (states.contains(WidgetState.hovered) ||
              states.contains(WidgetState.pressed)) {
            return HyprColors.dangerHover;
          }
          return Colors.transparent;
        }),
        foregroundColor: WidgetStateProperty.resolveWith<Color>((states) {
          if (states.contains(WidgetState.hovered) ||
              states.contains(WidgetState.pressed)) {
            return const Color(0xFFE6A095);
          }
          return NotificationPalette.fg3;
        }),
      ),
      icon: Builder(
        builder: (BuildContext context) {
          return CustomPaint(
            size: const Size(13, 13),
            painter: _NotificationEjectPainter(
              color: IconTheme.of(context).color ?? NotificationPalette.fg3,
            ),
          );
        },
      ),
    );
  }
}

class _NotificationTimePill extends StatelessWidget {
  const _NotificationTimePill({required this.label});

  final String label;

  @override
  Widget build(BuildContext context) {
    final String timeText = label.toLowerCase() == 'now'
        ? 'NOW'
        : '${label.toUpperCase()} AGO';
    return HyprBadge.text(
      label: timeText,
      color: Colors.black.withValues(alpha: 0.34),
      borderRadius: BorderRadius.circular(3),
      padding: const EdgeInsets.symmetric(horizontal: 7, vertical: 4),
      textColor: const Color(0xFFC9B89A),
      style: HyprTypography.compactMonoStrong.copyWith(
        fontSize: HyprTypography.size(9),
        letterSpacing: 1.0,
        height: 1,
        fontFeatures: HyprTypography.tabularNumbers,
      ),
    );
  }
}

class _NotificationEjectPainter extends CustomPainter {
  const _NotificationEjectPainter({required this.color});

  final Color color;

  @override
  void paint(Canvas canvas, Size size) {
    final Paint paint = Paint()
      ..color = color
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2
      ..strokeCap = StrokeCap.round
      ..strokeJoin = StrokeJoin.round;
    final Path triangle = Path()
      ..moveTo(size.width * 0.18, size.height * 0.62)
      ..lineTo(size.width * 0.5, size.height * 0.22)
      ..lineTo(size.width * 0.82, size.height * 0.62)
      ..close();
    canvas.drawLine(
      Offset(size.width * 0.16, size.height * 0.86),
      Offset(size.width * 0.84, size.height * 0.86),
      paint,
    );
    canvas.drawPath(triangle, paint);
  }

  @override
  bool shouldRepaint(covariant _NotificationEjectPainter oldDelegate) {
    return oldDelegate.color != color;
  }
}
