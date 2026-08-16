import 'package:flutter/material.dart';

import '../hypr_surface.dart';

typedef HyprHoverPlateBuilder =
    Widget Function(BuildContext context, {required bool hovered});

class HyprHoverPlate extends StatefulWidget {
  const HyprHoverPlate({
    super.key,
    required this.builder,
    required this.onPressed,
    this.semanticLabel,
    this.enabled = true,
    this.selected = false,
    this.danger = false,
    this.padding = HyprSpacing.none,
    this.borderRadius = HyprRadii.rowRadius,
    this.color = Colors.transparent,
    this.hoverColor = HyprColors.hover,
    this.selectedColor = HyprColors.hoverStrong,
    this.borderColor = Colors.transparent,
    this.hoverBorderColor = Colors.transparent,
    this.selectedBorderColor = HyprColors.border,
    this.borderWidth = 1,
    this.selectedBorderWidth = 1,
    this.clipBehavior = Clip.antiAlias,
  });

  final HyprHoverPlateBuilder builder;
  final VoidCallback? onPressed;
  final String? semanticLabel;
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
  final Clip clipBehavior;

  @override
  State<HyprHoverPlate> createState() => HyprHoverPlateState();
}

class HyprHoverPlateState extends State<HyprHoverPlate> {
  bool _hovered = false;

  bool get hovered => _hovered;

  @override
  Widget build(BuildContext context) {
    final bool interactive = widget.enabled && widget.onPressed != null;
    final Color fill = _fillColor(interactive);
    final Color stroke = _borderColor(interactive);
    final double strokeWidth = widget.selected
        ? widget.selectedBorderWidth
        : widget.borderWidth;
    final Widget plate = MouseRegion(
      onEnter: interactive ? (_) => setState(() => _hovered = true) : null,
      onExit: interactive ? (_) => setState(() => _hovered = false) : null,
      child: AnimatedContainer(
        duration: HyprMotion.hover,
        curve: HyprMotion.hoverCurve,
        decoration: ShapeDecoration(
          color: fill,
          shape: RoundedSuperellipseBorder(
            borderRadius: widget.borderRadius,
            side: BorderSide(color: stroke, width: strokeWidth),
          ),
        ),
        clipBehavior: widget.clipBehavior,
        child: Material(
          color: Colors.transparent,
          child: InkWell(
            onTap: interactive ? widget.onPressed : null,
            hoverColor: Colors.transparent,
            splashColor: Colors.transparent,
            highlightColor: Colors.transparent,
            child: Padding(
              padding: widget.padding,
              child: widget.builder(context, hovered: _hovered && interactive),
            ),
          ),
        ),
      ),
    );

    if (widget.semanticLabel == null) {
      return plate;
    }

    return Semantics(
      button: true,
      enabled: interactive,
      label: widget.semanticLabel,
      child: plate,
    );
  }

  Color _fillColor(bool interactive) {
    if (widget.danger && widget.selected) {
      return HyprColors.dangerHover;
    }
    if (widget.selected) {
      return widget.selectedColor;
    }
    if (interactive && _hovered) {
      return widget.danger ? HyprColors.dangerHoverSoft : widget.hoverColor;
    }
    return widget.color;
  }

  Color _borderColor(bool interactive) {
    if (widget.danger && widget.selected) {
      return HyprColors.danger.withValues(alpha: 0.52);
    }
    if (widget.selected) {
      return widget.selectedBorderColor;
    }
    if (interactive && _hovered) {
      return widget.danger
          ? HyprColors.danger.withValues(alpha: 0.38)
          : widget.hoverBorderColor;
    }
    return widget.borderColor;
  }
}
