import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';
import '../rust_commands.dart';

final scheduleControllerProvider = NotifierProvider<ScheduleController, void>(
  ScheduleController.new,
);

class ScheduleController extends Notifier<void> {
  @override
  void build() {}

  void setDailyWindow({
    required ScheduleAction action,
    required bool enabled,
    required int startHour,
    required int stopHour,
  }) {
    if (startHour < 0 || startHour > 23 || stopHour < 0 || stopHour > 23) {
      return;
    }
    _dispatch(
      ScheduleIntent.setDailyWindow(
        action: action,
        enabled: enabled,
        startHour: startHour,
        stopHour: stopHour,
      ),
    );
  }

  void _dispatch(ScheduleIntent intent) {
    ref.read(rustCommandDispatcherProvider).dispatch(intent);
  }
}
