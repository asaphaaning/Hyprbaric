import 'package:flutter/material.dart';

import '../hypr_surface.dart';

class HyprMetricCard extends StatelessWidget {
  const HyprMetricCard({
    super.key,
    required this.label,
    required this.value,
    required this.unit,
    required this.detail,
    this.alignEnd = false,
    this.width = 96,
    this.color,
    this.borderColor = HyprColors.borderSoft,
    this.borderRadius = HyprRadii.cardRadius,
    this.padding = HyprSpacing.metricCard,
    this.labelColor = HyprColors.textFaint,
    this.valueColor = HyprColors.text,
    this.unitColor = HyprColors.textFaint,
    this.detailColor = HyprColors.textFaint,
  });

  final String label;
  final String value;
  final String unit;
  final String detail;
  final bool alignEnd;
  final double width;
  final Color? color;
  final Color borderColor;
  final BorderRadius borderRadius;
  final EdgeInsetsGeometry padding;
  final Color labelColor;
  final Color valueColor;
  final Color unitColor;
  final Color detailColor;

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: ShapeDecoration(
        color: color ?? Colors.black.withValues(alpha: 0.5),
        shape: RoundedSuperellipseBorder(
          borderRadius: borderRadius,
          side: BorderSide(color: borderColor),
        ),
      ),
      child: SizedBox(
        width: width,
        child: Padding(
          padding: padding,
          child: Column(
            crossAxisAlignment: alignEnd
                ? CrossAxisAlignment.end
                : CrossAxisAlignment.start,
            children: <Widget>[
              Text(
                label,
                style: HyprTypography.compactMonoStrong.copyWith(
                  color: labelColor,
                  fontSize: HyprTypography.size(8.5),
                  letterSpacing: 1.02,
                  height: 1,
                ),
              ),
              const SizedBox(height: HyprSpacing.sm),
              RichText(
                textAlign: alignEnd ? TextAlign.right : TextAlign.left,
                text: TextSpan(
                  children: <InlineSpan>[
                    TextSpan(
                      text: value,
                      style: HyprTypography.compactMonoStrong.copyWith(
                        color: valueColor,
                        fontSize: HyprTypography.size(14),
                        fontWeight: FontWeight.w600,
                        letterSpacing: -0.14,
                        height: 1,
                      ),
                    ),
                    TextSpan(
                      text: unit,
                      style: HyprTypography.compactMono.copyWith(
                        color: unitColor,
                        fontSize: HyprTypography.size(9),
                        fontWeight: FontWeight.w500,
                        letterSpacing: 0.36,
                      ),
                    ),
                  ],
                ),
              ),
              const SizedBox(height: HyprSpacing.xs),
              Text(
                detail,
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
                textAlign: alignEnd ? TextAlign.right : TextAlign.left,
                style: HyprTypography.compactMono.copyWith(
                  color: detailColor,
                  fontSize: HyprTypography.size(8.5),
                  letterSpacing: 0.51,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
