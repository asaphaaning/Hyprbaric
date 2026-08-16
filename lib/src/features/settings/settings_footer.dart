import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../state/providers.dart';
import '../../widgets/hypr_surface.dart';

class SettingsVersionFooter extends ConsumerWidget {
  const SettingsVersionFooter({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final String version = ref
        .watch(appStatusProvider)
        .maybeWhen(data: (status) => status.version, orElse: () => '...');

    return Row(
      children: <Widget>[
        Text(
          'hyprbaric',
          style: HyprTypography.compactMono.copyWith(
            color: HyprColors.textFaint,
            fontWeight: FontWeight.w600,
          ),
        ),
        const Spacer(),
        Text(
          'v$version',
          style: HyprTypography.compactMono.copyWith(
            color: HyprColors.textFaint,
            fontSize: HyprTypography.size(10),
          ),
        ),
      ],
    );
  }
}
