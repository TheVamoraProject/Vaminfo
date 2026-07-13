#!/usr/bin/env bash

# ╔══════════════════════════════════════════════════════════════════════════╗
# ║              VamoraSys — vaminfo Installer                              ║
# ║                                                                          ║
# ║  HOW TO MAINTAIN THIS FILE:                                              ║
# ║  • Binary name     → change BINARY_NAME below (§ CONFIG)                ║
# ║  • Install paths   → change VAMINFO_DIR / BIN_DIR (§ CONFIG)            ║
# ║  • New distros     → add a case entry in distro_theme() (§ OS THEMES)   ║
# ║  • Config shape    → edit generate_config() (§ CONFIG GENERATOR)        ║
# ║  • Module defaults → edit the [modules] block inside generate_config()  ║
# ╚══════════════════════════════════════════════════════════════════════════╝

set -euo pipefail

# ══════════════════════════════════════════════════════════════════════════════
#  § INSTALLER CONFIG  — edit these to change paths / binary name
# ══════════════════════════════════════════════════════════════════════════════
BINARY_NAME="vaminfo"
VAMINFO_DIR="$HOME/.VamoraSys/apps/vaminfo"
ART_DIR="$VAMINFO_DIR/art"
CONFIG_FILE="$VAMINFO_DIR/config.vmf"

# Termux sets $PREFIX to its own usr tree; on normal Linux this falls back to /usr/local
INSTALL_PREFIX="${PREFIX:-/usr/local}"
BIN_DIR="$INSTALL_PREFIX/bin"

# ══════════════════════════════════════════════════════════════════════════════
#  § COLORS & STYLES
# ══════════════════════════════════════════════════════════════════════════════
RESET='\033[0m';    BOLD='\033[1m';    DIM='\033[2m'
RED='\033[0;31m';   LRED='\033[1;31m'
GREEN='\033[0;32m'; LGREEN='\033[1;32m'
YELLOW='\033[0;33m';LYELLOW='\033[1;33m'
BLUE='\033[0;34m';  LBLUE='\033[1;34m'
CYAN='\033[0;36m';  LCYAN='\033[1;36m'
WHITE='\033[0;37m'; LWHITE='\033[1;37m'
GRAY='\033[1;30m';  LMAGENTA='\033[1;35m'
BG_BLACK='\033[40m'; BG_BLUE='\033[44m'

# ══════════════════════════════════════════════════════════════════════════════
#  § HELPERS
# ══════════════════════════════════════════════════════════════════════════════
COLS=$(tput cols 2>/dev/null || echo 80)

center() {
    local text="$1" color="${2:-}"
    local len=${#text} pad=$(( (COLS - len) / 2 ))
    printf "%${pad}s" ""
    echo -e "${color}${text}${RESET}"
}

hr()      { local c="${1:-─}" col="${2:-$DIM}"; printf "${col}"; printf "%${COLS}s" | tr ' ' "$c"; printf "${RESET}\n"; }
divider() { hr "·" "$DIM$BLUE"; }

log_info()  { echo -e "  ${LBLUE}${BOLD}◆${RESET}  ${WHITE}$*${RESET}"; }
log_ok()    { echo -e "  ${LGREEN}${BOLD}✔${RESET}  ${LGREEN}$*${RESET}"; }
log_warn()  { echo -e "  ${LYELLOW}${BOLD}⚠${RESET}  ${LYELLOW}$*${RESET}"; }
log_error() { echo -e "  ${LRED}${BOLD}✘${RESET}  ${LRED}$*${RESET}" >&2; }
log_dim()   { echo -e "    ${DIM}${GRAY}$*${RESET}"; }
die()       { log_error "$*"; echo ""; exit 1; }

phase() {
    echo ""
    hr "━" "$BOLD$BLUE"
    echo -e "  ${BG_BLUE}${LWHITE}${BOLD}  $*  ${RESET}"
    hr "━" "$BOLD$BLUE"
    echo ""
}

# ══════════════════════════════════════════════════════════════════════════════
#  § ENVIRONMENT DETECTION  — detects Termux vs normal Linux/macOS
# ══════════════════════════════════════════════════════════════════════════════
IS_TERMUX=false
if [[ -n "${TERMUX_VERSION:-}" ]] || [[ -d "/data/data/com.termux" ]]; then
    IS_TERMUX=true
fi

# Wraps sudo — no-op on Termux since sudo doesn't exist there
maybe_sudo() { $IS_TERMUX && "$@" || sudo "$@"; }

# ══════════════════════════════════════════════════════════════════════════════
#  § OS DETECTION
# ══════════════════════════════════════════════════════════════════════════════
detect_os() {
    local id="" id_like="" pretty=""
    if [[ -f /etc/os-release ]]; then
        source /etc/os-release          # sets ID, ID_LIKE, PRETTY_NAME
        id="${ID:-}"; id_like="${ID_LIKE:-}"; pretty="${PRETTY_NAME:-}"
    elif [[ -f /etc/debian_version ]]; then id="debian"
    elif [[ -f /etc/arch-release ]];    then id="arch"
    elif [[ -f /etc/fedora-release ]];  then id="fedora"
    elif command -v uname &>/dev/null;  then id="$(uname -s | tr '[:upper:]' '[:lower:]')"
    fi
    # Override for Termux / Android — no /etc/os-release exists there
    if $IS_TERMUX; then id="android"; pretty="Android (Termux)"; fi
    echo "${id}|${id_like}|${pretty}"
}

# ══════════════════════════════════════════════════════════════════════════════
#  § OS THEMES
#  Each line:  "<ascii_file> <ascii_color> <title_color> <key_color> <value_color>"
#
#  TO ADD A NEW DISTRO:
#    1. Add a case entry:   yourdistro)  echo "yourdistro.vtxt <color> <color> <color> white" ;;
#    2. Drop the matching   yourdistro.vtxt  into the art/ folder
#    3. Valid colors: black red green yellow blue magenta cyan white
#                    bright_black bright_red bright_green bright_yellow
#                    bright_blue bright_magenta bright_cyan bright_white
# ══════════════════════════════════════════════════════════════════════════════
distro_theme() {
    local id="${1,,}" id_like="${2,,}"
    case "$id" in
        # ── Debian family ──────────────────────────────────────────────────
        debian)                 echo "debian.vtxt red red red white" ;;
        ubuntu)                 echo "ubuntu.vtxt yellow yellow yellow white" ;;
        linuxmint|mint)         echo "mint.vtxt green green green white" ;;
        pop|pop_os)             echo "pop.vtxt cyan cyan cyan white" ;;
        elementary)             echo "elementary.vtxt blue blue blue white" ;;
        kali)                   echo "kali.vtxt blue blue blue white" ;;
        raspbian)               echo "raspbian.vtxt red red red white" ;;
        mxlinux|mx)             echo "mx.vtxt blue blue blue white" ;;
        zorin)                  echo "zorin.vtxt blue blue blue white" ;;
        # ── Arch family ────────────────────────────────────────────────────
        arch)                   echo "arch.vtxt cyan cyan cyan white" ;;
        manjaro)                echo "manjaro.vtxt green green green white" ;;
        endeavouros|endeavour)  echo "endeavouros.vtxt magenta magenta magenta white" ;;
        garuda)                 echo "garuda.vtxt magenta magenta magenta white" ;;
        artix)                  echo "artix.vtxt cyan cyan cyan white" ;;
        blackarch)              echo "blackarch.vtxt red red red white" ;;
        # ── Red Hat family ─────────────────────────────────────────────────
        fedora)                 echo "fedora.vtxt blue blue blue white" ;;
        rhel)                   echo "rhel.vtxt red red red white" ;;
        centos)                 echo "centos.vtxt yellow yellow yellow white" ;;
        almalinux|alma)         echo "alma.vtxt yellow yellow yellow white" ;;
        rocky)                  echo "rocky.vtxt green green green white" ;;
        # ── SUSE ───────────────────────────────────────────────────────────
        opensuse*|suse)         echo "opensuse.vtxt green green green white" ;;
        # ── Other Linux ────────────────────────────────────────────────────
        gentoo)                 echo "gentoo.vtxt magenta magenta magenta white" ;;
        void)                   echo "void.vtxt green green green white" ;;
        nixos)                  echo "nixos.vtxt blue blue blue white" ;;
        alpine)                 echo "alpine.vtxt blue blue blue white" ;;
        slackware)              echo "slackware.vtxt blue blue blue white" ;;
        # ── Android / Termux ───────────────────────────────────────────────
        android)                echo "android.vtxt green green green white" ;;
        # ── BSD / macOS ────────────────────────────────────────────────────
        darwin|macos|macosx)    echo "macos.vtxt white white cyan white" ;;
        freebsd)                echo "freebsd.vtxt red red red white" ;;
        netbsd)                 echo "netbsd.vtxt yellow yellow yellow white" ;;
        openbsd)                echo "openbsd.vtxt yellow yellow yellow white" ;;
        # ── Fallback via ID_LIKE chain ──────────────────────────────────────
        *)
            if   [[ "$id_like" == *"debian"* || "$id_like" == *"ubuntu"* ]]; then
                echo "debian.vtxt red red red white"
            elif [[ "$id_like" == *"arch"* ]];  then
                echo "arch.vtxt cyan cyan cyan white"
            elif [[ "$id_like" == *"fedora"* || "$id_like" == *"rhel"* ]]; then
                echo "fedora.vtxt blue blue blue white"
            elif [[ "$id_like" == *"suse"* ]];  then
                echo "opensuse.vtxt green green green white"
            else
                # Unknown distro — VamoraSys generic fallback
                echo "ascii1.vtxt blue bright_blue bright_blue white"
            fi ;;
    esac
}

# ══════════════════════════════════════════════════════════════════════════════
#  § CONFIG GENERATOR
#  TO ADD/REMOVE A MODULE: edit the [modules] block below.
#  TO CHANGE MODULE ORDER: edit the module_order array below.
#  Format must be valid TOML — strings in quotes, booleans lowercase.
# ══════════════════════════════════════════════════════════════════════════════
generate_config() {
    local ascii_file="$1" ascii_color="$2" title_color="$3"
    local key_color="$4" value_color="$5"

    cat > "$CONFIG_FILE" <<EOF
ascii_file = "${ascii_file}"
ascii_color = "${ascii_color}"
title_color = "${title_color}"
key_color = "${key_color}"
value_color = "${value_color}"
separator = "-"
mini_mode = false
show_title = true
show_separator = true
module_order = [
    "color_blocks_big",
    "vamorasys_version",
    "vmf_version",
    "vamora_version_codename",
    "hostname",
    "kernel",
    "bios",
    "android_version",
    "android_device",
    "cpu",
    "gpu",
    "ram",
    "disk",
    "battery",
    "bluetooth",
    "uptime",
    "sys_age",
    "shell",
    "terminal",
    "tty_type",
    "desktop",
    "display_server",
    "resolution",
    "theme",
    "fs_type",
    "sudo_status",
    "local_ip",
    "public_ip",
    "network",
    "birthday_countdown",
    "quotes",
    "jokes",
    "color_blocks_small",
    "os",
]

[greetings]
enabled = false
birthday = ""
events = []

[modules]
hostname              = true
os                    = true
kernel                = true
bios                  = true
cpu                   = true
gpu                   = true
ram                   = true
disk                  = true
uptime                = false
shell                 = true
terminal              = true
desktop               = true
resolution            = true
display_server        = true
theme                 = true
tty_type              = true
fs_type               = true
sys_age               = true
local_ip              = true
public_ip             = false
network               = true
bluetooth             = true
battery               = true
android_version       = true
android_device        = true
sudo_status           = true
birthday_countdown    = false
vamorasys_version     = true
vmf_version           = true
vamora_version_codename = true
quotes                = false
jokes                 = false
color_blocks_big      = true
color_blocks_small    = false
EOF
}

# ══════════════════════════════════════════════════════════════════════════════
#  § DONE SCREEN
# ══════════════════════════════════════════════════════════════════════════════
done_screen() {
    local os_pretty="$1" afile="$2" acolor="$3"
    echo ""
    hr "═" "$BOLD$LGREEN"
    echo ""
    echo -e "${BOLD}${LGREEN}"
    center "██████╗  ██████╗ ███╗   ██╗███████╗██╗"
    center "██╔══██╗██╔═══██╗████╗  ██║██╔════╝██║"
    center "██║  ██║██║   ██║██╔██╗ ██║█████╗  ██║"
    center "██║  ██║██║   ██║██║╚██╗██║██╔══╝  ╚═╝"
    center "██████╔╝╚██████╔╝██║ ╚████║███████╗██╗"
    center "╚═════╝  ╚═════╝ ╚═╝  ╚═══╝╚══════╝╚═╝"
    echo -e "${RESET}"
    echo ""
    hr "─" "$DIM$LGREEN"
    echo ""
    echo -e "  ${DIM}${GRAY}System   ${RESET}  ${WHITE}${os_pretty}${RESET}"
    echo -e "  ${DIM}${GRAY}Binary   ${RESET}  ${LGREEN}${BOLD}${BIN_DIR}/${BINARY_NAME}${RESET}"
    echo -e "  ${DIM}${GRAY}Art      ${RESET}  ${LCYAN}${ART_DIR}/${RESET}"
    echo -e "  ${DIM}${GRAY}Config   ${RESET}  ${LCYAN}${CONFIG_FILE}${RESET}"
    echo -e "  ${DIM}${GRAY}Theme    ${RESET}  ${LYELLOW}${afile}${RESET}  ${DIM}(${acolor})${RESET}"
    echo ""
    hr "─" "$DIM$LGREEN"
    echo ""
    echo -e "  ${BOLD}${LWHITE}Commands:${RESET}"
    echo ""
    echo -e "  ${BG_BLACK}${BOLD}${LCYAN}  $ ${LGREEN}${BINARY_NAME}${RESET}${BG_BLACK}                — show system info      ${RESET}"
    echo -e "  ${BG_BLACK}${BOLD}${LCYAN}  $ ${LGREEN}${BINARY_NAME} config${RESET}${BG_BLACK}         — interactive setup      ${RESET}"
    echo -e "  ${BG_BLACK}${BOLD}${LCYAN}  $ ${LGREEN}${BINARY_NAME} --mini${RESET}${BG_BLACK}          — minimal quick view     ${RESET}"
    echo -e "  ${BG_BLACK}${BOLD}${LCYAN}  $ ${LGREEN}${BINARY_NAME} --help${RESET}${BG_BLACK}          — all commands           ${RESET}"
    echo ""
    hr "═" "$BOLD$LGREEN"
    echo ""
}

# ══════════════════════════════════════════════════════════════════════════════
#  MAIN
# ══════════════════════════════════════════════════════════════════════════════
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# ── Banner ────────────────────────────────────────────────────────────────────
clear 2>/dev/null || true
echo ""
echo -e "${BOLD}${LCYAN}"
center "██╗   ██╗ █████╗ ███╗   ███╗ ██████╗ ██████╗  █████╗ "
center "██║   ██║██╔══██╗████╗ ████║██╔═══██╗██╔══██╗██╔══██╗"
center "██║   ██║███████║██╔████╔██║██║   ██║██████╔╝███████║"
center "╚██╗ ██╔╝██╔══██║██║╚██╔╝██║██║   ██║██╔══██╗██╔══██║"
center " ╚████╔╝ ██║  ██║██║ ╚═╝ ██║╚██████╔╝██║  ██║██║  ██║"
center "  ╚═══╝  ╚═╝  ╚═╝╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝"
echo -e "${RESET}"
echo ""
hr "═" "$BOLD$CYAN"
center "vaminfo  ·  installer" "${DIM}${LCYAN}"
hr "═" "$BOLD$CYAN"
echo ""

# ── Phase 1 : Rust toolchain ──────────────────────────────────────────────────
phase "[ 1 / 4 ]  RUST TOOLCHAIN"

if ! command -v cargo &>/dev/null; then
    log_warn "Rust not found — installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
    # shellcheck source=/dev/null
    source "$HOME/.cargo/env"
    log_ok "Rust installed: $(rustc --version)"
else
    log_ok "Rust found: ${BOLD}$(rustc --version)${RESET}"
fi

if $IS_TERMUX; then
    log_info "Environment  →  ${BOLD}${LYELLOW}Termux (Android)${RESET}"
else
    log_info "Environment  →  ${BOLD}Linux / macOS${RESET}"
fi
log_info "Install target  →  ${BOLD}${BIN_DIR}${RESET}"

divider

# ── Phase 2 : Build ───────────────────────────────────────────────────────────
phase "[ 2 / 4 ]  COMPILING"

log_info "Running ${BOLD}cargo build --release${RESET} …"
echo ""
(cd "$SCRIPT_DIR" && cargo build --release 2>&1) | sed 's/^/    /'
echo ""

BINARY="$SCRIPT_DIR/target/release/$BINARY_NAME"
[[ ! -f "$BINARY" ]] && die "Binary not found at $BINARY — build may have failed."

log_ok "Binary ready  →  ${BOLD}${BINARY_NAME}${RESET}"
log_dim "$(du -sh "$BINARY" | cut -f1) on disk"
divider

# ── Phase 3 : Install files ───────────────────────────────────────────────────
phase "[ 3 / 4 ]  INSTALLING FILES"

# Binary
log_info "Copying binary to ${BOLD}${BIN_DIR}${RESET} …"
if [[ -w "$BIN_DIR" ]]; then
    cp "$BINARY" "$BIN_DIR/$BINARY_NAME"
    chmod +x "$BIN_DIR/$BINARY_NAME"
else
    log_warn "Need elevated privileges to write to ${BIN_DIR}"
    maybe_sudo cp "$BINARY" "$BIN_DIR/$BINARY_NAME"
    maybe_sudo chmod +x "$BIN_DIR/$BINARY_NAME"
fi
log_ok "Binary installed  →  ${BOLD}${BIN_DIR}/${BINARY_NAME}${RESET}"
echo ""

# Directories
mkdir -p "$VAMINFO_DIR" "$ART_DIR"
log_ok "Directories created"

# Art (cp -n = don't overwrite files the user may have customised)
if [[ -d "$SCRIPT_DIR/art" ]]; then
    log_info "Copying art assets …"
    cp -n "$SCRIPT_DIR/art/"*.vtxt "$ART_DIR/" 2>/dev/null || true
    ART_COUNT=$(find "$ART_DIR" -name "*.vtxt" | wc -l | tr -d ' ')
    log_ok "Art installed  →  ${BOLD}${ART_DIR}${RESET}  ${DIM}(${ART_COUNT} files)${RESET}"
else
    log_warn "No art/ folder found at ${SCRIPT_DIR}/art — skipping"
fi

divider

# ── Phase 4 : OS detection & config ───────────────────────────────────────────
phase "[ 4 / 4 ]  DETECTING OS & WRITING CONFIG"

IFS='|' read -r OS_ID OS_ID_LIKE OS_PRETTY <<< "$(detect_os)"
DISPLAY_OS="${OS_PRETTY:-$OS_ID}"

echo ""
echo -e "    ${BOLD}${LCYAN}┌─────────────────────────────────────┐${RESET}"
printf  "    ${BOLD}${LCYAN}│${RESET}  %-35s ${BOLD}${LCYAN}│${RESET}\n" "${DISPLAY_OS}"
printf  "    ${BOLD}${LCYAN}│${RESET}  ${DIM}id: %-10s  id_like: %-14s${RESET} ${BOLD}${LCYAN}│${RESET}\n" "${OS_ID}" "${OS_ID_LIKE:-none}"
echo -e "    ${BOLD}${LCYAN}└─────────────────────────────────────┘${RESET}"
echo ""

read -r AFILE ACOLOR TCOLOR KCOLOR VCOLOR <<< "$(distro_theme "$OS_ID" "$OS_ID_LIKE")"

if [[ ! -f "$CONFIG_FILE" ]]; then
    log_info "Generating config …"
    generate_config "$AFILE" "$ACOLOR" "$TCOLOR" "$KCOLOR" "$VCOLOR"
    log_ok "Config written  →  ${BOLD}${CONFIG_FILE}${RESET}"
else
    log_info "Config already exists — skipping  ${DIM}(delete it to regenerate)${RESET}"
fi

echo ""
echo -e "  ${BOLD}${LCYAN}Theme applied:${RESET}"
echo -e "  ${DIM}${GRAY}  ascii_file   ${RESET}  ${LYELLOW}${AFILE}${RESET}"
echo -e "  ${DIM}${GRAY}  ascii_color  ${RESET}  ${LMAGENTA}${ACOLOR}${RESET}"
echo -e "  ${DIM}${GRAY}  title_color  ${RESET}  ${LMAGENTA}${TCOLOR}${RESET}"
echo -e "  ${DIM}${GRAY}  key_color    ${RESET}  ${LMAGENTA}${KCOLOR}${RESET}"
echo -e "  ${DIM}${GRAY}  value_color  ${RESET}  ${LMAGENTA}${VCOLOR}${RESET}"
echo ""

# ── PATH verification ─────────────────────────────────────────────────────────
if command -v "$BINARY_NAME" &>/dev/null; then
    log_ok "${BINARY_NAME} is on PATH and ready"
else
    log_warn "${BINARY_NAME} installed but may not be on PATH yet"
    log_warn "Add this to your shell profile (~/.bashrc / ~/.zshrc):"
    echo ""
    echo -e "    ${LYELLOW}export PATH=\"${BIN_DIR}:\$PATH\"${RESET}"
    echo ""
fi

# ── Done ──────────────────────────────────────────────────────────────────────
done_screen "$DISPLAY_OS" "$AFILE" "$ACOLOR"
