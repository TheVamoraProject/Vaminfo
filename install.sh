#!/bin/bash
# ---------------------------------------------------------------
# Vaminfo - GitHub Installer (builds from source)
# https://github.com/TheVamoraProject/Vaminfo
# ---------------------------------------------------------------

set -e

REPO="https://github.com/TheVamoraProject/Vaminfo"
INSTALL_BIN="/usr/bin/vaminfo"
TMP_DIR="/tmp/vaminfo-build"

echo "Vaminfo installer (build from source)"
echo

# ---- Check dependencies ----
for dep in curl git cargo; do
    if ! command -v "$dep" &>/dev/null; then
        echo "Missing dependency: $dep"
        echo "Install it and try again."
        exit 1
    fi
done

# ---- Clone repo ----
echo "Cloning repository..."
rm -rf "$TMP_DIR"
git clone --depth=1 "$REPO" "$TMP_DIR"
cd "$TMP_DIR"

# ---- Build ----
echo "Building (this may take a moment)..."
cargo build --release

# ---- Install binary ----
echo "Installing to $INSTALL_BIN..."
sudo cp target/release/vaminfo "$INSTALL_BIN"
sudo chmod 755 "$INSTALL_BIN"

# ---- Install version info ----
sudo mkdir -p /etc/VamoraSys/default/vaminfo
sudo cp info.vmf /etc/VamoraSys/default/vaminfo/info.vmf

# ---- Cleanup ----
cd /
rm -rf "$TMP_DIR"

echo
echo "Vaminfo installed successfully!"
echo "   Run: vaminfo"
