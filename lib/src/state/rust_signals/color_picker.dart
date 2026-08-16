import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<ColorPickerCommandResult> _colorPickerCommandResultStream() async* {
  final latest = ColorPickerCommandResult.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in ColorPickerCommandResult.rustSignalStream) {
    yield rustSignal.message;
  }
}

/// Results from color picker commands.
final colorPickerCommandResultProvider =
    StreamProvider<ColorPickerCommandResult>(
      (ref) => _colorPickerCommandResultStream(),
    );
