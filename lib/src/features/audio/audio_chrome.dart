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
  static const Color rail = Color(0x3A101A22);
  static const Color railBorder = Color(0x30B4D8E8);
  static const Color slot = Color(0x4515333B);
  static const Color slotBorder = Color(0x33214756);
  static const Color output = Color(0xFF3BCB7C);
  static const Color input = Color(0xFF00B8C9);
  static const Color handle = Color(0xFF323D4D);
  static const Color handleFace = Color(0xFF677689);
  static const Color handleLine = Color(0xFF93A2B6);
  static const Color handleBorder = Color(0xFF17212D);
  static const Color accentBorder = HyprColors.accentSoft;
  static const Color label = Color(0x9A9AA5AF);
  static const Color quiet = Color(0xB6A2ACB7);
  static const Color value = Color(0xFFE8EEF5);
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
