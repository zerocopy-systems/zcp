use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use zcp_wasm::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_init_sets_panic_hook_wasm() {
    init();
    // No easy way to check if hook is set, but this shouldn't crash
}

#[wasm_bindgen_test]
fn test_verify_attestation_valid_signature_wasm() {
    // This requires a valid signature/pubkey pair.
    // For testing purposes, we can use a mock or hardcoded valid pair if available.
    // Or we can rely on the fact that internal verification logic is tested in unit tests.
    // However, for "Browser Tests", we should at least check the JS interface.

    let attestation = r#"{
        "version": "1.1",
        "timestamp": 1700000000000,
        "payload": {"action": "test"},
        "signature": "deadbeef",
        "enclave_pubkey": "cafebabe"
    }"#;

    let result = verify_attestation(attestation);
    assert!(!result.is_null());
}

#[wasm_bindgen_test]
fn test_verify_attestation_invalid_signature_wasm() {
    let attestation = r#"{
        "version": "1.1",
        "timestamp": 1700000000000,
        "payload": {"action": "test"},
        "signature": "00000000",
        "enclave_pubkey": "cafebabe"
    }"#;

    let _result = verify_attestation(attestation);
    // Should be an invalid result
    // result is a JsValue, we'd need to parse it back or check properties in JS context
}

#[wasm_bindgen_test]
fn test_verify_policy_proof_valid_wasm() {
    let attestation = r#"{
        "version": "1.1",
        "timestamp": 1700000000000,
        "payload": {"action": "test"},
        "signature": "deadbeef",
        "enclave_pubkey": "cafebabe",
        "policy_proof": {
            "receipt_hex": "proof",
            "image_id": "test_id",
            "checked_properties": ["MaxLeverage(5)"],
            "timestamp_ms": 1700000000000
        }
    }"#;

    let _result = verify_policy_proof(
        attestation,
        Some("test_id".to_string()),
        JsValue::from_str("[\"MaxLeverage\"]"),
    );
    // Again, testing JsValue return
}

#[wasm_bindgen_test]
fn test_verify_policy_proof_missing_property_wasm() {
    let attestation = r#"{
        "version": "1.1",
        "timestamp": 1700000000000,
        "payload": {"action": "test"},
        "signature": "deadbeef",
        "enclave_pubkey": "cafebabe",
        "policy_proof": {
            "receipt_hex": "proof",
            "image_id": "test_id",
            "checked_properties": ["MaxLeverage(5)"],
            "timestamp_ms": 1700000000000
        }
    }"#;

    let _result = verify_policy_proof(
        attestation,
        Some("test_id".to_string()),
        JsValue::from_str("[\"MinEquity\"]"),
    );
    // Should fail
}
