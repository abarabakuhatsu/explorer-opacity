[English](../../../README.md) | [日本語](./README.md) | [繁體中文](../zh/README.md)

# explorer-opacity

Windows のファイル エクスプローラー（Explorer）のウィンドウだけを常時半透明に保つ、軽量な常駐ツール。
対象は `CabinetWClass` のみで、デスクトップとタスクバーには触れない。
Rust 製の単一ポータブル exe で、ランタイム依存はない。

## インストール

```powershell
cargo build --release
```

- `cargo build --release` — リリース版バイナリを `target\release\explorer-opacity.exe` にビルドします。

## 使い方

```powershell
explorer-opacity.exe
explorer-opacity.exe status
explorer-opacity.exe install-autostart
```

- `explorer-opacity.exe` — 常駐を開始します（初回は `explorer-opacity.toml` を生成）。
- `explorer-opacity.exe status` — 自動起動・ログ・設定のパスを表示します。
- `explorer-opacity.exe install-autostart` — Windows 起動時に自動起動するよう登録します。

## 背景

設計判断は [`docs/adr/`](../../adr/) を参照。

## 構成

| コンポーネント | 役割 | ドキュメント |
|---|---|---|
| `explorer-opacity.exe` | 常駐本体（トレイ・ホットキー・shell hook・メッセージループ） | [ADR-0002](../../adr/0002-event-driven-vs-polling.md) |
| `window.rs` | ウィンドウ列挙とレイヤード alpha の適用/復元 | [ADR-0001](../../adr/0001-alpha-vs-backdrop.md) |
| `config.rs` | `explorer-opacity.toml` の読み書きと検証 | [ADR-0006](../../adr/0006-logging-config.md) |
| `logging.rs` | exe の隣に保存する任意のファイルログ | [ADR-0006](../../adr/0006-logging-config.md) |
| `autostart.rs` | `HKCU\...\Run` によるユーザー単位の自動起動 | |

## ライセンス

MIT — 詳細は [LICENSE](../../../LICENSE) を参照。
