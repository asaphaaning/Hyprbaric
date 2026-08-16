import '../../bindings/bindings.dart';

extension BrightnessStatusView on BrightnessStatus {
  bool get isAvailable => this is BrightnessStatusAvailable;

  int get displayValue => switch (this) {
    BrightnessStatusAvailable(:final value) => value,
    _ => 0,
  };

  String get displayLabel => switch (this) {
    BrightnessStatusAvailable(:final device) => '$device brightness',
    BrightnessStatusDiscovering(:final message) => message,
    BrightnessStatusUnavailable(:final message) => message,
    _ => 'Brightness unavailable',
  };
}
