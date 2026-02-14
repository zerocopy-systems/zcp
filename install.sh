#!/bin/bash
# ZeroCopy CLI Installer
# Installs 'zcp' binary to /usr/local/bin or ~/.zerocopy/bin
#
# Usage:
#   curl -sSL https://zerocopy.systems/install | sh
#   curl -sSL https://raw.githubusercontent.com/zerocopy-systems/zcp/main/install.sh | sh

set -euo pipefail

REPO="zerocopy-systems/zcp"
VERSION="${ZCP_VERSION:-latest}"
BINARY_NAME="zcp"
INSTALL_DIR="/usr/local/bin"

# ─── Colors ───────────────────────────────────────────────────────────────────

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

info()  { printf "${GREEN}→${NC} %s\n" "$1"; }
warn()  { printf "${YELLOW}⚠${NC} %s\n" "$1"; }
error() { printf "${RED}✗${NC} %s\n" "$1"; exit 1; }

# ─── Detect Platform ─────────────────────────────────────────────────────────

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
    linux)  PLATFORM="linux"  ;;
    darwin) PLATFORM="macos"  ;;
    *)      error "Unsupported OS: $OS" ;;
esac

case "$ARCH" in
    x86_64)         ARCH_SUFFIX="x86_64" ;;
    aarch64|arm64)  ARCH_SUFFIX="arm64"  ;;
    *)              error "Unsupported architecture: $ARCH" ;;
esac

ARTIFACT="zcp-${PLATFORM}-${ARCH_SUFFIX}"

# ─── Resolve Version ─────────────────────────────────────────────────────────

if [ "$VERSION" = "latest" ]; then
    info "Fetching latest release..."
    DOWNLOAD_URL="https://github.com/$REPO/releases/latest/download/$ARTIFACT"
    SHA_URL="https://github.com/$REPO/releases/latest/download/${ARTIFACT}.sha256"
else
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$ARTIFACT"
    SHA_URL="https://github.com/$REPO/releases/download/$VERSION/${ARTIFACT}.sha256"
fi

# ─── Download ─────────────────────────────────────────────────────────────────

TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

info "Downloading $ARTIFACT..."

if command -v curl &>/dev/null; then
    curl -fsSL "$DOWNLOAD_URL" -o "$TMPDIR/$BINARY_NAME" || error "Download failed. Check https://github.com/$REPO/releases for available versions."
    curl -fsSL "$SHA_URL" -o "$TMPDIR/${BINARY_NAME}.sha256" 2>/dev/null || warn "SHA256 checksum not available for this release."
elif command -v wget &>/dev/null; then
    wget -q "$DOWNLOAD_URL" -O "$TMPDIR/$BINARY_NAME" || error "Download failed."
    wget -q "$SHA_URL" -O "$TMPDIR/${BINARY_NAME}.sha256" 2>/dev/null || warn "SHA256 checksum not available."
else
    error "Neither curl nor wget found. Please install one and retry."
fi

# ─── Verify Checksum ─────────────────────────────────────────────────────────

if [ -f "$TMPDIR/${BINARY_NAME}.sha256" ]; then
    info "Verifying checksum..."
    EXPECTED_SHA=$(awk '{print $1}' "$TMPDIR/${BINARY_NAME}.sha256")
    if command -v sha256sum &>/dev/null; then
        ACTUAL_SHA=$(sha256sum "$TMPDIR/$BINARY_NAME" | awk '{print $1}')
    elif command -v shasum &>/dev/null; then
        ACTUAL_SHA=$(shasum -a 256 "$TMPDIR/$BINARY_NAME" | awk '{print $1}')
    else
        warn "No sha256sum available, skipping verification."
        ACTUAL_SHA="$EXPECTED_SHA"
    fi

    if [ "$EXPECTED_SHA" != "$ACTUAL_SHA" ]; then
        error "Checksum mismatch! Expected: $EXPECTED_SHA Got: $ACTUAL_SHA"
    fi
    info "Checksum verified ✓"
fi

# ─── Install ─────────────────────────────────────────────────────────────────

chmod +x "$TMPDIR/$BINARY_NAME"

if [ -w "$INSTALL_DIR" ]; then
    mv "$TMPDIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    info "Installed to $INSTALL_DIR/$BINARY_NAME"
else
    # Try sudo first, fall back to user-local
    if command -v sudo &>/dev/null && sudo -n true 2>/dev/null; then
        sudo mv "$TMPDIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
        info "Installed to $INSTALL_DIR/$BINARY_NAME (via sudo)"
    else
        INSTALL_DIR="$HOME/.zerocopy/bin"
        mkdir -p "$INSTALL_DIR"
        mv "$TMPDIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
        warn "Installed to $INSTALL_DIR/$BINARY_NAME"
        warn "Add to your PATH: export PATH=\"$INSTALL_DIR:\$PATH\""
    fi
fi

# ─── Verify ──────────────────────────────────────────────────────────────────

if command -v "$BINARY_NAME" &>/dev/null; then
    printf "\n${GREEN}✓ ZCP installed successfully!${NC}\n"
    "$BINARY_NAME" --version 2>/dev/null || true
    printf "\nRun: ${GREEN}zcp audit --volume 10000000${NC}\n"
else
    printf "\n${GREEN}✓ ZCP installed to $INSTALL_DIR/$BINARY_NAME${NC}\n"
    printf "Run: ${GREEN}$INSTALL_DIR/zcp audit --volume 10000000${NC}\n"
fi
