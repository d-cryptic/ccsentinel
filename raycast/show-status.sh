#!/bin/bash

# Required parameters:
# @raycast.schemaVersion 1
# @raycast.title Claude Sentinel Status
# @raycast.mode compact
# @raycast.packageName Claude Sentinel

# Optional parameters:
# @raycast.icon 🛡
# @raycast.description Show current Claude Sentinel profile, session, and daemon status

cst status 2>/dev/null || echo "cst not found. Install: cargo install cst-cli"
