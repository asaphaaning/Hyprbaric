import 'package:flutter/material.dart';

import '../../widgets/hypr_surface.dart';
import '../../widgets/primitives/primitives.dart';

abstract final class ControlColors {
  static const Color tile = Color(0x2E000000);
  static const Color tileHover = Color(0x0DFFFFFF);
  static const Color stroke = HyprColors.popupStroke;
  static const Color strokeHover = Color(0x24FFFFFF);
  static const Color danger = Color(0xFFE16658);
  static const Color amber = Color(0xFFFFC08F);
}

class ControlSectionLabel extends StatelessWidget {
  const ControlSectionLabel(this.label, {super.key});

  final String label;

  @override
  Widget build(BuildContext context) {
    return HyprSectionLabel(
      label,
      fontSize: HyprTypography.size(10),
      fontWeight: FontWeight.w700,
      letterSpacing: 2.2,
      trailingLine: true,
    );
  }
}
