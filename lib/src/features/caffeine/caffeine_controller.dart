import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../rust_commands.dart';

final caffeineControllerProvider = NotifierProvider<CaffeineController, void>(
  CaffeineController.new,
);

class CaffeineController extends Notifier<void> {
  @override
  void build() {}

  void setEnabled({required bool enabled}) {
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(CaffeineIntent.setEnabled(enabled: enabled));
  }
}
