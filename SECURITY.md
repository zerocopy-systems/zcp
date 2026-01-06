# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.x.x   | :white_check_mark: |

## Reporting a Vulnerability

We take security seriously. If you discover a vulnerability in ZCP:

### 1. **Do NOT open a public issue**

Security vulnerabilities should be reported privately.

### 2. **Email us directly**

Send details to: **security@zerocopy.systems**

Include:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Any suggested fixes

### 3. **Response Timeline**

| Action                | Timeline           |
| --------------------- | ------------------ |
| Acknowledgment        | 48 hours           |
| Initial assessment    | 7 days             |
| Fix timeline provided | 14 days            |
| Public disclosure     | After fix released |

## Security Measures

### Build Security

- ✅ All releases are signed with [Sigstore cosign](https://www.sigstore.dev/)
- ✅ SHA256 checksums provided for all binaries
- ✅ Reproducible builds from source
- ✅ Dependency scanning via `cargo-audit`

### Runtime Security

- ✅ No data exfiltration (results stay local by default)
- ✅ Minimal permissions required (`kms:Sign` only)
- ✅ No secrets stored or cached
- ✅ Memory-safe Rust implementation

### Supply Chain Security

- ✅ Pinned dependencies in Cargo.lock
- ✅ Automated dependency updates via Dependabot
- ✅ License compliance scanning (no copyleft)
- ✅ SBOM (Software Bill of Materials) in releases

## Verifying Releases

```bash
# Download binary and checksum
curl -LO https://github.com/zerocopy-systems/zcp/releases/latest/download/zcp-linux-x86_64
curl -LO https://github.com/zerocopy-systems/zcp/releases/latest/download/checksums.txt

# Verify checksum
sha256sum -c checksums.txt

# Verify Sigstore signature (if available)
cosign verify-blob --signature zcp-linux-x86_64.sig zcp-linux-x86_64
```

## Security Hall of Fame

We appreciate responsible disclosure. Contributors who report valid vulnerabilities will be acknowledged here (with permission).

---

Thank you for helping keep ZCP secure!
