#!/usr/bin/env bash
set -e

CYAN='\033[0;36m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
RESET='\033[0m'

info()    { echo -e "${CYAN}[vaminfo]${RESET} $*"; }
success() { echo -e "${GREEN}[ok]${RESET} $*"; }
error()   { echo -e "${RED}[err]${RESET} $*"; exit 1; }
warn()    { echo -e "${YELLOW}[!]${RESET} $*"; }

VAMINFO_DIR="$HOME/.VamoraSys/apps/vaminfo"
ART_DIR="$VAMINFO_DIR/art"
CONFIG_FILE="$VAMINFO_DIR/config.vmf"
INSTALL_PREFIX="${PREFIX:-/usr/local}"
BIN_DIR="$INSTALL_PREFIX/bin"

echo -e "${CYAN}"
cat << 'EOF'
 ██╗   ██╗ █████╗ ███╗   ███╗ ██████╗ ██████╗  █████╗ 
 ██║   ██║██╔══██╗████╗ ████║██╔═══██╗██╔══██╗██╔══██╗
 ██║   ██║███████║██╔████╔██║██║   ██║██████╔╝███████║
 ╚██╗ ██╔╝██╔══██║██║╚██╔╝██║██║   ██║██╔══██╗██╔══██║
  ╚████╔╝ ██║  ██║██║ ╚═╝ ██║╚██████╔╝██║  ██║██║  ██║
   ╚═══╝  ╚═╝  ╚═╝╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝
               Vaminfo Installer -- Vamora OS
EOF
echo -e "${RESET}"

# -- 1. Check Rust toolchain ---------------------------------------------------
info "Checking Rust toolchain..."
if ! command -v cargo &>/dev/null; then
    warn "Rust not found. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
    # shellcheck source=/dev/null
    source "$HOME/.cargo/env"
    success "Rust installed."
else
    RUST_VERSION=$(rustc --version)
    success "Rust found: $RUST_VERSION"
fi

# -- 2. Build vaminfo ----------------------------------------------------------
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
info "Building vaminfo (release)..."
(cd "$SCRIPT_DIR" && cargo build --release) || error "Build failed."
success "Build complete."

# -- 3. Install binary ---------------------------------------------------------
BINARY="$SCRIPT_DIR/target/release/vaminfo"
if [ ! -f "$BINARY" ]; then
    error "Binary not found at $BINARY -- build may have failed."
fi

info "Installing binary to $BIN_DIR/vaminfo..."
if [ -w "$BIN_DIR" ]; then
    cp "$BINARY" "$BIN_DIR/vaminfo"
    chmod +x "$BIN_DIR/vaminfo"
else
    warn "Need elevated privileges to write to $BIN_DIR. Trying sudo..."
    sudo cp "$BINARY" "$BIN_DIR/vaminfo"
    sudo chmod +x "$BIN_DIR/vaminfo"
fi
success "Binary installed to $BIN_DIR/vaminfo"

# -- 4. Create .VamoraSys directory structure ----------------------------------
info "Creating .VamoraSys directory structure..."
mkdir -p "$VAMINFO_DIR" "$ART_DIR"
success "Directories created: $VAMINFO_DIR"

# -- 5. Install bundled ASCII art ----------------------------------------------
if [ -d "$SCRIPT_DIR/art" ]; then
    info "Installing ASCII art..."
    cp -n "$SCRIPT_DIR/art/"*.vtxt "$ART_DIR/" 2>/dev/null || true
    success "ASCII art installed to $ART_DIR"
fi

# -- 6. Detect distro ---------------------------------------------------------
ASCII_FILE="ascii1.vtxt"
ASCII_COLOR="blue"

if [ -f /etc/os-release ]; then
    . /etc/os-release

    if [[ "$ID" == "debian" || "$ID_LIKE" == *"debian"* ]]; then
        ASCII_FILE="debian.vtxt"
        ASCII_COLOR="red"
        info "Detected Debian-based distribution."
    elif [[ "$ID" == "arch" || "$ID_LIKE" == *"arch"* ]]; then
        ASCII_FILE="arch.vtxt"
        ASCII_COLOR="blue"
        info "Detected Arch-based distribution."
    else
        info "Detected $PRETTY_NAME. Using default ASCII art."
    fi
fi

# -- 7. Generate default config if missing -----------------------------------
if [ ! -f "$CONFIG_FILE" ]; then
    info "Generating default config..."

    cat > "$CONFIG_FILE" << VMFEOF
# Vaminfo configuration -- Vamora OS
# Edit via: vaminfo config

ascii_file    = "$ASCII_FILE"
ascii_color   = "$ASCII_COLOR"
title_color   = "bright_blue"
key_color     = "bright_blue"
value_color   = "white"
separator     = "-"
mini_mode     = false
mini_mode_ascii = false
show_title    = true
show_separator = true

[modules]
hostname           = true
os                 = true
kernel             = true
bios               = true
cpu                = true
gpu                = true
ram                = true
disk               = true
uptime             = false
shell              = true
terminal           = true
desktop            = true
resolution         = true
theme              = true
local_ip           = true
bluetooth          = true
battery            = true
network            = true
media              = false
color_blocks_big   = true
color_blocks_small = false
VMFEOF

    success "Default config written to $CONFIG_FILE"
else
    info "Config already exists -- skipping."
fi

# -- 7. Verify installation ---------------------------------------------------
info "Verifying installation..."
if command -v vaminfo &>/dev/null; then
    success "vaminfo is on PATH and ready!"
else
    warn "vaminfo installed to $BIN_DIR but may not be on PATH."
    warn "Add this to your shell profile:"
    echo -e "    ${YELLOW}export PATH=\"$BIN_DIR:\$PATH\"${RESET}"
fi

echo ""
echo -e "${GREEN}============================================${RESET}"
echo -e "${GREEN}  Vaminfo installed successfully!          ${RESET}"
echo -e "${GREEN}============================================${RESET}"
echo ""
echo -e "  Run: ${CYAN}vaminfo${RESET}          -> show system info"
echo -e "  Run: ${CYAN}vaminfo config${RESET}   -> interactive setup wizard"
echo -e "  Run: ${CYAN}vaminfo --mini${RESET}   -> minimal quick view"
echo -e "  Run: ${CYAN}vaminfo --help${RESET}   -> all commands"
echo -e ""
echo -e "  Config: ${YELLOW}~/.VamoraSys/apps/vaminfo/config.vmf${RESET}"
echo -e "  Art:    ${YELLOW}~/.VamoraSys/apps/vaminfo/art/${RESET}"
echo ""
