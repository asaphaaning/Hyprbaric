import 'package:riverpod_annotation/riverpod_annotation.dart';

import '../../bindings/bindings.dart';
import '../../state/rust_signals/launcher.dart';
import '../rust_commands.dart';

part 'launcher_controller.g.dart';

class LauncherViewState {
  const LauncherViewState({
    required this.selectedIndex,
    required this.closeSerial,
    required this.iconPathsByEntryId,
    this.errorMessage,
    this.lastLaunchId,
  });

  const LauncherViewState.initial()
    : selectedIndex = 0,
      closeSerial = 0,
      iconPathsByEntryId = const <String, String>{},
      errorMessage = null,
      lastLaunchId = null;

  final int selectedIndex;
  final int closeSerial;
  final Map<String, String> iconPathsByEntryId;
  final String? errorMessage;
  final String? lastLaunchId;

  LauncherViewState copyWith({
    int? selectedIndex,
    int? closeSerial,
    Map<String, String>? iconPathsByEntryId,
    Object? errorMessage = _preserve,
    Object? lastLaunchId = _preserve,
  }) {
    return LauncherViewState(
      selectedIndex: selectedIndex ?? this.selectedIndex,
      closeSerial: closeSerial ?? this.closeSerial,
      iconPathsByEntryId: iconPathsByEntryId ?? this.iconPathsByEntryId,
      errorMessage: errorMessage == _preserve
          ? this.errorMessage
          : errorMessage as String?,
      lastLaunchId: lastLaunchId == _preserve
          ? this.lastLaunchId
          : lastLaunchId as String?,
    );
  }
}

const Object _preserve = Object();

@Riverpod(keepAlive: true)
class LauncherController extends _$LauncherController {
  @override
  LauncherViewState build() {
    ref.listen<AsyncValue<AppLauncherResults>>(appLauncherResultsProvider, (
      _,
      AsyncValue<AppLauncherResults> next,
    ) {
      next.whenData(rememberResults);
    });
    ref.listen<AsyncValue<AppLaunchResult>>(appLaunchResultProvider, (
      _,
      AsyncValue<AppLaunchResult> next,
    ) {
      next.whenData(_handleLaunchResult);
    });
    final AppLauncherResults? results = ref
        .read(appLauncherResultsProvider)
        .asData
        ?.value;
    return const LauncherViewState.initial().copyWith(
      iconPathsByEntryId: _iconPathsFrom(results),
    );
  }

  void opened() {
    state = state.copyWith(
      selectedIndex: 0,
      errorMessage: null,
      lastLaunchId: null,
    );
  }

  void closed() {
    state = state.copyWith(
      selectedIndex: 0,
      errorMessage: null,
      lastLaunchId: null,
    );
  }

  void updateQuery(String query) {
    state = state.copyWith(selectedIndex: 0, errorMessage: null);
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(LauncherIntent.query(query));
  }

  void rememberResults(AppLauncherResults results) {
    state = state.copyWith(
      iconPathsByEntryId: _mergeIconPaths(state.iconPathsByEntryId, results),
      selectedIndex: _clampIndex(state.selectedIndex, results.entries.length),
    );
  }

  void select(int index, int entryCount) {
    if (entryCount <= 0) {
      return;
    }
    state = state.copyWith(
      selectedIndex: index.clamp(0, entryCount - 1),
      errorMessage: null,
    );
  }

  void moveSelection(int delta, int entryCount) {
    if (entryCount <= 0) {
      return;
    }
    state = state.copyWith(
      selectedIndex: (state.selectedIndex + delta + entryCount) % entryCount,
      errorMessage: null,
    );
  }

  void launchSelected(List<AppLauncherEntry> entries) {
    if (entries.isEmpty) {
      return;
    }
    final int selectedIndex = _clampIndex(state.selectedIndex, entries.length);
    launch(entries[selectedIndex]);
  }

  void launch(AppLauncherEntry entry) {
    state = state.copyWith(lastLaunchId: entry.id, errorMessage: null);
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(LauncherIntent.launch(entry.id));
  }

  void _handleLaunchResult(AppLaunchResult result) {
    final String? lastLaunchId = state.lastLaunchId;
    if (lastLaunchId != null && result.id != lastLaunchId) {
      return;
    }

    switch (result.outcome) {
      case AppLaunchOutcome.started:
        state = state.copyWith(
          errorMessage: null,
          lastLaunchId: null,
          closeSerial: state.closeSerial + 1,
        );
      case AppLaunchOutcome.failed:
        state = state.copyWith(
          errorMessage: result.message ?? 'Unable to launch ${result.id}.',
        );
    }
  }
}

int _clampIndex(int index, int entryCount) {
  if (entryCount <= 0) {
    return 0;
  }
  return index.clamp(0, entryCount - 1);
}

Map<String, String> _iconPathsFrom(AppLauncherResults? results) {
  if (results == null) {
    return const <String, String>{};
  }
  return _mergeIconPaths(const <String, String>{}, results);
}

Map<String, String> _mergeIconPaths(
  Map<String, String> current,
  AppLauncherResults results,
) {
  Map<String, String>? next;
  for (final AppLauncherEntry entry in results.entries) {
    final String? iconPath = entry.iconPath;
    if (iconPath == null) {
      continue;
    }
    next ??= Map<String, String>.of(current);
    next[entry.id] = iconPath;
  }
  return next ?? current;
}
