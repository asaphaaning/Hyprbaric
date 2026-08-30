import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hyprbaric/src/features/audio/brightness_knob.dart';
import 'package:hyprbaric/src/features/audio/brightness_knob_painter.dart';

void main() {
  test('console lamp illumination repaints independently', () {
    const BrightnessKnobPainter painter = BrightnessKnobPainter(
      value: 0.75,
      lampValue: 0.74,
      enabled: true,
      console: true,
    );

    expect(
      painter.shouldRepaint(
        const BrightnessKnobPainter(
          value: 0.75,
          lampValue: 0.75,
          enabled: true,
          console: true,
        ),
      ),
      isTrue,
    );
  });

  testWidgets('console pointer settles before its lamps', (
    WidgetTester tester,
  ) async {
    await tester.pumpWidget(
      const MaterialApp(
        home: BrightnessKnob(
          value: 75,
          enabled: true,
          presentation: BrightnessKnobPresentation.console,
          onChanged: _ignore,
          onChangeEnd: _ignore,
        ),
      ),
    );

    final List<Duration> durations = tester
        .widgetList<TweenAnimationBuilder<double>>(
          find.byType(TweenAnimationBuilder<double>),
        )
        .map((TweenAnimationBuilder<double> animation) => animation.duration)
        .toList();

    expect(
      durations,
      containsAll(<Duration>[
        const Duration(milliseconds: 80),
        const Duration(milliseconds: 220),
      ]),
    );
  });
}

void _ignore(int value) {}
