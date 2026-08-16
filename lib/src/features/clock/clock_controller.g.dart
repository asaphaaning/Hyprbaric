// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'clock_controller.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(ClockController)
final clockControllerProvider = ClockControllerProvider._();

final class ClockControllerProvider
    extends $NotifierProvider<ClockController, void> {
  ClockControllerProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'clockControllerProvider',
        isAutoDispose: false,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$clockControllerHash();

  @$internal
  @override
  ClockController create() => ClockController();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(void value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<void>(value),
    );
  }
}

String _$clockControllerHash() => r'cf4bb771a5d498cb469142b0dfc848ed6d9ab4d4';

abstract class _$ClockController extends $Notifier<void> {
  void build();
  @$mustCallSuper
  @override
  WhenComplete runBuild() {
    final ref = this.ref as $Ref<void, void>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<void, void>,
              void,
              Object?,
              Object?
            >;
    return element.handleCreate(ref, build);
  }
}

@ProviderFor(ClockView)
final clockViewProvider = ClockViewProvider._();

final class ClockViewProvider
    extends $NotifierProvider<ClockView, ClockViewState> {
  ClockViewProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'clockViewProvider',
        isAutoDispose: false,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$clockViewHash();

  @$internal
  @override
  ClockView create() => ClockView();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(ClockViewState value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<ClockViewState>(value),
    );
  }
}

String _$clockViewHash() => r'b01419a0f2e0307c1eeed144de2b9d643c68c68d';

abstract class _$ClockView extends $Notifier<ClockViewState> {
  ClockViewState build();
  @$mustCallSuper
  @override
  WhenComplete runBuild() {
    final ref = this.ref as $Ref<ClockViewState, ClockViewState>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<ClockViewState, ClockViewState>,
              ClockViewState,
              Object?,
              Object?
            >;
    return element.handleCreate(ref, build);
  }
}
