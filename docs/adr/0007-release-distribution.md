# 7. リリース配布: タグ push で GitHub Release に zip を添付する

* ステータス: 承認
* 日付: 2026-10-01
* 決定者: ユーザー + AIエージェント

## コンテキスト（背景）
配布物は単一 exe（約 380 KB）。これまでは手動でビルドして Release に添付する必要があり、
手間と版の取り違えのリスクがあった。`v*.*.*` のタグを打ったら自動でビルド・zip 化し、
Release に登録したい。

## 決定事項（Decision）
`.github/workflows/release.yml` を追加し、次を行う。

- **トリガー**: `v*.*.*` のタグ push（`main` の CI とは別ワークフロー）。加えて
  `workflow_dispatch` によるドライラン。
- **品質ゲート**: `cargo fmt --check` / `clippy -D warnings` / `test` を通過した場合のみ進む。
- **バージョン整合**: `cargo metadata` の version とタグ（`v` を除く）が一致しなければ失敗。
- **成果物**: `explorer-opacity.exe` のみを
  `explorer-opacity-<version>-windows-x64.zip` に圧縮。
- **チェックサム**: `SHA256SUMS.txt` を生成して添付。
- **公開**: `gh release create` で直接公開。タグに `-` を含む場合は `--prerelease`、
  それ以外は `--latest`。リリースノートは `--generate-notes` で自動生成。
- **ドライラン**: `workflow_dispatch` では Release を作らず、`dist/` を workflow artifact に
  アップロードする。

## 影響・結果（Consequences）
### ポジティブな影響
- タグを打つだけで再現性のある配布が完了し、手動添付の手間とミスを削減できる。
- 品質ゲートとバージョン整合により、壊れた版・版ずれのリリースを防げる。
- ドライラン経路により、タグを打つ前にパッケージ内容を検証できる。

### ネガティブな影響・トレードオフ
- exe は**未署名**のため SmartScreen 警告が出る。`SHA256SUMS.txt` は破損検出が主目的で、
  真正性（なりすまし耐性）は保証しない。
- 対象は **x64 のみ**（aarch64 等は将来マトリクス化が必要）。
- タグ／リリースの削除は権限ポリシーで制限されるため、検証は「ドライラン → タグ」の順で行う。
- 初回は現行 version `0.1.0` に合わせ `v0.1.0` を打つ。
