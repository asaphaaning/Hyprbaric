import '../../bindings/bindings.dart';

extension NetworkEntryDisplayState on NetworkEntry {
  bool get isActive => switch (state) {
    NetworkEntryState.available => false,
    NetworkEntryState.active || NetworkEntryState.connecting => true,
  };

  bool get isConnecting => state == NetworkEntryState.connecting;
}
