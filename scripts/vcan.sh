#!/bin/sh
set -eu

usage() {
  echo "Usage: $0 <vcan_ifname>" >&2
  echo "Example: $0 vcan1" >&2
  exit 1
}

[ "${1:-}" ] || usage

IFACE="$1"

IP_BIN="$(command -v ip || true)"

[ -n "$IP_BIN" ] || { echo "Error: 'ip' command not found." >&2; exit 127; }

ensure_vcan() {
  IF="$1"
  if ! "$IP_BIN" link show "$IF" >/dev/null 2>&1; then
    echo "Creating $IF (vcan)…"
    "$IP_BIN" link add dev "$IF" type vcan
  fi
  "$IP_BIN" link set "$IF" mtu 16 || true
  "$IP_BIN" link set "$IF" up
}

# --- Main ---
ensure_vcan "$IFACE"

echo
echo "Virtual CAN interface '$IFACE' is up."
