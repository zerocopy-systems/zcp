# `zcp` // ZeroCopy Auditor

[![License](https://img.shields.io/github/license/zerocopy-systems/zcp?style=flat-square)](https://github.com/zerocopy-systems/zcp/blob/main/LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/zerocopy-systems/zcp/audit.yml?branch=main&style=flat-square)](https://github.com/zerocopy-systems/zcp/actions)
[![Downloads](https://img.shields.io/github/downloads/zerocopy-systems/zcp/total?style=flat-square)](https://github.com/zerocopy-systems/zcp/releases)

**The Observer Effect is Costing You Millions.**

`zcp` (ZeroCopy Auditor) is an eBPF-powered diagnostic wedge built for quantitative trading desks, HFT firms, and autonomous agent runners. It safely measures kernel-level scheduling loops and network stack bloat without inducing the artificial latency inherent to `strace`, Python `asyncio`, or Node.js Garbage Collection profilers.

> **Stop bleeding alpha.** Prove your infrastructure's true latency to the microsecond, and calculate your annual **Jitter Tax**.

## 🧠 Why B2B Desks Trust `zcp`

When operating below the 1ms boundary, traditional observability tools fail. If you attach a debugger to a live futures bot, the system slows down. If you don't attach one, you trade blind.

`zcp` bridges this gap using Linux **eBPF (Extended Berkeley Packet Filter)**. We load statically verified bytecode directly into the kernel:
1. `tracepoint:sched:sched_wakeup`
2. `tracepoint:sched:sched_switch`
3. `kprobe:tcp_recvmsg`

Instead of context-switching to print logs, `zcp` writes highly compact timestamps into a `bpf_perf_event_output` RingBuffer. The user-space CLI aggregates these signals completely asynchronously—giving you a 100% Observer-Free breakdown of your wait delays.

## 🚀 Quickstart

### 1. Installation

`zcp` is distributed as a single, statically-linked CO-RE binary for Linux (no external dependencies required).

```bash
# via bash
curl -sL https://zerocopy.systems/audit/install.sh | bash

# or via Cargo (requires nightly toolchain for eBPF)
cargo install --git https://github.com/zerocopy-systems/zcp
```


### 2. Run the Diagnostic Wedge

Because `zcp` injects tracepoints into the Linux CFS scheduler, it requires `sudo` (`CAP_BPF` and `CAP_PERFMON`).

```bash
# Example: Tracing a Python asyncio bot 
export BOT_PID=$(pgrep python)

sudo zcp audit --pid $BOT_PID --volume 50000000 --slippage 0.0001 --json
```

### 3. 📈 The Revenue Bridge

When you terminate the audit (`Ctrl-C`), `zcp` generates a `bill_of_health.json`. You can submit this diagnostic at [zerocopy.systems/audit?utm_source=github&utm_medium=oss_cli_readme&utm_campaign=jitter_tax](https://zerocopy.systems/audit?utm_source=github&utm_medium=oss_cli_readme&utm_campaign=jitter_tax) to receive a personalized architectural remedy roadmap.

## 🏗️ Architecture vs. Legacy Benchmarks

| Feature | Legacy Tracing (`strace`, `perf`) | `zcp` (ZeroCopy eBPF) |
| :--- | :--- | :--- |
| **Context Switches** | High (Induces Artificial Lag) | **Zero** (Kernel RingBuffer) |
| **Financial Translation** | None (Raw timestamps) | **Native** $L = V \times F \times P(J)$ |
| **Network Interception** | `libpcap` (TCP Copy Overhead) | **Direct Kernel Probes** |
| **Binary Size** | Large (Heavy dependencies) | **Single CO-RE Static Binary** |

## 🛡️ Security & Enterprise Integration

Trading infrastructure represents the lifeblood of your firm. We treat the security of our diagnostic tools as a P0 constraint.

*   `zcp` **does not** read payload data. It only measures timestamps at kernel tracepoints.
*   `zcp` **does not** send telemetry outward. The JSON report is generated exclusively on your local machine.
*   The eBPF bytecode is validated natively by the Linux kernel verifier before loading, absolutely guaranteeing it cannot crash or halt your trading threads.

See [SECURITY.md](SECURITY.md) for our vulnerability disclosure policy and bug bounty program.

## 🤝 Contributing

We welcome contributions from quantitative developers and infrastructure engineers.
Please read [CONTRIBUTING.md](CONTRIBUTING.md) for our strict PR requirements (linear history, verified benchmarks, etc.).

## ⚖️ License

Dual-licensed under [Apache 2.0](LICENSE) — optimized for unencumbered enterprise adoption.
