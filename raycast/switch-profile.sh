#!/bin/bash

# Required parameters:
# @raycast.schemaVersion 1
# @raycast.title Switch Claude Profile
# @raycast.mode compact
# @raycast.packageName Claude Sentinel

# Optional parameters:
# @raycast.icon 🛡
# @raycast.argument1 { "type": "text", "placeholder": "profile[:session]", "optional": true }
# @raycast.description Switch Claude Sentinel profile (leave blank to show current)

if [ -z "$1" ]; then
  # No argument — show current status
  cst status 2>/dev/null || echo "cst not found. Install: cargo install cst-cli"
else
  cst use "$1" && echo "Switched to $1"
fi
