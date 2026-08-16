import 'package:flutter/material.dart';

import '../hypr_surface.dart';

class HyprSectionLabel extends StatelessWidget {
  const HyprSectionLabel(
    this.label, {
    super.key,
    this.color = HyprColors.textFaint,
    this.fontSize,
    this.fontWeight = FontWeight.w600,
    this.letterSpacing = 0.96,
    this.trailingLine = false,
    this.lineColor = HyprColors.borderSoft,
    this.lineGap = HyprSpacing.xxl,
  });

  final String label;
  final Color color;
  final double? fontSize;
  final FontWeight fontWeight;
  final double letterSpacing;
  final bool trailingLine;
  final Color lineColor;
  final double lineGap;

  @override
  Widget build(BuildContext context) {
    final Text title = Text(
      label.toUpperCase(),
      style: HyprTypography.popTitle.copyWith(
        color: color,
        fontSize: fontSize,
        fontWeight: fontWeight,
        letterSpacing: letterSpacing,
      ),
    );

    if (!trailingLine) {
      return title;
    }

    return Row(
      children: <Widget>[
        title,
        SizedBox(width: lineGap),
        Expanded(
          child: DecoratedBox(
            decoration: BoxDecoration(color: lineColor),
            child: const SizedBox(height: HyprSpacing.hairline),
          ),
        ),
      ],
    );
  }
}
