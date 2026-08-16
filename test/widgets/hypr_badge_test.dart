import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hyprbaric/src/widgets/hypr_surface.dart';
import 'package:hyprbaric/src/widgets/primitives/primitives.dart';

void main() {
  testWidgets('HyprBadge applies fill, border, padding, and fixed size', (
    WidgetTester tester,
  ) async {
    const Color fill = Color(0xFF101820);
    const Color border = Color(0xFF55A7FF);
    const EdgeInsets padding = EdgeInsets.symmetric(horizontal: 5, vertical: 2);

    await tester.pumpWidget(
      const MaterialApp(
        home: Scaffold(
          body: HyprBadge(
            color: fill,
            borderColor: border,
            padding: padding,
            width: 40,
            height: 20,
            child: Text('A'),
          ),
        ),
      ),
    );

    final ShapeDecoration decoration = _decoration(tester);
    final RoundedSuperellipseBorder shape =
        decoration.shape as RoundedSuperellipseBorder;
    final Padding badgePadding = tester.widget<Padding>(
      find.descendant(
        of: find.byType(HyprBadge),
        matching: find.byType(Padding),
      ),
    );
    final SizedBox box = tester.widget<SizedBox>(
      find.descendant(
        of: find.byType(HyprBadge),
        matching: find.byType(SizedBox),
      ),
    );

    expect(decoration.color, fill);
    expect(shape.side.color, border);
    expect(badgePadding.padding, padding);
    expect(box.width, 40);
    expect(box.height, 20);
  });

  testWidgets('HyprBadge.text renders styled label', (
    WidgetTester tester,
  ) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: HyprBadge.text(
            label: '3',
            color: HyprColors.accentSoft,
            borderColor: HyprColors.accent,
            textColor: HyprColors.accent,
            style: HyprTypography.compactMonoStrong.copyWith(
              fontSize: HyprTypography.size(9.5),
            ),
          ),
        ),
      ),
    );

    final Text label = tester.widget<Text>(find.text('3'));
    expect(label.style?.color, HyprColors.accent);
    expect(label.style?.fontSize, HyprTypography.size(9.5));
    expect(_decoration(tester).color, HyprColors.accentSoft);
  });
}

ShapeDecoration _decoration(WidgetTester tester) {
  final DecoratedBox box = tester.widget<DecoratedBox>(
    find.descendant(
      of: find.byType(HyprBadge),
      matching: find.byType(DecoratedBox),
    ),
  );
  return box.decoration as ShapeDecoration;
}
