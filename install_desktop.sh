#!/bin/bash
set -euo pipefail

APP_DIR="$(cd "$(dirname "$0")" && pwd)"
DESKTOP_SRC="$APP_DIR/wasd_hud.desktop"
DESKTOP_DIR="$HOME/.local/share/applications"
DESKTOP_DST="$DESKTOP_DIR/wasd_hud.desktop"

mkdir -p "$DESKTOP_DIR"
cp "$DESKTOP_SRC" "$DESKTOP_DST"

if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database "$DESKTOP_DIR" >/dev/null 2>&1 || true
fi

echo "Installed: $DESKTOP_DST"
echo "You can now search for: WASD HUD"
