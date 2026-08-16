import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../bindings/bindings.dart';
import 'rust_signals.dart';

class FocusedWindowDisplay {
  const FocusedWindowDisplay({
    required this.title,
    this.appName,
    this.isFallback = false,
    this.isHidden = false,
  });

  const FocusedWindowDisplay.hidden()
    : appName = null,
      title = '',
      isFallback = true,
      isHidden = true;

  final String? appName;
  final String title;
  final bool isFallback;
  final bool isHidden;

  bool get hasAppTitleSplit {
    if (isHidden) {
      return false;
    }
    final String? app = appName;
    return app != null && app.toLowerCase() != title.toLowerCase();
  }

  String get tooltip {
    if (isHidden) {
      return '';
    }
    final String? app = appName;
    if (app == null || !hasAppTitleSplit) {
      return title;
    }
    return '$app - $title';
  }
}

String? _trimmed(String? value) {
  final String? trimmed = value?.trim();
  if (trimmed == null || trimmed.isEmpty) {
    return null;
  }
  return trimmed;
}

String _displayTitle(FocusedWindowStatus status) {
  return _display(status).title;
}

FocusedWindowDisplay _display(FocusedWindowStatus status) {
  final String? appName = _displayAppName(status.appName);
  final String? title = _trimmed(status.title);
  final String hostname = _trimmed(status.hostname) ?? 'Hyprbaric';
  if (_isEmptyDesktopStatus(status)) {
    return const FocusedWindowDisplay.hidden();
  }
  if (title == null) {
    if (appName != null) {
      return FocusedWindowDisplay(appName: appName, title: appName);
    }
    return FocusedWindowDisplay(title: hostname, isFallback: true);
  }
  return FocusedWindowDisplay(appName: appName, title: title);
}

bool _isEmptyDesktopStatus(FocusedWindowStatus status) {
  final String? title = _trimmed(status.title);
  final String? appName = _trimmed(status.appName);
  if (_isDesktopToken(title)) {
    return true;
  }
  if (title != null) {
    return false;
  }
  return _isDesktopToken(appName) ||
      (appName == null && _isDesktopToken(status.hostname));
}

bool _isDesktopToken(String? value) {
  final String? trimmed = _trimmed(value);
  if (trimmed == null) {
    return false;
  }
  final String normalized = trimmed
      .replaceFirst(RegExp(r'\.desktop$', caseSensitive: false), '')
      .toLowerCase();
  return normalized == 'desktop';
}

String? _displayAppName(String? value) {
  final String? trimmed = _trimmed(value);
  if (trimmed == null) {
    return null;
  }

  final String withoutDesktopSuffix = trimmed.replaceFirst(
    RegExp(r'\.desktop$', caseSensitive: false),
    '',
  );
  final String candidate = _reverseDnsTail(withoutDesktopSuffix);
  final List<String> words = candidate
      .split(RegExp(r'[\s._-]+'))
      .where((String word) => word.isNotEmpty)
      .map(_titleCaseWord)
      .toList(growable: false);
  if (words.isEmpty) {
    return null;
  }
  return words.join(' ');
}

String _reverseDnsTail(String value) {
  final List<String> parts = value
      .split('.')
      .where((String part) => part.isNotEmpty)
      .toList(growable: false);
  if (parts.length < 3) {
    return value;
  }
  final String prefix = parts.first.toLowerCase();
  if (prefix == 'app' ||
      prefix == 'com' ||
      prefix == 'dev' ||
      prefix == 'io' ||
      prefix == 'net' ||
      prefix == 'org') {
    return parts.last;
  }
  return value;
}

String _titleCaseWord(String word) {
  if (word.length <= 4 && word == word.toUpperCase()) {
    return word;
  }
  final String lower = word.toLowerCase();
  return lower[0].toUpperCase() + lower.substring(1);
}

/// Raw focused-window updates emitted from Rust via RINF.
final currentFocusedWindowStatusProvider = focusedWindowStatusProvider;

/// Human-friendly center label for the active window slot.
final currentWindowTitleProvider = Provider<String>((ref) {
  final AsyncValue<FocusedWindowStatus> status = ref.watch(
    currentFocusedWindowStatusProvider,
  );
  return status.maybeWhen(data: _displayTitle, orElse: () => 'Hyprbaric');
});

/// Structured center slot text for the app/title split treatment.
final currentWindowDisplayProvider = Provider<FocusedWindowDisplay>((ref) {
  final AsyncValue<FocusedWindowStatus> status = ref.watch(
    currentFocusedWindowStatusProvider,
  );
  return status.maybeWhen(
    data: _display,
    orElse: () =>
        const FocusedWindowDisplay(title: 'Hyprbaric', isFallback: true),
  );
});
