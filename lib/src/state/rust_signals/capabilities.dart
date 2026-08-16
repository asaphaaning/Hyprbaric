import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';

Stream<CapabilityStatus> _capabilityStatusStream() async* {
  final latest = CapabilityStatus.latestRustSignal;
  if (latest != null) {
    yield latest.message;
  }

  await for (final rustSignal in CapabilityStatus.rustSignalStream) {
    yield rustSignal.message;
  }
}

const CapabilityStatus defaultCapabilityStatus = CapabilityStatus(
  entries: <CapabilityEntry>[],
);

final capabilityStatusProvider = StreamProvider<CapabilityStatus>(
  (Ref ref) => _capabilityStatusStream(),
);

final currentCapabilityStatusProvider = Provider<CapabilityStatus>((Ref ref) {
  return ref
      .watch(capabilityStatusProvider)
      .maybeWhen(
        data: (CapabilityStatus status) => status,
        orElse: () => defaultCapabilityStatus,
      );
});
