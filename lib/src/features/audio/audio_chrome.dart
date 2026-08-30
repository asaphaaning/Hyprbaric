import 'package:flutter/material.dart';

import '../../bindings/bindings.dart';
import '../../widgets/hypr_surface.dart';

extension AudioStatusView on AudioStatus {
  bool get isAvailable => this is AudioStatusAvailable;

  AudioEndpoint? get output => switch (this) {
    AudioStatusAvailable(:final output) => output,
    AudioStatusUnavailable() => null,
    _ => null,
  };

  AudioEndpoint? get input => switch (this) {
    AudioStatusAvailable(:final input) => input,
    AudioStatusUnavailable() => null,
    _ => null,
  };

  String? get message => switch (this) {
    AudioStatusAvailable() => null,
    AudioStatusUnavailable(:final message) => message,
    _ => null,
  };
}

abstract final class AudioMixerColors {
  static const Color chassisTop = Color(0xF0161A20);
  static const Color chassisBottom = Color(0xFA0E1218);
  static const Color deckTop = Color(0xFF242527);
  static const Color deckMiddle = Color(0xFF202123);
  static const Color deckBottom = Color(0xFF1C1D1F);
  static const Color console = Color(0xF51D1E20);
  static const Color well = Color(0xD907090C);
  static const Color rail = Color(0xFF0C0E11);
  static const Color railBorder = Color(0x1CFFFFFF);
  static const Color slot = Color(0xFF24262A);
  static const Color slotBorder = Color(0x26000000);
  static const Color output = Color(0xFF3BCB7C);
  static const Color input = Color(0xFF43D879);
  static const Color warning = Color(0xFFE0C34D);
  static const Color peak = Color(0xFFD45146);
  static const Color handle = Color(0xFF34363B);
  static const Color handleFace = Color(0xFF55585E);
  static const Color handleLine = Color(0xFFB9BDC4);
  static const Color handleBorder = Color(0xFF16181C);
  static const Color accentBorder = HyprColors.accentSoft;
  static const Color label = Color(0x9A9AA5AF);
  static const Color quiet = Color(0xB6A2ACB7);
  static const Color value = Color(0xFFE8EEF5);
}

String audioDecibelReadout(int volume, {required bool muted}) {
  if (muted || volume <= 0) {
    return '−∞';
  }

  final double decibels = (volume.clamp(0, 100) / 100) * 60 - 60;
  return decibels.toStringAsFixed(1).replaceFirst('-', '−');
}

class AudioMixerDivider extends StatelessWidget {
  const AudioMixerDivider({super.key});

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: BoxDecoration(
        gradient: LinearGradient(
          begin: Alignment.topCenter,
          end: Alignment.bottomCenter,
          colors: <Color>[
            Colors.transparent,
            HyprColors.borderSoft.withValues(alpha: 0.75),
            HyprColors.borderSoft.withValues(alpha: 0.75),
            Colors.transparent,
          ],
          stops: const <double>[0, 0.08, 0.92, 1],
        ),
      ),
      child: const SizedBox(width: 1),
    );
  }
}

class AudioMessage extends StatelessWidget {
  const AudioMessage({super.key, required this.message});

  final String message;

  @override
  Widget build(BuildContext context) {
    return Text(
      message,
      maxLines: 2,
      overflow: TextOverflow.ellipsis,
      style: HyprTypography.popRow.copyWith(color: AudioMixerColors.label),
    );
  }
}
