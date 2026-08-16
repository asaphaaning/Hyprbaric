import 'package:flutter/material.dart';

import '../../bindings/bindings.dart';
import '../../widgets/hypr_surface.dart';
import '../../widgets/primitives/primitives.dart';
import 'network_chrome.dart';
import 'network_entry_state.dart';
import 'network_entry_tile.dart';

class NetworkWifiSection extends StatelessWidget {
  const NetworkWifiSection({
    super.key,
    required this.devicePresent,
    required this.wifiEnabled,
    required this.scanning,
    required this.networks,
    required this.expandedSsid,
    required this.selectedSsid,
    required this.passwordController,
    required this.passwordFocusNode,
    required this.showPassword,
    required this.inlineError,
    required this.onToggleWifiEnabled,
    required this.onToggleEntry,
    required this.onTogglePasswordVisibility,
    required this.onCancelPasswordPrompt,
    required this.onSubmit,
  });

  final bool devicePresent;
  final bool wifiEnabled;
  final bool scanning;
  final List<NetworkEntry> networks;
  final String? expandedSsid;
  final String? selectedSsid;
  final TextEditingController passwordController;
  final FocusNode passwordFocusNode;
  final bool showPassword;
  final String? inlineError;
  final VoidCallback onToggleWifiEnabled;
  final ValueChanged<NetworkEntry> onToggleEntry;
  final VoidCallback onTogglePasswordVisibility;
  final VoidCallback onCancelPasswordPrompt;
  final ValueChanged<NetworkEntry> onSubmit;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: <Widget>[
        Row(
          children: <Widget>[
            const NetworkSectionTitle('Wi-Fi'),
            const Spacer(),
            Material(
              color: Colors.transparent,
              shape: RoundedSuperellipseBorder(
                borderRadius: BorderRadius.circular(7),
              ),
              clipBehavior: Clip.antiAlias,
              child: InkWell(
                onTap: devicePresent ? onToggleWifiEnabled : null,
                hoverColor: HyprColors.hover,
                splashColor: Colors.transparent,
                highlightColor: Colors.transparent,
                child: Padding(
                  padding: const EdgeInsets.symmetric(
                    horizontal: 2,
                    vertical: 2,
                  ),
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: <Widget>[
                      Text(
                        wifiEnabled ? 'on' : 'off',
                        style: HyprTypography.compactMonoStrong.copyWith(
                          color: NetworkMenuColors.fg2,
                          fontSize: HyprTypography.size(11),
                          letterSpacing: 0.66,
                        ),
                      ),
                      const SizedBox(width: 8),
                      _NetworkWifiSwitch(value: wifiEnabled && devicePresent),
                    ],
                  ),
                ),
              ),
            ),
          ],
        ),
        const SizedBox(height: 6),
        ConstrainedBox(
          constraints: const BoxConstraints(maxHeight: 154),
          child: _NetworkWifiList(
            child: ListView(
              padding: EdgeInsets.zero,
              shrinkWrap: true,
              children: <Widget>[
                if (!devicePresent)
                  const NetworkEmptyState(message: 'No Wi-Fi device found.')
                else if (!wifiEnabled)
                  const NetworkEmptyState(message: 'Wi-Fi is turned off.')
                else if (networks.isEmpty)
                  NetworkEmptyState(
                    message: scanning
                        ? 'Scanning for networks...'
                        : 'No networks found.',
                  )
                else
                  for (final NetworkEntry entry in networks)
                    NetworkEntryTile(
                      entry: entry,
                      expanded: expandedSsid == entry.ssid,
                      selected:
                          selectedSsid == entry.ssid &&
                          !entry.isActive &&
                          !entry.isConnecting,
                      passwordController: passwordController,
                      passwordFocusNode: passwordFocusNode,
                      showPassword: showPassword,
                      errorMessage: expandedSsid == entry.ssid
                          ? inlineError
                          : null,
                      onTap: () => onToggleEntry(entry),
                      onTogglePasswordVisibility: onTogglePasswordVisibility,
                      onCancel: onCancelPasswordPrompt,
                      onSubmit: () => onSubmit(entry),
                    ),
              ],
            ),
          ),
        ),
      ],
    );
  }
}

class _NetworkWifiSwitch extends StatelessWidget {
  const _NetworkWifiSwitch({required this.value});

  final bool value;

  @override
  Widget build(BuildContext context) {
    return HyprToggleSwitch(
      value: value,
      width: 26,
      height: 14,
      padding: const EdgeInsets.all(1),
      thumbSize: 10,
      trackColor: const Color(0xFF252C36),
      activeTrackColor: HyprColors.accent.withValues(alpha: 0.40),
      borderColor: const Color(0xFF3B4652),
      activeBorderColor: HyprColors.accent,
      thumbColor: NetworkMenuColors.fg2,
      activeThumbColor: const Color(0xFFE8F5FF),
      duration: HyprMotion.switcher,
      curve: HyprMotion.switchInCurve,
    );
  }
}

class _NetworkWifiList extends StatelessWidget {
  const _NetworkWifiList({required this.child});

  final Widget child;

  @override
  Widget build(BuildContext context) {
    return DecoratedBox(
      decoration: BoxDecoration(
        color: Colors.black.withValues(alpha: 0.35),
        borderRadius: BorderRadius.circular(8),
        border: Border.all(color: Colors.white.withValues(alpha: 0.04)),
        boxShadow: const <BoxShadow>[
          BoxShadow(
            color: Color(0x99000000),
            blurRadius: 2,
            offset: Offset(0, 1),
            blurStyle: BlurStyle.inner,
          ),
        ],
      ),
      child: Padding(padding: const EdgeInsets.all(4), child: child),
    );
  }
}
