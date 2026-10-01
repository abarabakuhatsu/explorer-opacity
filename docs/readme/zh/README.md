[English](../../../README.md) | [日本語](../ja/README.md) | [繁體中文](./README.md)

# explorer-opacity

在 Windows 上讓檔案總管（Explorer）視窗保持半透明的輕量常駐工具。
僅針對 `CabinetWClass` 視窗，不會影響桌面與工作列。
以 Rust 撰寫的單一可攜式執行檔，無需執行階段相依。

## 安裝

```powershell
cargo build --release
```

- `cargo build --release` — 建置發行版執行檔至 `target\release\explorer-opacity.exe`。

## 使用方式

```powershell
explorer-opacity.exe
explorer-opacity.exe status
explorer-opacity.exe install-autostart
```

- `explorer-opacity.exe` — 啟動常駐工具（首次執行會產生 `explorer-opacity.toml`）。
- `explorer-opacity.exe status` — 顯示自動啟動、記錄與設定路徑。
- `explorer-opacity.exe install-autostart` — 註冊為 Windows 啟動時自動執行。

## 背景

設計決策請見 [`docs/adr/`](../../adr/)。

## 元件

| 元件 | 職責 | 文件 |
|---|---|---|
| `explorer-opacity.exe` | 常駐主體（系統匣、快速鍵、shell hook、訊息迴圈） | [ADR-0002](../../adr/0002-event-driven-vs-polling.md) |
| `window.rs` | 視窗列舉與 layered alpha 的套用/還原 | [ADR-0001](../../adr/0001-alpha-vs-backdrop.md) |
| `config.rs` | `explorer-opacity.toml` 的讀取與驗證 | [ADR-0006](../../adr/0006-logging-config.md) |
| `logging.rs` | 可選的檔案記錄，儲存於執行檔旁 | [ADR-0006](../../adr/0006-logging-config.md) |
| `autostart.rs` | 透過 `HKCU\...\Run` 的使用者層級自動啟動 | |

## 授權

MIT — 詳見 [LICENSE](../../../LICENSE)。
