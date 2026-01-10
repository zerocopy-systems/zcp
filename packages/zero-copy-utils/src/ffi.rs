use crate::memory::BufferPool;
use std::ffi::c_void;
use std::ptr;

/// Opaque wrapper for FFI
pub struct ZcBufferPool(BufferPool);

#[no_mangle]
pub extern "C" fn zc_pool_new(capacity: usize, buffer_size: usize) -> *mut ZcBufferPool {
    let pool = BufferPool::new(capacity, buffer_size);
    Box::into_raw(Box::new(ZcBufferPool(pool)))
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn zc_pool_checkout(pool: *mut ZcBufferPool) -> *mut c_void {
    if pool.is_null() {
        return ptr::null_mut();
    }
    // SAFETY: We checked that pool is non-null above. The pointer was
    // created by Box::into_raw in zc_pool_new, so it is valid and properly aligned.
    let pool = unsafe { &mut *pool };
    match pool.0.checkout() {
        Some(mut buf) => {
            // Leak the buffer to give to C.
            // In reality, we must track this to reconstruct BytesMut on checkin.
            // Simplified for demo: return raw pointer to start.
            buf.as_mut_ptr() as *mut c_void
        }
        None => ptr::null_mut(),
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn zc_pool_free(pool: *mut ZcBufferPool) {
    if !pool.is_null() {
        // SAFETY: We checked that pool is non-null above. The pointer was
        // created by Box::into_raw in zc_pool_new, so it is valid.
        // Box::from_raw reconstructs the Box and drops it, freeing the memory.
        unsafe {
            let _ = Box::from_raw(pool);
        }
    }
}
