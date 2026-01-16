//! Vendored shared types from sentinel-shared.
//!
//! This is a minimal copy of the types needed by zcp.
//! Uses serde instead of rkyv for simpler dependency graph.

use serde::{Deserialize, Serialize};

/// State payload for state export/import operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatePayload {
    /// Encrypted Key Handle (KMS Encrypted Blob)
    pub ciphertext: Vec<u8>,
    /// Metadata (e.g. key ID)
    pub key_id: String,
}
