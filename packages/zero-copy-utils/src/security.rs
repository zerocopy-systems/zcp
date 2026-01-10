#[cfg(target_os = "linux")]
use std::fs::File;
#[cfg(target_os = "linux")]
use std::io::{BufRead, BufReader};
// Removed cfg guard

/// Checks for the presence of a debugger by inspecting /proc/self/status.
/// Returns true if a debugger is detected (TracerPid != 0).
/// Returns true if a debugger is detected (TracerPid != 0).
pub fn check_debugger() -> bool {
    #[cfg(target_os = "linux")]
    {
        if let Ok(file) = File::open("/proc/self/status") {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(l) = line {
                    if l.starts_with("TracerPid:") {
                        let parts: Vec<&str> = l.split_whitespace().collect();
                        if parts.len() > 1 {
                            if let Ok(pid) = parts[1].parse::<i32>() {
                                return pid != 0;
                            }
                        }
                    }
                }
            }
        }
    }
    // On non-Linux (e.g. macOS dev), we assume no debugger for now or use ptrace if needed.
    // Ideally we'd use syssysctl on Mac but for Enclave context (Linux) this is sufficient.
    false
}

/// A simple RDTSC-based timing check to detect massive slowdowns (e.g. single stepping).
/// Returns true if execution was suspiciously slow.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn check_timing_anomaly() -> bool {
    use core::arch::x86_64::_rdtsc;

    // We measure a very simple operation
    // SAFETY: _rdtsc is an intrinsic that reads the CPU timestamp counter.
    // It has no side effects and is safe on all x86/x86_64 CPUs.
    let start = unsafe { _rdtsc() };

    // Simple ALU ops shouldn't take long
    let mut x = 0;
    for i in 0..100 {
        x += i;
    }
    std::hint::black_box(x); // Prevent compiler optimization

    // SAFETY: _rdtsc is an intrinsic that reads the CPU timestamp counter.
    // It has no side effects and is safe on all x86/x86_64 CPUs.
    let end = unsafe { _rdtsc() };
    let delta = end - start;

    // Threshold is arbitrary but should be well above normal execution
    // VM exits / Singlestep would make this huge.
    // 10,000 cycles is generous for 100 additions.
    delta > 10_000
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn check_timing_anomaly() -> bool {
    false
}
