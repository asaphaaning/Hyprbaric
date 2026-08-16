import 'package:flutter/material.dart';

import '../../widgets/hypr_surface.dart';
import '../../widgets/primitives/primitives.dart';
import 'settings_footer.dart';

enum SettingsTab {
  appearance(
    'Appearance',
    'How the bar looks and feels.',
    Icons.palette_outlined,
  ),
  modules('Modules', 'Show and hide bar modules.', Icons.grid_view_rounded),
  workspaces(
    'Workspaces',
    'Workspace indicator style and labels.',
    Icons.view_quilt_outlined,
  ),
  display(
    'Display',
    'Brightness and screen temperature.',
    Icons.monitor_rounded,
  ),
  keybinds(
    'Keybinds',
    'Global shortcuts for bar actions.',
    Icons.keyboard_rounded,
  ),
  about('About', 'Build info and licenses.', Icons.info_outline_rounded);

  const SettingsTab(this.label, this.subtitle, this.icon);

  final String label;
  final String subtitle;
  final IconData icon;
}

class SettingsSidebar extends StatelessWidget {
  const SettingsSidebar({
    super.key,
    required this.activeTab,
    required this.onTabChanged,
  });

  final SettingsTab activeTab;
  final ValueChanged<SettingsTab> onTabChanged;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 190,
      child: Padding(
        padding: const EdgeInsets.all(14),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: <Widget>[
            const SettingsSectionTitle('Bar settings'),
            const SizedBox(height: 14),
            for (final SettingsTab tab in SettingsTab.values)
              SettingsTabButton(
                tab: tab,
                active: tab == activeTab,
                onPressed: () => onTabChanged(tab),
              ),
            const Spacer(),
            const SettingsVersionFooter(),
          ],
        ),
      ),
    );
  }
}

class SettingsTabButton extends StatelessWidget {
  const SettingsTabButton({
    super.key,
    required this.tab,
    required this.active,
    required this.onPressed,
  });

  final SettingsTab tab;
  final bool active;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 5),
      child: HyprActionRow(
        title: tab.label,
        onPressed: onPressed,
        selected: active,
        padding: const EdgeInsets.symmetric(horizontal: 9, vertical: 8),
        borderRadius: BorderRadius.circular(8),
        hoverColor: HyprColors.hover,
        selectedColor: context.hyprPalette.fill,
        selectedBorderColor: context.hyprPalette.accentSoft,
        selectedBorderWidth: 1.8,
        titleColor: HyprColors.textMuted,
        selectedTitleColor: HyprColors.text,
        titleStyle: HyprTypography.popRow.copyWith(fontWeight: FontWeight.w600),
        leadingGap: 9,
        leadingBuilder:
            (
              BuildContext context, {
              required bool hovered,
              required bool selected,
            }) => Icon(
              tab.icon,
              size: 15,
              color: selected
                  ? context.hyprPalette.accentSoft
                  : HyprColors.textFaint,
            ),
      ),
    );
  }
}

class SettingsSectionTitle extends StatelessWidget {
  const SettingsSectionTitle(this.label, {super.key});

  final String label;

  @override
  Widget build(BuildContext context) {
    return Text(
      label.toUpperCase(),
      style: HyprTypography.popTitle.copyWith(color: HyprColors.textFaint),
    );
  }
}
