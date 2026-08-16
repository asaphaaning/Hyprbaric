import 'package:flutter/material.dart';

abstract final class SettingsOverlayLayout {
  static const double width = 760;
  static const double height = 540;
  static const double bodyPadding = 18;
  static const double headerGap = 18;
  static const double surfaceRadius = 18;

  static const Key contentKey = ValueKey<String>('settings-overlay-content');
}
