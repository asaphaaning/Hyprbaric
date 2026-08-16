import 'package:flutter/material.dart';

import '../../bindings/bindings.dart';
import '../../widgets/hypr_surface.dart';
import 'network_chrome.dart';

class NetworkInterfacesSection extends StatelessWidget {
  const NetworkInterfacesSection({super.key, required this.interfaces});

  final List<NetworkInterface> interfaces;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: <Widget>[
        const NetworkSectionTitle('Interfaces'),
        const SizedBox(height: 8),
        ConstrainedBox(
          constraints: const BoxConstraints(maxHeight: 120),
          child: interfaces.isEmpty
              ? const NetworkEmptyState(message: 'No interfaces found.')
              : ListView(
                  padding: EdgeInsets.zero,
                  shrinkWrap: true,
                  children: <Widget>[
                    for (final NetworkInterface interface in interfaces)
                      _NetworkInterfaceRow(interface: interface),
                  ],
                ),
        ),
      ],
    );
  }
}

class _NetworkInterfaceRow extends StatelessWidget {
  const _NetworkInterfaceRow({required this.interface});

  final NetworkInterface interface;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(bottom: 6),
      child: DecoratedBox(
        decoration: ShapeDecoration(
          color: const Color(0x22091017),
          shape: RoundedSuperellipseBorder(
            borderRadius: BorderRadius.circular(7),
            side: const BorderSide(color: NetworkMenuColors.cardBorder),
          ),
        ),
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
          child: Row(
            children: <Widget>[
              Expanded(
                child: Text(
                  interface.name,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  textAlign: TextAlign.center,
                  style: HyprTypography.compactMono.copyWith(
                    color: NetworkMenuColors.fg2,
                    fontSize: HyprTypography.size(11),
                  ),
                ),
              ),
              const SizedBox(width: 10),
              SizedBox(
                width: 92,
                child: Text(
                  interface.address ?? '-',
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  textAlign: TextAlign.right,
                  style: HyprTypography.compactMonoStrong.copyWith(
                    fontSize: HyprTypography.size(11),
                    fontWeight: FontWeight.w500,
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
