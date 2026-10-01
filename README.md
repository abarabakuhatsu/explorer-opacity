[English](./README.md) | [日本語](./docs/readme/ja/README.md) | [繁體中文](./docs/readme/zh/README.md)

<p align="center">
  <a href="https://github.com/abarabakuhatsu/explorer-opacity/actions/workflows/ci.yml"><img alt="CI" src="https://img.shields.io/github/actions/workflow/status/abarabakuhatsu/explorer-opacity/ci.yml?branch=main&style=flat&logo=github"></a>
  <a href="./LICENSE"><img alt="License: MIT" src="https://img.shields.io/github/license/abarabakuhatsu/explorer-opacity?style=flat"></a>
  <img alt="Platform: Windows 10 / 11" src="https://img.shields.io/badge/platform-Windows%2010%20%7C%2011-0078D6?style=flat&logo=windows&logoColor=white">
  <img alt="Rust 2021 edition" src="https://img.shields.io/badge/rust-2021%20edition-000000?style=flat&logo=rust">
  <img alt="Last commit" src="https://img.shields.io/github/last-commit/abarabakuhatsu/explorer-opacity?style=flat">
</p>

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
