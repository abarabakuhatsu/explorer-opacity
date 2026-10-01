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

## 背景

设计决策请见 [`docs/adr/`](../../adr/)。

## 组件

| 组件 | 职责 | 文档 |
|---|---|---|
| `explorer-opacity.exe` | 常驻主体（系统托盘、快捷键、shell hook、消息循环） | [ADR-0002](../../adr/0002-event-driven-vs-polling.md) |
| `window.rs` | 窗口枚举与 layered alpha 的应用/还原 | [ADR-0001](../../adr/0001-alpha-vs-backdrop.md) |
| `config.rs` | `explorer-opacity.toml` 的读取与校验 | [ADR-0006](../../adr/0006-logging-config.md) |
| `logging.rs` | 可选的日志文件，保存在可执行文件旁 | [ADR-0006](../../adr/0006-logging-config.md) |
| `autostart.rs` | 通过 `HKCU\...\Run` 的用户级自动启动 | |

## 许可

MIT — 详见 [LICENSE](../../../LICENSE)。
