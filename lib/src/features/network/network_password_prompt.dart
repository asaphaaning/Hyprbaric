import 'package:flutter/material.dart';

import '../../widgets/hypr_surface.dart';
import '../../widgets/primitives/primitives.dart';
import 'network_chrome.dart';

class NetworkPasswordPrompt extends StatelessWidget {
  const NetworkPasswordPrompt({
    super.key,
    required this.ssid,
    required this.controller,
    required this.focusNode,
    required this.showPassword,
    required this.connecting,
    required this.errorMessage,
    required this.onToggleVisibility,
    required this.onCancel,
    required this.onSubmit,
  });

  final String ssid;
  final TextEditingController controller;
  final FocusNode focusNode;
  final bool showPassword;
  final bool connecting;
  final String? errorMessage;
  final VoidCallback onToggleVisibility;
  final VoidCallback onCancel;
  final VoidCallback onSubmit;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(top: 6, left: 13),
      child: Stack(
        children: <Widget>[
          Positioned.fill(
            child: DecoratedBox(
              decoration: BoxDecoration(
                borderRadius: BorderRadius.circular(8),
                gradient: RadialGradient(
                  center: Alignment.topLeft,
                  radius: 1.25,
                  colors: <Color>[
                    HyprColors.accent.withValues(alpha: 0.10),
                    Colors.transparent,
                  ],
                ),
              ),
            ),
          ),
          DecoratedBox(
            decoration: ShapeDecoration(
              color: const Color(0x990A1118),
              shape: RoundedSuperellipseBorder(
                borderRadius: BorderRadius.circular(8),
                side: const BorderSide(color: HyprColors.popupStroke),
              ),
            ),
            child: Padding(
              padding: const EdgeInsets.all(12),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: <Widget>[
                  _NetworkPasswordTitle(ssid: ssid, connecting: connecting),
                  const SizedBox(height: 10),
                  if (connecting)
                    Row(
                      children: <Widget>[
                        const SizedBox.square(
                          dimension: 12,
                          child: CircularProgressIndicator(strokeWidth: 1.5),
                        ),
                        const SizedBox(width: 8),
                        Expanded(
                          child: Text(
                            'Joining $ssid...',
                            maxLines: 1,
                            overflow: TextOverflow.ellipsis,
                            style: HyprTypography.popRow.copyWith(
                              color: NetworkMenuColors.fg2,
                              fontSize: HyprTypography.size(11.5),
                            ),
                          ),
                        ),
                      ],
                    )
                  else ...<Widget>[
                    _NetworkPasswordInputRow(
                      controller: controller,
                      focusNode: focusNode,
                      showPassword: showPassword,
                      onSubmit: onSubmit,
                    ),
                    const SizedBox(height: 8),
                    ValueListenableBuilder<TextEditingValue>(
                      valueListenable: controller,
                      builder:
                          (
                            BuildContext context,
                            TextEditingValue value,
                            Widget? child,
                          ) {
                            return _NetworkPasswordStrengthRow(
                              password: value.text,
                              showPassword: showPassword,
                              onToggleVisibility: onToggleVisibility,
                              onCancel: onCancel,
                            );
                          },
                    ),
                  ],
                  if (errorMessage != null) ...<Widget>[
                    const SizedBox(height: 7),
                    Text(
                      errorMessage!,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      textAlign: TextAlign.right,
                      style: HyprTypography.popMeta.copyWith(
                        color: HyprColors.danger,
                        fontSize: HyprTypography.size(11),
                        fontWeight: FontWeight.w600,
                      ),
                    ),
                  ],
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _NetworkPasswordTitle extends StatelessWidget {
  const _NetworkPasswordTitle({required this.ssid, required this.connecting});

  final String ssid;
  final bool connecting;

  @override
  Widget build(BuildContext context) {
    return Row(
      children: <Widget>[
        Text(
          connecting ? 'joining ' : 'join ',
          style: HyprTypography.compactMono.copyWith(
            color: NetworkMenuColors.fg3,
            fontSize: HyprTypography.size(10.5),
            letterSpacing: 0.84,
            height: 1,
          ),
        ),
        Flexible(
          child: Text(
            ssid,
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
            style: HyprTypography.compactMonoStrong.copyWith(
              color: HyprColors.text,
              fontSize: HyprTypography.size(12),
              letterSpacing: 0.24,
              height: 1,
            ),
          ),
        ),
      ],
    );
  }
}

class _NetworkPasswordInputRow extends StatelessWidget {
  const _NetworkPasswordInputRow({
    required this.controller,
    required this.focusNode,
    required this.showPassword,
    required this.onSubmit,
  });

  final TextEditingController controller;
  final FocusNode focusNode;
  final bool showPassword;
  final VoidCallback onSubmit;

  @override
  Widget build(BuildContext context) {
    return HyprTextFieldChrome(
      focusNode: focusNode,
      color: Colors.black.withValues(alpha: 0.45),
      borderColor: HyprColors.popupStroke,
      borderRadius: BorderRadius.circular(7),
      padding: const EdgeInsets.fromLTRB(10, 2, 2, 2),
      child: Row(
        children: <Widget>[
          Expanded(
            child: TextField(
              controller: controller,
              focusNode: focusNode,
              obscureText: !showPassword,
              onSubmitted: (_) => onSubmit(),
              style: HyprTypography.compactMonoStrong.copyWith(
                color: HyprColors.text,
                fontSize: HyprTypography.size(13),
                letterSpacing: 0.78,
              ),
              decoration: InputDecoration(
                isDense: true,
                border: InputBorder.none,
                hintText: 'enter password',
                hintStyle: HyprTypography.compactMono.copyWith(
                  color: HyprColors.textFaint,
                  fontSize: HyprTypography.size(12),
                  letterSpacing: 0,
                ),
                contentPadding: const EdgeInsets.symmetric(vertical: 8),
              ),
            ),
          ),
          const SizedBox(width: 8),
          _NetworkConnectButton(onPressed: onSubmit),
        ],
      ),
    );
  }
}

class _NetworkConnectButton extends StatelessWidget {
  const _NetworkConnectButton({required this.onPressed});

  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return HyprInteractiveTile(
      key: const ValueKey<String>('network-connect-submit'),
      semanticLabel: 'Connect to network',
      onPressed: onPressed,
      width: 32,
      height: 32,
      borderRadius: BorderRadius.circular(7),
      color: Colors.transparent,
      hoverColor: Colors.white.withValues(alpha: 0.10),
      borderColor: Colors.transparent,
      hoverBorderColor: Colors.transparent,
      builder: (BuildContext context, HyprInteractiveTileState state) {
        return DecoratedBox(
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(7),
            gradient: const LinearGradient(
              begin: Alignment.topLeft,
              end: Alignment.bottomRight,
              colors: <Color>[Color(0xFF8AD8FF), Color(0xFF2AA9F2)],
            ),
          ),
          child: const Center(
            child: Icon(
              Icons.arrow_forward_rounded,
              color: Color(0xFF071018),
              size: 17,
            ),
          ),
        );
      },
    );
  }
}

class _NetworkPasswordStrengthRow extends StatelessWidget {
  const _NetworkPasswordStrengthRow({
    required this.password,
    required this.showPassword,
    required this.onToggleVisibility,
    required this.onCancel,
  });

  final String password;
  final bool showPassword;
  final VoidCallback onToggleVisibility;
  final VoidCallback onCancel;

  @override
  Widget build(BuildContext context) {
    final int strength = _passwordStrength(password);
    final Color color = switch (strength) {
      1 => HyprColors.danger,
      2 => NetworkMenuColors.warning,
      3 => NetworkMenuColors.good,
      _ => Colors.white.withValues(alpha: 0.08),
    };
    return Row(
      children: <Widget>[
        Row(
          children: <Widget>[
            for (int index = 0; index < 3; index += 1)
              Padding(
                padding: EdgeInsets.only(right: index == 2 ? 0 : 3),
                child: DecoratedBox(
                  decoration: BoxDecoration(
                    color: index < strength
                        ? color
                        : Colors.white.withValues(alpha: 0.08),
                    borderRadius: BorderRadius.circular(1),
                  ),
                  child: const SizedBox(width: 18, height: 3),
                ),
              ),
          ],
        ),
        const Spacer(),
        _NetworkPasswordTextButton(
          label: showPassword ? '● hide' : '○ show',
          onPressed: onToggleVisibility,
        ),
        const SizedBox(width: 10),
        _NetworkPasswordTextButton(label: 'cancel', onPressed: onCancel),
      ],
    );
  }

  int _passwordStrength(String password) {
    if (password.isEmpty) {
      return 0;
    }
    if (password.length < 6) {
      return 1;
    }
    if (password.length < 10) {
      return 2;
    }
    return 3;
  }
}

class _NetworkPasswordTextButton extends StatelessWidget {
  const _NetworkPasswordTextButton({
    required this.label,
    required this.onPressed,
  });

  final String label;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return HyprCommandButton(
      label: label,
      onPressed: onPressed,
      padding: const EdgeInsets.symmetric(vertical: 3),
      constraints: const BoxConstraints(minHeight: 18),
      foregroundColor: HyprColors.textFaint,
      hoverForegroundColor: HyprColors.textMuted,
      hoverColor: Colors.transparent,
      textStyle: HyprTypography.compactMonoStrong.copyWith(
        fontSize: HyprTypography.size(10),
      ),
      maxLines: 1,
    );
  }
}
