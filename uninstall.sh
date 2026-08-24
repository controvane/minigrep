#!/usr/bin/env bash
#
# Uninstall script for mgrp.
# Removes the binary from ~/.local/bin and removes the PATH export
# that install.sh added. Works on Linux and macOS.
#
# Usage: ./uninstall.sh
#
set -euo pipefail

BIN_DIR="${HOME}/.local/bin"
RC_FILE=""

# --- Remove the binary(ies) ------------------------------------------------
# Handle both the current (mgrp) and legacy (mgrep) names so updating the
# binary name does not leave the old one behind.
FOUND=false
for BIN_NAME in mgrp mgrep; do
    DEST="${BIN_DIR}/${BIN_NAME}"
    if [[ -f "${DEST}" ]]; then
        rm -f "${DEST}"
        echo "Removed ${DEST}"
        FOUND=true
    fi
done
if [[ "${FOUND}" != "true" ]]; then
    echo "No binary found (mgrp/mgrep); nothing to remove."
fi
# Clean up the now-empty install dir if it was ours.
rmdir "${BIN_DIR}" 2>/dev/null || true

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
    # Strip only the lines the installer added (and their comment markers),
    # covering both the legacy and current marker text. The final grep exits 1
    # when every line is filtered out (empty output), so tolerate that.
    grep -vF "# Added by mgrep installer" "${RC_FILE}" \
        | grep -vF "# Added by mgrp installer" \
        | grep -vF "export PATH=\"${BIN_DIR}:\$PATH\"" \
        > "${RC_FILE}.tmp" || true
    mv "${RC_FILE}.tmp" "${RC_FILE}"
    echo "Cleaned PATH export from ${RC_FILE}"
else
    echo "No ${RC_FILE} found; nothing to clean."
fi

echo "Done. Run 'source ${RC_FILE}' or open a new terminal to update your PATH."
