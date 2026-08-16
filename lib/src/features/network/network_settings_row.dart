import 'package:flutter/material.dart';

import '../../widgets/hypr_surface.dart';
import '../../widgets/primitives/primitives.dart';
import 'network_chrome.dart';

class NetworkSettingsRow extends StatelessWidget {
  const NetworkSettingsRow({super.key, required this.onPressed});

  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return HyprActionRow(
      semanticLabel: 'Network settings',
      onPressed: onPressed,
      borderRadius: BorderRadius.circular(7),
      hoverColor: NetworkMenuColors.hover,
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 6),
      title: 'Network settings…',
      titleColor: NetworkMenuColors.fg2,
      hoverTitleColor: NetworkMenuColors.fg1,
      titleStyle: HyprTypography.popRow.copyWith(
        fontSize: HyprTypography.size(12.5),
        fontWeight: FontWeight.w500,
      ),
      leadingBuilder:
          (
            BuildContext context, {
            required bool hovered,
            required bool selected,
          }) {
            return Text(
              '⚙',
              style: HyprTypography.compactMono.copyWith(
                color: hovered ? NetworkMenuColors.fg2 : NetworkMenuColors.fg3,
                fontSize: HyprTypography.size(12.5),
                height: 1,
              ),
            );
          },
    );
  }
}
