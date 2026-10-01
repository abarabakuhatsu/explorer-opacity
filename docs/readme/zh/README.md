[English](../../../README.md) | [日本語](../ja/README.md) | [简体中文](./README.md)

# explorer-opacity

在 Windows 上让文件资源管理器（Explorer）窗口保持半透明的轻量常驻工具。
仅针对 `CabinetWClass` 窗口，不影响桌面和任务栏。
使用 Rust 编写的单一便携式可执行文件，无运行时依赖。

## 下载

从 [Releases](https://github.com/abarabakuhatsu/explorer-opacity/releases/latest) 获取最新的
`explorer-opacity-<version>-windows-x64.zip`，解压后运行 `explorer-opacity.exe`。
可用随附的 `SHA256SUMS.txt` 校验下载文件：

```powershell
Get-FileHash .\explorer-opacity-<version>-windows-x64.zip -Algorithm SHA256
```

## 从源码构建

需要 Rust（`x86_64-pc-windows-msvc`）和 MSVC 链接器（或 MinGW 与 `-gnu` target）。

```powershell
cargo build --release
```

- `cargo build --release` — 构建发行版可执行文件到 `target\release\explorer-opacity.exe`。

## 使用方法

```powershell
explorer-opacity.exe
explorer-opacity.exe status
explorer-opacity.exe install-autostart
explorer-opacity.exe uninstall-autostart
explorer-opacity.exe restore
```

- `explorer-opacity.exe` — 启动常驻工具（首次运行会生成 `explorer-opacity.toml`）。
- `explorer-opacity.exe status` — 显示自动启动、日志和配置路径。
- `explorer-opacity.exe install-autostart` — 注册为 Windows 启动时自动运行。
- `explorer-opacity.exe uninstall-autostart` — 移除自动启动项。
- `explorer-opacity.exe restore` — 清除资源管理器残留的透明效果（恢复用）。

## 配置

配置文件为可执行文件旁的 `explorer-opacity.toml`（首次运行时生成）。

```toml
config_version = 1
enabled = true          # false 会禁用透明化
opacity = 85            # 10-100 (%)；越低文字越难读
opacity_step = 5        # 快捷键的调整步长（1-50）

[hotkeys]
toggle   = "Ctrl+Alt+T"
increase = "Ctrl+Alt+Up"
decrease = "Ctrl+Alt+Down"

[targets]
include = ["CabinetWClass"]
exclude = ["Progman", "WorkerW", "Shell_TrayWnd", "TaskManagerWindow"]

[logging]
enabled = true          # false 不写日志文件
path = ""               # 空 = 可执行文件旁的 explorer-opacity.log
```

- `opacity` 会被限制在 10–100，`opacity_step` 在 1–50。
- 类名匹配：`exclude` 优先；`include` 非空时只匹配其中列出的；两者都为空时恢复默认。
- `logging.path`：相对路径相对于可执行文件；展开 `%VAR%`；以分隔符结尾或已存在的目录会在其下追加 `explorer-opacity.log`。
- 通过托盘 “Reload config” 或重启生效。配置无效时使用默认值并记录到日志。

## 快捷键

- `Ctrl+Alt+T` — 开关透明化。
- `Ctrl+Alt+Up` — 提高不透明度。
- `Ctrl+Alt+Down` — 降低不透明度。

## 托盘菜单

Enabled / Increase opacity / Decrease opacity / Reload config / Start with Windows / Open log（禁用日志时隐藏）/ Exit

## 故障排查

- **SmartScreen 警告**：可执行文件未签名，Windows 可能提示。选择“更多信息”→“仍要运行”。
- **没有变透明**：仅针对 `CabinetWClass`（文件资源管理器）窗口。检查 `enabled`/`opacity`，并运行 `explorer-opacity.exe status`。
- **没有日志文件**：可执行文件所在目录可能不可写（如 Program Files）。将 `[logging] path` 设为可写位置。
- **崩溃后仍为半透明**：运行 `explorer-opacity.exe restore`（或重启 Explorer）以恢复。
- **快捷键无效**：可能与其他应用冲突。在配置中修改。

## 已知限制

- 文字也会半透明（默认 85% 时仍可读）。
- 背景不会被模糊，只是直接透出。
- 仅支持 Windows x64。
- 可执行文件未签名。

## 卸载

1. 从托盘菜单选择 Exit（还原窗口）。
2. 运行 `explorer-opacity.exe uninstall-autostart`。
3. 删除 `explorer-opacity.exe`、`explorer-opacity.toml` 和日志文件。

## 链接

- [Releases](https://github.com/abarabakuhatsu/explorer-opacity/releases)
- [设计决策 (docs/adr/)](../../adr/)
- [Issues](https://github.com/abarabakuhatsu/explorer-opacity/issues)

## 许可

MIT — 详见 [LICENSE](../../../LICENSE)。
