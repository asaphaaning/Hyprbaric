import 'package:riverpod_annotation/riverpod_annotation.dart';

import '../../bindings/bindings.dart';
import '../rust_commands.dart';

part 'network_controller.g.dart';

@Riverpod(keepAlive: true)
class NetworkController extends _$NetworkController {
  @override
  void build() {}

  void scan() {
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(const NetworkIntent.scan());
  }

  void setWifiEnabled({required bool enabled}) {
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(NetworkIntent.setWifiEnabled(enabled: enabled));
  }

  void connect(NetworkEntry entry, String? password) {
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(
          NetworkIntent.connect(
            ssid: entry.ssid,
            bssid: entry.bssid,
            password: password,
          ),
        );
  }

  void openSettings() {
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(const NetworkIntent.openSettings());
  }
}
