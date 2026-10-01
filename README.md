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

## Install

```powershell
cargo build --release
```

- `cargo build --release` — build the release binary to `target\release\explorer-opacity.exe`.

## Usage

```powershell
explorer-opacity.exe
explorer-opacity.exe status
explorer-opacity.exe install-autostart
```

- `explorer-opacity.exe` — start the resident tool (creates `explorer-opacity.toml` on first run).
- `explorer-opacity.exe status` — show autostart, log, and config paths.
- `explorer-opacity.exe install-autostart` — start the tool automatically with Windows.

## Background

Design decisions live in [`docs/adr/`](docs/adr/).

## License

MIT — see [LICENSE](./LICENSE).
