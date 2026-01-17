//! Vendored dependencies for standalone builds.
//!
//! This module contains minimal copies of shared code from the parent workspace.
//! These allow zcp to be built independently while maintaining API compatibility.

pub mod kernel;
pub mod shared;

// Re-export for convenience
pub use shared::StatePayload;
