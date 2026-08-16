import 'dart:async';

import 'package:riverpod_annotation/riverpod_annotation.dart';

import '../../bindings/bindings.dart';
import '../../state/providers.dart';
import '../rust_commands.dart';

part 'controls_controller.g.dart';

extension ScreenshotModeLabel on ScreenshotMode {
  String get label => switch (this) {
    ScreenshotMode.region => 'Region',
    ScreenshotMode.window => 'Window',
    ScreenshotMode.fullScreen => 'Full screen',
  };
}

@Riverpod(keepAlive: true)
class ControlsController extends _$ControlsController {
  @override
  void build() {
    ref.listen<AsyncValue<ScreenshotCommandResult>>(
      screenshotCommandResultProvider,
      (_, AsyncValue<ScreenshotCommandResult> next) {
        next.whenData(_handleScreenshotResult);
      },
    );
    ref.listen<AsyncValue<ColorPickerCommandResult>>(
      colorPickerCommandResultProvider,
      (_, AsyncValue<ColorPickerCommandResult> next) {
        next.whenData(_handleColorPickerResult);
      },
    );
    ref.listen<AsyncValue<RecordingCommandResult>>(
      recordingCommandResultProvider,
      (_, AsyncValue<RecordingCommandResult> next) {
        next.whenData(_handleRecordingResult);
      },
    );
    ref.listen<AsyncValue<CaffeineCommandResult>>(
      caffeineCommandResultProvider,
      (_, AsyncValue<CaffeineCommandResult> next) {
        next.whenData(_handleCaffeineResult);
      },
    );
    ref.listen<AsyncValue<NightLightCommandResult>>(
      nightLightCommandResultProvider,
      (_, AsyncValue<NightLightCommandResult> next) {
        next.whenData(_handleNightLightResult);
      },
    );
  }

  void showToast(String message) {
    ref
        .read(transientOverlayProvider.notifier)
        .showLocalToast(app: 'Controls', message: message);
  }

  void captureScreenshot(ScreenshotMode mode) {
    showToast(
      mode == ScreenshotMode.region
          ? 'Select a screenshot region'
          : 'Capturing ${mode.label.toLowerCase()}',
    );

    Future<void>.delayed(const Duration(milliseconds: 120), () {
      ref
          .read(rustCommandDispatcherProvider)
          .dispatch(ScreenshotIntent.capture(mode));
    });
  }

  void pickColor() {
    showToast('Pick a screen color');

    Future<void>.delayed(const Duration(milliseconds: 120), () {
      ref
          .read(rustCommandDispatcherProvider)
          .dispatch(const ColorPickerIntent.pick());
    });
  }

  void toggleRecording() {
    showToast('Select recording region');

    Future<void>.delayed(const Duration(milliseconds: 120), () {
      ref
          .read(rustCommandDispatcherProvider)
          .dispatch(const RecordingIntent.toggle(RecordingMode.region));
    });
  }

  void _handleScreenshotResult(ScreenshotCommandResult result) {
    if (result.command != ScreenshotCommand.capture) {
      return;
    }
    final overlays = ref.read(transientOverlayProvider.notifier);
    switch (result.outcome) {
      case ScreenshotCommandOutcome.started:
        return;
      case ScreenshotCommandOutcome.saved:
        final String fileName = result.path == null
            ? 'screenshot'
            : result.path!.split('/').last;
        overlays.showLocalToast(
          app: 'Screenshots',
          message: result.message ?? 'Saved and copied $fileName',
        );
      case ScreenshotCommandOutcome.cancelled:
        overlays.showLocalToast(
          app: 'Screenshots',
          message: '${result.mode.label} cancelled',
        );
      case ScreenshotCommandOutcome.failed:
        overlays.showLocalToast(
          app: 'Screenshots',
          message: result.message ?? '${result.mode.label} failed',
          urgency: NotificationUrgency.critical,
        );
    }
  }

  void _handleColorPickerResult(ColorPickerCommandResult result) {
    if (result.command != ColorPickerCommand.pick) {
      return;
    }
    final overlays = ref.read(transientOverlayProvider.notifier);
    switch (result.outcome) {
      case ColorPickerCommandOutcome.started:
        return;
      case ColorPickerCommandOutcome.picked:
        overlays.showLocalToast(
          app: 'Color picker',
          message: 'Copied ${result.color ?? 'color'}',
        );
      case ColorPickerCommandOutcome.cancelled:
        overlays.showLocalToast(
          app: 'Color picker',
          message: 'Color pick cancelled',
        );
      case ColorPickerCommandOutcome.failed:
        overlays.showLocalToast(
          app: 'Color picker',
          message: result.message ?? 'Color pick failed',
          urgency: NotificationUrgency.critical,
        );
    }
  }

  void _handleRecordingResult(RecordingCommandResult result) {
    final overlays = ref.read(transientOverlayProvider.notifier);
    switch (result) {
      case RecordingCommandResultStarted():
        return;
      case RecordingCommandResultSaved(:final path):
        final String fileName = path.split('/').last;
        overlays.showLocalToast(app: 'Recording', message: 'Saved $fileName');
      case RecordingCommandResultCancelled():
        overlays.showLocalToast(
          app: 'Recording',
          message: 'Recording cancelled',
        );
      case RecordingCommandResultFailed(:final message):
        overlays.showLocalToast(
          app: 'Recording',
          message: message,
          urgency: NotificationUrgency.critical,
        );
    }
  }

  void _handleCaffeineResult(CaffeineCommandResult result) {
    switch (result) {
      case CaffeineCommandResultStarted():
        return;
      case CaffeineCommandResultSaved(:final command):
        showToast(command.enabled ? 'Caffeine enabled' : 'Caffeine disabled');
      case CaffeineCommandResultFailed(:final message):
        ref
            .read(transientOverlayProvider.notifier)
            .showLocalToast(
              app: 'Controls',
              message: message,
              urgency: NotificationUrgency.critical,
            );
    }
  }

  void _handleNightLightResult(NightLightCommandResult result) {
    switch (result) {
      case NightLightCommandResultStarted():
        return;
      case NightLightCommandResultSaved(:final command):
        showToast(command.nightLightMessage);
      case NightLightCommandResultFailed(:final message):
        ref
            .read(transientOverlayProvider.notifier)
            .showLocalToast(
              app: 'Night light',
              message: message,
              urgency: NotificationUrgency.critical,
            );
    }
  }
}

extension on CaffeineCommand {
  bool get enabled => switch (this) {
    CaffeineCommandSetEnabled(:final enabled) => enabled,
    _ => false,
  };
}

extension on NightLightCommand {
  String get nightLightMessage => switch (this) {
    NightLightCommandSetEnabled(:final enabled) =>
      enabled ? 'Night light enabled' : 'Night light disabled',
    NightLightCommandSetTemperature(:final temperature) =>
      'Night light set to ${temperature}K',
    _ => 'Night light saved',
  };
}
