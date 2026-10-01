[English](../../../README.md) | [日本語](./README.md) | [简体中文](../zh/README.md)

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

## ライセンス

MIT — 詳細は [LICENSE](../../../LICENSE) を参照。
