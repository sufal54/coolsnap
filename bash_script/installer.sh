#!/usr/bin/env bash

set -euo pipefail

OUTPUT_PATH="/etc/systemd/system/coolsnap.service"
BIN_PATH="/opt"
CURRENT_DIR="$(pwd)"

# Install systemd service
rm -f "$OUTPUT_PATH"

cp \
    "$CURRENT_DIR/config/coolsnap.service" \
    "$OUTPUT_PATH"

HOME_DIR="$BIN_PATH/coolsnap"

# Recreate installation directory
rm -rf "$HOME_DIR"

mkdir -p "$HOME_DIR/bin"

echo "50" > "$HOME_DIR/temp_limit"

cp \
    "$CURRENT_DIR/bash_script/service" \
    "$HOME_DIR/bin/coolsnap"

cp \
    "$CURRENT_DIR/script/temptool.sh" \
    "$HOME_DIR/bin/temptool.sh"

LINK="/usr/local/bin/coolsnap"

rm -f "$LINK"

ln -s \
    "/opt/coolsnap/bin/temptool.sh" \
    "$LINK"