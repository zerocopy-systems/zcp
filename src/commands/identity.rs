use crate::Args;
use anyhow::{Context, Result};
use aws_nitro_enclaves_nsm_api::api::AttestationDoc;
use colored::*;
use coset::{CborSerializable, CoseSign1};
use serde::Deserialize;
use sha2::{Digest, Sha256};

pub fn run_verify_identity(handle: &str, proof_path: &str, args: &Args) -> Result<i32> {
    if !args.quiet {
        println!("{}", "VERIFYING SOVEREIGN IDENTITY".cyan().bold());
        println!("Handle: {}", handle);
        println!("Proof:  {}", proof_path);
    }

    // 1. Read and Parse Proof
    let content = std::fs::read_to_string(proof_path)
        .context(format!("Failed to read proof file: {}", proof_path))?;

    // Support both raw sidecar response and wrapped formats
    #[derive(Deserialize)]
    struct RawAttResponse {
        attestation_document: Option<String>,
    }

    let att_doc_hex = if let Ok(resp) = serde_json::from_str::<RawAttResponse>(&content) {
        resp.attestation_document
            .ok_or_else(|| anyhow::anyhow!("No attestation_document found in JSON"))?
    } else {
        // Fallback: assume the whole file might be the hex string if not valid JSON
        content.trim().to_string()
    };

    // 2. Decode Attestation (Hex)
    let att_doc = hex::decode(&att_doc_hex).context("Failed to decode HEX attestation document")?;

    // 3. Parse COSE/CBOR
    let sign1 = CoseSign1::from_slice(&att_doc)
        .map_err(|e| anyhow::anyhow!("COSE parse error: {:?}", e))?;

    let payload = sign1
        .payload
        .ok_or_else(|| anyhow::anyhow!("No payload in COSE Sign1"))?;

    let doc = AttestationDoc::from_binary(&payload)
        .map_err(|e| anyhow::anyhow!("Attestation CBOR parse error: {:?}", e))?;

    // 4. Verify Identity Commitment
    let mut hasher = Sha256::new();
    hasher.update(handle.as_bytes());
    let calculated_hash = hasher.finalize().to_vec();

    let mut success = true;

    if let Some(user_data) = doc.user_data {
        if user_data == calculated_hash {
            if !args.quiet {
                println!(
                    "{}",
                    "  [PASS] Identity Commitment Verified (SHA256 match)"
                        .green()
                        .bold()
                );
            }
        } else {
            success = false;
            if !args.quiet {
                println!("{}", "  [FAIL] Identity Commitment Mismatch!".red().bold());
                println!(
                    "  Expected (from Handle): {}",
                    hex::encode(&calculated_hash)
                );
                println!("  Actual   (from Enclave): {}", hex::encode(&user_data));
            }
        }
    } else {
        success = false;
        if !args.quiet {
            println!(
                "{}",
                "  [FAIL] No identity commitment (user_data) in Attestation"
                    .red()
                    .bold()
            );
        }
    }

    // 5. Display PCRs for full truth
    if !args.quiet {
        println!("\nHardware Root of Trust (PCRs):");
        // doc.pcrs is a BTreeMap<index, Vec<u8>> in NSM API
        for (i, pcr) in doc.pcrs.iter() {
            if *i < 8 {
                println!("  PCR{:0>1}: {}", i, hex::encode(pcr));
            }
        }
        println!("\nEnclave Timestamp: {}", doc.timestamp);
    }

    if success {
        if !args.quiet {
            println!(
                "\n{}",
                "VERIFICATION SUCCESSFUL: IDENTITY LINKED TO HARDWARE"
                    .green()
                    .bold()
                    .underline()
            );
        }
        Ok(0)
    } else {
        if !args.quiet {
            println!("\n{}", "VERIFICATION FAILED".red().bold());
        }
        Ok(1)
    }
}
