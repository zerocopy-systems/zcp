# ZCP — ZeroCopy Auditor CLI

[![Build](https://github.com/zerocopy-systems/zcp/actions/workflows/ci.yml/badge.svg)](https://github.com/zerocopy-systems/zcp/actions)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.82+-orange.svg)](https://www.rust-lang.org)

**Quantify your signing infrastructure's revenue leakage — the Jitter Tax.**

ZCP (ZeroCopy Auditor) is a free, open-source CLI tool that measures cryptographic signing latency and calculates the annual dollar loss (Jitter Tax) from slow key management infrastructure.

## 🚀 Quick Install

```bash
# Install to /usr/local/bin
curl -sSL https://zerocopy.systems/install | sh

# Verify installation
zcp --version
```

## 📊 Quick Start

```bash
# Calculate your Jitter Tax with $10M daily volume
zcp audit --volume 10000000

# Specify your signing provider
zcp audit --volume 10000000 --provider aws-kms

# Show detailed calculation breakdown
zcp audit --volume 10000000 --explain

# Generate a Markdown report
zcp audit --volume 10000000 --report jitter_audit.md
```

## 🎯 What It Calculates

The **Jitter Tax Formula**:
```
Annual Loss = (Latency_ms / 1000) × Slippage_Rate × Daily_Volume × Trading_Days
```

### Sample Output

```
╔════════════════════════════════════════════════════════════╗
║             ⚠  CRITICAL: JITTER TAX DETECTED               ║
╠════════════════════════════════════════════════════════════╣
║  Provider:              AWS KMS                            ║
║  Signing Latency:       150 ms                             ║
║  ESTIMATED ANNUAL LOSS: $54.8K                             ║
╚════════════════════════════════════════════════════════════╝

┌────────────────────────┬──────────────────┬──────────────────┐
│ Metric                 │ You (Current)    │ ZeroCopy         │
├────────────────────────┼──────────────────┼──────────────────┤
│ Time-to-Sign (P99)     │ 150 ms           │ 42 µs            │
│ Annual Jitter Tax      │ $54.8K           │ $0               │
│ Potential Savings      │ -                │ $54.8K           │
└────────────────────────┴──────────────────┴──────────────────┘
```

## 🔧 CLI Options

| Option | Description | Example |
|:-------|:------------|:--------|
| `--volume <USD>` | Daily trading volume | `--volume 10000000` |
| `--provider <NAME>` | Signing provider (aws-kms, mpc, hsm, sentinel) | `--provider aws-kms` |
| `--explain` | Show step-by-step calculation breakdown | `--explain` |
| `--report <FILE>` | Generate Markdown report | `--report audit.md` |
| `--accept` | Skip capability declaration prompt | `--accept` |
| `--address <ADDR>` | Wallet address (EVM 0x... or Solana) | `--address 0x...` |
| `--regime <TYPE>` | Market volatility (low, medium, high) | `--regime high` |
| `--json` | Output in JSON format | `--json` |
| `--sim` | Simulation mode (for testing) | `--sim` |

## 📈 Provider Latency Assumptions

| Provider | Latency (P99) | Source |
|:---------|:--------------|:-------|
| AWS KMS | 150 ms | AWS Re:Post Benchmarks |
| Fireblocks / MPC | 350 ms | Fireblocks Performance Docs |
| Local HSM | 5 ms | Industry Standard |
| ZeroCopy Sentinel | 42 µs | Internal Benchmarks |

## 🛡️ Security & Trust

Before running any analysis, ZCP displays a **Capability Declaration**:

```
┌─────────────────────────────────────────────┐
│  ZCP AUDIT - Capability Declaration         │
├─────────────────────────────────────────────┤
│  ✓ READ: System config, public chain data   │
│  ✗ WRITE: Nothing (except final report)     │
│  ✗ NETWORK: No calls unless --fetch-rpc     │
│  ✗ SECRETS: Does not access keystore files  │
└─────────────────────────────────────────────┘
```

- **No Data Exfiltration**: All results stay local unless you opt-in with `--submit`
- **Signed Releases**: All binaries are signed with Sigstore/Cosign
- **Reproducible Builds**: Build from source with `Dockerfile.reproducible`

### Verify Signatures

```bash
cosign verify-blob --signature zcp-linux-x86_64.sig \
  --certificate zcp-linux-x86_64.pem zcp-linux-x86_64
```

## 🏗️ Building from Source

```bash
# Prerequisites: Rust 1.82+
git clone https://github.com/zerocopy-systems/zcp.git
cd apps/zcp
cargo build --release

# Binary at: ../../target/release/zcp
```

### Reproducible Build (Docker)

```bash
docker build -f Dockerfile.reproducible -t zcp-build .
docker run --rm -v $(pwd)/output:/output zcp-build
shasum -a 256 output/zcp  # Compare to release SHA256
```

## 🧪 Running Tests

```bash
cargo test
# Currently: 44 tests passing
```

## 📋 Requirements

- Rust 1.82+ (for building from source)
- Optional: AWS credentials for `--publish` flag

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📜 License

Apache License 2.0 — See [LICENSE](LICENSE)

## 🔗 Links

- **Website**: [zerocopy.systems](https://zerocopy.systems)
- **Documentation**: [docs.zerocopy.systems](https://docs.zerocopy.systems)
- **Demo**: [zerocopy.systems/demo](https://zerocopy.systems/demo)

---

**⭐ Star this repo if it helps you quantify your Jitter Tax!**
