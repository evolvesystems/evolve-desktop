#!/bin/bash
# EvolveApp launcher
# Detects Nvidia GPU and disables broken DMABUF buffer sharing.
# GPU rendering still works via EGL (--device=all gives /dev/nvidia* access).

if [ -e /dev/nvidia0 ] || [ -f /proc/driver/nvidia/version ]; then
    export WEBKIT_DISABLE_DMABUF_RENDERER=1
fi

exec /app/bin/evolve-desktop-bin "$@"
