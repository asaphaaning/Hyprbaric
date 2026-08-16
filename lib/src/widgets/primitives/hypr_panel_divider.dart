import 'package:flutter/material.dart';

import '../hypr_surface.dart';

class HyprPanelDivider extends StatelessWidget {
  const HyprPanelDivider({super.key, this.color = HyprColors.borderSoft});

  final Color color;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: HyprSpacing.hairline,
      child: DecoratedBox(
        decoration: BoxDecoration(
          gradient: LinearGradient(
            colors: <Color>[
              const Color(0x00FFFFFF),
              color,
              const Color(0x00FFFFFF),
            ],
          ),
        ),
      ),
    );
  }
}

class HyprSectionBreak extends StatelessWidget {
  const HyprSectionBreak({
    super.key,
    this.before = HyprSpacing.xxl,
    this.after = HyprSpacing.xxl,
    this.color = HyprColors.borderSoft,
  });

  final double before;
  final double after;
  final Color color;

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: <Widget>[
        SizedBox(height: before),
        HyprPanelDivider(color: color),
        SizedBox(height: after),
      ],
    );
  }
}
