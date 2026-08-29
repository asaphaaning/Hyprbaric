import 'package:flutter_riverpod/flutter_riverpod.dart';

/// Ordered steps in the first-run journey.
enum SetupStep {
  welcome,
  transparency,
  accent,
  layout;

  static const List<SetupStep> sequence = <SetupStep>[
    welcome,
    transparency,
    accent,
    layout,
  ];

  String get label => switch (this) {
    SetupStep.welcome => 'Welcome',
    SetupStep.transparency => 'Transparency',
    SetupStep.accent => 'Accent',
    SetupStep.layout => 'Layout',
  };
}

/// Source that opened the setup journey.
enum SetupLaunch { automatic, manual }

/// An intentional request to show the guide from application UI.
enum SetupGuideRequest { show }

/// Only the first native view hosts automatic onboarding.
final setupGuideAutomaticHostProvider = Provider<bool>((_) => true);

final setupGuideRequestProvider =
    NotifierProvider<SetupGuideRequestController, SetupGuideRequest?>(
      SetupGuideRequestController.new,
    );

class SetupGuideRequestController extends Notifier<SetupGuideRequest?> {
  @override
  SetupGuideRequest? build() => null;

  /// Opens the setup guide regardless of persisted startup state.
  void show() {
    state = SetupGuideRequest.show;
  }

  /// Consumes a request after the local view has opened the guide.
  void consume(SetupGuideRequest request) {
    if (state == request) {
      state = null;
    }
  }
}
