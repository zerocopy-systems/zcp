//! ZCP WASM - Browser-based attestation verification
//!
//! This crate provides WebAssembly bindings for verifying ZeroCopy attestations
//! directly in web browsers, enabling self-service verification portals.

use k256::ecdsa::{signature::Verifier, Signature, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Initialize the WASM module
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    set_panic_hook();
}

/// ZK Proof of policy compliance
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PolicyProof {
    pub receipt_hex: String,
    pub image_id: String,
    pub checked_properties: Vec<String>,
    pub timestamp_ms: u64,
}

/// The standard .zcp Attestation Format
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ZcpAttestation {
    pub version: String,
    pub timestamp: u64,
    pub payload: serde_json::Value,
    pub signature: String,
    pub enclave_pubkey: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_proof: Option<PolicyProof>,
}

/// Result of attestation verification
#[derive(Serialize, Deserialize)]
pub struct VerificationResult {
    pub valid: bool,
    pub error: Option<String>,
    pub checked_properties: Option<Vec<String>>,
    pub timestamp_ms: Option<u64>,
}

/// Verify a ZCP attestation signature
///
/// # Arguments
/// * `attestation_json` - JSON string of the attestation
///
/// # Returns
/// * `VerificationResult` as JSON string
#[wasm_bindgen]
pub fn verify_attestation(attestation_json: &str) -> JsValue {
    let result = verify_attestation_internal(attestation_json);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

fn verify_attestation_internal(attestation_json: &str) -> VerificationResult {
    // Parse attestation
    let attestation: ZcpAttestation = match serde_json::from_str(attestation_json) {
        Ok(a) => a,
        Err(e) => {
            return VerificationResult {
                valid: false,
                error: Some(format!("Failed to parse attestation: {}", e)),
                checked_properties: None,
                timestamp_ms: None,
            }
        }
    };

    // Decode public key
    let pk_bytes = match hex::decode(&attestation.enclave_pubkey) {
        Ok(b) => b,
        Err(e) => {
            return VerificationResult {
                valid: false,
                error: Some(format!("Invalid public key hex: {}", e)),
                checked_properties: None,
                timestamp_ms: None,
            }
        }
    };

    let pub_key = match VerifyingKey::from_sec1_bytes(&pk_bytes) {
        Ok(k) => k,
        Err(_) => {
            return VerificationResult {
                valid: false,
                error: Some("Invalid public key format".to_string()),
                checked_properties: None,
                timestamp_ms: None,
            }
        }
    };

    // Decode signature
    let sig_bytes = match hex::decode(&attestation.signature) {
        Ok(b) => b,
        Err(e) => {
            return VerificationResult {
                valid: false,
                error: Some(format!("Invalid signature hex: {}", e)),
                checked_properties: None,
                timestamp_ms: None,
            }
        }
    };

    let signature = match Signature::from_slice(&sig_bytes) {
        Ok(s) => s,
        Err(_) => {
            return VerificationResult {
                valid: false,
                error: Some("Invalid signature format".to_string()),
                checked_properties: None,
                timestamp_ms: None,
            }
        }
    };

    // Hash the payload (matching the enclave's signing method)
    let msg = attestation.payload.to_string();
    let mut hasher = Sha256::new();
    hasher.update(msg.as_bytes());
    let msg_hash = hasher.finalize();

    // Verify signature
    if pub_key.verify(&msg_hash, &signature).is_err() {
        return VerificationResult {
            valid: false,
            error: Some("Signature verification failed".to_string()),
            checked_properties: None,
            timestamp_ms: None,
        };
    }

    // Extract policy proof info if present
    let (checked_properties, timestamp_ms) = if let Some(proof) = attestation.policy_proof {
        (Some(proof.checked_properties), Some(proof.timestamp_ms))
    } else {
        (None, None)
    };

    VerificationResult {
        valid: true,
        error: None,
        checked_properties,
        timestamp_ms,
    }
}

/// Verify only the policy proof portion
///
/// # Arguments
/// * `attestation_json` - JSON string of the attestation
/// * `expected_image_id` - Optional image ID to check (pass null to skip)
/// * `required_properties` - Array of property prefixes that must be present
///
/// # Returns
/// * `VerificationResult` as JSON
#[wasm_bindgen]
pub fn verify_policy_proof(
    attestation_json: &str,
    expected_image_id: Option<String>,
    required_properties: JsValue,
) -> JsValue {
    let result =
        verify_policy_proof_internal(attestation_json, expected_image_id, required_properties);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

fn verify_policy_proof_internal(
    attestation_json: &str,
    expected_image_id: Option<String>,
    required_properties: JsValue,
) -> VerificationResult {
    // Parse attestation
    let attestation: ZcpAttestation = match serde_json::from_str(attestation_json) {
        Ok(a) => a,
        Err(e) => {
            return VerificationResult {
                valid: false,
                error: Some(format!("Failed to parse attestation: {}", e)),
                checked_properties: None,
                timestamp_ms: None,
            }
        }
    };

    // Check if policy proof exists
    let proof = match attestation.policy_proof {
        Some(p) => p,
        None => {
            return VerificationResult {
                valid: false,
                error: Some("Policy proof missing".to_string()),
                checked_properties: None,
                timestamp_ms: None,
            }
        }
    };

    // Check image ID if provided
    if let Some(expected) = expected_image_id {
        if proof.image_id != expected {
            return VerificationResult {
                valid: false,
                error: Some(format!(
                    "Image ID mismatch: expected {}, got {}",
                    expected, proof.image_id
                )),
                checked_properties: None,
                timestamp_ms: None,
            };
        }
    }

    // Parse required properties from JsValue
    let required: Vec<String> =
        serde_wasm_bindgen::from_value(required_properties).unwrap_or_default();

    // Check all required properties are present
    for req in &required {
        if !proof.checked_properties.iter().any(|p| p.starts_with(req)) {
            return VerificationResult {
                valid: false,
                error: Some(format!("Missing required property: {}", req)),
                checked_properties: None,
                timestamp_ms: None,
            };
        }
    }

    // Check receipt is not empty
    if proof.receipt_hex.is_empty() {
        return VerificationResult {
            valid: false,
            error: Some("Empty receipt".to_string()),
            checked_properties: None,
            timestamp_ms: None,
        };
    }

    VerificationResult {
        valid: true,
        error: None,
        checked_properties: Some(proof.checked_properties),
        timestamp_ms: Some(proof.timestamp_ms),
    }
}

/// Check if an attestation has a policy proof
#[wasm_bindgen]
pub fn has_policy_proof(attestation_json: &str) -> bool {
    serde_json::from_str::<ZcpAttestation>(attestation_json)
        .map(|a| a.policy_proof.is_some())
        .unwrap_or(false)
}

/// Get the version of the WASM module
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_policy_proof() {
        let with_proof = r#"{
            "version": "1.1",
            "timestamp": 1700000000000,
            "payload": {"action": "test"},
            "signature": "deadbeef",
            "enclave_pubkey": "cafebabe",
            "policy_proof": {
                "receipt_hex": "proof",
                "image_id": "test",
                "checked_properties": ["MaxLeverage(5)"],
                "timestamp_ms": 1700000000000
            }
        }"#;

        let without_proof = r#"{
            "version": "1.0",
            "timestamp": 1700000000000,
            "payload": {"action": "test"},
            "signature": "deadbeef",
            "enclave_pubkey": "cafebabe"
        }"#;

        assert!(has_policy_proof(with_proof));
        assert!(!has_policy_proof(without_proof));
    }

    #[test]
    fn test_verify_policy_proof_missing() {
        let attestation = r#"{
            "version": "1.0",
            "timestamp": 1700000000000,
            "payload": {"action": "test"},
            "signature": "deadbeef",
            "enclave_pubkey": "cafebabe"
        }"#;

        let result = verify_policy_proof_internal(attestation, None, JsValue::NULL);
        assert!(!result.valid);
        assert!(result.error.unwrap().contains("missing"));
    }

    #[test]
    fn test_version() {
        assert!(!version().is_empty());
    }
}
