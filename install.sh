#!/usr/bin/env bash

# ░▒▓ VamoraSys — vaminfo Installer ▓▒░

set -euo pipefail

# ══════════════════════════════════════════════════════════════════
#  COLORS & STYLES
# ══════════════════════════════════════════════════════════════════
RESET='\033[0m'
BOLD='\033[1m'
DIM='\033[2m'
ITALIC='\033[3m'
BLINK='\033[5m'

BLACK='\033[0;30m';   GRAY='\033[1;30m'
RED='\033[0;31m';     LRED='\033[1;31m'
GREEN='\033[0;32m';   LGREEN='\033[1;32m'
YELLOW='\033[0;33m';  LYELLOW='\033[1;33m'
BLUE='\033[0;34m';    LBLUE='\033[1;34m'
MAGENTA='\033[0;35m'; LMAGENTA='\033[1;35m'
CYAN='\033[0;36m';    LCYAN='\033[1;36m'
WHITE='\033[0;37m';   LWHITE='\033[1;37m'

# Background
BG_BLACK='\033[40m';  BG_BLUE='\033[44m'; BG_CYAN='\033[46m'

# ══════════════════════════════════════════════════════════════════
#  TERMINAL WIDTH
# ══════════════════════════════════════════════════════════════════
COLS=$(tput cols 2>/dev/null || echo 80)

# ══════════════════════════════════════════════════════════════════
#  PRINT HELPERS
# ══════════════════════════════════════════════════════════════════

# Print a centered line (plain text, no color codes in $1)
center() {
    local text="$1"
    local color="${2:-}"
    local len=${#text}
    local pad=$(( (COLS - len) / 2 ))
    printf "%${pad}s" ""
    echo -e "${color}${text}${RESET}"
}

# Horizontal rule
hr() {
    local char="${1:-─}"
    local color="${2:-$DIM}"
    local line=""
    for ((i=0; i<COLS; i++)); do line+="$char"; done
    echo -e "${color}${line}${RESET}"
}

# Thin divider
divider() { hr "·" "$DIM$BLUE"; }

# Log helpers
log_info()    { echo -e "  ${LBLUE}${BOLD}  ◆${RESET}  ${WHITE}$*${RESET}"; }
log_ok()      { echo -e "  ${LGREEN}${BOLD}  ✔${RESET}  ${LGREEN}$*${RESET}"; }
log_warn()    { echo -e "  ${LYELLOW}${BOLD}  ⚠${RESET}  ${LYELLOW}$*${RESET}"; }
log_error()   { echo -e "  ${LRED}${BOLD}  ✘${RESET}  ${LRED}$*${RESET}" >&2; }
log_dim()     { echo -e "    ${DIM}${GRAY}$*${RESET}"; }
die()         { log_error "$*"; echo ""; exit 1; }

# Phase header  ── big styled section title
phase() {
    local title="$1"
    echo ""
    hr "━" "$BOLD$BLUE"
    echo -e "  ${BG_BLUE}${LWHITE}${BOLD}  $title  ${RESET}"
    hr "━" "$BOLD$BLUE"
    echo ""
}

# Animated spinner for long-running commands
spinner_run() {
    local label="$1"; shift
    local frames=('⠋' '⠙' '⠹' '⠸' '⠼' '⠴' '⠦' '⠧' '⠇' '⠏')
    local i=0

    # Run command in background, capture output
    local tmpout; tmpout=$(mktemp)
    "$@" >"$tmpout" 2>&1 &
    local pid=$!

    while kill -0 "$pid" 2>/dev/null; do
        printf "\r  ${LCYAN}${BOLD}%s${RESET}  ${WHITE}%s${RESET}  " "${frames[$i]}" "$label"
        i=$(( (i+1) % ${#frames[@]} ))
        sleep 0.08
    done

    wait "$pid"
    local exit_code=$?
    printf "\r%${COLS}s\r" ""   # clear spinner line

    if [[ $exit_code -eq 0 ]]; then
        log_ok "$label"
    else
        log_error "$label — FAILED"
        cat "$tmpout" | sed 's/^/    /'
        rm -f "$tmpout"
        exit $exit_code
    fi

    rm -f "$tmpout"
}

# Fake-but-real progress bar (used while cargo compiles)
progress_bar() {
    local label="$1"
    local duration="${2:-3}"
    local width=40
    local steps=$((width))
    local delay; delay=$(echo "scale=4; $duration / $steps" | bc 2>/dev/null || echo "0.07")

    echo -e "  ${DIM}${GRAY}$label${RESET}"
    printf "  ${LBLUE}["
    for ((i=0; i<steps; i++)); do
        printf "${LGREEN}█${RESET}"
        sleep "$delay" 2>/dev/null || true
    done
    printf "${LBLUE}]${RESET}  ${LGREEN}${BOLD}done${RESET}\n"
}

# ══════════════════════════════════════════════════════════════════
#  BANNER
# ══════════════════════════════════════════════════════════════════
banner() {
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
    echo -e "${BOLD}${BLUE}"
    center "███████╗██╗   ██╗███████╗"
    center "██╔════╝╚██╗ ██╔╝██╔════╝"
    center "███████╗ ╚████╔╝ ███████╗"
    center "╚════██║  ╚██╔╝  ╚════██║"
    center "███████║   ██║   ███████║"
    center "╚══════╝   ╚═╝   ╚══════╝"
    echo -e "${RESET}"
    echo ""
    hr "═" "$BOLD$CYAN"
    center "vaminfo  ·  installer" "${DIM}${LCYAN}"
    center "VamoraSys  v1.0" "${DIM}${WHITE}"
    hr "═" "$BOLD$CYAN"
    echo ""
}

# ══════════════════════════════════════════════════════════════════
#  OS DETECTION
# ══════════════════════════════════════════════════════════════════
detect_os() {
    local id="" id_like="" pretty=""
    if [[ -f /etc/os-release ]]; then
        # shellcheck source=/dev/null
        source /etc/os-release
        id="${ID:-}"
        id_like="${ID_LIKE:-}"
        pretty="${PRETTY_NAME:-}"
    elif [[ -f /etc/debian_version ]]; then id="debian"
    elif [[ -f /etc/arch-release ]];    then id="arch"
    elif [[ -f /etc/fedora-release ]];  then id="fedora"
    elif command -v uname &>/dev/null;  then id="$(uname -s | tr '[:upper:]' '[:lower:]')"
    fi
    echo "${id}|${id_like}|${pretty}"
}

distro_theme() {
    local id="$1" id_like="$2"
    id="${id,,}"; id_like="${id_like,,}"
    case "$id" in
        debian)                    echo "debian.vtxt red red red white" ;;
        ubuntu)                    echo "ubuntu.vtxt yellow yellow yellow white" ;;
        linuxmint|mint)            echo "mint.vtxt green green green white" ;;
        pop|pop_os)                echo "pop.vtxt cyan cyan cyan white" ;;
        elementary)                echo "elementary.vtxt blue blue blue white" ;;
        kali)                      echo "kali.vtxt blue blue blue white" ;;
        raspbian)                  echo "raspbian.vtxt red red red white" ;;
        mxlinux|mx)                echo "mx.vtxt blue blue blue white" ;;
        zorin)                     echo "zorin.vtxt blue blue blue white" ;;
        arch)                      echo "arch.vtxt cyan cyan cyan white" ;;
        manjaro)                   echo "manjaro.vtxt green green green white" ;;
        endeavouros|endeavour)     echo "endeavouros.vtxt magenta magenta magenta white" ;;
        garuda)                    echo "garuda.vtxt magenta magenta magenta white" ;;
        artix)                     echo "artix.vtxt cyan cyan cyan white" ;;
        blackarch)                 echo "blackarch.vtxt red red red white" ;;
        fedora)                    echo "fedora.vtxt blue blue blue white" ;;
        rhel)                      echo "rhel.vtxt red red red white" ;;
        centos)                    echo "centos.vtxt yellow yellow yellow white" ;;
        almalinux|alma)            echo "alma.vtxt yellow yellow yellow white" ;;
        rocky)                     echo "rocky.vtxt green green green white" ;;
        opensuse*|suse)            echo "opensuse.vtxt green green green white" ;;
        gentoo)                    echo "gentoo.vtxt magenta magenta magenta white" ;;
        void)                      echo "void.vtxt green green green white" ;;
        nixos)                     echo "nixos.vtxt blue blue blue white" ;;
        alpine)                    echo "alpine.vtxt blue blue blue white" ;;
        slackware)                 echo "slackware.vtxt blue blue blue white" ;;
        darwin|macos|macosx)       echo "macos.vtxt white white cyan white" ;;
        freebsd)                   echo "freebsd.vtxt red red red white" ;;
        netbsd)                    echo "netbsd.vtxt yellow yellow yellow white" ;;
        openbsd)                   echo "openbsd.vtxt yellow yellow yellow white" ;;
        *)
            if   [[ "$id_like" == *"debian"* || "$id_like" == *"ubuntu"* ]]; then
                echo "debian.vtxt red red red white"
            elif [[ "$id_like" == *"arch"* ]]; then
                echo "arch.vtxt cyan cyan cyan white"
            elif [[ "$id_like" == *"fedora"* || "$id_like" == *"rhel"* ]]; then
                echo "fedora.vtxt blue blue blue white"
            elif [[ "$id_like" == *"suse"* ]]; then
                echo "opensuse.vtxt green green green white"
            else
                echo "vamora1.vtxt blue bright_blue bright_blue white"
            fi ;;
    esac
}

# ══════════════════════════════════════════════════════════════════
#  CONFIG GENERATOR
# ══════════════════════════════════════════════════════════════════
generate_config() {
    local ascii_file="$1" ascii_color="$2" title_color="$3"
    local key_color="$4" value_color="$5" config_path="$6"
    cat > "$config_path" <<EOF
ascii_file = "${ascii_file}"
ascii_color = "${ascii_color}"
title_color = "${title_color}"
key_color = "${key_color}"
value_color = "${value_color}"
separator = "-"
mini_mode = false
mini_mode_ascii = false
show_title = true
show_separator = true

[modules]
hostname = true
os = true
kernel = true
bios = true
cpu = true
gpu = true
ram = true
disk = true
uptime = false
shell = true
terminal = true
desktop = true
resolution = true
theme = true
local_ip = true
bluetooth = true
battery = true
network = true
media = false
color_blocks_big = true
color_blocks_small = false
EOF
}

# ══════════════════════════════════════════════════════════════════
#  DONE SCREEN
# ══════════════════════════════════════════════════════════════════
done_screen() {
    local binname="$1" bindir="$2" os_pretty="$3"
    local afile="$4" acolor="$5"

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
    echo -e "  ${DIM}${GRAY}Binary   ${RESET}  ${LGREEN}${BOLD}${bindir}/${binname}${RESET}"
    echo -e "  ${DIM}${GRAY}Art      ${RESET}  ${LCYAN}~/.VamoraSys/apps/vaminfo/art/${RESET}"
    echo -e "  ${DIM}${GRAY}Config   ${RESET}  ${LCYAN}~/.VamoraSys/apps/vaminfo/config.vmf${RESET}"
    echo -e "  ${DIM}${GRAY}Theme    ${RESET}  ${LYELLOW}${afile}${RESET}  ${DIM}(${acolor})${RESET}"

    echo ""
    hr "─" "$DIM$LGREEN"
    echo ""
    echo -e "  ${BOLD}${LWHITE}Launch it anytime:${RESET}"
    echo ""
    echo -e "  ${BG_BLACK}${BOLD}${LCYAN}  $ ${LGREEN}${binname}${RESET}${BG_BLACK}                                  ${RESET}"
    echo ""
    hr "═" "$BOLD$LGREEN"
    echo ""
}

# ══════════════════════════════════════════════════════════════════
#  ENVIRONMENT DETECTION
# ══════════════════════════════════════════════════════════════════
IS_TERMUX=false
if [[ -n "${TERMUX_VERSION:-}" ]] || [[ -d "/data/data/com.termux" ]]; then
    IS_TERMUX=true
fi

# Pick the right bin dir and sudo wrapper
pick_bin_dir() {
    if $IS_TERMUX; then
        # Termux exposes $PREFIX; fall back to the well-known path
        echo "${PREFIX:-/data/data/com.termux/files/usr}/bin"
    elif [[ -w "/usr/local/bin" ]]; then
        echo "/usr/local/bin"
    elif [[ -w "/bin" ]]; then
        echo "/bin"
    else
        echo "/usr/local/bin"   # will use sudo below
    fi
}

# On Termux sudo doesn't exist — this wrapper is a no-op there
maybe_sudo() {
    if $IS_TERMUX; then
        "$@"
    else
        sudo "$@"
    fi
}

# ══════════════════════════════════════════════════════════════════
#  MAIN
# ══════════════════════════════════════════════════════════════════
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

banner

# ─── PHASE 1 : Preflight ──────────────────────────────────────
phase "[ 1 / 4 ]  PREFLIGHT CHECKS"

for cmd in cargo cp mkdir bc; do
    if command -v "$cmd" &>/dev/null; then
        log_ok "${BOLD}${cmd}${RESET}${LGREEN}  →  $(command -v "$cmd")"
    else
        [[ "$cmd" == "bc" ]] && { log_warn "bc not found — spinner timing may be imprecise"; continue; }
        die "'${cmd}' is not installed. Please install it and re-run."
    fi
done

BIN_DIR="$(pick_bin_dir)"

if $IS_TERMUX; then
    log_info "Environment     →  ${BOLD}${LYELLOW}Termux (Android)${RESET}"
    log_info "Install target  →  ${BOLD}${BIN_DIR}${RESET}  ${DIM}(Termux prefix)${RESET}"
else
    log_info "Environment     →  ${BOLD}Linux / macOS${RESET}"
    log_info "Install target  →  ${BOLD}${BIN_DIR}${RESET}"
fi

if [[ ! -f "${SCRIPT_DIR}/Cargo.toml" ]]; then
    die "No Cargo.toml found in ${SCRIPT_DIR}"
fi
log_ok "Cargo.toml found"

divider

# ─── PHASE 2 : Build ──────────────────────────────────────────
phase "[ 2 / 4 ]  COMPILING RUST PROJECT"

log_info "Running ${BOLD}cargo build --release${RESET} …"
echo ""

# Run cargo, stream output indented, then show progress bar
(cd "$SCRIPT_DIR" && cargo build --release 2>&1) | sed 's/^/    /'

echo ""
log_ok "Compilation finished"

BINARY_NAME="$(cd "$SCRIPT_DIR" && cargo metadata --no-deps --format-version 1 \
    | grep -o '"name":"[^"]*"' | head -1 | cut -d'"' -f4)"
BINARY_PATH="${SCRIPT_DIR}/target/release/${BINARY_NAME}"

if [[ ! -f "$BINARY_PATH" ]]; then
    BINARY_PATH="$(find "${SCRIPT_DIR}/target/release" -maxdepth 1 -type f -executable | head -1)"
    [[ -z "$BINARY_PATH" ]] && die "Could not locate compiled binary in target/release/"
    BINARY_NAME="$(basename "$BINARY_PATH")"
fi

log_ok "Binary ready  →  ${BOLD}${BINARY_NAME}${RESET}"
log_dim "$(du -sh "$BINARY_PATH" | cut -f1) on disk"
divider

# ─── PHASE 3 : Install files ──────────────────────────────────
phase "[ 3 / 4 ]  INSTALLING FILES"

# Binary
log_info "Copying binary to ${BOLD}${BIN_DIR}${RESET} …"
if [[ -w "$BIN_DIR" ]]; then
    cp "$BINARY_PATH" "${BIN_DIR}/${BINARY_NAME}"
    chmod +x "${BIN_DIR}/${BINARY_NAME}"
else
    log_warn "Need elevated privileges to write to ${BIN_DIR}"
    maybe_sudo cp "$BINARY_PATH" "${BIN_DIR}/${BINARY_NAME}"
    maybe_sudo chmod +x "${BIN_DIR}/${BINARY_NAME}"
fi
log_ok "Binary installed  →  ${BOLD}${BIN_DIR}/${BINARY_NAME}${RESET}"

echo ""

# Art
ART_SRC="${SCRIPT_DIR}/art"
ART_DST="${HOME}/.VamoraSys/apps/vaminfo/art"

if [[ ! -d "$ART_SRC" ]]; then
    log_warn "No 'art/' folder found at ${ART_SRC} — skipping"
else
    log_info "Copying art assets …"
    mkdir -p "$ART_DST"
    cp -r "${ART_SRC}/." "$ART_DST/"
    ART_COUNT=$(find "$ART_DST" -type f | wc -l | tr -d ' ')
    log_ok "Art installed  →  ${BOLD}${ART_DST}${RESET}  ${DIM}(${ART_COUNT} files)${RESET}"
fi

divider

# ─── PHASE 4 : OS detection & config ─────────────────────────
phase "[ 4 / 4 ]  DETECTING OS & WRITING CONFIG"

IFS='|' read -r OS_ID OS_ID_LIKE OS_PRETTY <<< "$(detect_os)"
DISPLAY_OS="${OS_PRETTY:-$OS_ID}"

echo ""
echo -e "  ${DIM}${GRAY}Detected system:${RESET}"
echo ""

# Draw a little OS card
PAD="    "
echo -e "${PAD}${BOLD}${LCYAN}┌─────────────────────────────────────┐${RESET}"
printf  "${PAD}${BOLD}${LCYAN}│${RESET}  %-35s ${BOLD}${LCYAN}│${RESET}\n" "$(echo -e "${LWHITE}${BOLD}${DISPLAY_OS}${RESET}")"
printf  "${PAD}${BOLD}${LCYAN}│${RESET}  ${DIM}%-35s${RESET} ${BOLD}${LCYAN}│${RESET}\n" "id: ${OS_ID}   id_like: ${OS_ID_LIKE:-none}"
echo -e "${PAD}${BOLD}${LCYAN}└─────────────────────────────────────┘${RESET}"
echo ""

read -r AFILE ACOLOR TCOLOR KCOLOR VCOLOR <<< "$(distro_theme "$OS_ID" "$OS_ID_LIKE")"

CONFIG_DIR="${HOME}/.VamoraSys/apps/vaminfo"
CONFIG_FILE="${CONFIG_DIR}/config.vmf"
mkdir -p "$CONFIG_DIR"
generate_config "$AFILE" "$ACOLOR" "$TCOLOR" "$KCOLOR" "$VCOLOR" "$CONFIG_FILE"

log_ok "Config written  →  ${BOLD}${CONFIG_FILE}${RESET}"
echo ""

# Theme preview table
echo -e "  ${BOLD}${LCYAN}Theme applied:${RESET}"
echo ""
echo -e "  ${DIM}${GRAY}  ascii_file   ${RESET}  ${LYELLOW}${AFILE}${RESET}"
echo -e "  ${DIM}${GRAY}  ascii_color  ${RESET}  ${LMAGENTA}${ACOLOR}${RESET}"
echo -e "  ${DIM}${GRAY}  title_color  ${RESET}  ${LMAGENTA}${TCOLOR}${RESET}"
echo -e "  ${DIM}${GRAY}  key_color    ${RESET}  ${LMAGENTA}${KCOLOR}${RESET}"
echo -e "  ${DIM}${GRAY}  value_color  ${RESET}  ${LMAGENTA}${VCOLOR}${RESET}"
echo ""

# ─── DONE ─────────────────────────────────────────────────────
done_screen "$BINARY_NAME" "$BIN_DIR" "$DISPLAY_OS" "$AFILE" "$ACOLOR"
