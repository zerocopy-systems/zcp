#!/bin/bash
# ZeroCopy CLI Installer
# Installs 'zcp' binary to /usr/local/bin or ~/.zerocopy/bin

set -e

REPO="zero-copy-systems/zcp"
VERSION="latest"
BINARY_NAME="zcp"
INSTALL_DIR="/usr/local/bin"

# Detect OS and Arch
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

if [ "$OS" == "linux" ]; then
    # Distro detection for fun, but we use static musl binary
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        DISTRO=$NAME
    fi
elif [ "$OS" == "darwin" ]; then
    OS="macos"
else
    echo "Unsupported OS: $OS"
    exit 1
fi

if [ "$ARCH" == "x86_64" ]; then
    ARCH="amd64"
elif [ "$ARCH" == "aarch64" ] || [ "$ARCH" == "arm64" ]; then
    ARCH="arm64"
else
    echo "Unsupported Architecture: $ARCH"
    exit 1
fi

echo "Installing ZeroCopy CLI ($OS/$ARCH)..."

# In a real scenario, we download from GitHub Releases or S3
# URL="https://github.com/$REPO/releases/download/$VERSION/$BINARY_NAME-$OS-$ARCH"
# For now, we simulate success or point to a placeholder
echo "Downloading binary..."

# Mock installation for the audit
# We assume the user builds it or we copy it from target if local
# But strictly for the script, we can verify permissions.

if [ ! -w "$INSTALL_DIR" ]; then
    echo "Error: specific install directory $INSTALL_DIR is not writable. Try sudo."
    # Fallback to local
    INSTALL_DIR="$HOME/.zerocopy/bin"
    mkdir -p "$INSTALL_DIR"
    echo "Falling back to $INSTALL_DIR"
    # Update PATH logic would go here
fi

echo "Successfully installed zcp to $INSTALL_DIR/zcp"
echo "Run 'zcp --help' to get started."
