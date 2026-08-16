import 'package:flutter/material.dart';

import '../hypr_surface.dart';

class HyprInlineTag extends StatelessWidget {
  const HyprInlineTag({
    super.key,
    required this.label,
    this.color = HyprColors.fill,
    this.borderColor = Colors.transparent,
    this.textColor = HyprColors.textMuted,
    this.padding = HyprSpacing.inlineTag,
    this.borderRadius = HyprRadii.tagRadius,
    this.style,
    this.uppercase = true,
    this.maxLines = 1,
    this.leading,
    this.trailing,
    this.leadingGap = 5,
    this.trailingGap = 5,
  });

  final String label;
  final Color color;
  final Color borderColor;
  final Color textColor;
  final EdgeInsetsGeometry padding;
  final BorderRadius borderRadius;
  final TextStyle? style;
  final bool uppercase;
  final int maxLines;
  final Widget? leading;
  final Widget? trailing;
  final double leadingGap;
  final double trailingGap;

  @override
  Widget build(BuildContext context) {
    final Widget text = Text(
      uppercase ? label.toUpperCase() : label,
      maxLines: maxLines,
      overflow: TextOverflow.ellipsis,
      style:
          style?.copyWith(color: textColor) ??
          HyprTypography.compactMonoStrong.copyWith(
            color: textColor,
            fontSize: HyprTypography.size(9.5),
            letterSpacing: 1.2,
            height: 1,
          ),
    );

    return DecoratedBox(
      decoration: ShapeDecoration(
        color: color,
        shape: RoundedSuperellipseBorder(
          borderRadius: borderRadius,
          side: BorderSide(color: borderColor),
        ),
      ),
      child: Padding(
        padding: padding,
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: <Widget>[
            if (leading != null) ...<Widget>[
              leading!,
              SizedBox(width: leadingGap),
            ],
            Flexible(child: text),
            if (trailing != null) ...<Widget>[
              SizedBox(width: trailingGap),
              trailing!,
            ],
          ],
        ),
      ),
    );
  }
}

class HyprBracketedTag extends StatelessWidget {
  const HyprBracketedTag({
    super.key,
    required this.label,
    required this.accent,
    this.textColor = HyprColors.textMuted,
    this.style,
    this.bracketStyle,
  });

  final String label;
  final Color accent;
  final Color textColor;
  final TextStyle? style;
  final TextStyle? bracketStyle;

  @override
  Widget build(BuildContext context) {
    final TextStyle effectiveBracketStyle =
        bracketStyle ??
        HyprTypography.compactMonoStrong.copyWith(
          color: accent.withValues(alpha: 0.72),
          fontSize: HyprTypography.size(9.5),
          fontWeight: FontWeight.w700,
          letterSpacing: 0,
          height: 1,
        );
    final TextStyle effectiveStyle =
        style ??
        HyprTypography.compactMonoStrong.copyWith(
          color: textColor,
          fontSize: HyprTypography.size(9.5),
          fontWeight: FontWeight.w700,
          letterSpacing: 1.52,
          height: 1,
        );

    return Row(
      mainAxisSize: MainAxisSize.min,
      children: <Widget>[
        Text('<', style: effectiveBracketStyle),
        Text(
          label.toUpperCase(),
          maxLines: 1,
          overflow: TextOverflow.ellipsis,
          style: effectiveStyle.copyWith(color: textColor),
        ),
        Text('>', style: effectiveBracketStyle),
      ],
    );
  }
}
