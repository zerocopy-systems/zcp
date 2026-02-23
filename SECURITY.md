# Security Policy

At ZeroCopy Systems, trading infrastructure relies on absolute mathematical certainty. Vulnerabilities in our diagnostic wedges can compromise kernel stability or inadvertently expose the execution logic of quantitative desks.

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| v0.1.x  | :white_check_mark: |

## Reporting a Vulnerability

If you discover a potential vulnerability within the `zcp` eBPF bytecode or aggregate user-space CLI, please do NOT disclose it publicly on GitHub Issues.

1.  **Email**: Contact us immediately at `security@zerocopy.systems`.
2.  **Bounty Program**: We offer up to **$5,000** for critical flaws (e.g., proving the eBPF probe can crash a `CAP_BPF` authorized kernel, or memory leaks within the `bpf_perf_event_output` buffers).
3.  **SLA**: We pledge to acknowledge your report within 12 hours and will collaborate with you to publish a secure, coordinated disclosure patch.

We take the Sovereign Trust of our clients extremely seriously. Thank you for protecting the ecosystem.
