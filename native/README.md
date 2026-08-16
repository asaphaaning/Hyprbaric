# Rust Crates

This folder contains Rust crates integrated into the Flutter app by the [Rinf](https://rinf.cunarist.com) framework.

- `hub` is the thin RINF bridge crate. Keep its crate name and location at `native/hub`; RINF compilation presets expect that entry shape.
- `hyprbaric` is the native runtime crate. Put backend subsystem modules, typed signals, supervision, and runtime configuration there unless a change is specifically about the RINF bridge.
- Other native crates may be added when a domain split materially improves the backend model.
