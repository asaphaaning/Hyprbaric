// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'tray_controller.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(TrayController)
final trayControllerProvider = TrayControllerProvider._();

final class TrayControllerProvider
    extends $NotifierProvider<TrayController, void> {
  TrayControllerProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'trayControllerProvider',
        isAutoDispose: false,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$trayControllerHash();

  @$internal
  @override
  TrayController create() => TrayController();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(void value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<void>(value),
    );
  }
}

String _$trayControllerHash() => r'df302774d5364db069e67ca22a0460173dfd97ef';

abstract class _$TrayController extends $Notifier<void> {
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
