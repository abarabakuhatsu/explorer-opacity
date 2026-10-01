<p align="center">
  <a href="https://github.com/abarabakuhatsu/explorer-opacity/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/abarabakuhatsu/explorer-opacity/ci.yml?branch=main&style=flat&logo=github"></a>
  <a href="./LICENSE"><img alt="License: MIT" src="https://img.shields.io/github/license/abarabakuhatsu/explorer-opacity?style=flat"></a>
  <img alt="Platform: Windows 10 / 11" src="https://img.shields.io/badge/platform-Windows%2010%20%7C%2011-0078D6?style=flat&logo=windows&logoColor=white">
  <img alt="Rust 2021 edition" src="https://img.shields.io/badge/rust-2021%20edition-000000?style=flat&logo=rust">
</p>

[English](./README.md) | [日本語](./docs/readme/ja/README.md) | [简体中文](./docs/readme/zh/README.md)

# explorer-opacity

A lightweight resident tool that keeps File Explorer windows translucent on Windows.
Only `CabinetWClass` windows are targeted; the desktop and taskbar are left untouched.
It is a single portable executable written in Rust, with no runtime dependency.

## Download

Get the latest `explorer-opacity-<version>-windows-x64.zip` from the
[Releases](https://github.com/abarabakuhatsu/explorer-opacity/releases/latest) page,
unzip it, and run `explorer-opacity.exe`. Verify the download against the attached
`SHA256SUMS.txt`:

```powershell
Get-FileHash .\explorer-opacity-<version>-windows-x64.zip -Algorithm SHA256
```

## Build from source

Requires Rust (`x86_64-pc-windows-msvc`) and an MSVC linker (or MinGW with the `-gnu` target).

```powershell
cargo build --release
```

- `cargo build --release` — build the release binary to `target\release\explorer-opacity.exe`.

## Usage

```powershell
explorer-opacity.exe
explorer-opacity.exe status
explorer-opacity.exe install-autostart
explorer-opacity.exe uninstall-autostart
```

- `explorer-opacity.exe` — start the resident tool (creates `explorer-opacity.toml` on first run).
- `explorer-opacity.exe status` — show autostart, log, and config paths.
- `explorer-opacity.exe install-autostart` — start the tool automatically with Windows.
- `explorer-opacity.exe uninstall-autostart` — remove the autostart entry.

## Configuration

Settings live in `explorer-opacity.toml` next to the executable (created on first run).

```toml
config_version = 1
enabled = true          # false disables transparency
opacity = 85            # 10-100 (%); lower values reduce text legibility
opacity_step = 5        # step for the hotkeys (1-50)

[hotkeys]
toggle   = "Ctrl+Alt+T"
increase = "Ctrl+Alt+Up"
decrease = "Ctrl+Alt+Down"

[targets]
include = ["CabinetWClass"]
exclude = ["Progman", "WorkerW", "Shell_TrayWnd", "TaskManagerWindow"]

[logging]
enabled = true          # false writes no log file
path = ""               # empty = <exe dir>\explorer-opacity.log
```

- `opacity` is clamped to 10-100 and `opacity_step` to 1-50.
- Class matching: `exclude` always wins; if `include` is non-empty only those match; if both are empty the defaults are restored.
- `logging.path`: relative paths are relative to the exe, `%VAR%` is expanded, and a trailing separator or an existing directory appends `explorer-opacity.log`.
- Changes apply via the tray "Reload config" or a restart. An invalid config falls back to defaults and is recorded in the log.

## Hotkeys

- `Ctrl+Alt+T` — toggle transparency on/off.
- `Ctrl+Alt+Up` — increase opacity.
- `Ctrl+Alt+Down` — decrease opacity.

## Tray menu

Enabled / Increase opacity / Decrease opacity / Reload config / Start with Windows / Open log (hidden when logging is disabled) / Exit.

## Troubleshooting

- **SmartScreen warning**: the executable is unsigned, so Windows may warn. Choose "More info" → "Run anyway".
- **Nothing becomes translucent**: only `CabinetWClass` (File Explorer) windows are targeted. Check `enabled`/`opacity` and run `explorer-opacity.exe status`.
- **No log file**: the exe folder may be read-only (e.g. Program Files). Set `[logging] path` to a writable location.
- **Left translucent after a crash**: force-killing the process skips restore. Restart Explorer to reset the affected windows.
- **Hotkeys do nothing**: another app may use the same chord; change them in the config.

## Known limitations

- Text is translucent too (at the default 85% it stays legible).
- The background is not blurred; it simply shows through.
- Windows x64 only.
- The executable is unsigned.

## Uninstall

1. Exit from the tray menu (this restores the windows).
2. Run `explorer-opacity.exe uninstall-autostart`.
3. Delete `explorer-opacity.exe`, `explorer-opacity.toml`, and the log file.

## Links

- [Releases](https://github.com/abarabakuhatsu/explorer-opacity/releases)
- [Design decisions (docs/adr/)](docs/adr/)
- [Issues](https://github.com/abarabakuhatsu/explorer-opacity/issues)

## License

MIT — see [LICENSE](./LICENSE).
