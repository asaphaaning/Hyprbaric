import 'package:flutter/material.dart';

import 'hypr_colors.dart';

class HyprDivider extends StatelessWidget {
  const HyprDivider({super.key, this.height = 20, this.margin});

  final double height;
  final EdgeInsetsGeometry? margin;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: margin ?? const EdgeInsets.symmetric(horizontal: 8),
      child: SizedBox(
        width: 1,
        height: height,
        child: const DecoratedBox(
          decoration: BoxDecoration(
            gradient: LinearGradient(
              begin: Alignment.topCenter,
              end: Alignment.bottomCenter,
              colors: <Color>[
                Color(0x00FFFFFF),
                HyprColors.border,
                Color(0x00FFFFFF),
              ],
              stops: <double>[0, 0.5, 1],
            ),
          ),
        ),
      ),
    );
  }
}
