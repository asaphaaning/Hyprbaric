import 'package:flutter/widgets.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

import '../../bindings/bindings.dart';
import '../rust_commands.dart';

part 'tray_controller.g.dart';

@Riverpod(keepAlive: true)
class TrayController extends _$TrayController {
  @override
  void build() {}

  void activate(String id, Offset position) {
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(
          TrayIntent.activate(
            id: id,
            x: position.dx.round(),
            y: position.dy.round(),
            kind: TrayActivationKind.primary,
          ),
        );
  }

  void openContextMenu(String id, Offset position) {
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(
          TrayIntent.activate(
            id: id,
            x: position.dx.round(),
            y: position.dy.round(),
            kind: TrayActivationKind.contextMenu,
          ),
        );
  }

  void activateMenuItem(String itemId, int menuItemId) {
    ref
        .read(rustCommandDispatcherProvider)
        .dispatch(
          TrayIntent.activateMenuItem(itemId: itemId, menuItemId: menuItemId),
        );
  }
}
