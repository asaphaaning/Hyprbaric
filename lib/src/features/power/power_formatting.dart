import '../../bindings/bindings.dart';

String formatBatteryPercent(PowerStatus? status) {
  final int? percentage = status?.percentage;
  if (percentage == null) {
    return '--%';
  }
  return '${percentage.clamp(0, 100)}%';
}

String formatRemaining(PowerStatus? status) {
  final int? seconds = status?.remainingSeconds?.toInt();
  if (seconds == null || seconds <= 0) {
    return '--';
  }
  final int hours = seconds ~/ 3600;
  final int minutes = (seconds % 3600) ~/ 60;
  if (hours <= 0) {
    return '${minutes}m';
  }
  return '${hours}h ${minutes.toString().padLeft(2, '0')}m';
}

String formatPowerRate(double? watts) {
  if (watts == null) {
    return '--W';
  }
  final String sign = watts > 0 ? '+' : '';
  return '$sign${watts.toStringAsFixed(1)}W';
}

String formatVoltage(double? voltage) {
  if (voltage == null) {
    return '--V';
  }
  return '${voltage.toStringAsFixed(2)}V';
}

String formatTemperature(double? temperature) {
  if (temperature == null) {
    return '--°C';
  }
  return '${temperature.toStringAsFixed(0)}°C';
}

String batteryStateLabel(PowerBatteryState state) => switch (state) {
  PowerBatteryState.charging => 'CHG',
  PowerBatteryState.discharging => 'DISCH',
  PowerBatteryState.empty => 'EMPTY',
  PowerBatteryState.full => 'FULL',
  PowerBatteryState.pendingCharge => 'WAIT',
  PowerBatteryState.pendingDischarge => 'WAIT',
  PowerBatteryState.unknown => 'UNK',
};

String profileLabel(PowerProfile profile) => switch (profile) {
  PowerProfile.saver => 'Saver',
  PowerProfile.balanced => 'Balanced',
  PowerProfile.performance => 'Perf',
};

String profileSubtitle(PowerProfile profile) => switch (profile) {
  PowerProfile.saver => 'low draw',
  PowerProfile.balanced => 'default',
  PowerProfile.performance => 'turbo',
};
