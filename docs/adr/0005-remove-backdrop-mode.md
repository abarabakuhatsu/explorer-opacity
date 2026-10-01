# 5. backdrop モードの廃止と alpha 専用化

* ステータス: 承認
* 日付: 2026-10-01
* 決定者: ユーザー + AIエージェント

## コンテキスト（背景）
ADR-0004 の実機検証により、backdrop 方式は Explorer の本文領域を透明化できないと確定した。
当初 ADR-0001 では `mode` で alpha / backdrop を切替可能にしていたが、backdrop は目的を
達成できないため、選択肢として残す意味がなくなった。

## 決定事項（Decision）
以下を削除し、alpha 専用に簡素化する。

- `src/win/backdrop.rs`（DWM バックドロップ / AccentPolicy）
- `src/bin/spike.rs`（`eo-spike`、比較の役目は終了。結果は ADR-0004 に記録）
- 設定: `Mode` / `BackdropMaterial` / `ModeOptions` と `mode` / `mode_options` フィールド
- トレイ: モード切替メニューと `MenuCommand::ModeAlpha/ModeBackdrop`
- `Cargo.toml` の不要な `windows-sys` 機能フラグ（DWM / Controls / Accessibility /
  DataExchange）と `[[bin]] eo-spike`

旧設定ファイルに `mode` / `[mode_options]` が残っていても、serde が未知フィールドを無視する
ため読み込みは失敗しない（後方互換）。

## 影響・結果（Consequences）
### ポジティブな影響
- コード・設定・UI・依存が減り、保守と検証が容易。
- exe サイズとビルド時間を削減。

### ネガティブな影響・トレードオフ
- 将来 backdrop 相当を再検討する場合は履歴（ADR-0004）から復元する必要がある。
- 診断用の `eo-spike` を失う（`alpha` の目視確認は本体の一時停止/再開で代替可能）。
