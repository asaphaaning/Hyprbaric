import 'package:riverpod_annotation/riverpod_annotation.dart';

import '../../bindings/bindings.dart';
import '../../state/rust_signals/session.dart';
import '../rust_commands.dart';

part 'session_controller.g.dart';

class SessionLauncherState {
  const SessionLauncherState({
    required this.actions,
    required this.selectedIndex,
    required this.confirmChoice,
    required this.closeSerial,
    required this.isOpen,
    this.confirmingAction,
    this.errorMessage,
  });

  factory SessionLauncherState.initial(List<SessionAction> actions) {
    return SessionLauncherState(
      actions: actions,
      selectedIndex: 0,
      confirmChoice: SessionConfirmChoice.confirm,
      closeSerial: 0,
      isOpen: false,
    );
  }

  final List<SessionAction> actions;
  final int selectedIndex;
  final SessionConfirmChoice confirmChoice;
  final SessionAction? confirmingAction;
  final String? errorMessage;
  final int closeSerial;
  final bool isOpen;

  SessionAction? get selectedAction {
    if (actions.isEmpty) {
      return null;
    }
    return actions[selectedIndex.clamp(0, actions.length - 1)];
  }

  bool get canOpen => actions.isNotEmpty;

  SessionLauncherState copyWith({
    List<SessionAction>? actions,
    int? selectedIndex,
    SessionConfirmChoice? confirmChoice,
    Object? confirmingAction = _preserve,
    Object? errorMessage = _preserve,
    int? closeSerial,
    bool? isOpen,
  }) {
    final List<SessionAction> nextActions = actions ?? this.actions;
    final int maxIndex = nextActions.isEmpty ? 0 : nextActions.length - 1;
    final int nextIndex = (selectedIndex ?? this.selectedIndex).clamp(
      0,
      maxIndex,
    );
    return SessionLauncherState(
      actions: nextActions,
      selectedIndex: nextIndex,
      confirmChoice: confirmChoice ?? this.confirmChoice,
      confirmingAction: confirmingAction == _preserve
          ? this.confirmingAction
          : confirmingAction as SessionAction?,
      errorMessage: errorMessage == _preserve
          ? this.errorMessage
          : errorMessage as String?,
      closeSerial: closeSerial ?? this.closeSerial,
      isOpen: isOpen ?? this.isOpen,
    );
  }
}

const Object _preserve = Object();

enum SessionConfirmChoice {
  cancel,
  confirm;

  SessionConfirmChoice move(int delta) {
    final List<SessionConfirmChoice> values = SessionConfirmChoice.values;
    final int next = (index + delta + values.length) % values.length;
    return values[next];
  }
}

@Riverpod(keepAlive: true)
class SessionController extends _$SessionController {
  @override
  SessionLauncherState build() {
    ref.listen<AsyncValue<SessionActionAvailability>>(
      sessionActionAvailabilityProvider,
      (_, AsyncValue<SessionActionAvailability> next) {
        _setAvailableActions(next.asData?.value);
      },
    );
    ref.listen<AsyncValue<SessionCommandResult>>(sessionCommandResultProvider, (
      _,
      AsyncValue<SessionCommandResult> next,
    ) {
      next.whenData(_handleCommandResult);
    });
    return SessionLauncherState.initial(
      _availableSessionActions(
        ref.read(sessionActionAvailabilityProvider).asData?.value,
      ),
    );
  }

  void opened() {
    state = state.copyWith(
      selectedIndex: 0,
      confirmChoice: SessionConfirmChoice.confirm,
      confirmingAction: null,
      errorMessage: null,
      isOpen: true,
    );
  }

  void closed() {
    state = state.copyWith(
      confirmingAction: null,
      confirmChoice: SessionConfirmChoice.confirm,
      errorMessage: null,
      isOpen: false,
    );
  }

  void moveSelection(int delta) {
    if (state.confirmingAction != null) {
      state = state.copyWith(
        confirmChoice: state.confirmChoice.move(delta),
        errorMessage: null,
      );
      return;
    }
    if (state.actions.isEmpty) {
      return;
    }
    state = state.copyWith(
      selectedIndex:
          (state.selectedIndex + delta + state.actions.length) %
          state.actions.length,
      errorMessage: null,
    );
  }

  void select(SessionAction action) {
    final int index = state.actions.indexOf(action);
    if (index < 0) {
      return;
    }
    state = state.copyWith(
      selectedIndex: index,
      confirmingAction: action,
      confirmChoice: SessionConfirmChoice.confirm,
      errorMessage: null,
    );
  }

  void activateSelection() {
    if (state.actions.isEmpty) {
      return;
    }
    if (state.confirmingAction case final SessionAction action) {
      switch (state.confirmChoice) {
        case SessionConfirmChoice.cancel:
          cancelConfirmation();
        case SessionConfirmChoice.confirm:
          execute(action);
      }
      return;
    }
    state = state.copyWith(
      confirmingAction: state.selectedAction,
      confirmChoice: SessionConfirmChoice.confirm,
      errorMessage: null,
    );
  }

  bool cancelConfirmation() {
    if (state.confirmingAction == null) {
      return false;
    }
    state = state.copyWith(
      confirmingAction: null,
      confirmChoice: SessionConfirmChoice.confirm,
      errorMessage: null,
    );
    return true;
  }

  void execute(SessionAction action) {
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(SessionIntent.execute(action));
  }

  void _setAvailableActions(SessionActionAvailability? availability) {
    final List<SessionAction> actions = _availableSessionActions(availability);
    final SessionAction? confirmingAction = state.confirmingAction;
    final bool keepConfirming =
        confirmingAction != null && actions.contains(confirmingAction);
    state = state.copyWith(
      actions: actions,
      confirmingAction: keepConfirming ? confirmingAction : null,
      confirmChoice: keepConfirming
          ? state.confirmChoice
          : SessionConfirmChoice.confirm,
      closeSerial: actions.isEmpty && state.isOpen
          ? state.closeSerial + 1
          : state.closeSerial,
    );
  }

  void _handleCommandResult(SessionCommandResult result) {
    if (!state.isOpen) {
      return;
    }
    switch (result.outcome) {
      case SessionCommandOutcome.started:
        state = state.copyWith(
          confirmingAction: null,
          confirmChoice: SessionConfirmChoice.confirm,
          errorMessage: null,
          isOpen: false,
          closeSerial: state.closeSerial + 1,
        );
      case SessionCommandOutcome.failed:
        final int index = state.actions.indexOf(result.action);
        state = state.copyWith(
          selectedIndex: index < 0 ? state.selectedIndex : index,
          confirmingAction: null,
          confirmChoice: SessionConfirmChoice.confirm,
          errorMessage:
              result.message ?? 'Unable to complete ${result.action.label}.',
        );
    }
  }
}

List<SessionAction> _availableSessionActions(
  SessionActionAvailability? availability,
) {
  final Set<SessionAction> available = <SessionAction>{
    SessionAction.lock,
    SessionAction.suspend,
    SessionAction.logout,
    SessionAction.restart,
    SessionAction.shutdown,
    if (availability?.firmwareRebootSupported ?? false)
      SessionAction.rebootToFirmware,
  };
  return <SessionAction>[
    SessionAction.lock,
    SessionAction.logout,
    SessionAction.restart,
    SessionAction.shutdown,
  ].where(available.contains).toList(growable: false);
}

extension SessionActionView on SessionAction {
  String get label => switch (this) {
    SessionAction.lock => 'Lock',
    SessionAction.suspend => 'Suspend',
    SessionAction.logout => 'Logout',
    SessionAction.restart => 'Reboot',
    SessionAction.shutdown => 'Shutdown',
    SessionAction.rebootToFirmware => 'Reboot to Firmware',
  };

  String? get detailLabel => switch (this) {
    SessionAction.lock => 'Super+L',
    SessionAction.logout => 'end session',
    SessionAction.suspend ||
    SessionAction.restart ||
    SessionAction.shutdown ||
    SessionAction.rebootToFirmware => null,
  };

  bool get isDanger => this == SessionAction.shutdown;

  String get confirmationPrompt => switch (this) {
    SessionAction.lock => 'Lock the current session now?',
    SessionAction.suspend => 'Suspend the machine now?',
    SessionAction.logout => 'End the current Hyprland session?',
    SessionAction.restart => 'Reboot the machine now?',
    SessionAction.shutdown => 'Power off the machine now?',
    SessionAction.rebootToFirmware => 'Restart into firmware setup now?',
  };
}
