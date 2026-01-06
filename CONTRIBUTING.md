# Contributing to ZCP

Thank you for your interest in contributing to ZCP! We welcome contributions from the community.

## 🚀 Quick Start

1. **Fork the repo** and clone locally
2. **Install Rust** 1.75+ via [rustup](https://rustup.rs/)
3. **Run tests**: `cargo test`
4. **Run lints**: `cargo clippy`
5. **Format code**: `cargo fmt`

## 📋 Before Contributing

### Check Existing Issues

- Look for [open issues](https://github.com/zerocopy-systems/zcp/issues)
- Check if someone is already working on it

### For New Features

- Open an issue first to discuss the feature
- Wait for maintainer approval before starting work

## 🔧 Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/zcp.git
cd zcp

# Add upstream remote
git remote add upstream https://github.com/zerocopy-systems/zcp.git

# Install development dependencies
cargo build

# Run tests
cargo test

# Run with example
cargo run -- audit --signer mock --samples 100
```

## 📝 Pull Request Process

1. **Create a branch** from `main`:

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** following our style guide

3. **Test your changes**:

   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

4. **Commit with conventional commits**:

   ```
   feat: add support for new signer
   fix: correct latency calculation
   docs: update README examples
   ```

5. **Push and create PR** against `main`

6. **Wait for review** — we aim to respond within 48 hours

## 🎨 Code Style

- **Format**: Run `cargo fmt` before committing
- **Lints**: Zero warnings with `cargo clippy`
- **Tests**: Add tests for new functionality
- **Docs**: Update docs for public API changes

## 🔐 Security

- **Never commit secrets** (API keys, credentials)
- **Report vulnerabilities** via [SECURITY.md](SECURITY.md)
- **Use safe patterns** — no `unsafe` without justification

## 📜 License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.

## 💬 Questions?

- Open a [Discussion](https://github.com/zerocopy-systems/zcp/discussions)
- Email: oss@zerocopy.systems

---

Thank you for contributing! 🙏
