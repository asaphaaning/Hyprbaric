import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../../bindings/bindings.dart';
import '../../rust_commands.dart';

final keybindingControllerProvider = Provider<KeybindingController>(
  KeybindingController.new,
);

class KeybindingController {
  const KeybindingController(this._ref);

  final Ref _ref;

  void load() {
    _dispatch(const ShortcutSettingsIntent.load());
  }

  void setBinding({
    required ShortcutSettingId shortcut,
    required ShortcutBindingInput binding,
  }) {
    _dispatch(
      ShortcutSettingsIntent.setBinding(shortcut: shortcut, binding: binding),
    );
  }

  void disable(ShortcutSettingId shortcut) {
    _dispatch(ShortcutSettingsIntent.disable(shortcut));
  }

  void reset(ShortcutSettingId shortcut) {
    _dispatch(ShortcutSettingsIntent.reset(shortcut));
  }

  void _dispatch(ShortcutSettingsIntent intent) {
    _ref.read(rustCommandDispatcherProvider).dispatch(intent);
  }
}
