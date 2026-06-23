# -- Detect distro ------------------------------------------------------------
ASCII_FILE="ascii1.vtxt"
ASCII_COLOR="blue"

if [ -f /etc/os-release ]; then
    . /etc/os-release

    case "$ID" in
        debian)
            ASCII_FILE="debian.vtxt"
            ASCII_COLOR="red"
            ;;
        arch)
            ASCII_FILE="arch.vtxt"
            ASCII_COLOR="blue"
            ;;
    esac
fi
