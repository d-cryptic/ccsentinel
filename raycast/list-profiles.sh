#!/bin/bash

# Required parameters:
# @raycast.schemaVersion 1
# @raycast.title List Claude Profiles
# @raycast.mode fullOutput
# @raycast.packageName Claude Sentinel

# Optional parameters:
# @raycast.icon 🛡
# @raycast.description List all Claude Sentinel profiles and their sessions

cst list 2>/dev/null || echo "cst not found. Install: cargo install cst-cli"
