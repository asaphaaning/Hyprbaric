import 'package:flutter/material.dart';

import '../../widgets/hypr_surface.dart';
import '../../widgets/primitives/primitives.dart';
import 'controls_chrome.dart';

class ControlRocker extends StatelessWidget {
  const ControlRocker({
    super.key,
    required this.label,
    required this.shortcut,
    required this.icon,
    required this.value,
    required this.onChanged,
    this.enabled = true,
  });

  final String label;
  final String shortcut;
  final IconData icon;
  final bool value;
  final ValueChanged<bool> onChanged;
  final bool enabled;

  @override
  Widget build(BuildContext context) {
    final bool active = value;
    return HyprInteractionRegion(
      enabled: enabled,
      onPressed: () => onChanged(!active),
      builder: (BuildContext context, HyprInteractionState state) {
        final bool interactive = state.enabled;
        return Material(
          color: Colors.transparent,
          shape: RoundedSuperellipseBorder(
            borderRadius: BorderRadius.circular(7),
          ),
          clipBehavior: Clip.antiAlias,
          child: AnimatedContainer(
            duration: HyprMotion.hover,
            curve: HyprMotion.hoverCurve,
            padding: const EdgeInsets.fromLTRB(8, 10, 8, 9),
            decoration: ShapeDecoration(
              color: active
                  ? Colors.black.withValues(alpha: 0.28)
                  : (!interactive
                        ? Colors.black.withValues(alpha: 0.08)
                        : state.hovered
                        ? ControlColors.tileHover
                        : ControlColors.tile),
              shape: RoundedSuperellipseBorder(
                borderRadius: BorderRadius.circular(7),
                side: BorderSide(
                  color: state.hovered && interactive
                      ? ControlColors.strokeHover
                      : HyprColors.borderSoft,
                ),
              ),
            ),
            child: Stack(
              children: <Widget>[
                Positioned(
                  top: 0,
                  right: 0,
                  child: Text(
                    shortcut,
                    maxLines: 1,
                    overflow: TextOverflow.clip,
                    style: HyprTypography.compactMonoStrong.copyWith(
                      color: HyprColors.textFaint.withValues(
                        alpha: interactive ? 0.55 : 0.28,
                      ),
                      fontSize: HyprTypography.size(8.5),
                    ),
                  ),
                ),
                Center(
                  child: Padding(
                    padding: const EdgeInsets.only(top: 6),
                    child: Column(
                      mainAxisSize: MainAxisSize.min,
                      children: <Widget>[
                        Icon(
                          icon,
                          size: 20,
                          color: !interactive
                              ? HyprColors.textFaint.withValues(alpha: 0.45)
                              : active
                              ? HyprColors.accent
                              : HyprColors.textFaint,
                        ),
                        const SizedBox(height: 10),
                        _RockerSwitch(value: active, enabled: interactive),
                        const SizedBox(height: 8),
                        Text(
                          label.toUpperCase(),
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                          style: HyprTypography.compactMonoStrong.copyWith(
                            color: active
                                ? HyprColors.text
                                : HyprColors.textFaint,
                            fontSize: HyprTypography.size(9.5),
                            letterSpacing: 1.7,
                          ),
                        ),
                      ],
                    ),
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

class _RockerSwitch extends StatelessWidget {
  const _RockerSwitch({required this.value, required this.enabled});

  final bool value;
  final bool enabled;

  @override
  Widget build(BuildContext context) {
    return HyprToggleSwitch(
      value: value,
      width: 20,
      height: 10,
      padding: EdgeInsets.zero,
      thumbSize: 0,
      trackColor: const Color(0xFF333B44).withValues(alpha: 0.95),
      activeTrackColor: enabled
          ? HyprColors.accent
          : HyprColors.textFaint.withValues(alpha: 0.35),
      borderColor: Colors.white.withValues(alpha: 0.10),
      activeBorderColor: HyprColors.accent.withValues(alpha: 0.6),
      activeShadow: const <BoxShadow>[
        BoxShadow(color: Color(0x9953D870), blurRadius: 9),
      ],
      inactiveShadow: const <BoxShadow>[
        BoxShadow(color: Color(0x99000000), blurRadius: 5),
      ],
      borderRadius: BorderRadius.circular(2),
      duration: HyprMotion.switcher,
      curve: HyprMotion.switchInCurve,
    );
  }
}
