import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<WorkspaceSettingsStatus> _workspaceSettingsStatusStream() async* {
  final latest = WorkspaceSettingsStatus.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in WorkspaceSettingsStatus.rustSignalStream) {
    yield rustSignal.message;
  }
}

Stream<WorkspaceSettingsCommandResult>
_workspaceSettingsCommandResultStream() async* {
  final latest = WorkspaceSettingsCommandResult.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal
      in WorkspaceSettingsCommandResult.rustSignalStream) {
    yield rustSignal.message;
  }
}

const WorkspaceSettingsStatus defaultWorkspaceSettingsStatus =
    WorkspaceSettingsStatus(
      indicatorStyle: WorkspaceIndicatorStyle.roman,
      clickable: true,
      visibleRange: WorkspaceVisibleRange.medium,
      visibleCount: 7,
    );

final workspaceSettingsStatusProvider = StreamProvider<WorkspaceSettingsStatus>(
  (ref) => _workspaceSettingsStatusStream(),
);

final workspaceSettingsCommandResultProvider =
    StreamProvider<WorkspaceSettingsCommandResult>(
      (ref) => _workspaceSettingsCommandResultStream(),
    );

final currentWorkspaceSettingsProvider = Provider<WorkspaceSettingsStatus>((
  ref,
) {
  return ref
      .watch(workspaceSettingsStatusProvider)
      .maybeWhen(
        data: (WorkspaceSettingsStatus status) => status,
        orElse: () => defaultWorkspaceSettingsStatus,
      );
});

extension WorkspaceVisibleRangeView on WorkspaceVisibleRange {
  String get label => switch (this) {
    WorkspaceVisibleRange.small => 'Small',
    WorkspaceVisibleRange.medium => 'Medium',
    WorkspaceVisibleRange.large => 'Large',
  };

  int get count => switch (this) {
    WorkspaceVisibleRange.small => 5,
    WorkspaceVisibleRange.medium => 7,
    WorkspaceVisibleRange.large => 9,
  };
}

extension WorkspaceIndicatorStyleView on WorkspaceIndicatorStyle {
  String get label => switch (this) {
    WorkspaceIndicatorStyle.roman => 'Roman',
    WorkspaceIndicatorStyle.numeric => 'Numeric',
  };
}
