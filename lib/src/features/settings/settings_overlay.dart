import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../layer_shell_controller.dart';
import '../../layer_shell_hit_region.dart';
import '../../state/providers.dart';
import '../../widgets/hypr_surface.dart';
import 'settings_overlay_content.dart';
import 'settings_overlay_layout.dart';
import 'settings_tabs.dart';

class SettingsModalOverlay extends ConsumerStatefulWidget {
  const SettingsModalOverlay({super.key, required this.onClose});

  final VoidCallback onClose;

  @override
  ConsumerState<SettingsModalOverlay> createState() =>
      _SettingsModalOverlayState();
}

class _SettingsModalOverlayState extends ConsumerState<SettingsModalOverlay> {
  static const String _regionOwner = 'settings-modal';
  final FocusNode _focusNode = FocusNode(debugLabel: 'settings-modal');
  late final LayerShellRegionManager _regionManager;
  SettingsTab _tab = SettingsTab.appearance;

  @override
  void initState() {
    super.initState();
    _regionManager = ref.read(layerShellRegionManagerProvider);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) {
        _claimKeyboard();
      }
    });
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _scheduleRegionUpdate();
  }

  @override
  void dispose() {
    unawaited(_setKeyboardMode(LayerShellKeyboardMode.none));
    _focusNode.dispose();
    unawaited(
      _regionManager.removePassiveRegions(
        owner: _regionOwner,
        debugLabel: 'settings-modal-close',
      ),
    );
    super.dispose();
  }

  void _claimKeyboard() {
    _focusNode.requestFocus();
    unawaited(_setKeyboardMode(LayerShellKeyboardMode.exclusive));
  }

  Future<void> _setKeyboardMode(LayerShellKeyboardMode mode) async {
    await LayerShellController.setKeyboardMode(mode);
  }

  void _scheduleRegionUpdate() {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) {
        return;
      }
      final Size size = MediaQuery.sizeOf(context);
      unawaited(
        _regionManager.setPassiveRegions(
          owner: _regionOwner,
          regions: <LayerShellMenuRegion>[
            LayerShellMenuRegion(
              rect: Offset.zero & size,
              radius: BorderRadius.zero,
            ),
          ],
          debugLabel: 'settings-modal-open',
        ),
      );
    });
  }

  @override
  Widget build(BuildContext context) {
    return Positioned.fill(
      child: Focus(
        autofocus: true,
        focusNode: _focusNode,
        child: AnimatedOpacity(
          opacity: 1,
          duration: HyprMotion.toast,
          curve: HyprMotion.toastCurve,
          child: GestureDetector(
            behavior: HitTestBehavior.opaque,
            onTap: widget.onClose,
            child: ColoredBox(
              color: Colors.black.withValues(alpha: 0.34),
              child: Center(
                child: GestureDetector(
                  onTap: _claimKeyboard,
                  child: HyprSurface(
                    borderRadius: BorderRadius.circular(
                      SettingsOverlayLayout.surfaceRadius,
                    ),
                    color: HyprColors.popoverSurface,
                    borderColor: HyprColors.popupStroke,
                    frame: HyprSurfaceFrame.popover,
                    child: SettingsOverlayContent(
                      tab: _tab,
                      onTabChanged: (SettingsTab tab) {
                        setState(() => _tab = tab);
                      },
                      onClose: widget.onClose,
                    ),
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
