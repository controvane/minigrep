#!/usr/bin/env bash
#
# Uninstall script for mgrep.
# Removes the binary from ~/.local/bin and removes the PATH export
# that install.sh added. Works on Linux and macOS.
#
# Usage: ./uninstall.sh
#
set -euo pipefail

BIN_NAME="mgrep"
BIN_DIR="${HOME}/.local/bin"
DEST="${BIN_DIR}/${BIN_NAME}"
RC_FILE=""
FOUND_RC=""

# --- Remove the binary -----------------------------------------------------
if [[ -f "${DEST}" ]]; then
    rm -f "${DEST}"
    echo "Removed ${DEST}"
    # Clean up the now-empty install dir if it was ours.
    rmdir "${BIN_DIR}" 2>/dev/null || true
else
    echo "No binary found at ${DEST}; nothing to remove."
fi

# --- Remove the PATH export from the user's shell config --------------------
SHELL_NAME="$(basename "${SHELL:-}")"

case "${SHELL_NAME}" in
    zsh)     RC_FILE="${HOME}/.zshrc" ;;
    bash)
        if [[ "$(uname)" == "Darwin" ]]; then
            RC_FILE="${HOME}/.bash_profile"
        else
            RC_FILE="${HOME}/.bashrc"
        fi
        ;;
    *)
        echo "Unsupported shell: ${SHELL_NAME}" >&2
        echo "Remove this line manually from your shell configuration:" >&2
        echo "export PATH=\"${BIN_DIR}:\$PATH\"" >&2
        exit 1
        ;;
esac

if [[ -f "${RC_FILE}" ]]; then
    # Strip only the line the installer added (and its comment marker).
    grep -vF "# Added by mgrep installer" "${RC_FILE}" \
        | grep -vF "export PATH=\"${BIN_DIR}:\$PATH\"" \
        > "${RC_FILE}.tmp" \
        && mv "${RC_FILE}.tmp" "${RC_FILE}"
    echo "Cleaned PATH export from ${RC_FILE}"
else
    echo "No ${RC_FILE} found; nothing to clean."
fi

echo "Done. Run 'source ${RC_FILE}' or open a new terminal to update your PATH."
