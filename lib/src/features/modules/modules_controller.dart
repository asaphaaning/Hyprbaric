import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';
import '../rust_commands.dart';

final modulesControllerProvider = NotifierProvider<ModulesController, void>(
  ModulesController.new,
);

class ModulesController extends Notifier<void> {
  @override
  void build() {}

  void setEnabled(ModuleId module, {required bool enabled}) {
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(ModuleIntent.setEnabled(module: module, enabled: enabled));
  }
}
