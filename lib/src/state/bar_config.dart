import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../bindings/bindings.dart';
import 'appearance.dart';

/// Immutable configuration describing core bar characteristics.
class BarConfig {
  const BarConfig({
    required this.height,
    required this.position,
    required this.opacity,
    required this.cornerRadius,
    required this.accentHue,
  });

  factory BarConfig.fromAppearance(AppearanceStatus appearance) {
    return BarConfig(
      height: 40,
      position: appearance.position,
      opacity: appearance.opacity,
      cornerRadius: appearance.cornerRadius,
      accentHue: appearance.accentHue,
    );
  }

  final double height;
  final AppearancePosition position;
  final int opacity;
  final int cornerRadius;
  final int accentHue;

  bool get isBottom => position == AppearancePosition.bottom;

  BarConfig copyWith({
    double? height,
    AppearancePosition? position,
    int? opacity,
    int? cornerRadius,
    int? accentHue,
  }) {
    return BarConfig(
      height: height ?? this.height,
      position: position ?? this.position,
      opacity: opacity ?? this.opacity,
      cornerRadius: cornerRadius ?? this.cornerRadius,
      accentHue: accentHue ?? this.accentHue,
    );
  }
}

final barConfigProvider = Provider<BarConfig>((ref) {
  return BarConfig.fromAppearance(ref.watch(currentAppearanceProvider));
});

/// Convenient access to the bar height so widgets can simply watch a `double`
/// while the underlying configuration remains extendable.
final barHeightProvider = Provider<double>((ref) {
  return ref.watch(barConfigProvider).height;
});

final barPositionProvider = Provider<AppearancePosition>((ref) {
  return ref.watch(barConfigProvider).position;
});
