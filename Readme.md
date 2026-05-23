# WASD HUD

一個用 Rust 撰寫的簡易桌面輸入可視化工具，會在螢幕上顯示鍵盤（WASD 區塊）與滑鼠基本按鍵與滾輪狀態，適合錄影、直播或操作展示。

## 功能簡介

- 顯示遊戲用的常用鍵位（左側）
- 顯示滑鼠左/右/中鍵與滾輪提示
- 視窗可拖移，並記住上次關閉位置
- 透明 HUD 視窗（`eframe/egui`）（在某些 WM 可能無法正常工作）

## 需要套件

### 必要

- Rust toolchain（`rustc` / `cargo`）
- Linux 桌面環境（僅支援 X11）

### 建議（置頂輔助）

本專案的 `run.sh` 目前會使用以下工具協助維持視窗置頂：

- `devilspie2`
- `wmctrl`

若缺少建議套件，程式仍可執行，但置頂行為可能不穩定。

## 開發環境版本

以下為目前開發機實際版本：

- `rustc`: `1.93.1 (01f6ddf75 2026-02-11)`
- `cargo`: `1.93.1 (083ac5135 2025-12-15)`

## 建置與執行

```bash
cd /path/to/wasd_hud
./run.sh
```

`run.sh` 會進行 release build，並啟動 HUD。

## 安裝桌面捷徑（可搜尋）

執行安裝腳本：

```bash
cd /path/to/wasd_hud
./install_desktop.sh
```

安裝後會建立：

- `~/.local/share/applications/wasd_hud.desktop`

你可以在系統搜尋輸入 `WASD HUD` 或 `wasd` 啟動。

## 刪除桌面捷徑

執行刪除腳本：

```bash
cd /path/to/wasd_hud
./uninstall_desktop.sh
```

