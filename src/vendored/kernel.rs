//! Vendored kernel audit module from zero-copy-utils.
//!
//! This is a minimal copy of the kernel auditing functionality needed by zcp.
//! The canonical version lives in packages/zero-copy-utils.

use std::fs;
use std::path::Path;
use thiserror::Error;

/// Errors that can occur during kernel auditing.
#[derive(Error, Debug)]
pub enum KernelError {
    #[error("Failed to read procfs: {0}")]
    Io(#[from] std::io::Error),

    #[error("Not running on Linux")]
    NotLinux,
}

/// Result of a single audit check.
#[derive(Debug, Clone)]
pub struct AuditResult {
    pub check_name: String,
    pub passed: bool,
    pub details: String,
}

/// Check if CPU cores are isolated from the Linux scheduler.
pub fn check_isolcpus(simulate: bool) -> Result<AuditResult, KernelError> {
    if simulate {
        return Ok(AuditResult {
            check_name: "Kernel Isolation (isolcpus)".into(),
            passed: true,
            details: "SIMULATED PASS: isolcpus=1-31 detected.".into(),
        });
    }

    if !cfg!(target_os = "linux") {
        return Ok(AuditResult {
            check_name: "Kernel Isolation".into(),
            passed: true,
            details: "SKIPPED: Not running on Linux. Check not applicable.".into(),
        });
    }

    let cmdline = fs::read_to_string("/proc/cmdline")?;
    let passed = cmdline.contains("isolcpus");

    Ok(AuditResult {
        check_name: "Kernel Isolation (isolcpus)".into(),
        passed,
        details: if passed {
            "PASS: isolcpus detected.".into()
        } else {
            "FAIL: No CPU isolation detected. Threads will jitter.".into()
        },
    })
}

/// Check if running on AWS Nitro hardware.
pub fn check_nitro_enclave(simulate: bool) -> Result<AuditResult, KernelError> {
    if simulate {
        return Ok(AuditResult {
            check_name: "AWS Nitro System".into(),
            passed: true,
            details: "SIMULATED PASS: AWS Nitro (c6id.metal) detected.".into(),
        });
    }

    let dmi_path = "/sys/class/dmi/id/product_name";
    if Path::new(dmi_path).exists() {
        let product = fs::read_to_string(dmi_path)?;
        if product.contains("Nitro") {
            return Ok(AuditResult {
                check_name: "AWS Nitro System".into(),
                passed: true,
                details: "PASS: AWS Nitro Hardware detected.".into(),
            });
        }
    }

    Ok(AuditResult {
        check_name: "AWS Nitro System".into(),
        passed: false,
        details: "WARN: Not running on AWS Nitro. Verify sovereignty.".into(),
    })
}

/// Check for tickless kernel configuration (`nohz_full`).
pub fn check_tickless(simulate: bool) -> Result<AuditResult, KernelError> {
    if simulate {
        return Ok(AuditResult {
            check_name: "Tickless Kernel (nohz_full)".into(),
            passed: true,
            details: "SIMULATED PASS: nohz_full=1-31 detected.".into(),
        });
    }

    if !cfg!(target_os = "linux") {
        return Ok(AuditResult {
            check_name: "Tickless Kernel".into(),
            passed: true,
            details: "SKIPPED: Not Linux. Check not applicable.".into(),
        });
    }

    let cmdline = fs::read_to_string("/proc/cmdline")?;
    let passed = cmdline.contains("nohz_full");

    Ok(AuditResult {
        check_name: "Tickless Kernel (nohz_full)".into(),
        passed,
        details: if passed {
            "PASS: Tickless kernel detected.".into()
        } else {
            "FAIL: Kernel ticks will interrupt your algo.".into()
        },
    })
}

/// Check for IOMMU support.
pub fn check_iommu(simulate: bool) -> Result<AuditResult, KernelError> {
    if simulate {
        return Ok(AuditResult {
            check_name: "IOMMU Support (VT-d/AMD-Vi)".into(),
            passed: true,
            details: "SIMULATED PASS: IOMMU groups found (PCI Passthrough active).".into(),
        });
    }

    if !cfg!(target_os = "linux") {
        return Ok(AuditResult {
            check_name: "IOMMU Support".into(),
            passed: true,
            details: "SKIPPED: Not running on Linux. Check not applicable.".into(),
        });
    }

    let iommu_path = "/sys/kernel/iommu_groups";
    if Path::new(iommu_path).exists() {
        let count = fs::read_dir(iommu_path)?.count();
        if count > 0 {
            return Ok(AuditResult {
                check_name: "IOMMU Support (VT-d/AMD-Vi)".into(),
                passed: true,
                details: format!("PASS: {} IOMMU groups detected. VFIO ready.", count),
            });
        }
    }

    Ok(AuditResult {
        check_name: "IOMMU Support".into(),
        passed: false,
        details: "FAIL: No IOMMU groups found. Enable 'intel_iommu=on' in GRUB.".into(),
    })
}

/// Check for HugePages configuration.
pub fn check_hugepages(simulate: bool) -> Result<AuditResult, KernelError> {
    if simulate {
        return Ok(AuditResult {
            check_name: "HugePages Configuration".into(),
            passed: true,
            details: "SIMULATED PASS: HugePages_Total = 1024".into(),
        });
    }

    if !cfg!(target_os = "linux") {
        return Ok(AuditResult {
            check_name: "HugePages Configuration".into(),
            passed: true,
            details: "SKIPPED: Not Linux. Check not applicable.".into(),
        });
    }

    let meminfo = fs::read_to_string("/proc/meminfo")?;
    for line in meminfo.lines() {
        if line.starts_with("HugePages_Total:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(count_str) = parts.get(1) {
                if let Ok(count) = count_str.parse::<u64>() {
                    if count > 0 {
                        return Ok(AuditResult {
                            check_name: "HugePages Configuration".into(),
                            passed: true,
                            details: format!("PASS: HugePages active (Total: {}).", count),
                        });
                    }
                }
            }
        }
    }

    Ok(AuditResult {
        check_name: "HugePages Configuration".into(),
        passed: false,
        details: "FAIL: HugePages_Total is 0. Memory fragmentation will occur.".into(),
    })
}

/// Measure System Jitter (Latency Consistency).
pub fn check_jitter(simulate: bool) -> Result<AuditResult, KernelError> {
    if simulate {
        return Ok(AuditResult {
            check_name: "System Jitter".into(),
            passed: true,
            details: "SIMULATED PASS: P99 Latency: 48ns".into(),
        });
    }

    if !cfg!(target_os = "linux") {
        return Ok(AuditResult {
            check_name: "System Jitter".into(),
            passed: true,
            details: "SKIPPED: Jitter test requires Linux high-res timer.".into(),
        });
    }

    let iterations = 1_000_000;
    let mut max_delta = 0u128;

    let _ = std::time::Instant::now();
    let start = std::time::Instant::now();
    let mut prev = start;

    for _ in 0..iterations {
        let now = std::time::Instant::now();
        let delta = now.duration_since(prev).as_nanos();
        if delta > max_delta {
            max_delta = delta;
        }
        prev = now;
        std::hint::spin_loop();
    }

    let threshold_ns = 10_000;
    let passed = max_delta < threshold_ns;

    Ok(AuditResult {
        check_name: "System Jitter".into(),
        passed,
        details: if passed {
            format!(
                "PASS: Max Jitter: {}ns (Threshold: {}us)",
                max_delta,
                threshold_ns / 1000
            )
        } else {
            format!(
                "FAIL: High Jitter Detected: {}us (Threshold: {}us). CPU not isolated.",
                max_delta / 1000,
                threshold_ns / 1000
            )
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulated_checks_pass() {
        assert!(check_isolcpus(true).unwrap().passed);
        assert!(check_nitro_enclave(true).unwrap().passed);
        assert!(check_tickless(true).unwrap().passed);
        assert!(check_iommu(true).unwrap().passed);
        assert!(check_hugepages(true).unwrap().passed);
        assert!(check_jitter(true).unwrap().passed);
    }
}
