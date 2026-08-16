import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:rinf/rinf.dart';

import 'src/bindings/bindings.dart';
import 'src/hyprbaric.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await initializeRust(assignRustSignal);
  runApp(const ProviderScope(child: Hyprbaric()));
}
