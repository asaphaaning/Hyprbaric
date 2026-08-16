import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hyprbaric/src/widgets/hypr_surface.dart';
import 'package:hyprbaric/src/widgets/primitives/primitives.dart';

void main() {
  testWidgets('HyprActionRow resolves title, subtitle, and hover state', (
    WidgetTester tester,
  ) async {
    var taps = 0;

    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: Center(
            child: HyprActionRow(
              title: 'Network settings',
              subtitle: 'Open NetworkManager',
              onPressed: () => taps += 1,
              titleColor: Colors.red,
              hoverTitleColor: Colors.blue,
              subtitleColor: Colors.green,
              hoverSubtitleColor: Colors.yellow,
              leadingBuilder:
                  (_, {required bool hovered, required bool selected}) =>
                      Text(hovered ? 'hover' : 'idle'),
            ),
          ),
        ),
      ),
    );

    expect(find.text('Network settings'), findsOneWidget);
    expect(find.text('Open NetworkManager'), findsOneWidget);
    expect(find.text('idle'), findsOneWidget);
    expect(_text(tester, 'Network settings').style?.color, Colors.red);
    expect(_text(tester, 'Open NetworkManager').style?.color, Colors.green);

    final TestGesture mouse = await tester.createGesture(
      kind: PointerDeviceKind.mouse,
    );
    await mouse.addPointer(location: Offset.zero);
    await mouse.moveTo(tester.getCenter(find.byType(HyprActionRow)));
    await tester.pumpAndSettle();

    expect(find.text('hover'), findsOneWidget);
    expect(_text(tester, 'Network settings').style?.color, Colors.blue);
    expect(_text(tester, 'Open NetworkManager').style?.color, Colors.yellow);

    await tester.tap(find.byType(HyprActionRow));
    expect(taps, 1);
    await mouse.removePointer();
  });

  testWidgets('HyprActionRow selected state flows into hover plate chrome', (
    WidgetTester tester,
  ) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: HyprActionRow(
            title: 'Appearance',
            onPressed: () {},
            selected: true,
            selectedColor: HyprColors.fillStrong,
            selectedBorderColor: HyprColors.borderSoft,
          ),
        ),
      ),
    );

    expect(_plateDecoration(tester).color, HyprColors.fillStrong);
    expect(_plateBorder(tester).color, HyprColors.borderSoft);
  });

  testWidgets(
    'HyprActionRow renders default icon badges and trailing content',
    (WidgetTester tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: HyprActionRow(
              title: 'Bar settings',
              onPressed: () {},
              icon: Icons.settings_rounded,
              iconColor: HyprColors.accent,
              iconBackgroundColor: const Color(0x3055A7FF),
              trailing: const Text('›'),
            ),
          ),
        ),
      );

      expect(find.byIcon(Icons.settings_rounded), findsOneWidget);
      expect(find.text('›'), findsOneWidget);
    },
  );
}

Text _text(WidgetTester tester, String value) {
  return tester.widget<Text>(find.text(value));
}

ShapeDecoration _plateDecoration(WidgetTester tester) {
  final AnimatedContainer container = tester.widget<AnimatedContainer>(
    find.descendant(
      of: find.byType(HyprActionRow),
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
