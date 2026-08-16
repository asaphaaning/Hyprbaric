import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

/// Describes a command exposed through the quick actions menu.
class MenuAction {
  const MenuAction({
    required this.id,
    required this.label,
    required this.icon,
    this.category,
  });

  final String id;
  final String label;
  final IconData icon;
  final String? category;

  MenuAction copyWith({
    String? id,
    String? label,
    IconData? icon,
    String? category,
  }) {
    return MenuAction(
      id: id ?? this.id,
      label: label ?? this.label,
      icon: icon ?? this.icon,
      category: category ?? this.category,
    );
  }
}

/// Async Riverpod notifier that can later source menu actions from external
/// services. Right now it seeds a static list so UI wiring is ready.
class MenuActionsNotifier extends AsyncNotifier<List<MenuAction>> {
  @override
  FutureOr<List<MenuAction>> build() {
    return const <MenuAction>[
      MenuAction(
        id: 'widgets',
        label: 'Open Dashboard',
        icon: Icons.dashboard_customize_outlined,
        category: 'system',
      ),
      MenuAction(
        id: 'appearance',
        label: 'Theme Picker',
        icon: Icons.palette_outlined,
        category: 'personalize',
      ),
      MenuAction(
        id: 'settings',
        label: 'Settings',
        icon: Icons.settings_outlined,
        category: 'system',
      ),
      MenuAction(
        id: 'register_shortcut',
        label: 'Register Shortcut',
        icon: Icons.keyboard_command_key_outlined,
        category: 'system',
      ),
      MenuAction(
        id: 'exit',
        label: 'Power Off',
        icon: Icons.power_settings_new_rounded,
        category: 'system',
      ),
    ];
  }

  /// Allows future integration points (e.g. Rust services) to push a new action
  /// list without the UI caring about where it originated.
  void replaceActions(List<MenuAction> actions) {
    state = AsyncValue.data(actions);
  }
}

final menuActionsProvider =
    AsyncNotifierProvider.autoDispose<MenuActionsNotifier, List<MenuAction>>(
      MenuActionsNotifier.new,
    );
