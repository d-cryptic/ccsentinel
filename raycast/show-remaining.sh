#!/bin/bash

# Required parameters:
# @raycast.schemaVersion 1
# @raycast.title Claude Quota Remaining
# @raycast.mode fullOutput
# @raycast.packageName Claude Sentinel

# Optional parameters:
# @raycast.icon 🛡
# @raycast.description Show Claude quota usage, token counts, and time until rate-limit reset

cst remaining 2>/dev/null || echo "cst not found. Install: cargo install cst-cli"
