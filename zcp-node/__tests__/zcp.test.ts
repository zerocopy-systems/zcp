import { ec as EC } from "elliptic";
import { createHash } from "crypto";
import {
  ZcpAttestation,
  PolicyProof,
  verifyAttestation,
  verifyPolicyProof,
  hasPolicyProof,
  getMaxLeverage,
  hasProperty,
  getVersionTuple,
} from "../src/index";

const ec = new EC("secp256k1");

// ============================================================================
// Test Helpers
// ============================================================================

function createMockAttestation(options: {
  withPolicyProof?: boolean;
  validSignature?: boolean;
  version?: string;
  properties?: string[];
}): ZcpAttestation {
  const key = ec.genKeyPair();
  const payload = { action: "test", value: 123 };
  const msg = JSON.stringify(payload);
  const msgHash = createHash("sha256").update(msg).digest();

  let signature: string;
  if (options.validSignature !== false) {
    const sig = key.sign(msgHash);
    signature = sig.toDER("hex");
  } else {
    signature = "invalid_signature_here";
  }

  const attestation: ZcpAttestation = {
    version: options.version || "1.1",
    timestamp: Date.now(),
    payload,
    signature,
    enclave_pubkey: key.getPublic("hex"),
  };

  if (options.withPolicyProof) {
    attestation.policy_proof = {
      receipt_hex: "deadbeef",
      image_id: "test_image_123",
      checked_properties: options.properties || [
        "MaxLeverage(5)",
        'AllowedPairs(["BTC-USDT"])',
      ],
      timestamp_ms: Date.now(),
    };
  }

  return attestation;
}

// ============================================================================
// verifyAttestation Tests
// ============================================================================

describe("verifyAttestation", () => {
  test("valid_signature returns true", () => {
    const attestation = createMockAttestation({ validSignature: true });
    expect(verifyAttestation(attestation)).toBe(true);
  });

  test("invalid_signature returns false", () => {
    const attestation = createMockAttestation({ validSignature: false });
    expect(verifyAttestation(attestation)).toBe(false);
  });

  test("malformed_pubkey returns false", () => {
    const attestation = createMockAttestation({ validSignature: true });
    attestation.enclave_pubkey = "not_a_valid_pubkey";
    expect(verifyAttestation(attestation)).toBe(false);
  });

  test("tampered_payload returns false", () => {
    const attestation = createMockAttestation({ validSignature: true });
    attestation.payload.value = 999; // Tamper with payload
    expect(verifyAttestation(attestation)).toBe(false);
  });
});

// ============================================================================
// verifyPolicyProof Tests
// ============================================================================

describe("verifyPolicyProof", () => {
  test("missing_proof returns error", () => {
    const attestation = createMockAttestation({ withPolicyProof: false });
    const result = verifyPolicyProof(attestation);
    expect(result.valid).toBe(false);
    expect(result.error).toBe("Policy proof missing");
  });

  test("valid_proof returns success", () => {
    const attestation = createMockAttestation({ withPolicyProof: true });
    const result = verifyPolicyProof(attestation);
    expect(result.valid).toBe(true);
    expect(result.checkedProperties).toHaveLength(2);
  });

  test("wrong_image_id returns error", () => {
    const attestation = createMockAttestation({ withPolicyProof: true });
    const result = verifyPolicyProof(attestation, "expected_different_image");
    expect(result.valid).toBe(false);
    expect(result.error).toContain("Image ID mismatch");
  });

  test("correct_image_id returns success", () => {
    const attestation = createMockAttestation({ withPolicyProof: true });
    const result = verifyPolicyProof(attestation, "test_image_123");
    expect(result.valid).toBe(true);
  });

  test("missing_required_property returns error", () => {
    const attestation = createMockAttestation({
      withPolicyProof: true,
      properties: ["MaxLeverage(5)"],
    });
    const result = verifyPolicyProof(attestation, undefined, ["MaxOrderSize"]);
    expect(result.valid).toBe(false);
    expect(result.error).toContain("Missing required property");
  });

  test("has_required_property returns success", () => {
    const attestation = createMockAttestation({
      withPolicyProof: true,
      properties: ["MaxLeverage(5)", "MaxOrderSize(10000)"],
    });
    const result = verifyPolicyProof(attestation, undefined, ["MaxLeverage"]);
    expect(result.valid).toBe(true);
  });

  test("empty_receipt returns error", () => {
    const attestation = createMockAttestation({ withPolicyProof: true });
    attestation.policy_proof!.receipt_hex = "";
    const result = verifyPolicyProof(attestation);
    expect(result.valid).toBe(false);
    expect(result.error).toBe("Empty receipt");
  });
});

// ============================================================================
// hasPolicyProof Tests
// ============================================================================

describe("hasPolicyProof", () => {
  test("with_proof returns true", () => {
    const attestation = createMockAttestation({ withPolicyProof: true });
    expect(hasPolicyProof(attestation)).toBe(true);
  });

  test("without_proof returns false", () => {
    const attestation = createMockAttestation({ withPolicyProof: false });
    expect(hasPolicyProof(attestation)).toBe(false);
  });
});

// ============================================================================
// getMaxLeverage Tests
// ============================================================================

describe("getMaxLeverage", () => {
  test("parses_leverage_correctly", () => {
    const attestation = createMockAttestation({
      withPolicyProof: true,
      properties: ["MaxLeverage(10)"],
    });
    expect(getMaxLeverage(attestation)).toBe(10);
  });

  test("returns_null_without_proof", () => {
    const attestation = createMockAttestation({ withPolicyProof: false });
    expect(getMaxLeverage(attestation)).toBeNull();
  });

  test("returns_null_without_leverage_property", () => {
    const attestation = createMockAttestation({
      withPolicyProof: true,
      properties: ['AllowedPairs(["BTC-USDT"])'],
    });
    expect(getMaxLeverage(attestation)).toBeNull();
  });
});

// ============================================================================
// hasProperty Tests
// ============================================================================

describe("hasProperty", () => {
  test("finds_existing_property", () => {
    const attestation = createMockAttestation({
      withPolicyProof: true,
      properties: ["MaxLeverage(5)", 'AllowedPairs(["ETH-USDT"])'],
    });
    expect(hasProperty(attestation, "MaxLeverage")).toBe(true);
    expect(hasProperty(attestation, "AllowedPairs")).toBe(true);
  });

  test("returns_false_for_missing_property", () => {
    const attestation = createMockAttestation({
      withPolicyProof: true,
      properties: ["MaxLeverage(5)"],
    });
    expect(hasProperty(attestation, "MaxOrderSize")).toBe(false);
  });

  test("returns_false_without_proof", () => {
    const attestation = createMockAttestation({ withPolicyProof: false });
    expect(hasProperty(attestation, "MaxLeverage")).toBe(false);
  });
});

// ============================================================================
// getVersionTuple Tests
// ============================================================================

describe("getVersionTuple", () => {
  test("parses_1_0_correctly", () => {
    const attestation = createMockAttestation({ version: "1.0" });
    expect(getVersionTuple(attestation)).toEqual([1, 0]);
  });

  test("parses_1_1_correctly", () => {
    const attestation = createMockAttestation({ version: "1.1" });
    expect(getVersionTuple(attestation)).toEqual([1, 1]);
  });

  test("parses_2_0_correctly", () => {
    const attestation = createMockAttestation({ version: "2.0" });
    expect(getVersionTuple(attestation)).toEqual([2, 0]);
  });

  test("returns_null_for_invalid_version", () => {
    const attestation = createMockAttestation({});
    attestation.version = "invalid";
    expect(getVersionTuple(attestation)).toBeNull();
  });
});
