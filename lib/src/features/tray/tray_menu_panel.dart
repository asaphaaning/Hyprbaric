import 'package:flutter/material.dart';

import '../../bindings/bindings.dart';
import '../../widgets/hypr_surface.dart';
import '../../widgets/primitives/primitives.dart';

typedef TrayMenuItemActivateCallback =
    void Function(String itemId, int menuItemId);

class TrayMenuPanel extends StatelessWidget {
  const TrayMenuPanel({
    super.key,
    required this.menu,
    required this.borderRadius,
    required this.onActivateItem,
  });

  final TrayMenuStatus? menu;
  final BorderRadius borderRadius;
  final TrayMenuItemActivateCallback onActivateItem;

  @override
  Widget build(BuildContext context) {
    final TrayMenuStatus? status = menu;
    return HyprPopoverPanel(
      borderRadius: borderRadius,
      constraints: const BoxConstraints(minWidth: 220, maxWidth: 300),
      padding: const EdgeInsets.all(6),
      child: status == null || status.items.isEmpty
          ? const SizedBox(
              height: 38,
              child: Center(child: _EmptyTrayMenuLabel()),
            )
          : Column(
              mainAxisSize: MainAxisSize.min,
              children: <Widget>[
                for (final TrayMenuItem item in status.items)
                  _TrayMenuRow(
                    itemId: status.itemId,
                    item: item,
                    onActivateItem: onActivateItem,
                  ),
              ],
            ),
    );
  }
}

class _TrayMenuRow extends StatelessWidget {
  const _TrayMenuRow({
    required this.itemId,
    required this.item,
    required this.onActivateItem,
  });

  final String itemId;
  final TrayMenuItem item;
  final TrayMenuItemActivateCallback onActivateItem;

  @override
  Widget build(BuildContext context) {
    switch (item.kind) {
      case TrayMenuItemKind.separator:
        return Padding(
          padding: const EdgeInsets.symmetric(vertical: 4),
          child: DecoratedBox(
            decoration: BoxDecoration(
              color: HyprColors.border.withValues(alpha: 0.72),
              borderRadius: BorderRadius.circular(99),
            ),
            child: const SizedBox(height: 1, width: double.infinity),
          ),
        );
      case TrayMenuItemKind.standard:
        final double indent = 12.0 * item.depth.clamp(0, 4);
        return Padding(
          padding: const EdgeInsets.symmetric(vertical: 1),
          child: HyprActionRow(
            title: item.label,
            enabled: item.enabled,
            onPressed: item.enabled
                ? () => onActivateItem(itemId, item.id)
                : null,
            padding: EdgeInsets.fromLTRB(10 + indent, 7, 10, 7),
            borderRadius: BorderRadius.circular(6),
            titleStyle: HyprTypography.popRow,
            titleColor: HyprColors.textMuted,
            hoverTitleColor: HyprColors.text,
            selectedTitleColor: HyprColors.text,
            hoverColor: HyprColors.hover,
          ),
        );
    }
  }
}

class _EmptyTrayMenuLabel extends StatelessWidget {
  const _EmptyTrayMenuLabel();

  @override
  Widget build(BuildContext context) {
    return Text(
      'No menu',
      style: HyprTypography.compactMono.copyWith(color: HyprColors.textFaint),
    );
  }
}
