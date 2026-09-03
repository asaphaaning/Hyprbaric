import 'package:flutter/foundation.dart';

import 'native/layer_shell_api.g.dart';

enum LayerShellLayer {
  background(NativeLayerShellLayer.background),
  bottom(NativeLayerShellLayer.bottom),
  top(NativeLayerShellLayer.top),
  overlay(NativeLayerShellLayer.overlay);

  const LayerShellLayer(this.native);
  final NativeLayerShellLayer native;
}

enum LayerShellKeyboardMode {
  none(NativeLayerShellKeyboardMode.none),
  exclusive(NativeLayerShellKeyboardMode.exclusive),
  onDemand(NativeLayerShellKeyboardMode.onDemand);

  const LayerShellKeyboardMode(this.native);
  final NativeLayerShellKeyboardMode native;
}

class LayerShellController {
  LayerShellController._();

  static final NativeLayerShellHostApi _api = NativeLayerShellHostApi();

  static bool get _isSupported =>
      !kIsWeb && defaultTargetPlatform == TargetPlatform.linux;

  static Future<void> setLayer(LayerShellLayer layer) =>
      _invoke('setLayer', () => _api.setLayer(layer.native));

  static Future<void> setNamespace(String namespace) =>
      _invoke('setNamespace', () => _api.setNamespace(namespace));

  static Future<void> setAnchors({
    bool? top,
    bool? bottom,
    bool? left,
    bool? right,
  }) {
    if (top == null && bottom == null && left == null && right == null) {
      return Future<void>.value();
    }
    return _invoke(
      'setAnchors',
      () => _api.setAnchors(
        NativeLayerShellAnchors(
          top: top,
          bottom: bottom,
          left: left,
          right: right,
        ),
      ),
    );
  }

  static Future<void> setMargins({
    int? top,
    int? bottom,
    int? left,
    int? right,
  }) {
    if (top == null && bottom == null && left == null && right == null) {
      return Future<void>.value();
    }
    return _invoke(
      'setMargins',
      () => _api.setMargins(
        NativeLayerShellMargins(
          top: top,
          bottom: bottom,
          left: left,
          right: right,
        ),
      ),
    );
  }

  static Future<void> setExclusiveZone(int zone) =>
      _invoke('setExclusiveZone', () => _api.setExclusiveZone(zone));

  static Future<void> setAutoExclusiveZone({required bool enabled}) =>
      _invoke('setAutoExclusiveZone', () => _api.setAutoExclusiveZone(enabled));

  static Future<void> setKeyboardMode(LayerShellKeyboardMode mode) =>
      _invoke('setKeyboardMode', () => _api.setKeyboardMode(mode.native));

  /// Compositor keyboard claims by owner.
  ///
  /// Several overlays (settings, setup guide, launchers, network password)
  /// need exclusive keyboard while open. Each one claims on open and releases
  /// on close; the mode stays exclusive until the last claim is released, so
  /// overlapping overlays cannot strand each other without keyboard access.
  static final Set<String> _keyboardOwners = <String>{};

  static Future<void> claimKeyboard(String owner) {
    _keyboardOwners.add(owner);
    return setKeyboardMode(LayerShellKeyboardMode.exclusive);
  }

  static Future<void> releaseKeyboard(String owner) {
    _keyboardOwners.remove(owner);
    if (_keyboardOwners.isNotEmpty) {
      return Future<void>.value();
    }
    return setKeyboardMode(LayerShellKeyboardMode.none);
  }

  @visibleForTesting
  static void debugResetKeyboardOwners() => _keyboardOwners.clear();

  static Future<void> setSize({int? width, int? height}) {
    if (width == null && height == null) return Future<void>.value();
    return _invoke(
      'setSize',
      () => _api.setSize(NativeLayerShellSize(width: width, height: height)),
    );
  }

  static Future<void> configurePanelDefaults({
    String namespace = 'hyprbaric',
    LayerShellLayer layer = LayerShellLayer.top,
    LayerShellKeyboardMode keyboardMode = LayerShellKeyboardMode.none,
    bool anchorTop = true,
    bool anchorBottom = false,
    bool anchorLeft = true,
    bool anchorRight = true,
    int? height,
    int exclusiveZone = 0,
    bool autoExclusiveZone = false,
    int? marginTop,
    int? marginBottom,
    int? marginLeft,
    int? marginRight,
  }) async {
    if (!_isSupported) {
      return;
    }

    await _invoke(
      'configurePanel',
      () => _api.configurePanel(
        NativeLayerShellPanelConfig(
          appNamespace: namespace,
          layer: layer.native,
          anchors: NativeLayerShellAnchors(
            top: anchorTop,
            bottom: anchorBottom,
            left: anchorLeft,
            right: anchorRight,
          ),
          margins: NativeLayerShellMargins(
            top: marginTop,
            bottom: marginBottom,
            left: marginLeft,
            right: marginRight,
          ),
          size: NativeLayerShellSize(height: height),
          exclusiveZone: exclusiveZone,
          autoExclusiveZone: autoExclusiveZone,
          keyboardMode: keyboardMode.native,
        ),
      ),
    );
  }

  static Future<void> _invoke(String method, Future<void> Function() action) {
    if (!_isSupported) return Future<void>.value();
    return action().catchError((Object error, StackTrace stackTrace) {
      debugPrint('Layer shell call $method failed: $error');
      debugPrint('$stackTrace');
    });
  }
}
