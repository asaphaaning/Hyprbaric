import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<AudioStatus> _audioStatusStream() async* {
  final latest = AudioStatus.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in AudioStatus.rustSignalStream) {
    yield rustSignal.message;
  }
}

Stream<AudioCommandResult> _audioCommandResultStream() async* {
  final latest = AudioCommandResult.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in AudioCommandResult.rustSignalStream) {
    yield rustSignal.message;
  }
}

Stream<BrightnessStatus> _brightnessStatusStream() async* {
  final latest = BrightnessStatus.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in BrightnessStatus.rustSignalStream) {
    yield rustSignal.message;
  }
}

Stream<BrightnessCommandResult> _brightnessCommandResultStream() async* {
  final latest = BrightnessCommandResult.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in BrightnessCommandResult.rustSignalStream) {
    yield rustSignal.message;
  }
}

/// Live PipeWire/WirePlumber audio state emitted from Rust.
final audioStatusProvider = StreamProvider<AudioStatus>(
  (ref) => _audioStatusStream(),
);

/// Results from volume and mute commands.
final audioCommandResultProvider = StreamProvider<AudioCommandResult>(
  (ref) => _audioCommandResultStream(),
);

/// Live sysfs backlight state emitted from Rust.
final brightnessStatusProvider = StreamProvider<BrightnessStatus>(
  (ref) => _brightnessStatusStream(),
);

/// Results from brightness commands.
final brightnessCommandResultProvider = StreamProvider<BrightnessCommandResult>(
  (ref) => _brightnessCommandResultStream(),
);
