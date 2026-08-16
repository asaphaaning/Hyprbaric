import 'dart:math' as math;

import 'package:flutter/foundation.dart';

class HyprLiveValue {
  HyprLiveValue({
    required int initialValue,
    this.minimum = 0,
    this.maximum = 100,
    this.commitInterval = Duration.zero,
  }) : assert(
         minimum <= maximum,
         'minimum must be less than or equal to maximum',
       ),
       _value = initialValue.clamp(minimum, maximum),
       _committedValue = initialValue.clamp(minimum, maximum);

  final int minimum;
  final int maximum;
  final Duration commitInterval;
  final Stopwatch _clock = Stopwatch()..start();

  int _value;
  int _committedValue;
  bool _active = false;

  int get value => _value;
  bool get active => _active;

  void sync(int value) {
    if (_active) {
      return;
    }
    final int next = _clamp(value);
    _value = next;
    _committedValue = next;
  }

  int begin([int? value]) {
    _active = true;
    if (value != null) {
      _value = _clamp(value);
    }
    return _value;
  }

  int preview(int value) {
    _value = _clamp(value);
    return _value;
  }

  int? commit({bool force = false}) {
    if (_value == _committedValue) {
      return null;
    }
    if (!force &&
        commitInterval > Duration.zero &&
        _clock.elapsed < commitInterval) {
      return null;
    }
    _committedValue = _value;
    _clock.reset();
    return _value;
  }

  int end() {
    _active = false;
    return _value;
  }

  void cancel() {
    _active = false;
    _value = _committedValue;
  }

  int _clamp(int value) => math.min(maximum, math.max(minimum, value));

  @visibleForTesting
  int get committedValue => _committedValue;
}
