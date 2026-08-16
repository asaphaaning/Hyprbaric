import 'package:flutter/material.dart';

import '../hypr_surface.dart';
import 'hypr_hover_plate.dart';

typedef HyprActionRowLeadingBuilder =
    Widget Function(
      BuildContext context, {
      required bool hovered,
      required bool selected,
    });

class HyprActionRow extends StatelessWidget {
  const HyprActionRow({
    super.key,
    required this.title,
    required this.onPressed,
    this.semanticLabel,
    this.subtitle,
    this.leading,
    this.leadingBuilder,
    this.icon,
    this.iconColor,
    this.iconBackgroundColor,
    this.trailing,
    this.enabled = true,
    this.selected = false,
    this.danger = false,
    this.padding = HyprSpacing.actionRow,
    this.borderRadius = HyprRadii.rowRadius,
    this.color = Colors.transparent,
    this.hoverColor = HyprColors.hover,
    this.selectedColor = HyprColors.hoverStrong,
    this.borderColor = Colors.transparent,
    this.hoverBorderColor = Colors.transparent,
    this.selectedBorderColor = HyprColors.border,
    this.borderWidth = 1,
    this.selectedBorderWidth = 1,
    this.titleColor = HyprColors.textMuted,
    this.hoverTitleColor = HyprColors.text,
    this.selectedTitleColor = HyprColors.text,
    this.subtitleColor = HyprColors.textFaint,
    this.hoverSubtitleColor = HyprColors.textFaint,
    this.trailingColor = HyprColors.textFaint,
    this.hoverTrailingColor = HyprColors.textMuted,
    this.titleStyle,
    this.subtitleStyle,
    this.leadingGap = HyprSpacing.xxl,
  });

  final String title;
  final VoidCallback? onPressed;
  final String? semanticLabel;
  final String? subtitle;
  final Widget? leading;
  final HyprActionRowLeadingBuilder? leadingBuilder;
  final IconData? icon;
  final Color? iconColor;
  final Color? iconBackgroundColor;
  final Widget? trailing;
  final bool enabled;
  final bool selected;
  final bool danger;
  final EdgeInsetsGeometry padding;
  final BorderRadius borderRadius;
  final Color color;
  final Color hoverColor;
  final Color selectedColor;
  final Color borderColor;
  final Color hoverBorderColor;
  final Color selectedBorderColor;
  final double borderWidth;
  final double selectedBorderWidth;
  final Color titleColor;
  final Color hoverTitleColor;
  final Color selectedTitleColor;
  final Color subtitleColor;
  final Color hoverSubtitleColor;
  final Color trailingColor;
  final Color hoverTrailingColor;
  final TextStyle? titleStyle;
  final TextStyle? subtitleStyle;
  final double leadingGap;

  @override
  Widget build(BuildContext context) {
    return HyprHoverPlate(
      semanticLabel: semanticLabel ?? title,
      onPressed: onPressed,
      enabled: enabled,
      selected: selected,
      danger: danger,
      padding: padding,
      borderRadius: borderRadius,
      color: color,
      hoverColor: hoverColor,
      selectedColor: selectedColor,
      borderColor: borderColor,
      hoverBorderColor: hoverBorderColor,
      selectedBorderColor: selectedBorderColor,
      borderWidth: borderWidth,
      selectedBorderWidth: selectedBorderWidth,
      builder: (BuildContext context, {required bool hovered}) {
        final Color effectiveTitleColor = selected
            ? selectedTitleColor
            : hovered
            ? hoverTitleColor
            : titleColor;
        final Color effectiveSubtitleColor = hovered
            ? hoverSubtitleColor
            : subtitleColor;
        final Color effectiveTrailingColor = hovered
            ? hoverTrailingColor
            : trailingColor;
        final Widget? leadingWidget =
            leadingBuilder?.call(
              context,
              hovered: hovered,
              selected: selected,
            ) ??
            leading ??
            _defaultLeading();

        return Row(
          children: <Widget>[
            if (leadingWidget != null) ...<Widget>[
              leadingWidget,
              SizedBox(width: leadingGap),
            ],
            Expanded(
              child: Column(
                mainAxisSize: MainAxisSize.min,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: <Widget>[
                  Text(
                    title,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: (titleStyle ?? HyprTypography.popRow).copyWith(
                      color: effectiveTitleColor,
                    ),
                  ),
                  if (subtitle != null) ...<Widget>[
                    const SizedBox(height: 1),
                    Text(
                      subtitle!,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: (subtitleStyle ?? HyprTypography.compactMono)
                          .copyWith(color: effectiveSubtitleColor),
                    ),
                  ],
                ],
              ),
            ),
            if (trailing != null) ...<Widget>[
              const SizedBox(width: HyprSpacing.xxl),
              IconTheme.merge(
                data: IconThemeData(color: effectiveTrailingColor),
                child: DefaultTextStyle.merge(
                  style: HyprTypography.compactMono.copyWith(
                    color: effectiveTrailingColor,
                  ),
                  child: trailing!,
                ),
              ),
            ],
          ],
        );
      },
    );
  }

  Widget? _defaultLeading() {
    final IconData? data = icon;
    if (data == null) {
      return null;
    }

    final Icon iconWidget = Icon(
      data,
      size: HyprIconSizes.tiny,
      color: iconColor ?? HyprColors.textFaint,
    );
    final Color? background = iconBackgroundColor;
    if (background == null) {
      return iconWidget;
    }

    return DecoratedBox(
      decoration: BoxDecoration(
        color: background,
        borderRadius: HyprRadii.controlRadius,
      ),
      child: SizedBox.fromSize(
        size: HyprIconSizes.defaultIconBadge,
        child: iconWidget,
      ),
    );
  }
}
