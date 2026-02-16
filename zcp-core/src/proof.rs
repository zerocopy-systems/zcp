//! ZCP Proof Schema - The `.zcp` attestation format.

use serde::{Deserialize, Serialize};

/// A ZeroCopy Proof - cryptographic evidence of sovereign execution.
///
/// This struct represents the `.zcp` file format, containing:
/// - The transaction signature
/// - Enclave measurement (PCR0)
/// - Execution physics (latency, jitter)
/// - AI risk assessment (if applicable)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZcpProof {
    /// Version of the proof schema
    pub version: u8,

    /// Transaction hash (0x-prefixed hex)
    pub tx_hash: String,

    /// Enclave measurement (PCR0 from Nitro attestation)
    pub enclave_measurement: String,

    /// Unix timestamp in nanoseconds
    pub timestamp_ns: u64,

    /// Signing latency in microseconds
    pub latency_us: u32,

    /// Latency jitter (standard deviation) in microseconds
    pub jitter_us: u32,

    /// AI Risk Score (0.0 - 1.0, lower is safer)
    pub ai_risk_score: Option<f32>,

    /// Signer's signature over the proof contents
    pub signer_signature: String,
}

impl ZcpProof {
    /// Parse a ZCP proof from JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize the proof to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Get the proof fingerprint (SHA256 of canonical JSON).
    pub fn fingerprint(&self) -> String {
        use sha2::{Digest, Sha256};

        // Create canonical representation (without signature)
        let canonical = format!(
            "{}|{}|{}|{}|{}|{}",
            self.version,
            self.tx_hash,
            self.enclave_measurement,
            self.timestamp_ns,
            self.latency_us,
            self.jitter_us,
        );

        let mut hasher = Sha256::new();
        hasher.update(canonical.as_bytes());
        hex::encode(hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_roundtrip() {
        let proof = ZcpProof {
            version: 1,
            tx_hash: "0xabc123".to_string(),
            enclave_measurement: "PCR0:sha256:deadbeef".to_string(),
            timestamp_ns: 1678889991000000000,
            latency_us: 42,
            jitter_us: 5,
            ai_risk_score: Some(0.001),
            signer_signature: "0xsig".to_string(),
        };

        let json = proof.to_json().unwrap();
        let parsed = ZcpProof::from_json(&json).unwrap();

        assert_eq!(parsed.tx_hash, proof.tx_hash);
        assert_eq!(parsed.latency_us, 42);
    }

    #[test]
    fn test_fingerprint() {
        let proof = ZcpProof {
            version: 1,
            tx_hash: "0xabc".to_string(),
            enclave_measurement: "PCR0".to_string(),
            timestamp_ns: 0,
            latency_us: 0,
            jitter_us: 0,
            ai_risk_score: None,
            signer_signature: "".to_string(),
        };

        let fp = proof.fingerprint();
        assert_eq!(fp.len(), 64); // SHA256 = 32 bytes = 64 hex chars
    }
}
