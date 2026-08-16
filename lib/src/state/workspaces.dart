import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../bindings/bindings.dart';
import 'rust_signals.dart';

String _formatWorkspace(WorkspaceStatus status) {
  if (status.isSpecial) {
    return 'Special (${status.name})';
  }
  return 'Workspace ${status.id}';
}

/// Raw workspace updates emitted from Rust via RINF.
final currentWorkspaceStatusProvider = workspaceStatusProvider;

/// Human-friendly label for the active workspace.
final currentWorkspaceLabelProvider = Provider<String>((ref) {
  final AsyncValue<WorkspaceStatus> status = ref.watch(
    currentWorkspaceStatusProvider,
  );
  return status.maybeWhen(data: _formatWorkspace, orElse: () => 'Workspace —');
});
