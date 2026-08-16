import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../rust_commands.dart';

final nightLightControllerProvider =
    NotifierProvider<NightLightController, void>(NightLightController.new);

class NightLightController extends Notifier<void> {
  @override
  void build() {}

  void setEnabled({required bool enabled}) {
    _dispatch(NightLightIntent.setEnabled(enabled: enabled));
  }

  void setTemperature(int temperature) {
    if (temperature <= 0) {
      return;
    }
    _dispatch(NightLightIntent.setTemperature(temperature: temperature));
  }

  void _dispatch(NightLightIntent intent) {
    ref.read(rustCommandDispatcherProvider).dispatch(intent);
  }
}
