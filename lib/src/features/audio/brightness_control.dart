import 'package:flutter/material.dart';

import '../../bindings/bindings.dart';
import '../../widgets/primitives/primitives.dart';
import 'brightness_knob.dart';
import 'brightness_status_view.dart';

export 'brightness_knob.dart' show BrightnessKnob, BrightnessKnobState;
export 'brightness_knob_painter.dart' show BrightnessKnobPainter;
export 'brightness_knob_readout.dart' show BrightnessKnobReadout;
export 'brightness_status_view.dart' show BrightnessStatusView;

class BrightnessControl extends StatefulWidget {
  const BrightnessControl({
    super.key,
    required this.status,
    required this.loading,
    required this.onSetBrightness,
  });

  final BrightnessStatus? status;
  final bool loading;
  final ValueChanged<int> onSetBrightness;

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
      widget.onSetBrightness(committed);
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

    return Padding(
      padding: const EdgeInsets.fromLTRB(0, 2, 0, 0),
      child: Center(
        child: Semantics(
          slider: available,
          label: label,
          value: available ? '$value' : '--',
          child: Opacity(
            opacity: available ? 1 : 0.45,
            child: BrightnessKnob(
              value: value.clamp(0, 100),
              enabled: available,
              onChanged: (int next) => _setValue(next.toDouble()),
              onChangeEnd: (int next) =>
                  _setValue(next.toDouble(), force: true),
            ),
          ),
        ),
      ),
    );
  }
}
