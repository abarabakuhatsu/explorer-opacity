[English](../../../README.md) | [日本語](../ja/README.md) | [简体中文](./README.md)

# explorer-opacity

在 Windows 上让文件资源管理器（Explorer）窗口保持半透明的轻量常驻工具。
仅针对 `CabinetWClass` 窗口，不影响桌面和任务栏。
使用 Rust 编写的单一便携式可执行文件，无运行时依赖。

## 安装

```powershell
cargo build --release
```

- `cargo build --release` — 构建发行版可执行文件到 `target\release\explorer-opacity.exe`。

## 使用方法

```powershell
explorer-opacity.exe
explorer-opacity.exe status
explorer-opacity.exe install-autostart
```

- `explorer-opacity.exe` — 启动常驻工具（首次运行会生成 `explorer-opacity.toml`）。
- `explorer-opacity.exe status` — 显示自动启动、日志和配置路径。
- `explorer-opacity.exe install-autostart` — 注册为 Windows 启动时自动运行。

## 许可

MIT — 详见 [LICENSE](../../../LICENSE)。
