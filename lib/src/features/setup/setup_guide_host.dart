import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';
import '../../state/providers.dart';
import '../rust_commands.dart';
import 'setup_guide_overlay.dart';
import 'setup_guide_state.dart';

/// Owns the setup journey for one Flutter view.
class SetupGuideHost extends ConsumerStatefulWidget {
  const SetupGuideHost({super.key});

  @override
  ConsumerState<SetupGuideHost> createState() => _SetupGuideHostState();
}

class _SetupGuideHostState extends ConsumerState<SetupGuideHost> {
  SetupLaunch? _launch;
  late final ProviderSubscription<AsyncValue<SetupStatus>> _statusSubscription;
  late final ProviderSubscription<SetupGuideRequest?> _requestSubscription;

  @override
  void initState() {
    super.initState();
    _statusSubscription = ref.listenManual<AsyncValue<SetupStatus>>(
      setupStatusProvider,
      (_, AsyncValue<SetupStatus> next) {
        next.whenData(_acceptStatus);
      },
      fireImmediately: true,
    );
    _requestSubscription = ref.listenManual<SetupGuideRequest?>(
      setupGuideRequestProvider,
      (_, SetupGuideRequest? request) {
        if (request == SetupGuideRequest.show) {
          _open(SetupLaunch.manual);
          scheduleMicrotask(
            () =>
                ref.read(setupGuideRequestProvider.notifier).consume(request!),
          );
        }
      },
    );
  }

  @override
  void dispose() {
    _statusSubscription.close();
    _requestSubscription.close();
    super.dispose();
  }

  void _acceptStatus(SetupStatus status) {
    if (status.state == SetupState.required &&
        ref.read(setupGuideAutomaticHostProvider)) {
      _open(SetupLaunch.automatic);
    }
  }

  void _open(SetupLaunch launch) {
    if (mounted && _launch == null) {
      setState(() => _launch = launch);
    }
  }

  void _complete(SetupOutcome outcome) {
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(SetupIntent.complete(outcome));
    setState(() => _launch = null);
  }

  @override
  Widget build(BuildContext context) {
    if (_launch == null) {
      return const SizedBox.shrink();
    }

    return SetupGuideOverlay(
      launch: _launch!,
      onFinished: () => _complete(SetupOutcome.finished),
      onSkipped: () => _complete(SetupOutcome.skipped),
    );
  }
}
