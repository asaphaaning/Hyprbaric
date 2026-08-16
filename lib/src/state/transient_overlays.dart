import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../bindings/bindings.dart';
import '../theme/hypr_tokens.dart';

const Duration toastLifetime = HyprDurations.toastLifetime;
const Duration osdLifetime = HyprDurations.osdLifetime;

@immutable
class ToastEntry {
  const ToastEntry({
    required this.id,
    required this.app,
    required this.message,
    required this.initials,
    required this.color,
    required this.urgency,
  });

  final int id;
  final String app;
  final String message;
  final String initials;
  final Color color;
  final NotificationUrgency urgency;
}

@immutable
class OsdEvent {
  const OsdEvent({
    required this.id,
    required this.kind,
    required this.label,
    required this.value,
    required this.muted,
  });

  final int id;
  final OsdKind kind;
  final String label;
  final int value;
  final bool muted;
}

enum OsdKind { volume, brightness }

@immutable
class TransientOverlayState {
  const TransientOverlayState({this.toasts = const <ToastEntry>[], this.osd});

  final List<ToastEntry> toasts;
  final OsdEvent? osd;

  TransientOverlayState copyWith({List<ToastEntry>? toasts, OsdEvent? osd}) {
    return TransientOverlayState(
      toasts: toasts ?? this.toasts,
      osd: osd ?? this.osd,
    );
  }

  TransientOverlayState withoutOsd() {
    return TransientOverlayState(toasts: toasts);
  }
}

class TransientOverlayNotifier extends Notifier<TransientOverlayState> {
  final Set<int> _seenNotificationIds = <int>{};
  final Map<int, Timer> _toastTimers = <int, Timer>{};
  bool _notificationsPrimed = false;
  Timer? _osdTimer;
  int _osdSequence = 0;
  int _localToastSequence = -1;

  @override
  TransientOverlayState build() {
    ref.onDispose(_disposeTimers);
    return const TransientOverlayState();
  }

  void reconcileNotifications(NotificationStatus? status) {
    if (status == null) {
      return;
    }

    final Set<int> currentIds = status.entries
        .map((NotificationEntry entry) => entry.id)
        .toSet();
    _removeClosedNotificationToasts(currentIds);

    if (status.dndEnabled) {
      _seenNotificationIds.addAll(currentIds);
      _notificationsPrimed = true;
      return;
    }

    if (!_notificationsPrimed) {
      _seenNotificationIds.addAll(currentIds);
      _notificationsPrimed = true;
      return;
    }

    for (final NotificationEntry entry in status.entries.reversed) {
      if (_seenNotificationIds.add(entry.id)) {
        _showToast(_toastFromNotification(entry));
      }
    }
  }

  void showVolumeOsd({required int value, required bool muted}) {
    _showOsd(kind: OsdKind.volume, label: 'Volume', value: value, muted: muted);
  }

  void showBrightnessOsd({required int value}) {
    _showOsd(
      kind: OsdKind.brightness,
      label: 'Brightness',
      value: value,
      muted: false,
    );
  }

  void _showOsd({
    required OsdKind kind,
    required String label,
    required int value,
    required bool muted,
  }) {
    final int clamped = value.clamp(0, 100).toInt();
    final int id = state.osd?.id ?? _osdSequence++;
    final OsdEvent event = OsdEvent(
      id: id,
      kind: kind,
      label: label,
      value: clamped,
      muted: muted,
    );
    state = state.copyWith(osd: event);
    _osdTimer?.cancel();
    _osdTimer = Timer(osdLifetime, () {
      if (state.osd?.id == event.id) {
        state = state.withoutOsd();
      }
    });
  }

  void dismissToast(int id) {
    _toastTimers.remove(id)?.cancel();
    state = state.copyWith(
      toasts: state.toasts
          .where((ToastEntry entry) => entry.id != id)
          .toList(growable: false),
    );
  }

  void clearToasts() {
    for (final Timer timer in _toastTimers.values) {
      timer.cancel();
    }
    _toastTimers.clear();
    state = state.copyWith(toasts: const <ToastEntry>[]);
  }

  void showLocalToast({
    required String app,
    required String message,
    NotificationUrgency urgency = NotificationUrgency.normal,
  }) {
    _showToast(
      ToastEntry(
        id: _localToastSequence--,
        app: app,
        message: message,
        initials: _initials(app),
        color: _badgeColor(app, urgency),
        urgency: urgency,
      ),
    );
  }

  void _showToast(ToastEntry entry) {
    _toastTimers.remove(entry.id)?.cancel();
    final List<ToastEntry> next = <ToastEntry>[
      ...state.toasts.where((ToastEntry item) => item.id != entry.id),
      entry,
    ];
    state = state.copyWith(
      toasts: next.length <= 3 ? next : next.sublist(next.length - 3),
    );
    _toastTimers[entry.id] = Timer(toastLifetime, () => dismissToast(entry.id));
  }

  void _removeClosedNotificationToasts(Set<int> currentIds) {
    final List<ToastEntry> next = state.toasts
        .where((ToastEntry entry) => currentIds.contains(entry.id))
        .toList(growable: false);
    if (next.length == state.toasts.length) {
      return;
    }
    for (final ToastEntry entry in state.toasts) {
      if (!currentIds.contains(entry.id)) {
        _toastTimers.remove(entry.id)?.cancel();
      }
    }
    state = state.copyWith(toasts: next);
  }

  void _disposeTimers() {
    for (final Timer timer in _toastTimers.values) {
      timer.cancel();
    }
    _toastTimers.clear();
    _osdTimer?.cancel();
    _osdTimer = null;
  }
}

ToastEntry _toastFromNotification(NotificationEntry entry) {
  return ToastEntry(
    id: entry.id,
    app: entry.app,
    message: entry.message,
    initials: _initials(entry.app),
    color: _badgeColor(entry.app, entry.urgency),
    urgency: entry.urgency,
  );
}

String _initials(String value) {
  final List<String> words = value
      .trim()
      .split(RegExp(r'\s+'))
      .where((String word) => word.isNotEmpty)
      .toList(growable: false);
  if (words.isEmpty) {
    return '?';
  }
  if (words.length == 1) {
    return words.single.characters.take(2).toString().toUpperCase();
  }
  return words
      .take(2)
      .map((String word) => word.characters.first.toUpperCase())
      .join();
}

Color _badgeColor(String app, NotificationUrgency urgency) {
  if (urgency == NotificationUrgency.critical) {
    return const Color(0xFFE16658);
  }
  const List<Color> palette = <Color>[
    Color(0xFFB45AE0),
    Color(0xFF1FA6C9),
    Color(0xFF5C8DF6),
    Color(0xFF5CA774),
    Color(0xFFC89142),
  ];
  final int index = app.codeUnits.fold<int>(
    0,
    (int sum, int unit) => sum + unit,
  );
  return palette[index % palette.length];
}

final transientOverlayProvider =
    NotifierProvider<TransientOverlayNotifier, TransientOverlayState>(
      TransientOverlayNotifier.new,
    );
