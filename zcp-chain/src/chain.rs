use colored::*;
use ethers::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::IsTerminal;
use zeroize::Zeroize;

// Generate type-safe bindings for the Smart Contract
abigen!(
    AuditRegistry,
    r#"[
        function publishAudit(bytes32 _contentHash, string memory _metadataUri) external returns (bytes32)
    ]"#
);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockchainProof {
    pub report_hash: String,
    pub transaction_hash: String,
    pub contract_address: String,
}

pub async fn publish_to_chain(
    content_hash_hex: &str,
    metadata: &str,
    quiet: bool,
    json: bool,
) -> Option<BlockchainProof> {
    // 1. Load Environment
    let rpc_url = std::env::var("ETH_RPC_URL").ok()?;
    let mut private_key = std::env::var("ETH_PRIVATE_KEY").ok()?;
    let contract_addr_str = std::env::var("AUDIT_REGISTRY_ADDRESS").ok()?;

    if !quiet && !json {
        println!(
            "\n{}",
            ">>> INITIATING BLOCKCHAIN VERIFICATION <<<".bold().blue()
        );
        println!("{}: {}", "REPORT HASH".cyan(), content_hash_hex);
        println!("Connecting to network...");
    }

    // 2. Setup Provider and Signer
    let provider = Provider::<Http>::try_from(rpc_url).ok()?;
    let wallet: LocalWallet = private_key.parse().ok()?;

    // SECURITY: Wipe private key from memory immediately after use
    private_key.zeroize();
    let chain_id = provider.get_chainid().await.ok()?.as_u64();
    let client = SignerMiddleware::new(provider, wallet.with_chain_id(chain_id));
    let client = std::sync::Arc::new(client);

    // 3. Setup Contract
    let address: Address = contract_addr_str.parse().ok()?;
    let contract = AuditRegistry::new(address, client.clone());

    // 4. Convert Hash
    let hash_bytes: [u8; 32] = hex::decode(content_hash_hex).ok()?.try_into().ok()?;

    // 5. Send Transaction
    let pb = if !quiet && !json && std::io::stdout().is_terminal() {
        use indicatif::{ProgressBar, ProgressStyle};
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.set_message("Anchoring proof to Ethereum...");
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    match contract
        .publish_audit(hash_bytes, metadata.to_string())
        .send()
        .await
    {
        Ok(pending_tx) => match pending_tx.await {
            Ok(Some(receipt)) => {
                if let Some(pb) = pb {
                    pb.finish_and_clear();
                }
                let tx_hash = format!("{:#x}", receipt.transaction_hash);
                if !quiet && !json {
                    println!("{}: {}", "TX HASH".green(), tx_hash);
                    println!("{}", "PROOF ANCHORED SUCCESSFULLY.".bold().green());
                }
                Some(BlockchainProof {
                    report_hash: content_hash_hex.to_string(),
                    transaction_hash: tx_hash,
                    contract_address: contract_addr_str,
                })
            }
            Ok(None) => {
                if !quiet && !json {
                    eprintln!("{}", "Transaction dropped.".red());
                }
                None
            }
            Err(e) => {
                if !quiet && !json {
                    eprintln!("Transaction error: {}", e);
                }
                None
            }
        },
        Err(e) => {
            if !quiet && !json {
                eprintln!("Contract call error: {}", e);
            }
            None
        }
    }
}
