[English](../../../README.md) | [日本語](./README.md) | [简体中文](../zh/README.md)

# explorer-opacity

Windows のファイル エクスプローラー（Explorer）のウィンドウだけを常時半透明に保つ、軽量な常駐ツール。
対象は `CabinetWClass` のみで、デスクトップとタスクバーには触れない。
Rust 製の単一ポータブル exe で、ランタイム依存はない。

## ダウンロード

[Releases](https://github.com/abarabakuhatsu/explorer-opacity/releases/latest) から最新の
`explorer-opacity-<version>-windows-x64.zip` を取得して展開し、`explorer-opacity.exe` を実行します。
ダウンロードは添付の `SHA256SUMS.txt` で検証できます。

```powershell
Get-FileHash .\explorer-opacity-<version>-windows-x64.zip -Algorithm SHA256
```

## ソースからビルド

Rust（`x86_64-pc-windows-msvc`）と MSVC リンカ（または MinGW と `-gnu` ターゲット）が必要です。

```powershell
cargo build --release
```

- `cargo build --release` — リリース版バイナリを `target\release\explorer-opacity.exe` にビルドします。

## 使い方

```powershell
explorer-opacity.exe
explorer-opacity.exe status
explorer-opacity.exe install-autostart
explorer-opacity.exe uninstall-autostart
```

- `explorer-opacity.exe` — 常駐を開始します（初回は `explorer-opacity.toml` を生成）。
- `explorer-opacity.exe status` — 自動起動・ログ・設定のパスを表示します。
- `explorer-opacity.exe install-autostart` — Windows 起動時に自動起動するよう登録します。
- `explorer-opacity.exe uninstall-autostart` — 自動起動の登録を解除します。

## 設定

設定は exe の隣の `explorer-opacity.toml` にあります（初回起動時に生成）。

```toml
config_version = 1
enabled = true          # false で透明化を無効化
opacity = 85            # 10〜100 (%)。低いほど文字が読みにくい
opacity_step = 5        # ホットキーの増減幅（1〜50）

[hotkeys]
toggle   = "Ctrl+Alt+T"
increase = "Ctrl+Alt+Up"
decrease = "Ctrl+Alt+Down"

[targets]
include = ["CabinetWClass"]
exclude = ["Progman", "WorkerW", "Shell_TrayWnd", "TaskManagerWindow"]

[logging]
enabled = true          # false でログを書かない
path = ""               # 空 = exe 隣の explorer-opacity.log
```

- `opacity` は 10〜100、`opacity_step` は 1〜50 にクランプされます。
- クラス判定: `exclude` が最優先。`include` が非空ならそれだけを対象。両方空なら既定に戻ります。
- `logging.path`: 相対パスは exe 起点、`%VAR%` は展開、末尾セパレータまたは既存ディレクトリなら配下に `explorer-opacity.log` を補完します。
- 反映はトレイの「Reload config」または再起動。不正な設定は既定で起動しログに記録されます。

## ホットキー

- `Ctrl+Alt+T` — 透明化 ON / OFF。
- `Ctrl+Alt+Up` — 不透明度を上げる。
- `Ctrl+Alt+Down` — 不透明度を下げる。

## トレイメニュー

Enabled / Increase opacity / Decrease opacity / Reload config / Start with Windows / Open log（ログ無効時は非表示）/ Exit

## トラブルシューティング

- **SmartScreen の警告**: exe は未署名のため警告が出ることがあります。「詳細情報」→「実行」を選びます。
- **透明にならない**: 対象は `CabinetWClass`（ファイル エクスプローラー）のみです。`enabled` / `opacity` を確認し、`explorer-opacity.exe status` を実行します。
- **ログが出ない**: exe のフォルダが書込不可（Program Files 等）の可能性があります。`[logging] path` に書込可能な場所を指定します。
- **クラッシュ後に透過が残る**: 強制終了では復元されません。Explorer を再起動すると元に戻ります。
- **ホットキーが効かない**: 他アプリと競合している可能性があります。設定で変更します。

## 既知の制限

- 文字も半透明になります（既定 85% では実用上問題ありません）。
- 背景はぼかされず、単純に透けます。
- Windows x64 のみ。
- exe は未署名です。

## アンインストール

1. トレイメニューから Exit（ウィンドウを元に戻します）。
2. `explorer-opacity.exe uninstall-autostart` を実行。
3. `explorer-opacity.exe`、`explorer-opacity.toml`、ログファイルを削除。

## リンク

- [Releases](https://github.com/abarabakuhatsu/explorer-opacity/releases)
- [設計判断 (docs/adr/)](../../adr/)
- [Issues](https://github.com/abarabakuhatsu/explorer-opacity/issues)

## ライセンス

MIT — 詳細は [LICENSE](../../../LICENSE) を参照。
