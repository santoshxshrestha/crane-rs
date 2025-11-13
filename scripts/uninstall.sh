#!/usr/bin/env bash
set -e

GREEN="\033[0;32m"
YELLOW="\033[1;33m"
CYAN="\033[0;36m"
RED="\033[0;31m"
BOLD="\033[1m"
RESET="\033[0m"
CHECK="${GREEN}✅${RESET}"
FAIL="${RED}❌${RESET}"
INFO="${CYAN}➜${RESET}"

DRY_RUN=false

for arg in "$@"; do
    case $arg in
    --dry-run)
        DRY_RUN=true
        shift
        ;;
    -h | --help)
        echo "Usage: $0 [options]"
        echo "Options:"
        echo "  --dry-run       Show what would be done without making changes"
        echo "  -h, --help      Show this help message"
        exit 0
        ;;
    *) ;;
    esac
done

if [ "$DRY_RUN" = true ]; then
    echo -e "${YELLOW}DRY RUN MODE - No changes will be made${RESET}"
    echo
fi

echo -e "${BOLD}${CYAN}"
echo "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓"
echo "┃           crane-rs Uninstaller       ┃"
echo "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛"
echo -e "${RESET}"

INSTALL_PATH="/usr/local/bin/crane-rs"
REPO_DIR="$HOME/crane-rs"

echo -e "${INFO} Removing crane-rs binary..."

if [ ! -f "$INSTALL_PATH" ]; then
    echo -e "${FAIL} crane-rs binary not found at ${INSTALL_PATH}"
else
    if [ "$DRY_RUN" = true ]; then
        echo -e "${YELLOW}[DRY RUN] Would remove crane-rs binary from ${INSTALL_PATH}${RESET}"
    else
        sudo rm -f "$INSTALL_PATH"
        echo -e "${CHECK} Removed crane-rs binary from ${INSTALL_PATH}"
    fi
fi

echo -e "${INFO} Removing crane-rs repository..."

if [ ! -d "$REPO_DIR" ]; then
    echo -e "${YELLOW}Warning: crane-rs repository not found at ${REPO_DIR}${RESET}"
else
    if [ "$DRY_RUN" = true ]; then
        echo -e "${YELLOW}[DRY RUN] Would remove crane-rs repository at ${REPO_DIR}${RESET}"
    else
        rm -rf "$REPO_DIR"
        echo -e "${CHECK} Removed crane-rs repository at ${REPO_DIR}"
    fi
fi

if [ "$DRY_RUN" = true ]; then
    echo -e "\n${BOLD}${YELLOW}[DRY RUN] crane-rs would be completely removed from your system.${RESET}"
else
    echo -e "\n${BOLD}${GREEN}crane-rs is completely removed from your system. ${CHECK}${RESET}"
fi
