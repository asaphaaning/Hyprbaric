import 'package:flutter/material.dart';

import '../hypr_surface.dart';

class HyprGlyphBadge extends StatelessWidget {
  const HyprGlyphBadge({
    super.key,
    required this.name,
    this.dimension = 20,
    this.maxCharacters = 2,
    this.backgroundColor,
    this.foregroundColor = HyprColors.surfaceStrong,
    this.borderColor = const Color(0x18FFFFFF),
    this.borderRadius = HyprRadii.badgeRadius,
    this.textStyle,
  });

  final String name;
  final double dimension;
  final int maxCharacters;
  final Color? backgroundColor;
  final Color foregroundColor;
  final Color borderColor;
  final BorderRadius borderRadius;
  final TextStyle? textStyle;

  static String initialsFor(String name, {int maxCharacters = 2}) {
    final List<String> parts = name
        .trim()
        .split(RegExp(r'[\s._-]+'))
        .where((String part) => part.isNotEmpty)
        .toList(growable: false);
    if (parts.isEmpty || maxCharacters <= 0) {
      return '?';
    }
    if (maxCharacters == 1) {
      return parts.first.characters.first.toUpperCase();
    }
    if (parts.length == 1) {
      final Characters characters = parts.single.characters;
      return characters.take(maxCharacters).toString().toUpperCase();
    }
    return parts
        .take(maxCharacters)
        .map((String part) => part.characters.first)
        .join()
        .toUpperCase();
  }

  static Color colorFor(String name) {
    const List<Color> palette = <Color>[
      Color(0xFF55A7FF),
      Color(0xFF8F7CFF),
      Color(0xFFB45BE2),
      Color(0xFFE16658),
      Color(0xFFE2B755),
      Color(0xFF46B884),
      Color(0xFF4CB9D8),
    ];
    final int hash = name.codeUnits.fold<int>(
      0,
      (int value, int unit) => 0x1fffffff & (value * 31 + unit),
    );
    return palette[hash % palette.length];
  }

  @override
  Widget build(BuildContext context) {
    final TextStyle style =
        textStyle ??
        HyprTypography.appBadge.copyWith(
          color: foregroundColor,
          fontWeight: FontWeight.w800,
        );

    return DecoratedBox(
      decoration: ShapeDecoration(
        color: backgroundColor ?? colorFor(name),
        shape: RoundedSuperellipseBorder(
          borderRadius: borderRadius,
          side: BorderSide(color: borderColor),
        ),
      ),
      child: SizedBox.square(
        dimension: dimension,
        child: Center(
          child: Text(
            initialsFor(name, maxCharacters: maxCharacters),
            maxLines: 1,
            overflow: TextOverflow.clip,
            style: style,
          ),
        ),
      ),
    );
  }
}
