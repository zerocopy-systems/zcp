# ZCP (ZeroCopy Auditor) — Institutional Latency Diagnostic Tool

[![Build](https://github.com/zerocopy-systems/zcp/actions/workflows/ci.yml/badge.svg)](https://github.com/zerocopy-systems/zcp/actions)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Standard](https://img.shields.io/badge/Latency-42μs_Core-green.svg)](https://zerocopy.systems)

**The industry standard for auditing institutional signing infrastructure.**

ZCP is a specialized forensic tool designed for Infrastructure leads and Quantitative researchers. It benchmarks your current signing setup (AWS KMS, Fireblocks, MPC) against the physics-based limit of cold-cache hardware (42µs Core Latency).

---

## 🏛️ The Institutional Readiness Audit

For deep technical due diligence, run the comprehensive scan mode. This generates a verifiable "Bill of Health" artifact for risk committees and LPs.

```bash
# Generate diligence package
zcp diligence
```

**Output: `sentinel_diligence_pack.zip`**

1. **`infrastructure_audit.md`**: Verifies if you are running on "Sovereign" hardware (Nitro Enclaves) or "Tenant" infrastructure.
2. **`performance_benchmark.md`**: 100-round high-fidelity latency trace.
3. **`loss_assessment.json`**: Calculated "Jitter Tax" based on your volume.

---

## 📊 Interactive Audit

If you just want to see the numbers quickly:

```bash
# 1. basic check
zcp audit

# 2. specific provider comparison
zcp audit --provider aws-kms --volume 50000000

# 3. explain the calculation
zcp audit --explain
```

### The Jitter Tax Formula

ZCP calculates revenue leakage using the "Variance Decay" model:

> _Every 1ms of jitter reduces sharpe ratio by 0.01 for HFT strategies._

```
Annual Loss = (Latency_ms / 1000) × Slippage_Rate × Daily_Volume × Trading_Days
```

---

## 🔧 Installation

### Homebrew (macOS & Linux)

```bash
brew install zerocopy-systems/tap/zcp
```

### One-Line Install

```bash
curl -sSL https://raw.githubusercontent.com/zerocopy-systems/zcp/main/install.sh | sh
```

### Cargo

```bash
cargo install zerocopy-audit
```

### Build from Source

Audit the auditor. Ensure the binary matches the code.

```bash
git clone https://github.com/zerocopy-systems/zcp.git
cd zcp
cargo build --release
sudo cp target/release/zcp /usr/local/bin/
```

---

## 🛡️ Capability Declaration

ZCP operates on a **"No-Trust"** basis. When you run it, it explicitly declares what it CANNOT do.

```
┌─────────────────────────────────────────────┐
│  ZCP AUDIT - Capability Declaration         │
├─────────────────────────────────────────────┤
│  ✓ READ: System config, public chain data   │
│  ✗ WRITE: Nothing (except final report)     │
│  ✗ NETWORK: No calls unless --fetch-rpc     │
│  ✗ SECRETS: Does not access keystore files  │
99: └─────────────────────────────────────────────┘
```

---

## 🔗 Links

- **[ZeroCopy Systems](https://zerocopy.systems)**
- **[Documentation](https://docs.zerocopy.systems)**
- **[Trojan Horse Strategy](https://zerocopy.systems/strategy)**

---

© 2024 ZeroCopy Systems. _Verified by Physics._
[![Build](https://github.com/zerocopy-systems/zcp/actions/workflows/ci.yml/badge.svg)](https://github.com/zerocopy-systems/zcp/actions)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.82+-orange.svg)](https://www.rust-lang.org)

**Quantify your signing infrastructure's revenue leakage — the Jitter Tax.**

ZCP (ZeroCopy Auditor) is a free, open-source CLI tool that measures cryptographic signing latency and calculates the annual dollar loss (Jitter Tax) from slow key management infrastructure.

## 🚀 Quick Install

### Homebrew (macOS & Linux)

```bash
brew install zerocopy-systems/tap/zcp
```

### One-Line Install

```bash
curl -sSL https://raw.githubusercontent.com/zerocopy-systems/zcp/main/install.sh | sh
```

### Cargo (from source)

```bash
cargo install zerocopy-audit
```

### Build from Source

Audit the auditor. Ensure the binary matches the code.

```bash
git clone https://github.com/zerocopy-systems/zcp.git
cd zcp
cargo build --release
sudo cp target/release/zcp /usr/local/bin/
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

| Option              | Description                                    | Example              |
| :------------------ | :--------------------------------------------- | :------------------- |
| `--volume <USD>`    | Daily trading volume                           | `--volume 10000000`  |
| `--provider <NAME>` | Signing provider (aws-kms, mpc, hsm, sentinel) | `--provider aws-kms` |
| `--explain`         | Show step-by-step calculation breakdown        | `--explain`          |
| `--report <FILE>`   | Generate Markdown report                       | `--report audit.md`  |
| `--accept`          | Skip capability declaration prompt             | `--accept`           |
| `--address <ADDR>`  | Wallet address (EVM 0x... or Solana)           | `--address 0x...`    |
| `--regime <TYPE>`   | Market volatility (low, medium, high)          | `--regime high`      |
| `--json`            | Output in JSON format                          | `--json`             |
| `--sim`             | Simulation mode (for testing)                  | `--sim`              |

## 📈 Provider Latency Assumptions

| Provider          | Latency (P99) | Source                      |
| :---------------- | :------------ | :-------------------------- |
| AWS KMS           | 150 ms        | AWS Re:Post Benchmarks      |
| Fireblocks / MPC  | 350 ms        | Fireblocks Performance Docs |
| Local HSM         | 5 ms          | Industry Standard           |
| ZeroCopy Sentinel | 42 µs         | Internal Benchmarks         |

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
cd zcp
cargo build --release
sudo cp target/release/zcp /usr/local/bin/
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

MIT License — See [LICENSE](LICENSE)

## 🔗 Links

- **Website**: [zerocopy.systems](https://zerocopy.systems)
- **Documentation**: [docs.zerocopy.systems](https://docs.zerocopy.systems)
- **Demo**: [zerocopy.systems/demo](https://zerocopy.systems/demo)

---

**⭐ Star this repo if it helps you quantify your Jitter Tax!**
