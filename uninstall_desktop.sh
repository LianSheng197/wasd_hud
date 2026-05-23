#!/bin/bash
set -euo pipefail

DESKTOP_DIR="$HOME/.local/share/applications"
DESKTOP_DST="$DESKTOP_DIR/wasd_hud.desktop"

if [[ -f "$DESKTOP_DST" ]]; then
  rm -f "$DESKTOP_DST"
  echo "Removed: $DESKTOP_DST"
else
  echo "Not found: $DESKTOP_DST"
fi

if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database "$DESKTOP_DIR" >/dev/null 2>&1 || true
fi

echo "Uninstall complete."
