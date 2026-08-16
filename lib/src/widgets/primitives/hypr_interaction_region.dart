import 'package:flutter/material.dart';

@immutable
class HyprInteractionState {
  const HyprInteractionState({
    required this.hovered,
    required this.pressed,
    required this.enabled,
  });

  final bool hovered;
  final bool pressed;
  final bool enabled;

  bool get active => hovered || pressed;
}

typedef HyprInteractionRegionBuilder =
    Widget Function(BuildContext context, HyprInteractionState state);

class HyprInteractionRegion extends StatefulWidget {
  const HyprInteractionRegion({
    super.key,
    required this.builder,
    this.onPressed,
    this.onTapUp,
    this.onSecondaryTapUp,
    this.semanticLabel,
    this.enabled = true,
    this.trackHover = true,
    this.cursor = SystemMouseCursors.click,
    this.behavior = HitTestBehavior.opaque,
  });

  final HyprInteractionRegionBuilder builder;
  final VoidCallback? onPressed;
  final ValueChanged<TapUpDetails>? onTapUp;
  final ValueChanged<TapUpDetails>? onSecondaryTapUp;
  final String? semanticLabel;
  final bool enabled;
  final bool trackHover;
  final MouseCursor cursor;
  final HitTestBehavior behavior;

  @override
  State<HyprInteractionRegion> createState() => _HyprInteractionRegionState();
}

class _HyprInteractionRegionState extends State<HyprInteractionRegion> {
  bool _hovered = false;
  bool _pressed = false;

  bool get _respondsToTap =>
      widget.enabled &&
      (widget.onPressed != null ||
          widget.onTapUp != null ||
          widget.onSecondaryTapUp != null);

  bool get _tracksPointer =>
      widget.enabled && (widget.trackHover || _respondsToTap);

  void _setHovered(bool value) {
    if (_hovered == value) {
      return;
    }
    setState(() => _hovered = value);
  }

  void _setPressed(bool value) {
    if (_pressed == value) {
      return;
    }
    setState(() => _pressed = value);
  }

  @override
  Widget build(BuildContext context) {
    final HyprInteractionState state = HyprInteractionState(
      hovered: _tracksPointer && _hovered,
      pressed: _respondsToTap && _pressed,
      enabled: _respondsToTap,
    );
    final Widget region = MouseRegion(
      cursor: _respondsToTap ? widget.cursor : MouseCursor.defer,
      onEnter: _tracksPointer ? (_) => _setHovered(true) : null,
      onExit: _tracksPointer
          ? (_) {
              _setHovered(false);
              _setPressed(false);
            }
          : null,
      child: GestureDetector(
        behavior: widget.behavior,
        onTap: _respondsToTap ? widget.onPressed : null,
        onTapDown: _respondsToTap ? (_) => _setPressed(true) : null,
        onTapCancel: _respondsToTap ? () => _setPressed(false) : null,
        onTapUp: _respondsToTap
            ? (TapUpDetails details) {
                _setPressed(false);
                widget.onTapUp?.call(details);
              }
            : null,
        onSecondaryTapUp: _respondsToTap
            ? (TapUpDetails details) {
                _setPressed(false);
                widget.onSecondaryTapUp?.call(details);
              }
            : null,
        child: widget.builder(context, state),
      ),
    );

    final String? label = widget.semanticLabel;
    if (label == null) {
      return region;
    }

    return Semantics(
      button: true,
      enabled: _respondsToTap,
      label: label,
      child: region,
    );
  }
}
