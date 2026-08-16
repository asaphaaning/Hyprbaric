import 'package:riverpod_annotation/riverpod_annotation.dart';

import '../../bindings/bindings.dart';
import '../../state/rust_signals/clock.dart';
import '../rust_commands.dart';

part 'clock_controller.g.dart';

class ClockViewState {
  const ClockViewState({
    required this.timeLabel,
    required this.dateLabel,
    required this.monthLabel,
    required this.weekNumber,
    required this.utcOffset,
    required this.days,
  });

  factory ClockViewState.fromStatus(ClockStatus? status) {
    if (status == null) {
      return fallback;
    }
    return ClockViewState(
      timeLabel: status.timeLabel,
      dateLabel: status.dateLabel,
      monthLabel: status.monthLabel,
      weekNumber: status.weekNumber,
      utcOffset: status.utcOffset,
      days: status.days,
    );
  }

  static const ClockViewState fallback = ClockViewState(
    timeLabel: '--:--',
    dateLabel: '---, --- --',
    monthLabel: 'Calendar',
    weekNumber: 0,
    utcOffset: 'UTC+00:00',
    days: <CalendarDay>[],
  );

  final String timeLabel;
  final String dateLabel;
  final String monthLabel;
  final int weekNumber;
  final String utcOffset;
  final List<CalendarDay> days;
}

@Riverpod(keepAlive: true)
class ClockController extends _$ClockController {
  @override
  void build() {}

  void calendar(CalendarCommand command) {
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(ClockIntent.calendar(command));
  }
}

@Riverpod(keepAlive: true)
class ClockView extends _$ClockView {
  @override
  ClockViewState build() {
    final ClockStatus? status = ref.watch(clockStatusProvider).asData?.value;
    return ClockViewState.fromStatus(status);
  }
}
