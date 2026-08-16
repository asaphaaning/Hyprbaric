import 'package:flutter/material.dart';

import '../../widgets/hypr_surface.dart';
import '../../widgets/primitives/primitives.dart';
import 'controls_chrome.dart';

class ControlCapturePad extends StatefulWidget {
  const ControlCapturePad({
    super.key,
    required this.label,
    required this.shortcut,
    required this.icon,
    required this.onPressed,
  });

  final String label;
  final String shortcut;
  final IconData icon;
  final VoidCallback onPressed;

  @override
  State<ControlCapturePad> createState() => ControlCapturePadState();
}

class ControlCapturePadState extends State<ControlCapturePad> {
  @override
  Widget build(BuildContext context) {
    return HyprIconTile(
      onPressed: widget.onPressed,
      color: ControlColors.tile,
      hoverColor: ControlColors.tileHover,
      borderColor: ControlColors.stroke,
      hoverBorderColor: ControlColors.strokeHover,
      borderRadius: BorderRadius.circular(10),
      builder:
          (
            BuildContext context, {
            required bool hovered,
            required bool pressed,
          }) {
            return Stack(
              children: <Widget>[
                Positioned(
                  top: 7,
                  right: 8,
                  child: Text(
                    widget.shortcut,
                    style: HyprTypography.compactMono.copyWith(
                      color: HyprColors.textFaint,
                      fontSize: HyprTypography.size(8.5),
                      fontWeight: FontWeight.w500,
                      letterSpacing: 0.16,
                    ),
                  ),
                ),
                Center(
                  child: Padding(
                    padding: const EdgeInsets.only(top: 7),
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: <Widget>[
                        Icon(
                          widget.icon,
                          size: 22,
                          color: hovered
                              ? HyprColors.text
                              : const Color(0xFFEBC7A8),
                        ),
                        const SizedBox(height: 11),
                        Text(
                          widget.label.toUpperCase(),
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                          style: HyprTypography.compactMonoStrong.copyWith(
                            color: hovered
                                ? HyprColors.text
                                : HyprColors.textMuted,
                            fontSize: HyprTypography.size(10),
                            letterSpacing: 2.0,
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ],
            );
          },
    );
  }
}

class ControlRecordPad extends StatefulWidget {
  const ControlRecordPad({
    super.key,
    required this.active,
    required this.enabled,
    required this.phase,
    required this.elapsed,
    required this.onPressed,
  });

  final bool active;
  final bool enabled;
  final String phase;
  final String elapsed;
  final VoidCallback? onPressed;

  @override
  State<ControlRecordPad> createState() => ControlRecordPadState();
}

class ControlRecordPadState extends State<ControlRecordPad> {
  @override
  Widget build(BuildContext context) {
    return HyprIconTile(
      onPressed: widget.onPressed,
      enabled: widget.enabled,
      active: widget.active,
      padding: const EdgeInsets.fromLTRB(8, 8, 8, 8),
      color: ControlColors.tile,
      hoverColor: ControlColors.tileHover,
      activeColor: ControlColors.danger.withValues(alpha: 0.15),
      borderColor: ControlColors.stroke,
      hoverBorderColor: ControlColors.strokeHover,
      activeBorderColor: ControlColors.danger.withValues(alpha: 0.45),
      borderRadius: BorderRadius.circular(10),
      builder:
          (
            BuildContext context, {
            required bool hovered,
            required bool pressed,
          }) {
            return Column(
              children: <Widget>[
                Row(
                  children: <Widget>[
                    Expanded(
                      child: Text(
                        widget.phase,
                        maxLines: 1,
                        overflow: TextOverflow.clip,
                        style: HyprTypography.compactMonoStrong.copyWith(
                          color: widget.active
                              ? ControlColors.danger
                              : HyprColors.textMuted.withValues(
                                  alpha: widget.enabled ? 1 : 0.45,
                                ),
                          fontSize: HyprTypography.size(8.5),
                          letterSpacing: 2.0,
                        ),
                      ),
                    ),
                    const SizedBox(width: 4),
                    Text(
                      '⇧⌘R',
                      style: HyprTypography.compactMono.copyWith(
                        color: HyprColors.textFaint,
                        fontSize: HyprTypography.size(8.5),
                      ),
                    ),
                  ],
                ),
                const Spacer(),
                Text(
                  widget.elapsed,
                  style: HyprTypography.compactMonoStrong.copyWith(
                    color: !widget.enabled
                        ? HyprColors.textFaint.withValues(alpha: 0.45)
                        : widget.active
                        ? ControlColors.danger
                        : ControlColors.amber,
                    fontSize: HyprTypography.size(16.5),
                    letterSpacing: 1.3,
                    fontFeatures: HyprTypography.tabularNumbers,
                    shadows: const <Shadow>[
                      Shadow(color: Color(0x99FFC08F), blurRadius: 12),
                    ],
                  ),
                ),
              ],
            );
          },
    );
  }
}
