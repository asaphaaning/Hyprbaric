import 'package:flutter/material.dart';

import '../bindings/bindings.dart';
import 'hypr_surface.dart';
import 'notification_panel_style.dart';
import 'notification_row.dart';
import 'primitives/primitives.dart';

class NotificationHeader extends StatelessWidget {
  const NotificationHeader({
    super.key,
    required this.count,
    required this.canClear,
    required this.onClearAll,
  });

  final int count;
  final bool canClear;
  final VoidCallback onClearAll;

  @override
  Widget build(BuildContext context) {
    return HyprPanelHeader(
      title: 'Notifications',
      uppercaseTitle: true,
      titleTrailing: count > 0 ? NotificationCountPill(count: count) : null,
      actionKey: const ValueKey<String>('notifications-clear-all'),
      actionLabel: 'clear all',
      actionEnabled: canClear,
      onAction: onClearAll,
      titleStyle: HyprTypography.popTitle.copyWith(
        color: HyprColors.textFaint,
        fontSize: HyprTypography.size(10.5),
        letterSpacing: 2.0,
        fontWeight: FontWeight.w700,
      ),
      actionColor: NotificationPalette.fg3,
      actionStyle: HyprTypography.compactMono.copyWith(
        fontSize: HyprTypography.size(11),
      ),
    );
  }
}

class NotificationCountPill extends StatelessWidget {
  const NotificationCountPill({super.key, required this.count});

  final int count;

  @override
  Widget build(BuildContext context) {
    return KeyedSubtree(
      key: const ValueKey<String>('notifications-count-pill'),
      child: HyprBadge.text(
        label: '$count',
        color: HyprColors.accentSoft.withValues(alpha: 0.34),
        borderColor: HyprColors.accent.withValues(alpha: 0.22),
        borderRadius: BorderRadius.circular(4),
        padding: const EdgeInsets.symmetric(horizontal: 7, vertical: 3),
        textColor: HyprColors.accent,
        style: HyprTypography.compactMonoStrong.copyWith(
          fontSize: HyprTypography.size(9.5),
          height: 1,
        ),
      ),
    );
  }
}

class NotificationList extends StatelessWidget {
  const NotificationList({
    super.key,
    required this.entries,
    required this.onDismiss,
  });

  final List<NotificationEntry> entries;
  final ValueChanged<int> onDismiss;

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: ShapeDecoration(
        color: Colors.black.withValues(alpha: 0.18),
        shape: RoundedSuperellipseBorder(
          borderRadius: BorderRadius.circular(7),
          side: BorderSide(
            color: HyprColors.popupStroke.withValues(alpha: 0.65),
          ),
        ),
      ),
      child: ClipRRect(
        borderRadius: BorderRadius.circular(7),
        child: ListView.separated(
          shrinkWrap: true,
          padding: EdgeInsets.zero,
          itemCount: entries.length,
          separatorBuilder: (_, _) => DecoratedBox(
            decoration: BoxDecoration(
              color: HyprColors.borderSoft.withValues(alpha: 0.55),
            ),
            child: const SizedBox(height: 1),
          ),
          itemBuilder: (BuildContext context, int index) {
            final NotificationEntry entry = entries[index];
            return NotificationRow(
              entry: entry,
              onDismiss: () => onDismiss(entry.id),
            );
          },
        ),
      ),
    );
  }
}

class NotificationEmptyState extends StatelessWidget {
  const NotificationEmptyState({super.key, required this.message});

  final String message;

  @override
  Widget build(BuildContext context) {
    return HyprEmptyState(
      message: message,
      symbol: '◎',
      padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 32),
      messageTransform: (String value) => value.toLowerCase(),
      borderColor: HyprColors.popupStroke,
      borderRadius: BorderRadius.circular(9),
      symbolStyle: HyprTypography.compactMono.copyWith(
        color: NotificationPalette.fg3.withValues(alpha: 0.45),
        fontSize: HyprTypography.size(22),
        height: 1,
      ),
      messageStyle: HyprTypography.popRow.copyWith(
        color: NotificationPalette.fg3,
        fontSize: HyprTypography.size(11.5),
      ),
    );
  }
}
