以下のAutoHotkey v2版なら、マスターの条件をそのまま満たせます。

Explorerを常時85%の不透明度にする
Explorerを閉じても、新しく開いたExplorerへ自動適用
Explorer.exeの再起動後も自動復帰
アクティブ/非アクティブで透明度が変わらない
通常のクリック操作はそのまま
スクリプト終了時に変更したExplorerを不透明へ戻す
常駐負荷はかなり小さい
デスクトップそのものは透明化しない
#Requires AutoHotkey v2.0
#SingleInstance Force

; ============================================================
; Explorer 常時透明化
;
; 透明度:
;   255 = 完全不透明
;   217 ≒ 85% 不透明
;
; Explorer のファイルウィンドウだけを対象にします。
; デスクトップやタスクバーは対象外です。
; ============================================================

EXPLORER_ALPHA := 217
CHECK_INTERVAL := 500

; 透明化したウィンドウを記録
modifiedWindows := Map()

; Explorerを定期チェック
SetTimer CheckExplorerWindows, CHECK_INTERVAL

; スクリプト終了時に元へ戻す
OnExit RestoreExplorerWindows


CheckExplorerWindows() {
    global EXPLORER_ALPHA, modifiedWindows

    ; Windows Explorer のファイルウィンドウ
    for hwnd in WinGetList("ahk_class CabinetWClass") {
        ; 既に処理済みなら何もしない
        if modifiedWindows.Has(hwnd)
            continue

        try {
            WinSetTransparent(EXPLORER_ALPHA, "ahk_id " hwnd)
            modifiedWindows[hwnd] := true
        }
    }

    ; 閉じられたウィンドウを記録から削除
    for hwnd in modifiedWindows {
        if !WinExist("ahk_id " hwnd)
            modifiedWindows.Delete(hwnd)
    }
}


RestoreExplorerWindows(*) {
    global modifiedWindows

    ; スクリプト終了時に透明化したウィンドウを元へ戻す
    for hwnd in modifiedWindows {
        if WinExist("ahk_id " hwnd) {
            try WinSetTransparent("Off", "ahk_id " hwnd)
        }
    }
}
透明度について

Windows APIの透明度は 0～255 です。

このスクリプトの

EXPLORER_ALPHA := 217

は、

217 / 255 = 約85.1%

なので、約85%不透明 = 約15%透過です。

つまり、

┌─────────────────────────┐
│ Explorer                │
│                         │
│   背後の壁紙などが       │
│   約15%見える            │
│                         │
└─────────────────────────┘

という状態です。

「85%透過」の意味を85%の背景が透けるという意味で考えていた場合は値が逆なので、その場合は EXPLORER_ALPHA := 38 にしてください。

この方式がAcrylicと違う点

このスクリプトで使っているのは、

WinSetTransparent()

です。

Acrylic / Mica / Blurなどの背景効果ではありません。

そのため、

Explorerがアクティブ
    ↓
15%透過

Explorerが非アクティブ
    ↓
15%透過

別のウィンドウを操作
    ↓
15%透過

Explorerを再び操作
    ↓
15%透過

となります。

つまり、今回嫌っていた

非アクティブになるとAcrylicが解除されて不透明になる

という挙動は発生しません。

また、WinSetTransparent はウィンドウ全体のアルファ値を変更するだけなので、Explorerの通常のクリック・ドラッグ・ダブルクリックなどを無効化する処理ではありません。

Explorer再起動にも対応

この部分がポイントです。

SetTimer CheckExplorerWindows, CHECK_INTERVAL

500msごとにExplorerのファイルウィンドウを調べています。

例えば、

Explorer
  ↓
explorer.exe 再起動
  ↓
新しいExplorerウィンドウ
  ↓
最大0.5秒以内に検出
  ↓
自動的に85%へ

となります。

Explorerがクラッシュして再起動されたケースでも、新しいウィンドウのHWNDは変わりますが、タイマーが新しいウィンドウを検出して再適用します。

スクリプト終了時

終了時には、

OnExit RestoreExplorerWindows

によって、スクリプトが透明化したウィンドウだけに対して、

WinSetTransparent("Off", ...)

を実行します。

したがって通常の終了なら、

AutoHotkey終了
      ↓
Explorerの透明化解除
      ↓
通常のExplorerへ戻る

となります。

ただし、これは通常のスクリプト終了・AutoHotkeyの終了処理が走った場合です。

タスクマネージャーからAutoHotkey.exeを強制終了した場合や、PCがクラッシュした場合など、OnExit自体が実行されない状況では復元処理はできません。

もっとも、Explorer自身がWindows終了時まで透過状態になってしまうような恒久設定ではないので、Explorerを再起動すれば通常状態に戻ります。

AutoHotkeyで起動時から常駐させる

例えば、

C:\Users\<ユーザー名>\Scripts\ExplorerTransparent.ahk

として保存します。

その後、スタートアップにショートカットを置けばWindowsログイン時から有効になります。

Win + R →

shell:startup

でスタートアップフォルダを開き、.ahkへのショートカットを入れればOKです。

もう少し実用的にするなら

このままでも動きますが、マスターの用途なら私は次の機能も追加した版にします。

Alt + Shift + T
    ↓
透明化 ON / OFF

Alt + Shift + ↑
    ↓
透明度を5%濃くする

Alt + Shift + ↓
    ↓
透明度を5%薄くする

さらにタスクトレイから透明度を変更できるようにすれば、**Explorer専用の「透明度コントローラー」**としてかなり使いやすくできます。
