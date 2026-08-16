/// Shared duration vocabulary for Hyprbaric UI and transient feedback.
///
/// `HyprMotion` owns the curve vocabulary. This type names the timing values so
/// non-animated state, timers, and widgets can share the same closed scale.
abstract final class HyprDurations {
  static const Duration queryDebounce = Duration(milliseconds: 35);
  static const Duration pressed = Duration(milliseconds: 60);
  static const Duration hover = Duration(milliseconds: 120);
  static const Duration selection = Duration(milliseconds: 130);
  static const Duration switcher = Duration(milliseconds: 180);
  static const Duration popup = Duration(milliseconds: 220);
  static const Duration workspace = Duration(milliseconds: 260);
  static const Duration toast = Duration(milliseconds: 280);
  static const Duration growPopup = Duration(milliseconds: 360);
  static const Duration osdPeakTick = Duration(milliseconds: 60);
  static const Duration osdLifetime = Duration(milliseconds: 1500);
  static const Duration toastLifetime = Duration(milliseconds: 3800);
}
