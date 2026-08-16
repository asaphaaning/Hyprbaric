import 'package:flutter/material.dart';

import '../../widgets/hypr_surface.dart';
import '../../widgets/primitives/primitives.dart';
import 'controls_chrome.dart';

class ControlInspectButton extends StatelessWidget {
  const ControlInspectButton({
    super.key,
    required this.label,
    required this.shortcut,
    required this.icon,
    this.onPressed,
    this.active = false,
  });

  final String label;
  final String shortcut;
  final IconData icon;
  final VoidCallback? onPressed;
  final bool active;

  @override
  Widget build(BuildContext context) {
    final bool enabled = onPressed != null;
    return HyprInteractionRegion(
      enabled: enabled,
      onPressed: onPressed,
      builder: (BuildContext context, HyprInteractionState state) {
        final bool interactive = state.enabled;
        return Material(
          color: Colors.transparent,
          shape: RoundedSuperellipseBorder(
            borderRadius: BorderRadius.circular(8),
          ),
          clipBehavior: Clip.antiAlias,
          child: AnimatedContainer(
            duration: HyprMotion.hover,
            curve: HyprMotion.hoverCurve,
            height: 54,
            padding: const EdgeInsets.symmetric(horizontal: 12),
            decoration: ShapeDecoration(
              color: !interactive
                  ? Colors.black.withValues(alpha: 0.08)
                  : active
                  ? HyprColors.accentSoft.withValues(alpha: 0.18)
                  : (state.hovered
                        ? ControlColors.tileHover
                        : ControlColors.tile),
              shape: RoundedSuperellipseBorder(
                borderRadius: BorderRadius.circular(8),
                side: BorderSide(
                  color: !interactive
                      ? ControlColors.stroke.withValues(alpha: 0.45)
                      : active
                      ? HyprColors.accent.withValues(alpha: 0.42)
                      : (state.hovered
                            ? ControlColors.strokeHover
                            : ControlColors.stroke),
                ),
              ),
            ),
            child: Row(
              children: <Widget>[
                DecoratedBox(
                  decoration: ShapeDecoration(
                    color: Colors.black.withValues(alpha: 0.26),
                    shape: RoundedSuperellipseBorder(
                      borderRadius: BorderRadius.circular(6),
                      side: BorderSide(
                        color: !interactive
                            ? HyprColors.popupStroke.withValues(alpha: 0.45)
                            : active
                            ? HyprColors.accent.withValues(alpha: 0.30)
                            : HyprColors.popupStroke,
                      ),
                    ),
                  ),
                  child: SizedBox.square(
                    dimension: 28,
                    child: Icon(
                      icon,
                      size: 15,
                      color: !interactive
                          ? HyprColors.textFaint.withValues(alpha: 0.40)
                          : active
                          ? HyprColors.accent
                          : (state.hovered
                                ? HyprColors.text
                                : const Color(0xFFEBC7A8)),
                    ),
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: <Widget>[
                      Text(
                        label.toUpperCase(),
                        maxLines: 1,
                        overflow: TextOverflow.ellipsis,
                        style: HyprTypography.compactMonoStrong.copyWith(
                          color: !interactive
                              ? HyprColors.textFaint.withValues(alpha: 0.45)
                              : active
                              ? HyprColors.text
                              : HyprColors.textMuted,
                          fontSize: HyprTypography.size(10),
                          letterSpacing: 2.2,
                        ),
                      ),
                      const SizedBox(height: 3),
                      Text(
                        shortcut,
                        style: HyprTypography.compactMono.copyWith(
                          color: !interactive
                              ? HyprColors.textFaint.withValues(alpha: 0.38)
                              : HyprColors.textFaint,
                          fontSize: HyprTypography.size(8.5),
                        ),
                      ),
                    ],
                  ),
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}
