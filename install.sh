#!/usr/bin/env bash
#
# Install script for mgrep.
# Copies the release binary to ~/.local/bin and ensures the directory
# is on the PATH. Works on Linux and macOS.
#
# Usage: ./install.sh
#
set -euo pipefail

BIN_NAME="mgrep"
BIN_DIR="${HOME}/.local/bin"
DEST="${BIN_DIR}/${BIN_NAME}"

# Locate the release binary relative to this script (works even if the
# script is run from another directory).
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SOURCE="${SCRIPT_DIR}/release/${BIN_NAME}"

if [[ ! -f "${SOURCE}" ]]; then
    echo "error: release binary not found at ${SOURCE}" >&2
    echo "Run 'cargo build --release' then copy it here with 'cp target/release/mgrep release/mgrep'." >&2
    exit 1
fi

# Create the install directory if it does not exist.
mkdir -p "${BIN_DIR}"

cp "${SOURCE}" "${DEST}"
chmod +x "${DEST}"
echo "Installed ${BIN_NAME} -> ${DEST}"

# If the directory is already on the PATH, we are done.
case ":${PATH}:" in
    *":${BIN_DIR}:"*)
        echo "${BIN_DIR} is already on your PATH."
        exit 0
        ;;
esac

# Otherwise, append the PATH export to the user's shell config.
# Detect the login shell to pick the right rc file.
SHELL_NAME="$(basename "${SHELL:-}")"

case "${SHELL_NAME}" in
    zsh)
        RC_FILE="${HOME}/.zshrc"
        ;;
    bash)
        if [[ "$(uname)" == "Darwin" ]]; then
            # On macOS, bash login shells read .bash_profile.
            RC_FILE="${HOME}/.bash_profile"
        else
            RC_FILE="${HOME}/.bashrc"
        fi
        ;;
    *)
        echo "Unsupported shell: ${SHELL_NAME}" >&2
        echo "Add the following line to your shell configuration:" >&2
        echo "export PATH=\"${BIN_DIR}:\$PATH\"" >&2
        exit 1
        ;;
esac

# Guard against double-adding on repeated runs.
if ! grep -qF "export PATH=\"${BIN_DIR}" "${RC_FILE}" 2>/dev/null; then
    printf '\n# Added by mgrep installer\nexport PATH="%s:$PATH"\n' "${BIN_DIR}" >> "${RC_FILE}"
    echo "Added PATH export to ${RC_FILE}"
else
    echo "PATH export already present in ${RC_FILE}"
fi

echo "Done. Run 'source ${RC_FILE}' or open a new terminal, then try '${BIN_NAME} --help'."
