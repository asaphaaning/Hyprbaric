import 'package:riverpod_annotation/riverpod_annotation.dart';

import '../../bindings/bindings.dart';
import '../rust_commands.dart';

part 'power_controller.g.dart';

@Riverpod(keepAlive: true)
class PowerController extends _$PowerController {
  @override
  void build() {}

  void setProfile(PowerProfile profile) {
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(PowerIntent.setProfile(profile));
  }
}
