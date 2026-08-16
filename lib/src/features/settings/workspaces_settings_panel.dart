import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';
import '../../state/providers.dart';
import '../../widgets/hypr_surface.dart';
import '../../widgets/primitives/primitives.dart';

class WorkspacesSettingsPanel extends ConsumerWidget {
  const WorkspacesSettingsPanel({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final WorkspaceSettingsStatus status = ref.watch(
      currentWorkspaceSettingsProvider,
    );

    return ListView(
      padding: EdgeInsets.zero,
      children: <Widget>[
        _SegmentRow<WorkspaceIndicatorStyle>(
          label: 'Indicator style',
          subtitle: _indicatorSubtitle(status.indicatorStyle),
          values: WorkspaceIndicatorStyle.values,
          value: status.indicatorStyle,
          labelFor: (WorkspaceIndicatorStyle value) => value.label,
          onChanged: (WorkspaceIndicatorStyle value) {
            ref
                .read(workspaceSettingsControllerProvider.notifier)
                .setIndicatorStyle(value);
          },
        ),
        const SizedBox(height: 8),
        _ClickableRow(
          value: status.clickable,
          onChanged: (bool value) {
            ref
                .read(workspaceSettingsControllerProvider.notifier)
                .setClickable(clickable: value);
          },
        ),
        const SizedBox(height: 8),
        _SegmentRow<WorkspaceVisibleRange>(
          label: 'Visible range',
          subtitle:
              '${status.visibleRange.label} keeps ${status.visibleCount} indicators visible.',
          valueLabel: '${status.visibleCount}',
          values: WorkspaceVisibleRange.values,
          value: status.visibleRange,
          labelFor: (WorkspaceVisibleRange value) => value.label,
          onChanged: (WorkspaceVisibleRange value) {
            ref
                .read(workspaceSettingsControllerProvider.notifier)
                .setVisibleRange(value);
          },
        ),
      ],
    );
  }

  String _indicatorSubtitle(WorkspaceIndicatorStyle style) {
    return switch (style) {
      WorkspaceIndicatorStyle.roman =>
        'Show workspace numbers as Roman numerals.',
      WorkspaceIndicatorStyle.numeric =>
        'Show workspace numbers as plain digits.',
    };
  }
}

class _ClickableRow extends StatelessWidget {
  const _ClickableRow({required this.value, required this.onChanged});

  final bool value;
  final ValueChanged<bool> onChanged;

  @override
  Widget build(BuildContext context) {
    return HyprInteractionRegion(
      enabled: true,
      onPressed: () => onChanged(!value),
      builder: (BuildContext context, HyprInteractionState state) {
        return _WorkspaceSettingsRow(
          label: 'Clickable workspaces',
          subtitle: value
              ? 'Direct indicator clicks switch workspaces.'
              : 'Direct indicator clicks are ignored.',
          hovered: state.hovered,
          trailing: Row(
            mainAxisSize: MainAxisSize.min,
            children: <Widget>[
              HyprBadge.text(
                label: value ? 'On' : 'Off',
                padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                color: Colors.black.withValues(alpha: 0.12),
                borderColor: HyprColors.popupStroke.withValues(alpha: 0.65),
                borderRadius: HyprRadii.cardRadius,
                textColor: value
                    ? context.hyprPalette.accent
                    : HyprColors.textMuted,
                style: HyprTypography.compactMonoStrong.copyWith(
                  fontSize: HyprTypography.size(11),
                ),
              ),
              const SizedBox(width: 10),
              HyprToggleSwitch(value: value),
            ],
          ),
        );
      },
    );
  }
}

class _SegmentRow<T> extends StatelessWidget {
  const _SegmentRow({
    required this.label,
    required this.subtitle,
    required this.values,
    required this.value,
    required this.labelFor,
    required this.onChanged,
    this.valueLabel,
  });

  final String label;
  final String subtitle;
  final List<T> values;
  final T value;
  final String Function(T value) labelFor;
  final ValueChanged<T> onChanged;
  final String? valueLabel;

  @override
  Widget build(BuildContext context) {
    return _WorkspaceSettingsRow(
      label: label,
      subtitle: subtitle,
      valueLabel: valueLabel,
      child: Row(
        children: <Widget>[
          for (final T option in values) ...<Widget>[
            _SegmentButton(
              label: labelFor(option),
              selected: option == value,
              onPressed: () => onChanged(option),
            ),
            if (option != values.last) const SizedBox(width: 5),
          ],
        ],
      ),
    );
  }
}

class _WorkspaceSettingsRow extends StatelessWidget {
  const _WorkspaceSettingsRow({
    required this.label,
    required this.subtitle,
    this.hovered = false,
    this.valueLabel,
    this.child,
    this.trailing,
  });

  final String label;
  final String subtitle;
  final bool hovered;
  final String? valueLabel;
  final Widget? child;
  final Widget? trailing;

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: ShapeDecoration(
        color: hovered
            ? context.hyprPalette.fill
            : Colors.black.withValues(alpha: 0.16),
        shape: RoundedSuperellipseBorder(
          borderRadius: HyprRadii.panelRadius,
          side: BorderSide(
            color: hovered
                ? context.hyprPalette.borderSoft
                : HyprColors.popupStroke,
          ),
        ),
      ),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 11),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: <Widget>[
            Row(
              children: <Widget>[
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: <Widget>[
                      Text(
                        label,
                        style: HyprTypography.popRow.copyWith(
                          fontSize: HyprTypography.size(13),
                          fontWeight: FontWeight.w600,
                        ),
                      ),
                      const SizedBox(height: 2),
                      Text(
                        subtitle,
                        style: HyprTypography.popRow.copyWith(
                          color: HyprColors.textFaint,
                          fontSize: HyprTypography.size(11),
                        ),
                      ),
                    ],
                  ),
                ),
                if (valueLabel != null) _ValueBadge(label: valueLabel!),
                ?trailing,
              ],
            ),
            if (child != null) ...<Widget>[const SizedBox(height: 8), child!],
          ],
        ),
      ),
    );
  }
}

class _ValueBadge extends StatelessWidget {
  const _ValueBadge({required this.label});

  final String label;

  @override
  Widget build(BuildContext context) {
    return HyprBadge.text(
      label: label,
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
      color: Colors.black.withValues(alpha: 0.12),
      borderColor: HyprColors.popupStroke.withValues(alpha: 0.65),
      borderRadius: HyprRadii.cardRadius,
      textColor: HyprColors.textMuted,
      style: HyprTypography.compactMonoStrong.copyWith(
        fontSize: HyprTypography.size(11),
      ),
    );
  }
}

class _SegmentButton extends StatelessWidget {
  const _SegmentButton({
    required this.label,
    required this.selected,
    required this.onPressed,
  });

  final String label;
  final bool selected;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return HyprCommandButton(
      label: label,
      onPressed: selected ? null : onPressed,
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 7),
      constraints: const BoxConstraints(minHeight: 30),
      color: selected
          ? context.hyprPalette.fillStrong
          : Colors.black.withValues(alpha: 0.12),
      borderColor: selected
          ? context.hyprPalette.borderSoft
          : HyprColors.popupStroke,
      foregroundColor: selected ? HyprColors.text : HyprColors.textMuted,
      textStyle: HyprTypography.compactMonoStrong.copyWith(
        fontSize: HyprTypography.size(10.5),
      ),
    );
  }
}
