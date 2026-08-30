import 'package:flutter/material.dart';

import '../../bindings/bindings.dart';
import '../../widgets/hypr_surface.dart';
import '../../widgets/primitives/primitives.dart';
import 'audio_chrome.dart';
import 'brightness_knob.dart';
import 'brightness_status_view.dart';

export 'brightness_knob.dart' show BrightnessKnob, BrightnessKnobState;
export 'brightness_knob_painter.dart' show BrightnessKnobPainter;
export 'brightness_knob_readout.dart' show BrightnessKnobReadout;
export 'brightness_status_view.dart' show BrightnessStatusView;

/// Layouts supported by [BrightnessControl].
enum BrightnessControlPresentation { standalone, console }

class BrightnessControl extends StatefulWidget {
  const BrightnessControl({
    super.key,
    required this.status,
    required this.loading,
    required this.onSetBrightness,
    this.presentation = BrightnessControlPresentation.standalone,
  });

  final BrightnessStatus? status;
  final bool loading;
  final ValueChanged<int> onSetBrightness;
  final BrightnessControlPresentation presentation;

  @override
  State<BrightnessControl> createState() => BrightnessControlState();
}

class BrightnessControlState extends State<BrightnessControl> {
  double? _preview;
  final HyprLiveValue _commits = HyprLiveValue(
    initialValue: 0,
    minimum: 1,
    commitInterval: const Duration(milliseconds: 75),
  );

  @override
  void didUpdateWidget(covariant BrightnessControl oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (_preview != null && widget.status?.displayValue == _preview!.round()) {
      _preview = null;
      _commits.sync(widget.status?.displayValue ?? 0);
    }
  }

  void _setValue(double value, {bool force = false}) {
    final double next = value.clamp(1, 100).toDouble();
    setState(() => _preview = next);
    final int level = next.round();
    _commits.preview(level);
    final int? committed = _commits.commit(force: force);
    if (committed != null) {
      final ValueChanged<int> commit = widget.onSetBrightness;
      WidgetsBinding.instance.addPostFrameCallback((_) => commit(committed));
    }
  }

  @override
  Widget build(BuildContext context) {
    final BrightnessStatus? status = widget.status;
    final bool available = status?.isAvailable ?? false;
    final int value = (_preview ?? status?.displayValue.toDouble() ?? 0)
        .round();
    final String label = widget.loading
        ? 'Reading display...'
        : available
        ? status?.displayLabel ?? 'Display brightness'
        : status?.displayLabel ?? 'Brightness unavailable';

    final Widget knob = Center(
      child: Semantics(
        slider: available,
        label: label,
        value: available ? '$value' : '--',
        child: Opacity(
          opacity: available ? 1 : 0.45,
          child: BrightnessKnob(
            value: value.clamp(0, 100),
            enabled: available,
            presentation:
                widget.presentation == BrightnessControlPresentation.console
                ? BrightnessKnobPresentation.console
                : BrightnessKnobPresentation.labeled,
            onChanged: (int next) => _setValue(next.toDouble()),
            onChangeEnd: (int next) => _setValue(next.toDouble(), force: true),
          ),
        ),
      ),
    );

    if (widget.presentation == BrightnessControlPresentation.console) {
      return Column(
        mainAxisSize: MainAxisSize.min,
        children: <Widget>[
          DecoratedBox(
            decoration: BoxDecoration(
              color: AudioMixerColors.well,
              borderRadius: BorderRadius.circular(6),
              boxShadow: const <BoxShadow>[
                BoxShadow(
                  color: Color(0x70000000),
                  blurRadius: 3,
                  offset: Offset(0, 1),
                  blurStyle: BlurStyle.inner,
                ),
              ],
            ),
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 4),
              child: Text.rich(
                TextSpan(
                  text: 'DISPLAY ',
                  children: <InlineSpan>[
                    TextSpan(
                      text: available ? '$value%' : '--',
                      style: const TextStyle(color: AudioMixerColors.quiet),
                    ),
                  ],
                ),
                style: HyprTypography.compactMonoStrong.copyWith(
                  color: AudioMixerColors.value,
                  fontSize: HyprTypography.size(10),
                  letterSpacing: 2.2,
                ),
              ),
            ),
          ),
          const SizedBox(height: 28),
          knob,
        ],
      );
    }

    return Padding(padding: const EdgeInsets.fromLTRB(0, 2, 0, 0), child: knob);
  }
}
