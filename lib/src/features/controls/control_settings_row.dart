import 'package:flutter/material.dart';
import 'package:iconsax_flutter/iconsax_flutter.dart';

import '../../widgets/hypr_surface.dart';
import '../../widgets/primitives/primitives.dart';
import 'controls_chrome.dart';

class ControlSettingsRow extends StatelessWidget {
  const ControlSettingsRow({super.key, required this.onPressed});

  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return HyprHoverPlate(
      onPressed: onPressed,
      semanticLabel: 'Settings',
      borderRadius: BorderRadius.circular(9),
      color: Colors.black.withValues(alpha: 0.22),
      hoverColor: Colors.black.withValues(alpha: 0.38),
      borderColor: HyprColors.popupStroke,
      hoverBorderColor: ControlColors.strokeHover,
      padding: const EdgeInsets.symmetric(horizontal: 11, vertical: 9),
      builder: (BuildContext context, {required bool hovered}) {
        return Center(
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: <Widget>[
              const Icon(
                Iconsax.setting_5_copy,
                size: 15,
                color: HyprColors.accentSoft,
              ),
              const SizedBox(width: 8),
              Text(
                'Settings',
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
                textAlign: TextAlign.center,
                style: HyprTypography.popRowStrong.copyWith(
                  color: HyprColors.text,
                  fontSize: HyprTypography.size(12),
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}
