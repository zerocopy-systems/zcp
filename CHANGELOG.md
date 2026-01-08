# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0-beta] - 2026-01-08

### Added

- **Jitter Tax Calculator** - The core feature: calculate annual revenue loss from signing latency
  - Formula: `AnnualLoss = (Latency_ms / 1000) × SlippageRate × DailyVolume × TradingDays`
  - Interactive provider selection (AWS KMS, Fireblocks/MPC, Local HSM, ZeroCopy Sentinel)
  - `--explain` flag for step-by-step calculation breakdown with source citations

- **Rich Terminal Output**
  - ASCII art logo with ZCP branding
  - Dramatic reveal animation for Jitter Tax results
  - Color-coded loss boxes (green/yellow/red based on severity)
  - Comparison table: You vs. ZeroCopy Sentinel

- **New CLI Flags**
  - `--volume <USD>` - Daily trading volume for loss calculation
  - `--provider <NAME>` - Signing provider (aws-kms, mpc, hsm, sentinel, or custom ms)
  - `--explain` - Show detailed calculation breakdown
  - `--report <FILE>` - Generate Markdown report
  - `--accept` - Skip capability declaration prompt
  - `--address <ADDR>` - Wallet address (EVM 0x... or Solana base58)

- **Capability Declaration Banner**
  - Trust-building transparency: shows what the tool will/won't do
  - Interactive prompt with `--accept` bypass option

- **Wallet Address Validation**
  - EVM address validation with EIP-55 checksum support
  - Solana address validation (base58)
  - Auto-detection of chain from address format

- **Report Generation**
  - `--report` flag generates Markdown reports
  - Includes executive summary, Jitter Tax analysis, methodology, and CTA
  - Convertible to PDF via pandoc

- **Supply Chain Security**
  - GitHub Actions workflow with Sigstore/Cosign keyless signing
  - `Dockerfile.reproducible` for binary verification
  - SHA256 checksums for all release artifacts

### Improved

- **--help output** - Enhanced with examples, quick start section, and branding
- **Test Coverage** - 44 unit tests covering wallet validation, jitter tax calculations, edge cases

### Technical

- New modules: `jitter_tax.rs`, `wallet.rs`, `rich_output.rs`, `pdf_report.rs`, `capability.rs`
- Provider latency assumptions based on documented sources (AWS Re:Post, Fireblocks docs)
- Market regime multipliers: Low (1×), Medium (2.5×), High (5×)

---

## [0.1.0] - 2025-12-15

### Initial Release

- Basic audit functionality with system checks
- AWS Nitro Enclave detection
- Kernel configuration validation (isolcpus, nohz_full, IOMMU, HugePages)
- JSON output format
- `--publish` flag for blockchain proof
