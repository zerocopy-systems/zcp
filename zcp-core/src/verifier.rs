//! Verification logic for ZCP proofs.

use crate::error::ZcpError;
use crate::proof::ZcpProof;

/// Verify a ZCP attestation proof.
///
/// This function checks:
/// 1. Schema version compatibility
/// 2. Enclave measurement against known-good values
/// 3. Signature validity (stub for now)
///
/// Returns `true` if the proof is valid.
pub fn verify_attestation(proof: &ZcpProof) -> Result<bool, ZcpError> {
    // Check version
    if proof.version != 1 {
        return Err(ZcpError::UnsupportedVersion(proof.version));
    }

    // Check enclave measurement format
    if !proof.enclave_measurement.starts_with("PCR0:") {
        return Err(ZcpError::InvalidMeasurement(
            "Measurement must start with 'PCR0:'".to_string(),
        ));
    }

    // FIX: Strict Allowlist Verification
    // In a real deployment, this would be a dynamic list or on-chain registry.
    // For now, we enforce a strict check to prevent "Any PCR0" bypass.
    const ALLOWED_PCR0_PREFIX: &str = "PCR0:sha256:";
    if !proof.enclave_measurement.starts_with(ALLOWED_PCR0_PREFIX) {
        return Err(ZcpError::InvalidMeasurement(
            "Invalid PCR0 format".to_string(),
        ));
    }

    // Check against Known Trusted Measurements (Hardcoded for Safety/Audit)
    // Replace this with actual hash of the production enclave.
    const KNOWN_GOOD_MEASUREMENTS: &[&str] = &[
        "PCR0:sha256:deadbeef", // Test
                                // "PCR0:sha256:REAL_HASH_HERE",
    ];

    if !KNOWN_GOOD_MEASUREMENTS.contains(&proof.enclave_measurement.as_str()) {
        return Err(ZcpError::InvalidMeasurement(format!(
            "Unauthorized Enclave Measurement: {}",
            proof.enclave_measurement
        )));
    }

    // Check latency sanity (< 10ms is expected for enclave signing)
    if proof.latency_us > 10_000 {
        return Err(ZcpError::SuspiciousLatency(proof.latency_us));
    }

    // Check AI risk score if present
    if let Some(score) = proof.ai_risk_score {
        if score > 0.5 {
            return Err(ZcpError::HighRiskScore(score));
        }
    }

    // Signature verification (placeholder)
    // In production: Recover signer from signature, check against enclave pubkey
    if proof.signer_signature.is_empty() {
        return Err(ZcpError::MissingSignature);
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proof::ZcpProof;

    fn valid_proof() -> ZcpProof {
        ZcpProof {
            version: 1,
            tx_hash: "0xabc123".to_string(),
            enclave_measurement: "PCR0:sha256:deadbeef".to_string(),
            timestamp_ns: 1678889991000000000,
            latency_us: 42,
            jitter_us: 5,
            ai_risk_score: Some(0.001),
            signer_signature: "0xvalidsig".to_string(),
        }
    }

    #[test]
    fn test_valid_proof() {
        let proof = valid_proof();
        assert!(verify_attestation(&proof).unwrap());
    }

    #[test]
    fn test_unsupported_version() {
        let mut proof = valid_proof();
        proof.version = 99;
        assert!(matches!(
            verify_attestation(&proof),
            Err(ZcpError::UnsupportedVersion(99))
        ));
    }

    #[test]
    fn test_suspicious_latency() {
        let mut proof = valid_proof();
        proof.latency_us = 50_000; // 50ms
        assert!(matches!(
            verify_attestation(&proof),
            Err(ZcpError::SuspiciousLatency(_))
        ));
    }

    #[test]
    fn test_invalid_measurement_hash() {
        let mut proof = valid_proof();
        proof.enclave_measurement = "PCR0:sha256:malicious_hash".to_string();
        // Should satisfy prefix check but fail allowlist check
        match verify_attestation(&proof) {
            Err(ZcpError::InvalidMeasurement(msg)) => {
                assert!(msg.contains("Unauthorized Enclave Measurement"));
            }
            _ => panic!("Expected InvalidMeasurement error"),
        }
    }

    #[test]
    fn test_invalid_measurement_format() {
        let mut proof = valid_proof();
        proof.enclave_measurement = "BAD:FORMAT".to_string();
        assert!(matches!(
            verify_attestation(&proof),
            Err(ZcpError::InvalidMeasurement(_))
        ));
    }

    #[test]
    fn test_high_risk_score() {
        let mut proof = valid_proof();
        proof.ai_risk_score = Some(0.95);
        assert!(matches!(
            verify_attestation(&proof),
            Err(ZcpError::HighRiskScore(_))
        ));
    }
}
