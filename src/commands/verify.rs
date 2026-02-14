use crate::Args;
use anyhow::{Context, Result};
use colored::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use walkdir::WalkDir;
use ethers::prelude::*;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PerfReport {
    pub timestamp: u64,
    pub duration_seconds: u64,
    pub operations_count: u64,
    pub average_latency_ns: u64,
    pub p50_latency_ns: u64,
    pub p90_latency_ns: u64,
    pub p99_latency_ns: u64,
    pub p999_latency_ns: u64,
    pub binary_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BenchmarkOutput {
    pub report: PerfReport,
    pub attestation_doc_b64: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BatchVerifyReport {
    pub total_files: usize,
    pub verified_files: usize,
    pub failed_files: usize,
    pub results: Vec<VerifyResult>,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyResult {
    pub file_path: String,
    pub success: bool,
    pub error: Option<String>,
    pub p99_latency_ns: Option<u64>,
    pub is_mock: bool,
}

pub fn run_verify(file_path: &str, args: &Args) -> Result<i32> {
    let res = verify_single_file(file_path, !args.json)?;
    
    if args.json {
        println!("{}", serde_json::to_string_pretty(&res)?);
    } else if res.success {
        println!("\n{}", "PROOF VERIFIED & VALID".green().bold());
    } else {
        println!("\n{}", "VERIFICATION FAILED".red().bold());
        if let Some(e) = res.error {
            println!("Error: {}", e);
        }
    }

    if res.success {
        Ok(0)
    } else {
        Ok(1)
    }
}

pub fn run_batch_verify(dir_path: &str, args: &Args) -> Result<i32> {
    println!("{}", "RUNNING BATCH PROOF VERIFICATION".cyan().bold());
    println!("Directory: {}", dir_path);

    let mut results = Vec::new();
    let mut total = 0;
    let mut verified = 0;
    let mut failed = 0;

    for entry in WalkDir::new(dir_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
    {
        total += 1;
        let path_str = entry.path().to_string_lossy().to_string();
        
        match verify_single_file(&path_str, false) {
            Ok(res) => {
                if res.success {
                    verified += 1;
                    println!("[{}] {}", "PASS".green(), path_str);
                } else {
                    failed += 1;
                    println!("[{}] {} - {}", "FAIL".red(), path_str, res.error.as_deref().unwrap_or("Unknown error"));
                }
                results.push(res);
            }
            Err(e) => {
                failed += 1;
                println!("[{}] {} - Error: {}", "ERR ".red(), path_str, e);
                results.push(VerifyResult {
                    file_path: path_str,
                    success: false,
                    error: Some(e.to_string()),
                    p99_latency_ns: None,
                    is_mock: false,
                });
            }
        }
    }

    if !args.json {
        println!("\nBatch Summary:");
        println!("  Total:    {}", total);
        println!("  Verified: {}", verified.to_string().green());
        println!("  Failed:   {}", failed.to_string().red());
    }

    let report = BatchVerifyReport {
        total_files: total,
        verified_files: verified,
        failed_files: failed,
        results,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs(),
    };

    // Save report to verify_report.json
    let report_json = serde_json::to_string_pretty(&report)?;
    std::fs::write("verify_report.json", &report_json)?;
    
    if args.json {
        println!("{}", report_json);
    } else {
        println!("\nReport saved to: verify_report.json");
    }

    if failed == 0 && total > 0 {
        Ok(0)
    } else {
        Ok(1)
    }
}

fn verify_single_file(file_path: &str, verbose: bool) -> Result<VerifyResult> {
    if verbose {
        println!("{}", "VERIFYING PROOF OF PERFORMANCE".cyan().bold());
        println!("File: {}", file_path);
    }

    // 1. Read File
    let content = std::fs::read_to_string(file_path)
        .context(format!("Failed to read proof file: {}", file_path))?;

    // 2. Parse JSON
    let proof: BenchmarkOutput = match serde_json::from_str(&content) {
        Ok(p) => p,
        Err(e) => {
            return Ok(VerifyResult {
                file_path: file_path.to_string(),
                success: false,
                error: Some(format!("Parse error: {}", e)),
                p99_latency_ns: None,
                is_mock: false,
            });
        }
    };

    if verbose {
        println!("Report Timestamp: {}", proof.report.timestamp);
        println!("Report Duration:  {}s", proof.report.duration_seconds);
        println!("Report P99:       {} ns", proof.report.p99_latency_ns);
    }

    // 3. Hash Report
    let report_json = serde_json::to_string(&proof.report)?;
    let mut hasher = Sha256::new();
    hasher.update(report_json.as_bytes());
    let calculated_hash = hasher.finalize();

    // 4. Decode Attestation
    use base64::{engine::general_purpose, Engine as _};
    let att_doc = match general_purpose::STANDARD.decode(&proof.attestation_doc_b64) {
        Ok(doc) => doc,
        Err(e) => {
            return Ok(VerifyResult {
                file_path: file_path.to_string(),
                success: false,
                error: Some(format!("Base64 decode error: {}", e)),
                p99_latency_ns: Some(proof.report.p99_latency_ns),
                is_mock: false,
            });
        }
    };

    let mut is_mock = false;
    let mut success = true;
    let mut error_msg = None;

    // 5. Verification Logic
    if proof.attestation_doc_b64.starts_with("TU9DS1") {
        is_mock = true;
        if verbose {
            println!("{}", "  [INFO] Mock Attestation Detected (Dev Mode)".yellow());
        }
    } else {
        if verbose {
            println!("{}", "  [INFO] Real Nitro Attestation Detected".green());
        }

        // COSE Verification
        use coset::{CborSerializable, CoseSign1};
        let sign1 = match CoseSign1::from_slice(&att_doc) {
            Ok(s) => s,
            Err(e) => {
                return Ok(VerifyResult {
                    file_path: file_path.to_string(),
                    success: false,
                    error: Some(format!("COSE parse error: {:?}", e)),
                    p99_latency_ns: Some(proof.report.p99_latency_ns),
                    is_mock: false,
                });
            }
        };

        let payload = match sign1.payload {
            Some(p) => p,
            None => {
                return Ok(VerifyResult {
                    file_path: file_path.to_string(),
                    success: false,
                    error: Some("No payload in COSE Sign1".to_string()),
                    p99_latency_ns: Some(proof.report.p99_latency_ns),
                    is_mock: false,
                });
            }
        };

        use aws_nitro_enclaves_nsm_api::api::AttestationDoc;
        let doc = match AttestationDoc::from_binary(&payload) {
            Ok(d) => d,
            Err(e) => {
                return Ok(VerifyResult {
                    file_path: file_path.to_string(),
                    success: false,
                    error: Some(format!("Attestation CBOR parse error: {:?}", e)),
                    p99_latency_ns: Some(proof.report.p99_latency_ns),
                    is_mock: false,
                });
            }
        };

        if verbose {
            println!("  [INFO] Enclave Location: {} ({})", doc.module_id, doc.timestamp);
        }

        // User Data Binding
        if let Some(user_data) = doc.user_data {
            if user_data != calculated_hash {
                success = false;
                error_msg = Some("User Data Binding Mismatch (Report Hash matches Attestation)".to_string());
                if verbose {
                    println!("{}", "  [FAIL] User Data Binding (Hash Mismatch!)".red().bold());
                }
            } else if verbose {
                println!("{}", "  [PASS] User Data Binding (Report Hash matches Attestation)".green().bold());
            }
        } else {
            success = false;
            error_msg = Some("No User Data in Attestation".to_string());
            if verbose {
                println!("{}", "  [FAIL] No User Data in Attestation".red().bold());
            }
        }
    }

    // Latency Check
    if proof.report.p99_latency_ns >= 100_000 {
        if verbose {
             println!("{}", format!("  [WARN] Latency {}µs > 100µs SLA", proof.report.p99_latency_ns / 1000).yellow());
        }
    } else if verbose {
        println!("{}", "  [PASS] Latency < 100µs (SLA Met)".green().bold());
    }

    if verbose && success {
        println!();
        println!("{}", "PROOF VERIFIED & VALID".green().bold().underline());
    }

    Ok(VerifyResult {
        file_path: file_path.to_string(),
        success,
        error: error_msg,
        p99_latency_ns: Some(proof.report.p99_latency_ns),
        is_mock,
    })
}

pub async fn run_anchor(file_path: &str, agent_id_hex: &str, _args: &Args) -> Result<i32> {
    println!("{}", "ANCHORING PROOF ON-CHAIN".cyan().bold());
    println!("File: {}", file_path);
    println!("Agent ID: {}", agent_id_hex);

    // 1. Read and parse proof to get receipt hash
    let content = std::fs::read_to_string(file_path)?;
    let proof: BenchmarkOutput = serde_json::from_str(&content)?;
    
    // We anchor the keccak256 hash of the receipt_hex or attestation_doc_b64
    // For now, let's use the full attestation_doc_b64 as the unique identifier
    let proof_hash = ethers::utils::keccak256(proof.attestation_doc_b64.as_bytes());
    let image_id: [u8; 32] = hex::decode(&proof.report.binary_hash).ok().and_then(|h| h.try_into().ok()).unwrap_or([0u8; 32]);
    let agent_id: [u8; 32] = hex::decode(agent_id_hex.trim_start_matches("0x"))?.try_into().map_err(|_| anyhow::anyhow!("Invalid Agent ID length"))?;

    // 2. Setup Provider/Signer (similar to main.rs publish_to_chain)
    let rpc_url = std::env::var("ETH_RPC_URL").context("ETH_RPC_URL not set")?;
    let private_key = std::env::var("ETH_PRIVATE_KEY").context("ETH_PRIVATE_KEY not set")?;
    let contract_addr_str = std::env::var("PROOF_ANCHOR_ADDRESS").context("PROOF_ANCHOR_ADDRESS not set")?;

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let wallet: LocalWallet = private_key.parse()?;
    let chain_id = provider.get_chainid().await?.as_u64();
    let client = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id));
    let client = Arc::new(client);

    // 3. Setup Contract
    let address: Address = contract_addr_str.parse()?;
    let contract = crate::ProofAnchor::new(address, client);

    // 4. Send Transaction
    println!("Connecting to network and anchoring...");
    let call = contract.anchor_proof(agent_id, proof_hash, image_id, vec![]);
    let tx = call.send().await?;
    let receipt = tx.await?.context("Transaction dropped")?;

    println!("{}: {:#x}", "TX HASH".green(), receipt.transaction_hash);
    println!("{}", "PROOF ANCHORED SUCCESSFULLY.".bold().green());

    Ok(0)
}

pub async fn run_onchain_verify(file_path: &str, _args: &Args) -> Result<i32> {
    println!("{}", "VERIFYING PROOF ON-CHAIN".cyan().bold());
    println!("File: {}", file_path);

    // 1. Read and parse proof
    let content = std::fs::read_to_string(file_path)?;
    let proof: BenchmarkOutput = serde_json::from_str(&content)?;
    let proof_hash = ethers::utils::keccak256(proof.attestation_doc_b64.as_bytes());

    // 2. Setup Provider
    let rpc_url = std::env::var("ETH_RPC_URL").context("ETH_RPC_URL not set")?;
    let contract_addr_str = std::env::var("PROOF_ANCHOR_ADDRESS").context("PROOF_ANCHOR_ADDRESS not set")?;

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let client = Arc::new(provider);

    // 3. Setup Contract
    let address: Address = contract_addr_str.parse()?;
    let contract = crate::ProofAnchor::new(address, client);

    // 4. Query
    match contract.proof_exists(proof_hash).call().await {
        Ok(exists) => {
            if exists {
                println!("{}", "  [PASS] Proof found in on-chain registry".green().bold());
                println!("{}", "ON-CHAIN VERIFICATION SUCCESSFUL".green().bold().underline());
                Ok(0)
            } else {
                println!("{}", "  [FAIL] Proof NOT found in on-chain registry".red().bold());
                println!("{}", "ON-CHAIN VERIFICATION FAILED".red().bold().underline());
                Ok(1)
            }
        }
        Err(e) => {
            eprintln!("Contract query error: {}", e);
            Ok(2)
        }
    }
}
