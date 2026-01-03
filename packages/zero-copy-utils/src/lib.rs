//! # ZeroCopy Utils
//!
//! Core utilities for HFT infrastructure latency optimization.
//!
//! This crate provides low-level utilities for:
//! - **Kernel configuration auditing**: Verify isolcpus, tickless, IOMMU, HugePages
//! - **Memory management**: Zero-copy buffer operations
//! - **Runtime introspection**: System capability detection
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use zero_copy_utils::kernel;
//!
//! // Check if the system is configured for HFT
//! let result = kernel::check_isolcpus(false).unwrap();
//! if result.passed {
//!     println!("CPU isolation configured correctly");
//! }
//! ```
//!
//! ## Modules
//!
//! - [`kernel`]: Linux kernel configuration checks (isolcpus, nohz_full, IOMMU)
//! - [`memory`]: Zero-copy memory operations and HugePages management
//! - [`runtime`]: Runtime system introspection utilities
//!
//! ## Feature Flags
//!
//! - `simulation`: Enable simulation mode for testing without real hardware

pub mod ffi;
pub mod fix;
pub mod gap_detector;
pub mod kernel;
pub mod memory;
pub mod network;
pub mod runtime;
pub mod security;

/// Initialize the ZeroCopy Utils library.
///
/// Call this once at application startup to configure logging
/// and perform initial system checks.
///
/// # Example
///
/// ```rust
/// zero_copy_utils::init();
/// ```
pub fn init() {
    #[cfg(debug_assertions)]
    eprintln!(
        "ZeroCopy Utils v{} initialized (debug mode)",
        env!("CARGO_PKG_VERSION")
    );
}

/// Library version string.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
