#!/bin/bash
# EvolveApp launcher
# Tries native Wayland first (full GPU). If it crashes (Nvidia DMABUF bug),
# auto-relaunches with DMABUF disabled.

CRASH_FLAG="${XDG_CONFIG_HOME:-$HOME/.config}/evolveapp-dmabuf-crash"

# If previous run crashed due to DMABUF, skip straight to workaround
if [ -f "$CRASH_FLAG" ]; then
    export WEBKIT_DISABLE_DMABUF_RENDERER=1
    exec /app/bin/evolve-desktop-bin "$@"
fi

# Try native launch
/app/bin/evolve-desktop-bin "$@"
EXIT_CODE=$?

# If it crashed quickly (within a few seconds = DMABUF bug), mark and relaunch
if [ $EXIT_CODE -ne 0 ] && [ $EXIT_CODE -ne 130 ]; then
    echo "EvolveApp crashed (exit $EXIT_CODE) — retrying with DMABUF workaround..."
    touch "$CRASH_FLAG"
    export WEBKIT_DISABLE_DMABUF_RENDERER=1
    exec /app/bin/evolve-desktop-bin "$@"
fi
