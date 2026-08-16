import 'package:flutter/material.dart';

import '../hypr_surface.dart';

class HyprBadge extends StatelessWidget {
  const HyprBadge({
    super.key,
    required this.child,
    this.color = Colors.transparent,
    this.borderColor,
    this.borderWidth = 1,
    this.borderRadius = HyprRadii.badgeRadius,
    this.padding = HyprSpacing.badge,
    this.width,
    this.height,
  });

  factory HyprBadge.text({
    Key? key,
    required String label,
    Color color = Colors.transparent,
    Color? borderColor,
    Color textColor = HyprColors.textMuted,
    BorderRadius borderRadius = HyprRadii.badgeRadius,
    EdgeInsetsGeometry padding = HyprSpacing.badge,
    TextStyle? style,
    int maxLines = 1,
    TextOverflow overflow = TextOverflow.clip,
    double? width,
    double? height,
  }) {
    return HyprBadge(
      key: key,
      color: color,
      borderColor: borderColor,
      borderRadius: borderRadius,
      padding: padding,
      width: width,
      height: height,
      child: Text(
        label,
        maxLines: maxLines,
        overflow: overflow,
        textAlign: TextAlign.center,
        style: (style ?? HyprTypography.compactMonoStrong).copyWith(
          color: textColor,
        ),
      ),
    );
  }

  final Widget child;
  final Color color;
  final Color? borderColor;
  final double borderWidth;
  final BorderRadius borderRadius;
  final EdgeInsetsGeometry padding;
  final double? width;
  final double? height;

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: ShapeDecoration(
        color: color,
        shape: RoundedSuperellipseBorder(
          borderRadius: borderRadius,
          side: borderColor == null
              ? BorderSide.none
              : BorderSide(color: borderColor!, width: borderWidth),
        ),
      ),
      child: SizedBox(
        width: width,
        height: height,
        child: Padding(
          padding: padding,
          child: Center(widthFactor: width == null ? 1 : null, child: child),
        ),
      ),
    );
  }
}
