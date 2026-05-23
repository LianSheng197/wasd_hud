#!/bin/bash

set -euo pipefail

cargo build --release

target/release/wasd_hud &
APP_PID=$!

RULE_DIR="/tmp/wasd_hud_devilspie2"
RULE_FILE="${RULE_DIR}/wasd_hud_pid.lua"
DS2_LOG="/tmp/wasd_hud_devilspie2.log"
mkdir -p "$RULE_DIR"

cat > "$RULE_FILE" <<EOF
if (get_window_pid() == ${APP_PID}) then
  debug_print("wasd_hud_pid.lua matched pid=${APP_PID}")
  set_window_above(true)
end
EOF

WMC_HELPER_PID=""
if command -v devilspie2 >/dev/null 2>&1; then
  devilspie2 --debug --folder "$RULE_DIR" >"$DS2_LOG" 2>&1 &
  DS2_PID=$!
  sleep 1
  if ! kill -0 "${DS2_PID}" 2>/dev/null; then
    echo "devilspie2 exited early, check log: $DS2_LOG" >&2
  fi
else
  echo "devilspie2 not found; skipping PID-based topmost helper." >&2
  DS2_PID=""
fi

# Safety net: keep only this app PID on top (no keyword matching).
if command -v wmctrl >/dev/null 2>&1; then
  (
    while kill -0 "${APP_PID}" 2>/dev/null; do
      wmctrl -lp 2>/dev/null | awk -v pid="${APP_PID}" '$3==pid {print $1}' | while read -r wid; do
        wmctrl -i -r "$wid" -b add,above >/dev/null 2>&1 || true
      done
      sleep 0.5
    done
  ) &
  WMC_HELPER_PID=$!
fi

cleanup() {
  trap - EXIT INT TERM
  if kill -0 "${APP_PID}" 2>/dev/null; then
    kill "${APP_PID}" 2>/dev/null || true
  fi
  if [[ -n "${DS2_PID}" ]] && kill -0 "${DS2_PID}" 2>/dev/null; then
    kill "${DS2_PID}" 2>/dev/null || true
  fi
  if [[ -n "${WMC_HELPER_PID}" ]] && kill -0 "${WMC_HELPER_PID}" 2>/dev/null; then
    kill "${WMC_HELPER_PID}" 2>/dev/null || true
  fi
  rm -f "$RULE_FILE"
}
trap cleanup EXIT INT TERM

wait "$APP_PID"
