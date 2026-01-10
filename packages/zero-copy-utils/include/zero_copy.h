#ifndef ZERO_COPY_H
#define ZERO_COPY_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque pointer to BufferPool
typedef struct zc_buffer_pool zc_buffer_pool_t;

// Initialize a buffer pool
zc_buffer_pool_t* zc_pool_new(size_t capacity, size_t buffer_size);

// Borrow a buffer from the pool (returns pointer to data)
void* zc_pool_checkout(zc_buffer_pool_t* pool);

// Return a buffer to the pool
void zc_pool_checkin(zc_buffer_pool_t* pool, void* buffer);

// Free the pool
void zc_pool_free(zc_buffer_pool_t* pool);

#ifdef __cplusplus
}
#endif

#endif // ZERO_COPY_H
