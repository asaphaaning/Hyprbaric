import 'package:flutter/material.dart';

import '../hypr_surface.dart';
import 'hypr_interactive_tile.dart';

typedef HyprIconTileBuilder =
    Widget Function(
      BuildContext context, {
      required bool hovered,
      required bool pressed,
    });

class HyprIconTile extends StatelessWidget {
  const HyprIconTile({
    super.key,
    required this.builder,
    required this.onPressed,
    this.semanticLabel,
    this.enabled = true,
    this.active = false,
    this.padding = HyprSpacing.none,
    this.borderRadius = HyprRadii.fieldRadius,
    this.color = Colors.transparent,
    this.hoverColor = HyprColors.hover,
    this.activeColor = HyprColors.hoverStrong,
    this.borderColor = Colors.transparent,
    this.hoverBorderColor = HyprColors.borderSoft,
    this.activeBorderColor = HyprColors.border,
    this.pressedScale = 0.94,
    this.clipBehavior = Clip.antiAlias,
  });

  final HyprIconTileBuilder builder;
  final VoidCallback? onPressed;
  final String? semanticLabel;
  final bool enabled;
  final bool active;
  final EdgeInsetsGeometry padding;
  final BorderRadius borderRadius;
  final Color color;
  final Color hoverColor;
  final Color activeColor;
  final Color borderColor;
  final Color hoverBorderColor;
  final Color activeBorderColor;
  final double pressedScale;
  final Clip clipBehavior;

  @override
  Widget build(BuildContext context) {
    return HyprInteractiveTile(
      semanticLabel: semanticLabel,
      onPressed: onPressed,
      enabled: enabled,
      selected: active,
      padding: padding,
      borderRadius: borderRadius,
      color: color,
      hoverColor: hoverColor,
      selectedColor: activeColor,
      borderColor: borderColor,
      hoverBorderColor: hoverBorderColor,
      selectedBorderColor: activeBorderColor,
      pressedScale: pressedScale,
      clipBehavior: clipBehavior,
      builder: (BuildContext context, HyprInteractiveTileState state) {
        return builder(context, hovered: state.hovered, pressed: state.pressed);
      },
    );
  }
}
