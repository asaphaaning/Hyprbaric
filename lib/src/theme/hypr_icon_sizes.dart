import 'package:flutter/widgets.dart';

/// Shared icon and compact-control sizing vocabulary.
abstract final class HyprIconSizes {
  static const double micro = 7;
  static const double tiny = 12;
  static const double small = 14;
  static const double compact = 15;
  static const double bar = 17;
  static const double medium = 18;
  static const double action = 22;
  static const double large = 24;

  static const Size barButton = Size.square(30);
  static const Size navigationButton = Size.square(28);
  static const Size compactButton = Size(28, 26);
  static const Size defaultIconBadge = Size.square(22);
}
