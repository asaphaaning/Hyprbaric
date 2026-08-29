import 'package:flutter_test/flutter_test.dart';
import 'package:hyprbaric/src/features/audio/audio_chrome.dart';

void main() {
  test('audio decibel readout maps the percentage range to minus sixty dB', () {
    expect(audioDecibelReadout(100, muted: false), '0.0');
    expect(audioDecibelReadout(50, muted: false), '−30.0');
    expect(audioDecibelReadout(0, muted: false), '−∞');
    expect(audioDecibelReadout(80, muted: true), '−∞');
  });
}
