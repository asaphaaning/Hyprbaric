import 'package:riverpod_annotation/riverpod_annotation.dart';

import '../../bindings/bindings.dart';
import '../../state/providers.dart';
import '../rust_commands.dart';

part 'notification_controller.g.dart';

@Riverpod(keepAlive: true)
class NotificationController extends _$NotificationController {
  @override
  void build() {
    ref.listen<AsyncValue<NotificationStatus>>(notificationStatusProvider, (
      _,
      AsyncValue<NotificationStatus> next,
    ) {
      ref
          .read(transientOverlayProvider.notifier)
          .reconcileNotifications(next.asData?.value);
    });
  }

  void dismiss(int id) {
    ref.read(transientOverlayProvider.notifier).dismissToast(id);
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(NotificationIntent.dismiss(id));
  }

  void clearAll() {
    ref.read(transientOverlayProvider.notifier).clearToasts();
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(const NotificationIntent.clearAll());
  }

  void setDoNotDisturb({required bool enabled}) {
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(NotificationIntent.setDoNotDisturb(enabled: enabled));
  }

  void dismissToast(int id) {
    ref.read(transientOverlayProvider.notifier).dismissToast(id);
  }
}
