import 'dart:async';

import 'package:flutter/material.dart';

import '../../bindings/bindings.dart';
import '../../widgets/primitives/primitives.dart';
import 'control_capture_pads.dart';
import 'control_inspect_button.dart';
import 'control_rocker.dart';
import 'control_settings_row.dart';
import 'controls_chrome.dart';

class ControlsPanel extends StatefulWidget {
  const ControlsPanel({
    super.key,
    required this.borderRadius,
    required this.onCaptureScreenshot,
    required this.onPickColor,
    required this.onToggleRecording,
    required this.onOpenSettings,
    required this.onToast,
    required this.dndEnabled,
    required this.onSetDoNotDisturb,
    required this.nightLightStatus,
    required this.onSetNightLight,
    required this.caffeineStatus,
    required this.onSetCaffeine,
    this.recordingStatus,
  });

  final BorderRadius borderRadius;
  final ValueChanged<ScreenshotMode> onCaptureScreenshot;
  final VoidCallback onPickColor;
  final VoidCallback onToggleRecording;
  final VoidCallback onOpenSettings;
  final ValueChanged<String> onToast;
  final bool dndEnabled;
  final ValueChanged<bool> onSetDoNotDisturb;
  final NightLightStatus? nightLightStatus;
  final ValueChanged<bool> onSetNightLight;
  final CaffeineStatus? caffeineStatus;
  final ValueChanged<bool> onSetCaffeine;
  final RecordingStatus? recordingStatus;

  @override
  State<ControlsPanel> createState() => ControlsPanelState();
}

class ControlsPanelState extends State<ControlsPanel> {
  Timer? _recordingTicker;

  @override
  void initState() {
    super.initState();
    _syncRecordingTicker();
  }

  @override
  void didUpdateWidget(covariant ControlsPanel oldWidget) {
    super.didUpdateWidget(oldWidget);
    _syncRecordingTicker();
  }

  @override
  void dispose() {
    _recordingTicker?.cancel();
    super.dispose();
  }

  void _syncRecordingTicker() {
    if (widget.recordingStatus.ticks && _recordingTicker == null) {
      _recordingTicker = Timer.periodic(const Duration(seconds: 1), (_) {
        if (mounted) {
          setState(() {});
        }
      });
      return;
    }
    if (!widget.recordingStatus.ticks && _recordingTicker != null) {
      _recordingTicker?.cancel();
      _recordingTicker = null;
    }
  }

  @override
  Widget build(BuildContext context) {
    return HyprPopoverPanel(
      borderRadius: widget.borderRadius,
      constraints: const BoxConstraints(minWidth: 432, maxWidth: 432),
      padding: const EdgeInsets.fromLTRB(16, 15, 16, 16),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: <Widget>[
          const ControlSectionLabel('Capture'),
          const SizedBox(height: 11),
          SizedBox(
            height: 84,
            child: Row(
              children: <Widget>[
                Expanded(
                  child: ControlCapturePad(
                    label: 'Region',
                    shortcut: '⇧⌘S',
                    icon: Icons.center_focus_strong_rounded,
                    onPressed: () =>
                        widget.onCaptureScreenshot(ScreenshotMode.region),
                  ),
                ),
                const SizedBox(width: 7),
                Expanded(
                  child: ControlCapturePad(
                    label: 'Window',
                    shortcut: '⌘Prt',
                    icon: Icons.web_asset_rounded,
                    onPressed: () =>
                        widget.onCaptureScreenshot(ScreenshotMode.window),
                  ),
                ),
                const SizedBox(width: 7),
                Expanded(
                  child: ControlCapturePad(
                    label: 'Full',
                    shortcut: 'Prt',
                    icon: Icons.desktop_windows_rounded,
                    onPressed: () =>
                        widget.onCaptureScreenshot(ScreenshotMode.fullScreen),
                  ),
                ),
                const SizedBox(width: 7),
                Expanded(
                  child: ControlRecordPad(
                    active: widget.recordingStatus.active,
                    enabled: widget.recordingStatus.isAvailable,
                    phase: widget.recordingStatus.phase,
                    elapsed: widget.recordingStatus.elapsed,
                    onPressed: widget.recordingStatus.isAvailable
                        ? widget.onToggleRecording
                        : null,
                  ),
                ),
              ],
            ),
          ),
          const HyprSectionBreak(before: 14, after: 13),
          const ControlSectionLabel('Inspect'),
          const SizedBox(height: 11),
          Row(
            children: <Widget>[
              Expanded(
                child: ControlInspectButton(
                  label: 'Color Pick',
                  shortcut: '⇧⌘P',
                  icon: Icons.colorize_rounded,
                  onPressed: widget.onPickColor,
                ),
              ),
              const SizedBox(width: 9),
              const Expanded(
                child: ControlInspectButton(
                  label: 'Magnify',
                  shortcut: '⌘M',
                  icon: Icons.zoom_in_rounded,
                ),
              ),
            ],
          ),
          const HyprSectionBreak(before: 14, after: 13),
          const ControlSectionLabel('Toggles'),
          const SizedBox(height: 11),
          SizedBox(
            height: 100,
            child: Row(
              children: <Widget>[
                Expanded(
                  child: ControlRocker(
                    key: const ValueKey<String>('controls-dnd-rocker'),
                    label: 'DND',
                    shortcut: '⇧⌘D',
                    icon: Icons.do_not_disturb_on_outlined,
                    value: widget.dndEnabled,
                    onChanged: widget.onSetDoNotDisturb,
                  ),
                ),
                const SizedBox(width: 6),
                Expanded(
                  child: ControlRocker(
                    key: const ValueKey<String>('controls-night-light-rocker'),
                    label: 'Night',
                    shortcut: '⇧⌘N',
                    icon: Icons.nights_stay_rounded,
                    value: widget.nightLightStatus.enabled,
                    enabled: widget.nightLightStatus.isAvailable,
                    onChanged: widget.onSetNightLight,
                  ),
                ),
                const SizedBox(width: 6),
                Expanded(
                  child: ControlRocker(
                    key: const ValueKey<String>('controls-caffeine-rocker'),
                    label: 'Caffeine',
                    shortcut: '⇧⌘F12',
                    icon: Icons.local_cafe_rounded,
                    value: widget.caffeineStatus.enabled,
                    enabled: widget.caffeineStatus.isAvailable,
                    onChanged: widget.onSetCaffeine,
                  ),
                ),
              ],
            ),
          ),
          const SizedBox(height: 14),
          ControlSettingsRow(onPressed: widget.onOpenSettings),
        ],
      ),
    );
  }
}

extension on NightLightStatus? {
  bool get enabled => switch (this) {
    NightLightStatusAvailable(:final enabled) => enabled,
    NightLightStatusUnavailable(:final enabled) => enabled,
    _ => false,
  };

  bool get isAvailable => this is NightLightStatusAvailable;
}

extension on CaffeineStatus? {
  bool get enabled => switch (this) {
    CaffeineStatusAvailable(:final enabled) => enabled,
    _ => false,
  };

  bool get isAvailable => this is CaffeineStatusAvailable;
}

extension on RecordingStatus? {
  bool get isAvailable =>
      this is RecordingStatusIdle ||
      this is RecordingStatusSelecting ||
      this is RecordingStatusRecording ||
      this is RecordingStatusStopping;

  bool get active =>
      this is RecordingStatusSelecting ||
      this is RecordingStatusRecording ||
      this is RecordingStatusStopping;

  bool get ticks =>
      this is RecordingStatusRecording || this is RecordingStatusStopping;

  String get phase => switch (this) {
    RecordingStatusIdle() => 'STBY',
    RecordingStatusSelecting() => 'SEL',
    RecordingStatusRecording() => 'REC',
    RecordingStatusStopping() => 'STOP',
    RecordingStatusUnavailable() => 'N/A',
    _ => 'N/A',
  };

  String get elapsed {
    final int? startedAtMs = switch (this) {
      RecordingStatusRecording(:final startedAtMs) => startedAtMs.toInt(),
      RecordingStatusStopping(:final startedAtMs) => startedAtMs.toInt(),
      _ => null,
    };
    if (startedAtMs == null) {
      return '00:00';
    }
    final DateTime startedAt = DateTime.fromMillisecondsSinceEpoch(startedAtMs);
    final Duration elapsed = DateTime.now().difference(startedAt);
    final int minutes = elapsed.inMinutes.clamp(0, 99);
    final int seconds = elapsed.inSeconds.remainder(60);
    return '${minutes.toString().padLeft(2, '0')}:${seconds.toString().padLeft(2, '0')}';
  }
}
