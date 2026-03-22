#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "=== Building cst integration test images ==="

echo "--- Ubuntu 22.04 ---"
docker build -f "$REPO_ROOT/docker/test/Dockerfile" \
    -t cst-test-ubuntu "$REPO_ROOT"

echo "--- Debian Bookworm ---"
docker build -f "$REPO_ROOT/docker/test/Dockerfile.debian" \
    -t cst-test-debian "$REPO_ROOT"

echo ""
echo "=== Running Ubuntu tests ==="
docker run --rm cst-test-ubuntu

echo ""
echo "=== Running Debian tests ==="
docker run --rm cst-test-debian

echo ""
echo "=== All Docker integration tests passed ==="
