import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../layer_shell_hit_region.dart';
import '../state/providers.dart';
import '../theme/hypr_motion.dart';
import 'osd_overlay.dart';
import 'toast_overlay.dart';

class ToastHost extends ConsumerStatefulWidget {
  const ToastHost({
    super.key,
    required this.barHeight,
    required this.onToastPressed,
  });

  final double barHeight;
  final ValueChanged<ToastEntry> onToastPressed;

  @override
  ConsumerState<ToastHost> createState() => _ToastHostState();
}

class _ToastHostState extends ConsumerState<ToastHost> {
  static const String _regionOwner = 'toast-host';
  final Map<int, GlobalKey> _keys = <int, GlobalKey>{};
  late final LayerShellRegionManager _regionManager;
  bool _syncScheduled = false;

  @override
  void initState() {
    super.initState();
    _regionManager = ref.read(layerShellRegionManagerProvider);
  }

  @override
  void dispose() {
    unawaited(
      _regionManager.removePassiveRegions(
        owner: _regionOwner,
        debugLabel: 'toasts',
      ),
    );
    super.dispose();
  }

  void _scheduleRegionSync(List<ToastEntry> toasts) {
    if (_syncScheduled) {
      return;
    }
    _syncScheduled = true;
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _syncScheduled = false;
      if (!mounted) {
        return;
      }
      final List<LayerShellMenuRegion> regions = <LayerShellMenuRegion>[];
      for (final ToastEntry toast in toasts) {
        final BuildContext? toastContext = _keys[toast.id]?.currentContext;
        final RenderBox? box = toastContext?.findRenderObject() as RenderBox?;
        if (box == null || !box.hasSize) {
          continue;
        }
        final Offset topLeft = box.localToGlobal(Offset.zero);
        regions.add(
          LayerShellMenuRegion(
            rect: topLeft & box.size,
            radius: BorderRadius.circular(4),
          ),
        );
      }
      unawaited(
        _regionManager.setPassiveRegions(
          owner: _regionOwner,
          regions: regions,
          debugLabel: 'toasts',
        ),
      );
    });
  }

  @override
  Widget build(BuildContext context) {
    final List<ToastEntry> toasts = ref.watch(
      transientOverlayProvider.select(
        (TransientOverlayState state) => state.toasts,
      ),
    );
    _keys.removeWhere(
      (int id, _) => !toasts.any((ToastEntry toast) => toast.id == id),
    );
    for (final ToastEntry toast in toasts) {
      _keys.putIfAbsent(toast.id, GlobalKey.new);
    }
    _scheduleRegionSync(toasts);

    if (toasts.isEmpty) {
      return const SizedBox.shrink();
    }

    return Positioned(
      top: widget.barHeight + 16,
      left: 0,
      right: 0,
      child: IgnorePointer(
        ignoring: false,
        child: Center(
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: <Widget>[
              for (final ToastEntry toast in toasts) ...<Widget>[
                ToastPill(
                  key: _keys[toast.id],
                  entry: toast,
                  onPressed: () => widget.onToastPressed(toast),
                ),
                const SizedBox(height: 10),
              ],
            ],
          ),
        ),
      ),
    );
  }
}

class OsdHost extends ConsumerWidget {
  const OsdHost({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final OsdEvent? osd = ref.watch(
      transientOverlayProvider.select(
        (TransientOverlayState state) => state.osd,
      ),
    );
    return Positioned(
      left: 0,
      right: 0,
      bottom: 80,
      child: IgnorePointer(
        child: Center(
          child: AnimatedSwitcher(
            duration: HyprMotion.switcher,
            switchInCurve: HyprMotion.switchInCurve,
            switchOutCurve: HyprMotion.switchOutCurve,
            transitionBuilder: (Widget child, Animation<double> animation) {
              return FadeTransition(
                opacity: animation,
                child: SlideTransition(
                  position: Tween<Offset>(
                    begin: const Offset(0, 0.12),
                    end: Offset.zero,
                  ).animate(animation),
                  child: child,
                ),
              );
            },
            child: osd == null
                ? const SizedBox.shrink(key: ValueKey<String>('osd-empty'))
                : OsdPanel(key: ValueKey<int>(osd.id), event: osd),
          ),
        ),
      ),
    );
  }
}
