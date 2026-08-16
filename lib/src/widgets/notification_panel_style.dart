import 'dart:math' as math;

import 'package:flutter/material.dart';

import '../bindings/bindings.dart';
import 'hypr_surface.dart';

abstract final class NotificationPalette {
  static const Color fg1 = Color(0xECCBD2DA);
  static const Color fg3 = Color(0xA0929DA8);
}

Color notificationAccent(NotificationEntry entry) {
  return switch (entry.urgency) {
    NotificationUrgency.critical => HyprColors.danger,
    NotificationUrgency.low => _badgeColor(entry.app).withValues(alpha: 0.82),
    NotificationUrgency.normal => _badgeColor(entry.app),
  };
}

String notificationAgeLabel(Uint64 createdAtMs) {
  final Duration age = DateTime.now().difference(
    DateTime.fromMillisecondsSinceEpoch(createdAtMs.toInt()),
  );
  if (age.inSeconds < 60) {
    return '${math.max(1, age.inSeconds)}s';
  }
  if (age.inMinutes < 60) {
    return '${age.inMinutes}m';
  }
  if (age.inHours < 24) {
    return '${age.inHours}h';
  }
  return '${age.inDays}d';
}

Color _badgeColor(String name) {
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
