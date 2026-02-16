# ==============================================================================
# ZCP CLI - Reproducible Builds Dockerfile
# Task 1.3: Allow users to verify the binary matches the source code
# ==============================================================================
#
# Usage:
#   docker build -f Dockerfile.reproducible -t zcp-build .
#   docker run --rm -v $(pwd)/target:/output zcp-build
#
# Then compare the hash:
#   shasum -a 256 target/release/zcp
#   curl -sL https://github.com/zerocopy-systems/zcp/releases/latest/download/zcp.sha256
# ==============================================================================

FROM rust:1.82-slim-bookworm AS builder

# Deterministic build settings
ENV CARGO_HOME=/cargo
ENV RUSTFLAGS="-C target-feature=+crt-static"
ENV SOURCE_DATE_EPOCH=0
ENV CARGO_INCREMENTAL=0

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create deterministic workspace
WORKDIR /build

# Copy only dependency manifests first (for layer caching)
COPY Cargo.toml Cargo.lock ./
COPY packages/zero-copy-utils/Cargo.toml ./packages/zero-copy-utils/
COPY packages/sentinel-shared/Cargo.toml ./packages/sentinel-shared/
COPY apps/zcp/Cargo.toml ./apps/zcp/

# Create stub lib files to allow cargo to resolve dependencies
RUN mkdir -p packages/zero-copy-utils/src && echo "pub fn stub(){}" > packages/zero-copy-utils/src/lib.rs
RUN mkdir -p packages/sentinel-shared/src && echo "pub fn stub(){}" > packages/sentinel-shared/src/lib.rs
RUN mkdir -p apps/zcp/src && echo "fn main(){}" > apps/zcp/src/main.rs

# Pre-fetch dependencies (cached layer)
RUN cargo fetch --locked

# Now copy actual source
COPY packages/ ./packages/
COPY apps/zcp/ ./apps/zcp/

# Build with locked deps and reproducible flags
RUN cargo build \
    --package zerocopy-audit \
    --release \
    --locked \
    --target-dir /output

# Output stage - just the binary
FROM scratch AS export
COPY --from=builder /output/release/zcp /zcp

# Default: copy binary to /output mount
FROM debian:bookworm-slim AS final

# Security: Run as non-root user
RUN useradd --create-home --shell /bin/bash zcp
USER zcp

COPY --from=builder /output/release/zcp /usr/local/bin/zcp
ENTRYPOINT ["/usr/local/bin/zcp"]
