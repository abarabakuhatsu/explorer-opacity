[English](./README.md) | [日本語](./docs/readme/ja/README.md) | [繁體中文](./docs/readme/zh/README.md)

# explorer-opacity

A lightweight resident tool that keeps File Explorer windows translucent on Windows.
Only `CabinetWClass` windows are targeted; the desktop and taskbar are left untouched.
It is a single portable executable written in Rust, with no runtime dependency.

## Components

| Component | Scope | Docs |
|---|---|---|
| `explorer-opacity.exe` | Resident tool: tray, hotkeys, shell hook, message loop | [ADR-0002](docs/adr/0002-event-driven-vs-polling.md) |
| `window.rs` | Window enumeration and layered-alpha apply/restore | [ADR-0001](docs/adr/0001-alpha-vs-backdrop.md) |
| `config.rs` | `explorer-opacity.toml` parsing and validation | [ADR-0006](docs/adr/0006-logging-config.md) |
| `logging.rs` | Optional file logging, stored next to the exe | [ADR-0006](docs/adr/0006-logging-config.md) |
| `autostart.rs` | Per-user autostart via `HKCU\...\Run` | |

## Install

```powershell
cargo build --release
# output: target\release\explorer-opacity.exe
```

## Usage

```powershell
explorer-opacity.exe                 # start resident (creates explorer-opacity.toml)
explorer-opacity.exe status
explorer-opacity.exe install-autostart
```

## Background

Design decisions live in [`docs/adr/`](docs/adr/).

## License

MIT — see [LICENSE](./LICENSE).
