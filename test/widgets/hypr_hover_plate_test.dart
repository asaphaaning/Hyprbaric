import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hyprbaric/src/widgets/hypr_surface.dart';
import 'package:hyprbaric/src/widgets/primitives/primitives.dart';

void main() {
  testWidgets('HyprHoverPlate resolves hover chrome and builder state', (
    WidgetTester tester,
  ) async {
    var taps = 0;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: Center(
            child: HyprHoverPlate(
              onPressed: () => taps += 1,
              color: Colors.black,
              hoverColor: HyprColors.hover,
              borderColor: Colors.red,
              hoverBorderColor: Colors.blue,
              padding: const EdgeInsets.all(8),
              builder: (BuildContext context, {required bool hovered}) {
                return Text(hovered ? 'hovered' : 'idle');
              },
            ),
          ),
        ),
      ),
    );

    expect(find.text('idle'), findsOneWidget);
    expect(_plateDecoration(tester).color, Colors.black);
    expect(_plateBorder(tester).color, Colors.red);

    final TestGesture mouse = await tester.createGesture(
      kind: PointerDeviceKind.mouse,
    );
    await mouse.addPointer(location: Offset.zero);
    await mouse.moveTo(tester.getCenter(find.byType(HyprHoverPlate)));
    await tester.pumpAndSettle();

    expect(find.text('hovered'), findsOneWidget);
    expect(_plateDecoration(tester).color, HyprColors.hover);
    expect(_plateBorder(tester).color, Colors.blue);

    await tester.tap(find.byType(HyprHoverPlate));
    expect(taps, 1);
    await mouse.removePointer();
  });

  testWidgets('HyprHoverPlate keeps selected chrome without hover', (
    WidgetTester tester,
  ) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: HyprHoverPlate(
            selected: true,
            onPressed: () {},
            selectedColor: HyprColors.hoverStrong,
            selectedBorderColor: HyprColors.border,
            builder: (_, {required bool hovered}) => const Text('selected'),
          ),
        ),
      ),
    );

    expect(_plateDecoration(tester).color, HyprColors.hoverStrong);
    expect(_plateBorder(tester).color, HyprColors.border);
  });

  testWidgets('HyprHoverPlate suppresses disabled taps and hover state', (
    WidgetTester tester,
  ) async {
    var taps = 0;
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: HyprHoverPlate(
            enabled: false,
            onPressed: () => taps += 1,
            color: Colors.black,
            hoverColor: HyprColors.hover,
            builder: (_, {required bool hovered}) =>
                Text(hovered ? 'hovered' : 'idle'),
          ),
        ),
      ),
    );

    final TestGesture mouse = await tester.createGesture(
      kind: PointerDeviceKind.mouse,
    );
    await mouse.addPointer(location: Offset.zero);
    await mouse.moveTo(tester.getCenter(find.byType(HyprHoverPlate)));
    await tester.pumpAndSettle();
    await tester.tap(find.byType(HyprHoverPlate));

    expect(find.text('idle'), findsOneWidget);
    expect(_plateDecoration(tester).color, Colors.black);
    expect(taps, 0);
    await mouse.removePointer();
  });
}

ShapeDecoration _plateDecoration(WidgetTester tester) {
  final AnimatedContainer container = tester.widget<AnimatedContainer>(
    find.descendant(
      of: find.byType(HyprHoverPlate),
      matching: find.byType(AnimatedContainer),
    ),
  );
  return container.decoration! as ShapeDecoration;
}

BorderSide _plateBorder(WidgetTester tester) {
  final ShapeDecoration decoration = _plateDecoration(tester);
  final RoundedSuperellipseBorder shape =
      decoration.shape as RoundedSuperellipseBorder;
  return shape.side;
}
