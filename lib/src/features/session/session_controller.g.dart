// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'session_controller.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(SessionController)
final sessionControllerProvider = SessionControllerProvider._();

final class SessionControllerProvider
    extends $NotifierProvider<SessionController, SessionLauncherState> {
  SessionControllerProvider._()
    : super(
        from: null,
        argument: null,
        retry: null,
        name: r'sessionControllerProvider',
        isAutoDispose: false,
        dependencies: null,
        $allTransitiveDependencies: null,
      );

  @override
  String debugGetCreateSourceHash() => _$sessionControllerHash();

  @$internal
  @override
  SessionController create() => SessionController();

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(SessionLauncherState value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<SessionLauncherState>(value),
    );
  }
}

String _$sessionControllerHash() => r'cc3123b980731a0fee98ca640a8803a4a2dc340b';

abstract class _$SessionController extends $Notifier<SessionLauncherState> {
  SessionLauncherState build();
  @$mustCallSuper
  @override
  WhenComplete runBuild() {
    final ref = this.ref as $Ref<SessionLauncherState, SessionLauncherState>;
    final element =
        ref.element
            as $ClassProviderElement<
              AnyNotifier<SessionLauncherState, SessionLauncherState>,
              SessionLauncherState,
              Object?,
              Object?
            >;
    return element.handleCreate(ref, build);
  }
}
