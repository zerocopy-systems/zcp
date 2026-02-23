# Contributing to ZeroCopy Systems (`zcp`)

Thank you for your interest in contributing to the ZeroCopy eBPF Auditor. Our primary user base consists of Infrastructure CTOs, high-frequency trading desks, and latency-sensitive cryptocurrency operations. 

As such, we maintain extremely stringent parameters regarding code quality, execution speed, and PR history.

## The Prime Directives

1.  **Zero Observer Effect**: No PRs will be accepted that introduce measurable execution latency into the user-space CLI or the Linux kernel probes (e.g., no excessive I/O string allocations during the hot path).
2.  **No New Dependencies**: `zcp` is engineered to be a standalone, statically linked, CO-RE compliant binary. We will heavily scrutinize any attempt to expand the `Cargo.toml` footprint. Do not import massive frameworks if the standard library suffices.
3.  **Strictly Linear Git History**: We do not allow merge commits. Every PR must be a clean, squashed `rebase` onto `main`.

## Development Setup (Rust Nightly)

Because compiling `aya-bpf` requires specialized Linux unstable features, you must use the Rust Nightly toolchain.

```bash
# Provide the eBPF target
rustup toolchain install nightly --component rust-src
rustup target add bpfel-unknown-none

# Build the kernel module and user-space CLI
cargo build --release
```

## Pull Request Guidelines

*   **Prefixes**: Please use Conventional Commits (e.g., `feat:`, `fix:`, `docs:`, `chore:`).
*   **Benchmarks**: If your PR touches the aggregation engine or eBPF rings, you *must* run `cargo bench` and include the output in the PR body. Any regression >1us will be rejected.
*   **Signatures**: We require GPG-signed commits for all external contributors to verify identity.

If you have a massive architectural change in mind, please open a GitHub Discussion first. We do not want you to waste time on a 3,000-line PR that gets rejected due to misalignment with our B2B Sovereign Engine philosophy.
