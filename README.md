# explorer-opacity

Windows のファイル エクスプローラー（Explorer）のウィンドウだけを常時半透明に保つ、軽量な
常駐ツールです。デスクトップやタスクバーは対象外です。

原案の AutoHotkey 版（`docs/chatgpt-prototype_2026-10-01.md`）を、単一 exe・イベント駆動の
Rust 実装に置き換えたものです。

## 主な特徴

- Explorer のファイルウィンドウ（`CabinetWClass`）を既定 85% 不透明に保つ
- 新規ウィンドウ・`explorer.exe` 再起動後も自動適用（シェルフック + 30 秒のセーフティ掃引）
- アクティブ / 非アクティブで見た目が変わらない
- 通常のクリック・ドラッグ・ダブルクリックを妨げない
- 終了時に変更したウィンドウを元へ戻す
- タスクトレイとホットキーで ON/OFF・透明度増減
- ポータブル単一 exe（設定は exe の隣）
- 多重起動防止、Windows 自動起動トグル

## 動作要件

- Windows 10 / 11
- ビルドには Rust（`x86_64-pc-windows-msvc`）と MSVC リンカ、または MinGW + `-gnu` ターゲット

## ビルド

```powershell
cargo build --release
# 成果物: target\release\explorer-opacity.exe
```

## 使い方

```powershell
# 常駐開始（設定ファイルが無ければ既定値を生成）
explorer-opacity.exe

# 自動起動の登録 / 解除 / 状態確認
explorer-opacity.exe install-autostart
explorer-opacity.exe uninstall-autostart
explorer-opacity.exe status
```

初回起動時に exe の隣へ `explorer-opacity.toml` を生成します。

## 設定ファイル（`explorer-opacity.toml`）

```toml
config_version = 1
enabled = true
opacity = 85            # 不透明度（10〜100 %）
opacity_step = 5        # ホットキーでの増減幅

[hotkeys]
toggle = "Ctrl+Alt+T"
increase = "Ctrl+Alt+Up"
decrease = "Ctrl+Alt+Down"

[targets]
include = ["CabinetWClass"]
exclude = ["Progman", "WorkerW", "Shell_TrayWnd", "TaskManagerWindow"]
```

- `opacity` は 10〜100 にクランプされます（0 はウィンドウが見えなくなるため禁止）。
- クラス判定の規則: `exclude` が最優先。`include` が非空ならそれだけを対象。`include` が空なら
  `exclude` 以外すべてが対象。
- 設定はトレイメニューの「Reload config」または再起動で反映されます。
- 不正な設定は既定値で起動し、`%LOCALAPPDATA%\explorer-opacity\explorer-opacity.log` に記録
  されます。
- 旧バージョンの `mode` / `[mode_options]` が残っていても無視され、読み込みは失敗しません。

## 方式

ウィンドウ全体に一様な alpha を掛ける方式（`SetLayeredWindowAttributes` + `LWA_ALPHA`）のみを
採用しています。DWM の Acrylic/Mica バックドロップは、Explorer が本文領域を自前で不透明に
描画するため本文を透明化できず、不採用としました（`docs/adr/0004-backdrop-spike.md`）。

## ホットキー

- `Ctrl+Alt+T`: 透明化 ON / OFF
- `Ctrl+Alt+Up`: 不透明度を上げる
- `Ctrl+Alt+Down`: 不透明度を下げる

## トレイメニュー

Enabled / Increase opacity / Decrease opacity / Reload config / Start with Windows / Open log / Exit

## 手動テスト チェックリスト

- [ ] 新しく開いた Explorer が半透明になる
- [ ] `explorer.exe` を再起動しても自動適用される
- [ ] 最大化・スナップ・タブ追加後も透過が維持される
- [ ] 他ウィンドウを操作しても（非アクティブでも）透過が維持される
- [ ] 終了（トレイ → Exit）で元の不透明に戻る
- [ ] 設定変更（Reload config）が即時反映される
- [ ] 二重起動しない
- [ ] 自動起動トグルが `Run` キーへ反映される
- [ ] 強制終了後は Explorer を再起動すると元に戻る

## 既知の制約

- 文字も半透明になります。既定 85% では実用上の問題はありませんが、不透明度を下げるほど
  可読性が落ちます。
- 背景はぼかされず、単純に透けます。
- 強制終了（タスクマネージャー等）では復元処理が走りません。Explorer を再起動すると元に戻り
  ます。
- 未署名 exe のため SmartScreen の警告が出る場合があります（「詳細情報」→「実行」）。

## 構成

```
src/
  main.rs            CLI エントリ
  lib.rs             ライブラリ公開
  config.rs          設定の読み書き・検証
  filter.rs          クラス判定・alpha 変換（純関数）
  logging.rs         ファイルロガー
  win/
    hook.rs          常駐本体（隠しウィンドウ・シェルフック・トレイ・ホットキー）
    window.rs        列挙・クラス判定・レイヤード alpha 適用/復元
    tray.rs          トレイアイコンとメニュー
    hotkey.rs        ホットキー解析・登録
    autostart.rs     HKCU Run 登録
docs/adr/            Architecture Decision Records
```

## ライセンス

MIT
