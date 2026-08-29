import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hyprbaric/src/bindings/bindings.dart';
import 'package:hyprbaric/src/features/rust_commands.dart';
import 'package:hyprbaric/src/features/setup/setup_guide_host.dart';
import 'package:hyprbaric/src/features/setup/setup_guide_state.dart';
import 'package:hyprbaric/src/state/providers.dart';
import 'package:hyprbaric/src/theme/hypr_palette.dart';

void main() {
  testWidgets('required setup opens once on the automatic host', (
    WidgetTester tester,
  ) async {
    final _RecordingDispatcher dispatcher = _RecordingDispatcher();
    await tester.binding.setSurfaceSize(const Size(1200, 800));
    addTearDown(() => tester.binding.setSurfaceSize(null));

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          setupGuideAutomaticHostProvider.overrideWithValue(true),
          setupStatusProvider.overrideWith(
            (_) => Stream<SetupStatus>.value(
              const SetupStatus(state: SetupState.required),
            ),
          ),
          rustCommandDispatcherProvider.overrideWithValue(dispatcher),
        ],
        child: _surface(const SetupGuideHost()),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.byKey(const ValueKey<String>('setup-guide')), findsOneWidget);
    expect(find.text('Welcome to\nHyprbaric'), findsOneWidget);

    await tester.tap(find.text('SKIP'));
    await tester.pumpAndSettle();

    expect(find.byKey(const ValueKey<String>('setup-guide')), findsNothing);
    expect(
      dispatcher.intents.map((RustIntent intent) => intent.debugLabel),
      contains('setup_complete:skipped'),
    );
  });

  testWidgets('manual request opens setup on a non-automatic host', (
    WidgetTester tester,
  ) async {
    int openingCount = 0;
    await tester.binding.setSurfaceSize(const Size(1200, 800));
    addTearDown(() => tester.binding.setSurfaceSize(null));

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          setupGuideAutomaticHostProvider.overrideWithValue(false),
          setupStatusProvider.overrideWith(
            (_) => Stream<SetupStatus>.value(
              const SetupStatus(state: SetupState.required),
            ),
          ),
        ],
        child: _surface(
          Stack(
            fit: StackFit.expand,
            children: <Widget>[
              SetupGuideHost(onReady: () => openingCount += 1),
              const _ManualLaunchButton(),
            ],
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.byKey(const ValueKey<String>('setup-guide')), findsNothing);

    await tester.tap(find.text('Launch'));
    await tester.pumpAndSettle();

    expect(find.byKey(const ValueKey<String>('setup-guide')), findsOneWidget);
    expect(openingCount, 1);
    final Finder scrim = find.byKey(
      const ValueKey<String>('setup-guide-scrim'),
    );
    expect(tester.getTopLeft(scrim).dy, 43);
    expect(tester.getSize(scrim), const Size(1200, 757));
  });

  testWidgets('guide preserves the v6 split geometry and control vocabulary', (
    WidgetTester tester,
  ) async {
    await tester.binding.setSurfaceSize(const Size(1200, 800));
    addTearDown(() => tester.binding.setSurfaceSize(null));

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          setupGuideAutomaticHostProvider.overrideWithValue(true),
          setupStatusProvider.overrideWith(
            (_) => Stream<SetupStatus>.value(
              const SetupStatus(state: SetupState.required),
            ),
          ),
        ],
        child: _surface(const SetupGuideHost()),
      ),
    );
    await tester.pumpAndSettle();

    final Finder card = find.byKey(const ValueKey<String>('setup-guide'));
    final Finder preview = find.byKey(
      const ValueKey<String>('setup-guide-preview'),
    );
    expect(tester.getSize(card), const Size(980, 600));
    expect(tester.getSize(preview), const Size(441, 600));
    expect(find.text('LIVE PREVIEW'), findsNothing);
    expect(
      find.byKey(const ValueKey<String>('setup-guide-seam')),
      findsOneWidget,
    );

    await tester.tap(find.text('GET STARTED'));
    await tester.pumpAndSettle();
    expect(
      find.byKey(const ValueKey<String>('setup-guide-amount-slider')),
      findsOneWidget,
    );
    expect(
      find.byKey(const ValueKey<String>('setup-guide-hue-slider')),
      findsNothing,
    );

    await tester.tap(find.text('CONTINUE'));
    await tester.pumpAndSettle();
    expect(
      find.byKey(const ValueKey<String>('setup-guide-hue-slider')),
      findsOneWidget,
    );
  });
}

Widget _surface(Widget child) {
  return MaterialApp(
    theme: ThemeData(
      useMaterial3: true,
      extensions: const <ThemeExtension<dynamic>>[HyprPalette.fallback],
    ),
    home: Scaffold(body: Stack(children: <Widget>[child])),
  );
}

class _ManualLaunchButton extends ConsumerWidget {
  const _ManualLaunchButton();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return TextButton(
      onPressed: () => ref.read(setupGuideRequestProvider.notifier).show(),
      child: const Text('Launch'),
    );
  }
}

class _RecordingDispatcher extends RustCommandDispatcher {
  final List<RustIntent> intents = <RustIntent>[];

  @override
  void dispatch(RustIntent intent) {
    intents.add(intent);
  }
}
