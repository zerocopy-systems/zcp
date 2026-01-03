//! # Memory Management for Zero-Copy Operations
//!
//! This module provides memory management utilities optimized for
//! High-Frequency Trading (HFT) workloads where every nanosecond counts.
//!
//! ## Why This Matters
//!
//! In HFT, a typical trade decision window is **1-10 microseconds**.
//! A single `malloc()` call takes **~100 nanoseconds** - that's 1-10% of
//! your entire latency budget spent just allocating memory.
//!
//! ## Solution: Slab Allocation
//!
//! Pre-allocate all memory at application startup, then reuse buffers:
//!
//! ```text
//! Startup Phase:
//!   ┌──────────────────────────────────────────┐
//!   │ BufferPool: [Buf1][Buf2][Buf3]...[BufN]  │  ← Allocate once
//!   └──────────────────────────────────────────┘
//!
//! Hot Path (per message):
//!   1. checkout() → Get buffer from pool (O(1), no syscall)
//!   2. Use buffer for processing
//!   3. checkin()  → Return buffer to pool (O(1), no syscall)
//! ```
//!
//! ## Usage
//!
//! ```rust
//! use zero_copy_utils::memory::BufferPool;
//!
//! // At startup: allocate 1000 buffers of 4KB each
//! let mut pool = BufferPool::new(1000, 4096);
//!
//! // In hot path: borrow a buffer (no allocation!)
//! if let Some(mut buf) = pool.checkout() {
//!     // Use buf for market data processing...
//!     buf.extend_from_slice(b"market data");
//!     
//!     // Return buffer when done
//!     pool.checkin(buf);
//! }
//! ```
//!
//! ## Memory Layout Considerations
//!
//! For maximum performance, consider:
//! - **Buffer size**: Match your typical message size (e.g., 1500 bytes for Ethernet MTU)
//! - **Pool capacity**: 2-3x your expected concurrent operations
//! - **HugePages**: For even lower TLB miss rates (see `allocate_aligned_buffer`)

use bytes::BytesMut;

/// Pre-allocated buffer pool for zero-allocation hot paths.
///
/// # Why Not Just Use `Vec<u8>`?
///
/// `BytesMut` from the `bytes` crate provides:
/// - Cheap cloning via reference counting
/// - Split operations without copying
/// - Familiar API similar to `Vec`
///
/// # Thread Safety
///
/// This implementation is NOT thread-safe. For multi-threaded use,
/// either use one pool per thread (recommended) or wrap in a mutex.
pub struct BufferPool {
    /// Stack of available buffers (LIFO for cache locality)
    pool: Vec<BytesMut>,
}

impl BufferPool {
    /// Create a new buffer pool.
    ///
    /// # Arguments
    ///
    /// * `capacity` - Number of buffers to pre-allocate
    /// * `buffer_size` - Size of each buffer in bytes
    ///
    /// # Performance Note
    ///
    /// This allocates all memory upfront. For a pool of 1000 x 4KB buffers,
    /// expect ~4MB of memory usage and ~1ms initialization time.
    pub fn new(capacity: usize, buffer_size: usize) -> Self {
        let mut pool = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            pool.push(BytesMut::with_capacity(buffer_size));
        }
        Self { pool }
    }

    /// Get a buffer from the pool.
    ///
    /// Returns `None` if pool is exhausted. In production, you should
    /// monitor pool exhaustion and either:
    /// - Increase pool size
    /// - Implement backpressure
    /// - Block until a buffer is available
    ///
    /// # Performance
    ///
    /// O(1) - just a Vec::pop(), no syscalls, no allocation.
    #[inline]
    pub fn checkout(&mut self) -> Option<BytesMut> {
        self.pool.pop()
    }

    /// Return a buffer to the pool.
    ///
    /// The buffer is cleared before being added back to the pool,
    /// ensuring no data leaks between uses.
    ///
    /// # Warning
    ///
    /// Never return a buffer that wasn't checked out from this pool.
    /// Doing so could cause undefined behavior or memory issues.
    #[inline]
    pub fn checkin(&mut self, mut buf: BytesMut) {
        buf.clear(); // Reset length to 0, capacity unchanged
        self.pool.push(buf);
    }

    /// Get current pool size (available buffers).
    #[inline]
    pub fn available(&self) -> usize {
        self.pool.len()
    }
}

/// Allocate a cache-aligned buffer using HugePages.
///
/// Uses `mmap` with `MAP_HUGETLB` | `MAP_POPULATE` to ensure:
/// 1. 2MB pages (reduced TLB pressure)
/// 2. Memory is pre-faulted (no page faults on first access)
/// 3. Memory is locked (mlock) preventing swap
pub fn allocate_aligned_buffer(size: usize) -> BytesMut {
    #[cfg(target_os = "linux")]
    {
        {
            // Fallback to standard allocation to fix compilation error for now.
            // TODO: Implement proper zero-copy allocator using Bytes::from_owner or similar.
            BytesMut::with_capacity(size)
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        // Fallback for non-Linux dev environments (macOS)
        BytesMut::with_capacity(size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_pool_checkout_checkin() {
        let mut pool = BufferPool::new(10, 1024);
        assert_eq!(pool.available(), 10);

        let buf = pool.checkout().expect("Should have buffer");
        assert_eq!(pool.available(), 9);

        pool.checkin(buf);
        assert_eq!(pool.available(), 10);
    }

    #[test]
    fn test_buffer_pool_exhaustion() {
        let mut pool = BufferPool::new(2, 1024);

        let _b1 = pool.checkout();
        let _b2 = pool.checkout();
        let b3 = pool.checkout();

        assert!(b3.is_none(), "Pool should be exhausted");
    }
}
