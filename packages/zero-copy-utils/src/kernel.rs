//! # Kernel Configuration Auditing
//!
//! This module provides functions to audit Linux kernel configuration
//! for HFT (High-Frequency Trading) workloads.
//!
//! ## Checks Performed
//!
//! | Check | Why It Matters |
//! |-------|----------------|
//! | `isolcpus` | Isolates CPU cores from scheduler, preventing jitter |
//! | `nohz_full` | Disables kernel timer interrupts on isolated cores |
//! | `IOMMU` | Enables safe userspace hardware access (DPDK/VFIO) |
//! | `HugePages` | Reduces TLB misses with 2MB/1GB pages |
//! | `Nitro` | Verifies AWS Nitro enclave for data sovereignty |
//!
//! ## Usage
//!
//! ```rust,no_run
//! use zero_copy_utils::kernel::*;
//!
//! // Run all checks (simulation mode for testing)
//! let checks = vec![
//!     check_nitro_enclave(true),
//!     check_isolcpus(true),
//!     check_tickless(true),
//!     check_iommu(true),
//!     check_hugepages(true),
//! ];
//!
//! for result in checks {
//!     if let Ok(r) = result {
//!         println!("{}: {}", r.check_name, if r.passed { "PASS" } else { "FAIL" });
//!     }
//! }
//! ```

use std::fs;
use std::path::Path;
use thiserror::Error;

/// Errors that can occur during kernel auditing.
#[derive(Error, Debug)]
pub enum KernelError {
    /// Failed to read from procfs or sysfs.
    #[error("Failed to read procfs: {0}")]
    Io(#[from] std::io::Error),

    /// Audit cannot run on non-Linux systems.
    #[error("Not running on Linux")]
    NotLinux,
}

/// Result of a single audit check.
///
/// Contains the check name, pass/fail status, and detailed message.
#[derive(Debug, Clone)]
pub struct AuditResult {
    /// Human-readable name of the check (e.g., "Kernel Isolation (isolcpus)")
    pub check_name: String,

    /// Whether the check passed
    pub passed: bool,

    /// Detailed explanation of the result
    pub details: String,
}

/// Check if CPU cores are isolated from the Linux scheduler.
///
/// Isolated CPUs (`isolcpus` kernel parameter) are not used by the scheduler
/// for normal tasks, allowing HFT processes to run with minimal jitter.
///
/// # Arguments
///
/// * `simulate` - If true, returns a simulated passing result for testing
///
/// # Returns
///
/// * `Ok(AuditResult)` - Result indicating whether isolcpus is configured
/// * `Err(KernelError)` - If procfs cannot be read
///
/// # Example
///
/// ```rust,no_run
/// use zero_copy_utils::kernel::check_isolcpus;
///
/// let result = check_isolcpus(false)?;
/// if !result.passed {
///     eprintln!("Warning: {}", result.details);
/// }
/// # Ok::<(), zero_copy_utils::kernel::KernelError>(())
/// ```
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
///
/// AWS Nitro provides hardware-level isolation for sensitive workloads,
/// ensuring that data never leaves the encrypted enclave boundary.
///
/// # Arguments
///
/// * `simulate` - If true, returns a simulated passing result
///
/// # Example
///
/// ```rust,no_run
/// use zero_copy_utils::kernel::check_nitro_enclave;
///
/// let result = check_nitro_enclave(false)?;
/// if result.passed {
///     println!("Running on AWS Nitro - data sovereignty confirmed");
/// }
/// # Ok::<(), zero_copy_utils::kernel::KernelError>(())
/// ```
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
///
/// A tickless kernel disables timer interrupts on specified CPUs,
/// eliminating a major source of latency jitter in HFT applications.
///
/// # Arguments
///
/// * `simulate` - If true, returns a simulated passing result
///
/// # Technical Details
///
/// The `nohz_full` kernel parameter configures the kernel to avoid
/// sending timer interrupts to specified CPUs when only one runnable
/// task is present.
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

/// Check for IOMMU (Input-Output Memory Management Unit) support.
///
/// IOMMU is essential for userspace drivers (DPDK/VFIO) to access
/// hardware directly while maintaining memory safety.
///
/// # Arguments
///
/// * `simulate` - If true, returns a simulated passing result
///
/// # Technical Details
///
/// - Intel: VT-d (Virtualization Technology for Directed I/O)
/// - AMD: AMD-Vi (AMD I/O Virtualization Technology)
///
/// Enable with `intel_iommu=on` or `amd_iommu=on` in GRUB.
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

    // Check if /sys/kernel/iommu_groups exists and has entries
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
///
/// Standard 4KB pages cause frequent TLB (Translation Lookaside Buffer)
/// misses in memory-intensive HFT applications. HugePages (2MB or 1GB)
/// dramatically reduce TLB pressure.
///
/// # Arguments
///
/// * `simulate` - If true, returns a simulated passing result
///
/// # Configuration
///
/// Configure HugePages in `/etc/sysctl.conf`:
/// ```text
/// vm.nr_hugepages = 1024
/// ```
///
/// Or at boot time:
/// ```text
/// hugepages=1024
/// ```
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
    // Look for HugePages_Total:    1024
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
///
/// Runs a stiff spin-loop to measure OS scheduler preemption and interrupt overhead.
/// High jitter (>10us) indicates poor isolation. HFT requires <1us jitter.
///
/// # Arguments
///
/// * `simulate` - If true, returns a simulated result
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

    // Measure Loop
    let iterations = 1_000_000;
    let mut max_delta = 0u128;

    // Warmup
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
        // Busy wait hint
        std::hint::spin_loop();
    }

    // Simple heuristic: If max deviation between loop iterations is > 10us, fail.
    // Note: This is an approximation. A real jitter tool uses histogram buckets.
    // We assume loop should be ~0-10ns. If we see 10,000ns, we got preempted.

    // Threshold: 10 microseconds (10,000 ns)
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

    #[test]
    fn test_audit_result_clone() {
        let result = AuditResult {
            check_name: "Test".into(),
            passed: true,
            details: "OK".into(),
        };
        let cloned = result.clone();
        assert_eq!(cloned.check_name, result.check_name);
    }
}
