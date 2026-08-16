import 'package:flutter/material.dart';

import '../bindings/bindings.dart';
import 'notification_panel_parts.dart';
import 'primitives/primitives.dart';

class NotificationPanel extends StatelessWidget {
  const NotificationPanel({
    super.key,
    required this.borderRadius,
    required this.status,
    required this.onDismiss,
    required this.onClearAll,
  });

  final BorderRadius borderRadius;
  final NotificationStatus? status;
  final ValueChanged<int> onDismiss;
  final VoidCallback onClearAll;

  @override
  Widget build(BuildContext context) {
    final NotificationStatus? snapshot = status;
    final List<NotificationEntry> entries = snapshot?.entries ?? const [];
    final bool available = snapshot?.available ?? true;
    final String? message = snapshot?.message;
    final bool canClear = entries.isNotEmpty;

    return HyprPopoverPanel(
      borderRadius: borderRadius,
      constraints: const BoxConstraints(
        minWidth: 360,
        maxWidth: 360,
        maxHeight: 384,
      ),
      padding: const EdgeInsets.fromLTRB(16, 17, 16, 16),
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: <Widget>[
          NotificationHeader(
            count: entries.length,
            canClear: canClear,
            onClearAll: onClearAll,
          ),
          const SizedBox(height: 12),
          if (entries.isEmpty)
            NotificationEmptyState(
              message: available
                  ? 'No new notifications'
                  : (message ?? 'Notifications are unavailable.'),
            )
          else
            Flexible(
              child: NotificationList(entries: entries, onDismiss: onDismiss),
            ),
        ],
      ),
    );
  }
}
