# ZCP — ZeroCopy Auditor CLI

[![Build](https://github.com/zerocopy-systems/zcp/actions/workflows/ci.yml/badge.svg)](https://github.com/zerocopy-systems/zcp/actions)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)

**Quantify your signing infrastructure's alpha decay.**

ZCP (ZeroCopy Auditor) is a free, open-source CLI tool that measures cryptographic signing latency and jitter. It helps HFT firms and crypto trading operations identify how much profit they're losing to slow key management infrastructure.

## 🚀 Quick Install

```bash
# Install to /usr/local/bin
curl -sSL https://zerocopy.systems/zcp | sh

# Verify installation
zcp --version
```

**⚠️ Security Note:** Always verify the SHA256 checksum of downloaded binaries. See [Releases](https://github.com/zerocopy-systems/zcp/releases) for checksums.

## 📊 What It Measures

| Metric | Description |
|--------|-------------|
| **Latency (P50/P95/P99)** | Signing operation round-trip time |
| **Jitter (Std Dev)** | Variance in signing latency |
| **Throughput** | Maximum signatures per second |
| **Alpha Decay** | Estimated profit loss due to latency |

## 🔧 Usage

```bash
# Audit AWS KMS
zcp audit --signer aws-kms --region us-east-1 --key-id alias/your-key

# Audit with JSON output
zcp audit --signer aws-kms --region us-east-1 --json

# Generate benchmark report
zcp audit --signer aws-kms --region us-east-1 --output report.json
```

### Sample Output

```json
{
  "signer": "aws-kms",
  "region": "us-east-1",
  "samples": 10000,
  "results": {
    "latency_p50_ms": 145.2,
    "latency_p95_ms": 198.7,
    "latency_p99_ms": 312.4,
    "jitter_stddev_ms": 42.1,
    "throughput_max_sps": 312
  },
  "score": 35,
  "grade": "D",
  "recommendation": "CRITICAL: Jitter exceeds threshold."
}
```

## 📈 Grading Scale

| Grade | Latency | Assessment |
|-------|---------|------------|
| **A** | < 1ms | Institutional Grade |
| **B** | 1-10ms | Competitive |
| **C** | 10-50ms | At Risk |
| **D** | 50-150ms | Bleeding Alpha |
| **F** | > 150ms | Critical |

## 🔐 Supported Signers

- ✅ AWS KMS
- ✅ GCP Cloud HSM
- ✅ Azure Key Vault
- ✅ HashiCorp Vault
- ✅ Fireblocks MPC
- ✅ YubiHSM 2
- 🚧 Local PKCS#11 (coming soon)

## 🛡️ Security

- **No Data Exfiltration**: All results stay local unless you opt-in with `--submit`
- **Read-Only Access**: Requires only `kms:Sign` permission (no key management)
- **Signed Releases**: All binaries are signed with Sigstore cosign
- **Reproducible Builds**: Build from source with identical outputs

See [SECURITY.md](SECURITY.md) for vulnerability reporting.

## 🏗️ Building from Source

```bash
# Prerequisites: Rust 1.75+
git clone https://github.com/zerocopy-systems/zcp.git
cd zcp
cargo build --release

# Binary at: target/release/zcp
```

## 📋 Requirements

- AWS credentials (for AWS KMS auditing)
- `kms:Sign` permission on target key
- Rust 1.75+ (for building from source)

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📜 License

Apache License 2.0 — See [LICENSE](LICENSE)

## 🔗 Links

- **Website**: [zerocopy.systems](https://zerocopy.systems)
- **Documentation**: [zerocopy.systems/docs](https://zerocopy.systems/docs)
- **Jitter Tax Calculator**: [zerocopy.systems/pricing](https://zerocopy.systems/pricing)

---

**⭐ Star this repo if it helps you quantify your alpha decay!**
