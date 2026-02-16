//! Error types for ZCP verification.

use thiserror::Error;

/// Errors that can occur during ZCP proof verification.
#[derive(Error, Debug)]
pub enum ZcpError {
    /// Proof version is not supported
    #[error("Unsupported proof version: {0}")]
    UnsupportedVersion(u8),

    /// Invalid enclave measurement format
    #[error("Invalid enclave measurement: {0}")]
    InvalidMeasurement(String),

    /// Latency is suspiciously high (possible replay or non-enclave origin)
    #[error("Suspicious latency: {0}µs (expected < 10,000µs)")]
    SuspiciousLatency(u32),

    /// AI risk score is above acceptable threshold
    #[error("High AI risk score: {0} (threshold: 0.5)")]
    HighRiskScore(f32),

    /// Signature is missing
    #[error("Missing signature")]
    MissingSignature,

    /// Signature verification failed
    #[error("Invalid signature")]
    InvalidSignature,

    /// JSON parsing error
    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),
}
