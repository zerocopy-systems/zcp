//! # Runtime Utilities for Low-Latency Applications
//!
//! This module provides runtime primitives optimized for HFT workloads
//! where consistent, predictable latency is critical.
//!
//! ## The Problem with `sleep()`
//!
//! When you call `std::thread::sleep()`, the OS puts your thread to sleep
//! and schedules something else. Waking up involves:
//!
//! 1. Timer interrupt fires
//! 2. Kernel scheduler runs
//! 3. Your thread is marked runnable
//! 4. Context switch back to your thread
//!
//! This takes **50-100 microseconds** - an eternity in HFT.
//!
//! ## Solution: Spin-Waiting
//!
//! Instead of sleeping, we keep the CPU busy in a tight loop:
//!
//! ```text
//! while not_ready {
//!     spin_loop();  // Tell CPU we're spinning (improves hyperthreading)
//! }
//! ```
//!
//! Wake-up latency: **< 1 microsecond**
//!
//! ## ⚠️ Critical Warnings
//!
//! **DO NOT** use spin-waiting unless:
//! 1. The thread runs on an **isolated CPU** (`isolcpus` kernel parameter)
//! 2. The wait time is **very short** (microseconds, not milliseconds)
//! 3. You have **measured** that sleep() is too slow for your use case
//!
//! Spin-waiting on a shared CPU will:
//! - Starve other processes
//! - Cause thermal throttling
//! - Waste energy
//! - Make your laptop sound like a jet engine

use std::hint;
use std::time::{Duration, Instant};

/// A busy-waiter that keeps the CPU active for ultra-low-latency wake-up.
///
/// # When to Use
///
/// - Waiting for market data with <10μs latency requirements
/// - Synchronizing between threads on isolated cores
/// - Implementing lock-free data structures
///
/// # When NOT to Use
///
/// - Any wait longer than a few milliseconds
/// - On shared/non-isolated CPUs
/// - In applications that aren't latency-critical
///
/// # Example
///
/// ```rust
/// use zero_copy_utils::runtime::BusyWaiter;
/// use std::time::Duration;
///
/// // Wait exactly 100 microseconds with <1μs jitter
/// BusyWaiter::spin_for(Duration::from_micros(100));
/// ```
pub struct BusyWaiter;

impl BusyWaiter {
    /// Spin for at least the specified duration.
    ///
    /// This burns 100% CPU on the calling core but ensures
    /// sub-microsecond wake-up precision.
    ///
    /// # Arguments
    ///
    /// * `duration` - Minimum time to spin
    ///
    /// # CPU Usage
    ///
    /// This will use 100% of one CPU core for the entire duration.
    /// Only use on isolated cores where this is acceptable.
    ///
    /// # Precision
    ///
    /// Actual spin time will be >= `duration`. The overhead is
    /// typically 50-200 nanoseconds due to the loop check.
    #[inline]
    pub fn spin_for(duration: Duration) {
        let start = Instant::now();
        while start.elapsed() < duration {
            // PAUSE instruction on x86
            // ARM: YIELD instruction
            // This tells the CPU we're in a spin loop and allows
            // hyperthreaded sibling cores to use more resources
            hint::spin_loop();
        }
    }

    /// Spin until a specific deadline.
    ///
    /// More efficient than `spin_for` when you have an absolute
    /// deadline rather than a relative duration.
    ///
    /// # Use Case
    ///
    /// Trading strategy needs to submit order at exactly T+100μs:
    ///
    /// ```ignore
    /// let deadline = Instant::now() + Duration::from_micros(100);
    /// prepare_order();  // Takes ~50μs
    /// BusyWaiter::spin_until(deadline);  // Wait remaining ~50μs
    /// submit_order();
    /// ```
    #[inline]
    pub fn spin_until(deadline: Instant) {
        while Instant::now() < deadline {
            hint::spin_loop();
        }
    }

    /// Spin with a condition check.
    ///
    /// More efficient than polling in a loop with sleep because
    /// there's no OS involvement.
    ///
    /// # Arguments
    ///
    /// * `condition` - Function that returns true when we should stop
    /// * `timeout` - Maximum time to spin
    ///
    /// # Returns
    ///
    /// `true` if condition became true, `false` if timeout expired.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use std::sync::atomic::{AtomicBool, Ordering};
    ///
    /// let ready = AtomicBool::new(false);
    ///
    /// // In another thread: ready.store(true, Ordering::Release);
    ///
    /// let success = BusyWaiter::spin_until_condition(
    ///     || ready.load(Ordering::Acquire),
    ///     Duration::from_millis(1),
    /// );
    /// ```
    pub fn spin_until_condition<F>(condition: F, timeout: Duration) -> bool
    where
        F: Fn() -> bool,
    {
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            if condition() {
                return true;
            }
            hint::spin_loop();
        }
        false
    }
}

/// Pin the current thread to a specific CPU core.
///
/// # Required for HFT
///
/// Pinning ensures:
/// 1. No context switches to other cores
/// 2. L1/L2 cache locality
/// 3. Memory allocated on local NUMA node
///
/// # Arguments
///
/// * `core_id` - The logical CPU core ID (0..N-1)
pub fn pin_current_thread(_core_id: usize) -> std::io::Result<()> {
    #[cfg(target_os = "linux")]
    // SAFETY: We pass valid pointers to CPU_SET and sched_setaffinity.
    // The cpu_set_t is zeroed before use, and core_id is validated by the caller.
    // sched_setaffinity only affects the current thread (pid=0).
    unsafe {
        use std::mem;
        let mut set: libc::cpu_set_t = mem::zeroed();
        libc::CPU_SET(core_id, &mut set);

        let ret = libc::sched_setaffinity(0, mem::size_of::<libc::cpu_set_t>(), &set);
        if ret != 0 {
            return Err(std::io::Error::last_os_error());
        }
    }
    Ok(())
}

/// Get current timestamp in nanoseconds since UNIX EPOCH.
///
/// Uses `CLOCK_REALTIME` via simple system call.
/// For monotonic (duration measuring) use `std::time::Instant`.
/// This is for wall-clock logging.
#[inline]
pub fn now_nanos() -> u64 {
    #[cfg(target_os = "linux")]
    // SAFETY: clock_gettime is a safe system call when passed a valid timespec.
    // We zero the struct before use and pass a correct clock ID.
    unsafe {
        let mut ts: libc::timespec = std::mem::zeroed();
        libc::clock_gettime(libc::CLOCK_REALTIME, &mut ts);
        (ts.tv_sec as u64) * 1_000_000_000 + (ts.tv_nsec as u64)
    }
    #[cfg(not(target_os = "linux"))]
    {
        use std::time::SystemTime;
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spin_for_minimum_duration() {
        let start = Instant::now();
        BusyWaiter::spin_for(Duration::from_micros(100));
        let elapsed = start.elapsed();

        // Should be at least 100μs
        assert!(elapsed >= Duration::from_micros(100));
        // Should be less than 1ms (not way over)
        assert!(elapsed < Duration::from_millis(1));
    }

    #[test]
    fn test_spin_until_condition_success() {
        let result = BusyWaiter::spin_until_condition(
            || true, // Immediately true
            Duration::from_millis(1),
        );
        assert!(result);
    }

    #[test]
    fn test_spin_until_condition_timeout() {
        let result = BusyWaiter::spin_until_condition(
            || false, // Never true
            Duration::from_micros(100),
        );
        assert!(!result);
    }
}
