// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'launcher_controller.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(LauncherController)
final launcherControllerProvider = LauncherControllerProvider._();

final class LauncherControllerProvider
    extends $NotifierProvider<LauncherController, LauncherViewState> {
  LauncherControllerProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'launcherControllerProvider',
        isAutoDispose: false,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$launcherControllerHash();

  @$internal
  @override
  LauncherController create() => LauncherController();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(LauncherViewState value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<LauncherViewState>(value),
    );
  }
}

String _$launcherControllerHash() =>
    r'30aaf800757015cf26f4f606a0a80a24d2997f88';

abstract class _$LauncherController extends $Notifier<LauncherViewState> {
  LauncherViewState build();
  @$mustCallSuper
  @override
  WhenComplete runBuild() {
    final ref = this.ref as $Ref<LauncherViewState, LauncherViewState>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<LauncherViewState, LauncherViewState>,
              LauncherViewState,
              Object?,
              Object?
            >;
    return element.handleCreate(ref, build);
  }
}
