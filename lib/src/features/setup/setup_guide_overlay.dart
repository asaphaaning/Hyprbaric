import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../bindings/bindings.dart';
import '../../layer_shell_controller.dart';
import '../../layer_shell_hit_region.dart';
import '../../state/providers.dart';
import '../../widgets/hypr_surface.dart';
import 'setup_guide_state.dart';

/// The split-stage setup guide from the v6 product prototype.
class SetupGuideOverlay extends ConsumerStatefulWidget {
  const SetupGuideOverlay({
    super.key,
    required this.launch,
    required this.onFinished,
    required this.onSkipped,
  });

  final SetupLaunch launch;
  final VoidCallback onFinished;
  final VoidCallback onSkipped;

  @override
  ConsumerState<SetupGuideOverlay> createState() => _SetupGuideOverlayState();
}

class _SetupGuideOverlayState extends ConsumerState<SetupGuideOverlay> {
  static const String _regionOwner = 'setup-guide';
  static const List<int> _accentPresets = <int>[
    197,
    238,
    275,
    310,
    345,
    25,
    70,
    145,
  ];

  final FocusNode _focusNode = FocusNode(debugLabel: 'setup-guide');
  late final LayerShellController _layerShellController;
  late final LayerShellRegionManager _regionManager;
  SetupStep _step = SetupStep.welcome;

  @override
  void initState() {
    super.initState();
    _layerShellController = ref.read(layerShellControllerProvider);
    _regionManager = ref.read(layerShellRegionManagerProvider);
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) {
        _focusNode.requestFocus();
        unawaited(
          _layerShellController.setKeyboardMode(
            LayerShellKeyboardMode.exclusive,
          ),
        );
        _updateRegion();
      }
    });
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    WidgetsBinding.instance.addPostFrameCallback((_) => _updateRegion());
  }

  @override
  void dispose() {
    unawaited(
      _layerShellController.setKeyboardMode(LayerShellKeyboardMode.none),
    );
    unawaited(
      _regionManager.removePassiveRegions(
        owner: _regionOwner,
        debugLabel: 'setup-guide-close',
      ),
    );
    _focusNode.dispose();
    super.dispose();
  }

  void _updateRegion() {
    if (!mounted) {
      return;
    }
    final Size size = MediaQuery.sizeOf(context);
    unawaited(
      _regionManager.setPassiveRegions(
        owner: _regionOwner,
        regions: <LayerShellMenuRegion>[
          LayerShellMenuRegion(
            rect: Offset.zero & size,
            radius: BorderRadius.zero,
          ),
        ],
        debugLabel: 'setup-guide-open',
      ),
    );
  }

  void _go(SetupStep step) {
    ref.read(appearancePreviewProvider.notifier).clear();
    setState(() => _step = step);
  }

  void _next() {
    final int index = SetupStep.sequence.indexOf(_step);
    if (index == SetupStep.sequence.length - 1) {
      widget.onFinished();
      return;
    }
    _go(SetupStep.sequence[index + 1]);
  }

  void _back() {
    final int index = SetupStep.sequence.indexOf(_step);
    if (index > 0) {
      _go(SetupStep.sequence[index - 1]);
    }
  }

  void _setOpacity(int value) {
    ref.read(appearanceControllerProvider.notifier).setOpacity(value);
    ref.read(appearancePreviewProvider.notifier).clear();
  }

  void _previewOpacity(int value) {
    ref
        .read(appearancePreviewProvider.notifier)
        .preview(ref.read(currentAppearanceProvider).copyWith(opacity: value));
  }

  void _setAccent(int value) {
    ref.read(appearanceControllerProvider.notifier).setAccentHue(value);
    ref.read(appearancePreviewProvider.notifier).clear();
  }

  void _previewAccent(int value) {
    ref
        .read(appearancePreviewProvider.notifier)
        .preview(
          ref.read(currentAppearanceProvider).copyWith(accentHue: value),
        );
  }

  @override
  Widget build(BuildContext context) {
    final AppearanceStatus appearance = ref.watch(currentAppearanceProvider);
    final WorkspaceSettingsStatus workspaces = ref.watch(
      currentWorkspaceSettingsProvider,
    );

    return Positioned.fill(
      child: Focus(
        autofocus: true,
        focusNode: _focusNode,
        child: ColoredBox(
          color: const Color(0xB20A0C12),
          child: Center(
            child: LayoutBuilder(
              builder: (BuildContext context, BoxConstraints constraints) {
                final double width = constraints.maxWidth.clamp(660, 980);
                final double height = constraints.maxHeight.clamp(500, 600);
                return SizedBox(
                  key: const ValueKey<String>('setup-guide'),
                  width: width - 48,
                  height: height - 40,
                  child: HyprSurface(
                    borderRadius: BorderRadius.circular(22),
                    color: const Color(0xF1272B35),
                    borderColor: HyprColors.popupStroke,
                    frame: HyprSurfaceFrame.popover,
                    shadow: true,
                    child: Row(
                      children: <Widget>[
                        Expanded(
                          flex: 45,
                          child: _PreviewStage(
                            step: _step,
                            appearance: appearance,
                            workspaces: workspaces,
                          ),
                        ),
                        Container(
                          width: 1,
                          color: context.hyprPalette.borderSoft,
                        ),
                        Expanded(
                          flex: 55,
                          child: _ControlPane(
                            step: _step,
                            appearance: appearance,
                            workspaces: workspaces,
                            accentPresets: _accentPresets,
                            onStepSelected: _go,
                            onBack: _back,
                            onNext: _next,
                            onSkip: widget.onSkipped,
                            onOpacityPreview: _previewOpacity,
                            onOpacityCommitted: _setOpacity,
                            onAccentPreview: _previewAccent,
                            onAccentCommitted: _setAccent,
                            onPositionChanged: (AppearancePosition position) {
                              ref
                                  .read(appearanceControllerProvider.notifier)
                                  .setPosition(position);
                            },
                            onWorkspaceStyleChanged:
                                (WorkspaceIndicatorStyle style) {
                                  ref
                                      .read(
                                        workspaceSettingsControllerProvider
                                            .notifier,
                                      )
                                      .setIndicatorStyle(style);
                                },
                          ),
                        ),
                      ],
                    ),
                  ),
                );
              },
            ),
          ),
        ),
      ),
    );
  }
}

class _PreviewStage extends StatelessWidget {
  const _PreviewStage({
    required this.step,
    required this.appearance,
    required this.workspaces,
  });

  final SetupStep step;
  final AppearanceStatus appearance;
  final WorkspaceSettingsStatus workspaces;

  @override
  Widget build(BuildContext context) {
    final String heading = switch (step) {
      SetupStep.welcome => 'Your bar, right now',
      SetupStep.transparency =>
        appearance.opacity == 100 ? 'Flat matte' : 'Translucent',
      SetupStep.accent => 'One accent bus',
      SetupStep.layout =>
        appearance.position == AppearancePosition.bottom
            ? 'Docked bottom'
            : 'Docked top',
    };
    final String detail = switch (step) {
      SetupStep.welcome => 'Everything here updates as you choose.',
      SetupStep.transparency =>
        appearance.opacity == 100
            ? 'An opaque plate with no wallpaper read-through.'
            : 'Wallpaper and compositor blur can read through.',
      SetupStep.accent => 'One hue drives every active state.',
      SetupStep.layout => 'Placement and workspace labels.',
    };

    return ClipRRect(
      borderRadius: const BorderRadius.horizontal(left: Radius.circular(22)),
      child: DecoratedBox(
        decoration: BoxDecoration(
          gradient: RadialGradient(
            center: const Alignment(-0.7, -0.8),
            radius: 1.7,
            colors: <Color>[
              context.hyprPalette.accent.withValues(alpha: 0.18),
              const Color(0xFF171B24),
              const Color(0xFF10131A),
            ],
          ),
        ),
        child: Padding(
          padding: const EdgeInsets.fromLTRB(26, 26, 34, 22),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: <Widget>[
              Row(
                children: <Widget>[
                  Container(
                    width: 9,
                    height: 2,
                    color: context.hyprPalette.accent,
                  ),
                  const SizedBox(width: 8),
                  Text(
                    'LIVE PREVIEW',
                    style: HyprTypography.compactMonoStrong.copyWith(
                      color: context.hyprPalette.accentSoft,
                      fontSize: 9,
                      letterSpacing: 1.6,
                    ),
                  ),
                ],
              ),
              const Spacer(),
              _MiniDesktop(appearance: appearance),
              const SizedBox(height: 14),
              _PreviewMeter(step: step, appearance: appearance),
              const Spacer(),
              Text(
                heading,
                style: HyprTypography.popRow.copyWith(
                  fontWeight: FontWeight.w700,
                  fontSize: 14,
                ),
              ),
              const SizedBox(height: 4),
              Text(
                detail,
                style: HyprTypography.popRow.copyWith(
                  color: HyprColors.textFaint,
                  fontSize: 11,
                ),
              ),
              const SizedBox(height: 12),
              _WorkspacePreview(style: workspaces.indicatorStyle),
            ],
          ),
        ),
      ),
    );
  }
}

class _MiniDesktop extends StatelessWidget {
  const _MiniDesktop({required this.appearance});

  final AppearanceStatus appearance;

  @override
  Widget build(BuildContext context) {
    final Alignment alignment = appearance.position == AppearancePosition.bottom
        ? Alignment.bottomCenter
        : Alignment.topCenter;
    return Container(
      height: 138,
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(12),
        gradient: const LinearGradient(
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
          colors: <Color>[Color(0xFF314462), Color(0xFF432E58)],
        ),
        border: Border.all(color: HyprColors.popupStroke),
      ),
      child: Stack(
        children: <Widget>[
          Positioned(
            left: 18,
            top: 42,
            child: Container(
              width: 125,
              height: 68,
              decoration: BoxDecoration(
                color: const Color(0x9912151D),
                borderRadius: BorderRadius.circular(7),
                border: Border.all(color: HyprColors.popupStroke),
              ),
            ),
          ),
          Align(
            alignment: alignment,
            child: Padding(
              padding: const EdgeInsets.all(9),
              child: Container(
                height: 23,
                decoration: BoxDecoration(
                  color: const Color(
                    0xFF20242E,
                  ).withValues(alpha: appearance.opacity / 100),
                  borderRadius: BorderRadius.circular(7),
                  border: Border.all(color: HyprColors.popupStroke),
                ),
                child: Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 8),
                  child: Row(
                    children: <Widget>[
                      Container(
                        width: 14,
                        height: 4,
                        decoration: BoxDecoration(
                          color: context.hyprPalette.accent,
                          borderRadius: BorderRadius.circular(2),
                        ),
                      ),
                      const SizedBox(width: 5),
                      for (int index = 0; index < 3; index++) ...<Widget>[
                        const CircleAvatar(
                          radius: 2,
                          backgroundColor: HyprColors.textFaint,
                        ),
                        const SizedBox(width: 4),
                      ],
                      const Spacer(),
                      Container(
                        width: 34,
                        height: 8,
                        decoration: BoxDecoration(
                          color: HyprColors.textFaint.withValues(alpha: 0.35),
                          borderRadius: BorderRadius.circular(3),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _PreviewMeter extends StatelessWidget {
  const _PreviewMeter({required this.step, required this.appearance});

  final SetupStep step;
  final AppearanceStatus appearance;

  @override
  Widget build(BuildContext context) {
    final double value = step == SetupStep.transparency
        ? appearance.opacity / 100
        : appearance.accentHue / 359;
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: Colors.black.withValues(alpha: 0.18),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: HyprColors.popupStroke),
      ),
      child: Column(
        children: <Widget>[
          Row(
            children: <Widget>[
              Text(
                step == SetupStep.transparency ? 'OPACITY' : 'ACCENT',
                style: HyprTypography.compactMonoStrong.copyWith(
                  color: HyprColors.textFaint,
                  fontSize: 9,
                  letterSpacing: 1.4,
                ),
              ),
              const Spacer(),
              Text(
                step == SetupStep.transparency
                    ? '${appearance.opacity}%'
                    : '${appearance.accentHue}°',
                style: HyprTypography.compactMonoStrong.copyWith(fontSize: 9),
              ),
            ],
          ),
          const SizedBox(height: 9),
          ClipRRect(
            borderRadius: BorderRadius.circular(3),
            child: LinearProgressIndicator(
              value: value,
              minHeight: 5,
              backgroundColor: Colors.black.withValues(alpha: 0.35),
              color: context.hyprPalette.accent,
            ),
          ),
        ],
      ),
    );
  }
}

class _WorkspacePreview extends StatelessWidget {
  const _WorkspacePreview({required this.style});

  final WorkspaceIndicatorStyle style;

  @override
  Widget build(BuildContext context) {
    const List<String> roman = <String>['I', 'II', 'III', 'IV', 'V'];
    return Row(
      children: List<Widget>.generate(5, (int index) {
        final bool active = index == 0;
        final bool occupied = index == 1 || index == 3;
        return Container(
          width: 31,
          height: 18,
          margin: const EdgeInsets.only(right: 5),
          decoration: BoxDecoration(
            color: active ? context.hyprPalette.fillStrong : Colors.transparent,
            borderRadius: BorderRadius.circular(6),
            border: active
                ? Border.all(color: context.hyprPalette.borderSoft)
                : null,
          ),
          child: Stack(
            alignment: Alignment.center,
            clipBehavior: Clip.none,
            children: <Widget>[
              Text(
                style == WorkspaceIndicatorStyle.roman
                    ? roman[index]
                    : '${index + 1}',
                style: HyprTypography.workspace.copyWith(
                  color: active
                      ? context.hyprPalette.accentSoft
                      : HyprColors.textFaint,
                  fontSize: 9,
                ),
              ),
              if (occupied && !active)
                Positioned(
                  bottom: -1,
                  child: CircleAvatar(
                    radius: 1.5,
                    backgroundColor: context.hyprPalette.accentSoft,
                  ),
                ),
            ],
          ),
        );
      }),
    );
  }
}

class _ControlPane extends StatelessWidget {
  const _ControlPane({
    required this.step,
    required this.appearance,
    required this.workspaces,
    required this.accentPresets,
    required this.onStepSelected,
    required this.onBack,
    required this.onNext,
    required this.onSkip,
    required this.onOpacityPreview,
    required this.onOpacityCommitted,
    required this.onAccentPreview,
    required this.onAccentCommitted,
    required this.onPositionChanged,
    required this.onWorkspaceStyleChanged,
  });

  final SetupStep step;
  final AppearanceStatus appearance;
  final WorkspaceSettingsStatus workspaces;
  final List<int> accentPresets;
  final ValueChanged<SetupStep> onStepSelected;
  final VoidCallback onBack;
  final VoidCallback onNext;
  final VoidCallback onSkip;
  final ValueChanged<int> onOpacityPreview;
  final ValueChanged<int> onOpacityCommitted;
  final ValueChanged<int> onAccentPreview;
  final ValueChanged<int> onAccentCommitted;
  final ValueChanged<AppearancePosition> onPositionChanged;
  final ValueChanged<WorkspaceIndicatorStyle> onWorkspaceStyleChanged;

  @override
  Widget build(BuildContext context) {
    final int index = SetupStep.sequence.indexOf(step);
    return Padding(
      padding: const EdgeInsets.fromLTRB(34, 26, 34, 22),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: <Widget>[
          Text(
            '${(index + 1).toString().padLeft(2, '0')} — ${step.label.toUpperCase()}',
            style: HyprTypography.compactMonoStrong.copyWith(
              color: HyprColors.textFaint,
              fontSize: 9,
              letterSpacing: 1.5,
            ),
          ),
          const SizedBox(height: 18),
          Text(
            _title(step),
            style: HyprTypography.settingHeading.copyWith(
              fontSize: 25,
              height: 1.15,
            ),
          ),
          const SizedBox(height: 10),
          Text(
            _subtitle(step),
            style: HyprTypography.popRow.copyWith(
              color: HyprColors.textMuted,
              fontSize: 12.5,
              height: 1.55,
            ),
          ),
          const SizedBox(height: 24),
          Expanded(child: _controls(context)),
          const SizedBox(height: 18),
          Row(
            children: <Widget>[
              _Progress(active: step, onSelected: onStepSelected),
              const Spacer(),
              _GuideButton(
                label: index == 0 ? 'Skip' : 'Back',
                quiet: true,
                onPressed: index == 0 ? onSkip : onBack,
              ),
              const SizedBox(width: 12),
              _GuideButton(
                label: index == SetupStep.sequence.length - 1
                    ? 'Finish'
                    : index == 0
                    ? 'Get started'
                    : 'Continue',
                onPressed: onNext,
              ),
            ],
          ),
        ],
      ),
    );
  }

  Widget _controls(BuildContext context) {
    return switch (step) {
      SetupStep.welcome => const _FeatureList(),
      SetupStep.transparency => Column(
        children: <Widget>[
          Row(
            children: <Widget>[
              Expanded(
                child: _ChoiceCard(
                  title: 'Frosted glass',
                  subtitle:
                      'Translucent depth; Hyprland blur can read through.',
                  selected: appearance.opacity < 100,
                  icon: Icons.blur_on_rounded,
                  onPressed: () => onOpacityCommitted(77),
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: _ChoiceCard(
                  title: 'Flat matte',
                  subtitle: 'Fully opaque and crisp on low-power hardware.',
                  selected: appearance.opacity == 100,
                  icon: Icons.crop_square_rounded,
                  onPressed: () => onOpacityCommitted(100),
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          _SliderTray(
            title: 'Background opacity',
            subtitle: 'How much wallpaper shows through.',
            value: appearance.opacity.toDouble(),
            min: 20,
            max: 100,
            divisions: 80,
            valueLabel: '${appearance.opacity}%',
            onChanged: (double value) => onOpacityPreview(value.round()),
            onChangeEnd: (double value) => onOpacityCommitted(value.round()),
          ),
        ],
      ),
      SetupStep.accent => Column(
        children: <Widget>[
          Wrap(
            spacing: 9,
            runSpacing: 9,
            children: accentPresets
                .map((int hue) {
                  final Color color = HSLColor.fromAHSL(
                    1,
                    hue.toDouble(),
                    .86,
                    .56,
                  ).toColor();
                  return InkWell(
                    key: ValueKey<String>('setup-accent-$hue'),
                    onTap: () => onAccentCommitted(hue),
                    borderRadius: BorderRadius.circular(11),
                    child: AnimatedContainer(
                      duration: const Duration(milliseconds: 160),
                      width: 36,
                      height: 36,
                      decoration: BoxDecoration(
                        gradient: LinearGradient(
                          begin: Alignment.topLeft,
                          end: Alignment.bottomRight,
                          colors: <Color>[color.withValues(alpha: .95), color],
                        ),
                        borderRadius: BorderRadius.circular(11),
                        border: Border.all(
                          color: appearance.accentHue == hue
                              ? Colors.white.withValues(alpha: .85)
                              : Colors.white.withValues(alpha: .12),
                          width: appearance.accentHue == hue ? 2 : 1,
                        ),
                        boxShadow: appearance.accentHue == hue
                            ? <BoxShadow>[
                                BoxShadow(
                                  color: color.withValues(alpha: .45),
                                  blurRadius: 14,
                                ),
                              ]
                            : null,
                      ),
                    ),
                  );
                })
                .toList(growable: false),
          ),
          const SizedBox(height: 18),
          _SliderTray(
            title: 'Fine tune',
            subtitle: 'Anywhere on the colour wheel.',
            value: appearance.accentHue.toDouble(),
            min: 0,
            max: 359,
            divisions: 359,
            valueLabel: '${appearance.accentHue}°',
            onChanged: (double value) => onAccentPreview(value.round()),
            onChangeEnd: (double value) => onAccentCommitted(value.round()),
          ),
        ],
      ),
      SetupStep.layout => Column(
        children: <Widget>[
          Row(
            children: <Widget>[
              Expanded(
                child: _ChoiceCard(
                  title: 'Top',
                  subtitle: 'Classic panel placement.',
                  selected: appearance.position == AppearancePosition.top,
                  icon: Icons.vertical_align_top_rounded,
                  onPressed: () => onPositionChanged(AppearancePosition.top),
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: _ChoiceCard(
                  title: 'Bottom',
                  subtitle: 'Out of the way of titlebars.',
                  selected: appearance.position == AppearancePosition.bottom,
                  icon: Icons.vertical_align_bottom_rounded,
                  onPressed: () => onPositionChanged(AppearancePosition.bottom),
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),
          _SegmentTray(
            title: 'Workspace indicators',
            subtitle: 'Occupied workspaces keep their coloured dot.',
            children: <Widget>[
              _Segment(
                label: 'I · II',
                selected:
                    workspaces.indicatorStyle == WorkspaceIndicatorStyle.roman,
                onPressed: () =>
                    onWorkspaceStyleChanged(WorkspaceIndicatorStyle.roman),
              ),
              _Segment(
                label: '1 · 2',
                selected:
                    workspaces.indicatorStyle ==
                    WorkspaceIndicatorStyle.numeric,
                onPressed: () =>
                    onWorkspaceStyleChanged(WorkspaceIndicatorStyle.numeric),
              ),
            ],
          ),
        ],
      ),
    };
  }
}

String _title(SetupStep step) => switch (step) {
  SetupStep.welcome => 'Welcome to\nHyprbaric',
  SetupStep.transparency => 'Frosted, or flat?',
  SetupStep.accent => 'Pick an accent',
  SetupStep.layout => 'Where should it live?',
};

String _subtitle(SetupStep step) => switch (step) {
  SetupStep.welcome =>
    'Three quick choices and your bar is dressed. Every option also lives in Bar settings, so nothing here is permanent.',
  SetupStep.transparency =>
    'Transparency lets compositor blur show through. Flat matte is cheaper and stays crisp on low-power hardware.',
  SetupStep.accent =>
    'One hue drives glows, active states, meters, and highlights across the whole bar.',
  SetupStep.layout =>
    'Dock the bar, then choose how your workspace labels are written.',
};

class _FeatureList extends StatelessWidget {
  const _FeatureList();

  @override
  Widget build(BuildContext context) {
    return const Column(
      children: <Widget>[
        _Feature('Frosted glass or flat matte', 'your call'),
        SizedBox(height: 16),
        _Feature('One accent hue', 'drives every highlight'),
        SizedBox(height: 16),
        _Feature('Dock and label style', 'top or bottom'),
      ],
    );
  }
}

class _Feature extends StatelessWidget {
  const _Feature(this.title, this.detail);

  final String title;
  final String detail;

  @override
  Widget build(BuildContext context) {
    return Row(
      children: <Widget>[
        Container(width: 9, height: 2, color: context.hyprPalette.accent),
        const SizedBox(width: 11),
        Expanded(
          child: Text.rich(
            TextSpan(
              children: <InlineSpan>[
                TextSpan(text: title),
                TextSpan(
                  text: ' — $detail',
                  style: HyprTypography.popRow.copyWith(
                    color: HyprColors.textFaint,
                  ),
                ),
              ],
            ),
            maxLines: 2,
            overflow: TextOverflow.ellipsis,
            style: HyprTypography.popRow,
          ),
        ),
      ],
    );
  }
}

class _ChoiceCard extends StatelessWidget {
  const _ChoiceCard({
    required this.title,
    required this.subtitle,
    required this.selected,
    required this.icon,
    required this.onPressed,
  });

  final String title;
  final String subtitle;
  final bool selected;
  final IconData icon;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onPressed,
      borderRadius: BorderRadius.circular(13),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 160),
        height: 126,
        padding: const EdgeInsets.all(14),
        decoration: BoxDecoration(
          color: selected
              ? context.hyprPalette.fillStrong
              : Colors.black.withValues(alpha: .18),
          borderRadius: BorderRadius.circular(13),
          border: Border.all(
            color: selected
                ? context.hyprPalette.accentSoft.withValues(alpha: .65)
                : HyprColors.popupStroke,
            width: selected ? 2 : 1,
          ),
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: <Widget>[
            Icon(
              icon,
              size: 24,
              color: selected
                  ? context.hyprPalette.accentSoft
                  : HyprColors.textFaint,
            ),
            const Spacer(),
            Text(
              title,
              style: HyprTypography.popRow.copyWith(
                fontWeight: FontWeight.w700,
              ),
            ),
            const SizedBox(height: 4),
            Text(
              subtitle,
              maxLines: 2,
              style: HyprTypography.popRow.copyWith(
                color: HyprColors.textFaint,
                fontSize: 10.5,
                height: 1.35,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _SliderTray extends StatelessWidget {
  const _SliderTray({
    required this.title,
    required this.subtitle,
    required this.value,
    required this.min,
    required this.max,
    required this.divisions,
    required this.valueLabel,
    required this.onChanged,
    required this.onChangeEnd,
  });

  final String title;
  final String subtitle;
  final double value;
  final double min;
  final double max;
  final int divisions;
  final String valueLabel;
  final ValueChanged<double> onChanged;
  final ValueChanged<double> onChangeEnd;

  @override
  Widget build(BuildContext context) {
    return _Tray(
      child: Row(
        children: <Widget>[
          Expanded(
            child: _TrayCopy(title: title, subtitle: subtitle),
          ),
          SizedBox(
            width: 150,
            child: Slider(
              value: value.clamp(min, max),
              min: min,
              max: max,
              divisions: divisions,
              activeColor: context.hyprPalette.accent,
              onChanged: onChanged,
              onChangeEnd: onChangeEnd,
            ),
          ),
          SizedBox(
            width: 38,
            child: Text(
              valueLabel,
              textAlign: TextAlign.right,
              style: HyprTypography.compactMonoStrong.copyWith(fontSize: 10),
            ),
          ),
        ],
      ),
    );
  }
}

class _SegmentTray extends StatelessWidget {
  const _SegmentTray({
    required this.title,
    required this.subtitle,
    required this.children,
  });

  final String title;
  final String subtitle;
  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    return _Tray(
      child: Row(
        children: <Widget>[
          Expanded(
            child: _TrayCopy(title: title, subtitle: subtitle),
          ),
          ...children,
        ],
      ),
    );
  }
}

class _Tray extends StatelessWidget {
  const _Tray({required this.child});

  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 15, vertical: 14),
      decoration: BoxDecoration(
        color: Colors.black.withValues(alpha: .2),
        borderRadius: BorderRadius.circular(11),
        border: Border.all(color: HyprColors.popupStroke),
      ),
      child: child,
    );
  }
}

class _TrayCopy extends StatelessWidget {
  const _TrayCopy({required this.title, required this.subtitle});

  final String title;
  final String subtitle;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: <Widget>[
        Text(title, style: HyprTypography.popRow),
        const SizedBox(height: 4),
        Text(
          subtitle,
          style: HyprTypography.popRow.copyWith(
            color: HyprColors.textFaint,
            fontSize: 10.5,
          ),
        ),
      ],
    );
  }
}

class _Segment extends StatelessWidget {
  const _Segment({
    required this.label,
    required this.selected,
    required this.onPressed,
  });

  final String label;
  final bool selected;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(left: 5),
      child: _GuideButton(label: label, quiet: !selected, onPressed: onPressed),
    );
  }
}

class _Progress extends StatelessWidget {
  const _Progress({required this.active, required this.onSelected});

  final SetupStep active;
  final ValueChanged<SetupStep> onSelected;

  @override
  Widget build(BuildContext context) {
    final int activeIndex = SetupStep.sequence.indexOf(active);
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 7, vertical: 6),
      decoration: BoxDecoration(
        color: Colors.black.withValues(alpha: .25),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Row(
        children: SetupStep.sequence
            .map((SetupStep step) {
              final int index = SetupStep.sequence.indexOf(step);
              return InkWell(
                onTap: () => onSelected(step),
                child: AnimatedContainer(
                  duration: const Duration(milliseconds: 220),
                  width: index == activeIndex ? 18 : 6,
                  height: 6,
                  margin: const EdgeInsets.symmetric(horizontal: 3),
                  decoration: BoxDecoration(
                    color: index == activeIndex
                        ? context.hyprPalette.accent
                        : index < activeIndex
                        ? HyprColors.textFaint
                        : HyprColors.popupStroke,
                    borderRadius: BorderRadius.circular(3),
                  ),
                ),
              );
            })
            .toList(growable: false),
      ),
    );
  }
}

class _GuideButton extends StatelessWidget {
  const _GuideButton({
    required this.label,
    required this.onPressed,
    this.quiet = false,
  });

  final String label;
  final VoidCallback onPressed;
  final bool quiet;

  @override
  Widget build(BuildContext context) {
    return FilledButton(
      onPressed: onPressed,
      style: FilledButton.styleFrom(
        minimumSize: const Size(0, 34),
        padding: const EdgeInsets.symmetric(horizontal: 15),
        backgroundColor: quiet
            ? const Color(0xFF343945)
            : context.hyprPalette.accent.withValues(alpha: .75),
        foregroundColor: quiet ? HyprColors.textMuted : Colors.white,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(9),
          side: BorderSide(
            color: quiet
                ? HyprColors.popupStroke
                : context.hyprPalette.accentSoft.withValues(alpha: .35),
          ),
        ),
        textStyle: HyprTypography.compactMonoStrong.copyWith(
          fontSize: 9.5,
          letterSpacing: 1.1,
        ),
      ),
      child: Text(label.toUpperCase()),
    );
  }
}
