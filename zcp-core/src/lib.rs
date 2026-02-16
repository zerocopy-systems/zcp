//! ZeroCopy Core SDK
//!
//! Verify Sovereign Attestations (`.zcp` proofs) from ZeroCopy Enclaves.
//!
//! # Example
//! ```ignore
//! use zcp_core::{ZcpProof, verify_attestation};
//!
//! let proof = ZcpProof::from_json(raw_json)?;
//! if verify_attestation(&proof) {
//!     println!("Transaction was signed in a verified enclave!");
//! }
//! ```

mod error;
mod proof;
mod verifier;

pub use error::ZcpError;
pub use proof::ZcpProof;
pub use verifier::verify_attestation;

/// Re-export for convenience
pub mod prelude {
    pub use crate::{verify_attestation, ZcpError, ZcpProof};
}
