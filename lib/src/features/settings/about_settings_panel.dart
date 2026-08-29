import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';
import '../../state/providers.dart';
import '../../widgets/hypr_surface.dart';
import '../../widgets/primitives/primitives.dart';
import '../setup/setup_guide_state.dart';

class AboutSettingsPanel extends ConsumerWidget {
  const AboutSettingsPanel({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final CapabilityStatus status = ref.watch(currentCapabilityStatusProvider);

    return ListView(
      padding: EdgeInsets.zero,
      children: <Widget>[
        const _ProductHeader(),
        const SizedBox(height: 10),
        HyprCommandButton(
          key: const ValueKey<String>('run-setup-guide'),
          label: 'Run setup guide again',
          icon: const Icon(Icons.auto_awesome_rounded, size: 15),
          onPressed: () => ref.read(setupGuideRequestProvider.notifier).show(),
          padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 9),
          constraints: const BoxConstraints(minHeight: 36),
          color: Colors.black.withValues(alpha: 0.14),
          borderColor: HyprColors.popupStroke,
          foregroundColor: HyprColors.textMuted,
          hoverForegroundColor: HyprColors.text,
          hoverBorderColor: context.hyprPalette.borderSoft,
          textStyle: HyprTypography.compactMonoStrong.copyWith(
            fontSize: HyprTypography.size(11),
          ),
        ),
        if (status.entries.isNotEmpty) ...<Widget>[
          const SizedBox(height: 12),
          Text(
            'System',
            style: HyprTypography.compactMonoStrong.copyWith(
              color: HyprColors.textFaint,
              fontSize: HyprTypography.size(11),
              letterSpacing: 0,
            ),
          ),
          const SizedBox(height: 8),
          ..._spacedRows(
            status.entries.map(
              (CapabilityEntry entry) => _CapabilityRow(entry: entry),
            ),
          ),
        ],
      ],
    );
  }
}

class _ProductHeader extends ConsumerWidget {
  const _ProductHeader();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final String version = ref
        .watch(appStatusProvider)
        .maybeWhen(
          data: (AppStatus status) => status.version,
          orElse: () => '...',
        );

    return DecoratedBox(
      decoration: ShapeDecoration(
        color: Colors.black.withValues(alpha: 0.16),
        shape: const RoundedSuperellipseBorder(
          borderRadius: HyprRadii.panelRadius,
          side: BorderSide(color: HyprColors.popupStroke),
        ),
      ),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 12),
        child: Row(
          children: <Widget>[
            DecoratedBox(
              decoration: ShapeDecoration(
                color: context.hyprPalette.fillStrong,
                shape: RoundedSuperellipseBorder(
                  borderRadius: HyprRadii.cardRadius,
                  side: BorderSide(color: context.hyprPalette.borderSoft),
                ),
              ),
              child: Padding(
                padding: const EdgeInsets.all(8),
                child: Icon(
                  Icons.auto_awesome_motion_rounded,
                  color: context.hyprPalette.accent,
                  size: 17,
                ),
              ),
            ),
            const SizedBox(width: 10),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: <Widget>[
                  Text(
                    'Hyprbaric',
                    style: HyprTypography.popRow.copyWith(
                      fontSize: HyprTypography.size(13),
                      fontWeight: FontWeight.w700,
                    ),
                  ),
                  const SizedBox(height: 2),
                  Text(
                    'Flutter bar for Hyprland with a Rust runtime.',
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: HyprTypography.popRow.copyWith(
                      color: HyprColors.textFaint,
                      fontSize: HyprTypography.size(11),
                    ),
                  ),
                ],
              ),
            ),
            HyprBadge.text(
              label: 'v$version',
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
              color: Colors.black.withValues(alpha: 0.12),
              borderColor: HyprColors.popupStroke.withValues(alpha: 0.65),
              borderRadius: HyprRadii.cardRadius,
              textColor: HyprColors.textMuted,
              style: HyprTypography.compactMonoStrong.copyWith(
                fontSize: HyprTypography.size(11),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _CapabilityRow extends StatelessWidget {
  const _CapabilityRow({required this.entry});

  final CapabilityEntry entry;

  @override
  Widget build(BuildContext context) {
    final _StatusTone tone = _StatusTone.forAvailability(entry.availability);

    return DecoratedBox(
      decoration: ShapeDecoration(
        color: Colors.black.withValues(alpha: 0.16),
        shape: RoundedSuperellipseBorder(
          borderRadius: HyprRadii.panelRadius,
          side: BorderSide(
            color: entry.availability == CapabilityAvailability.available
                ? HyprColors.popupStroke
                : tone.color.withValues(alpha: 0.34),
          ),
        ),
      ),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 11),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: <Widget>[
            Row(
              children: <Widget>[
                Expanded(
                  child: Text(
                    entry.label,
                    maxLines: 1,
                    overflow: TextOverflow.ellipsis,
                    style: HyprTypography.popRow.copyWith(
                      fontSize: HyprTypography.size(13),
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                ),
                const SizedBox(width: 8),
                HyprBadge.text(
                  label: entry.tier.label,
                  padding: const EdgeInsets.symmetric(
                    horizontal: 7,
                    vertical: 3,
                  ),
                  color: Colors.black.withValues(alpha: 0.10),
                  borderColor: HyprColors.popupStroke.withValues(alpha: 0.65),
                  borderRadius: HyprRadii.cardRadius,
                  textColor: HyprColors.textFaint,
                  style: HyprTypography.compactMonoStrong.copyWith(
                    fontSize: HyprTypography.size(10),
                  ),
                ),
                const SizedBox(width: 6),
                HyprBadge.text(
                  label: tone.label,
                  padding: const EdgeInsets.symmetric(
                    horizontal: 7,
                    vertical: 3,
                  ),
                  color: tone.color.withValues(alpha: 0.08),
                  borderColor: tone.color.withValues(alpha: 0.28),
                  borderRadius: HyprRadii.cardRadius,
                  textColor: tone.color,
                  style: HyprTypography.compactMonoStrong.copyWith(
                    fontSize: HyprTypography.size(10),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 3),
            Text(
              entry.detail,
              maxLines: 2,
              overflow: TextOverflow.ellipsis,
              style: HyprTypography.popRow.copyWith(
                color: HyprColors.textFaint,
                fontSize: HyprTypography.size(11),
              ),
            ),
            if (entry.message case final String message) ...<Widget>[
              const SizedBox(height: 7),
              Text(
                message,
                maxLines: 2,
                overflow: TextOverflow.ellipsis,
                style: HyprTypography.compactMonoStrong.copyWith(
                  color: tone.color.withValues(alpha: 0.86),
                  fontSize: HyprTypography.size(10),
                ),
              ),
            ],
            if (entry.commands.isNotEmpty) ...<Widget>[
              const SizedBox(height: 8),
              Wrap(
                spacing: 5,
                runSpacing: 5,
                children: entry.commands
                    .map(
                      (String command) => HyprBadge.text(
                        label: command,
                        padding: const EdgeInsets.symmetric(
                          horizontal: 6,
                          vertical: 2,
                        ),
                        color: Colors.black.withValues(alpha: 0.12),
                        borderColor: HyprColors.popupStroke.withValues(
                          alpha: 0.50,
                        ),
                        borderRadius: HyprRadii.compactRadius,
                        textColor: HyprColors.textMuted,
                        style: HyprTypography.compactMonoStrong.copyWith(
                          fontSize: HyprTypography.size(10),
                        ),
                      ),
                    )
                    .toList(),
              ),
            ],
          ],
        ),
      ),
    );
  }
}

class _StatusTone {
  const _StatusTone({required this.label, required this.color});

  factory _StatusTone.forAvailability(CapabilityAvailability availability) {
    return switch (availability) {
      CapabilityAvailability.available => const _StatusTone(
        label: 'Ready',
        color: HyprColors.accent,
      ),
      CapabilityAvailability.degraded => const _StatusTone(
        label: 'Partial',
        color: Color(0xFFF0C66A),
      ),
      CapabilityAvailability.missing => const _StatusTone(
        label: 'Missing',
        color: HyprColors.danger,
      ),
    };
  }

  final String label;
  final Color color;
}

extension on CapabilityTier {
  String get label {
    return switch (this) {
      CapabilityTier.core => 'Core',
      CapabilityTier.service => 'Service',
      CapabilityTier.optional => 'Optional',
    };
  }
}

List<Widget> _spacedRows(Iterable<Widget> rows) {
  final List<Widget> widgets = <Widget>[];
  for (final Widget row in rows) {
    if (widgets.isNotEmpty) {
      widgets.add(const SizedBox(height: 8));
    }
    widgets.add(row);
  }
  return widgets;
}
