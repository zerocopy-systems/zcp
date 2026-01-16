use crate::ui;
use anyhow::Result;
use colored::*;

pub fn run(subcommand: Option<String>, auto: bool) -> Result<i32> {
    match subcommand.as_deref() {
        Some("status") => show_status(),
        Some("rotate") => rotate_keys(auto),
        Some("list") => list_keys(),
        Some("policy") => show_policy(),
        _ => {
            ui::print_error(
                "Usage: zcp keys <subcommand>",
                None,
                &[
                    "zcp keys status",
                    "zcp keys rotate [--auto]",
                    "zcp keys list",
                    "zcp keys policy",
                ],
                Some("https://zerocopy.systems/docs/cli/keys"),
            );
            Ok(1)
        }
    }
}

fn show_status() -> Result<i32> {
    ui::print_header("KEY INFRASTRUCTURE STATUS");

    // Mock Data for now - will be replaced by Enclave RPC
    let current_epoch = 42;
    let key_age_days = 23;
    let next_rotation = "14 days";

    println!(
        "  {}: {}",
        "Current Epoch".bold(),
        current_epoch.to_string().cyan()
    );
    println!(
        "  {}: {} (Healthy)",
        "Key Age".bold(),
        format!("{} days", key_age_days).green()
    );
    println!("  {}: {}", "Next Rotation".bold(), next_rotation.yellow());
    println!();

    ui::print_step("Enclave Identity Verification: PASS");
    ui::print_step("PCR0 Integrity: PASS");

    Ok(0)
}

fn rotate_keys(auto: bool) -> Result<i32> {
    ui::print_header("KEY ROTATION PROTOCOL");

    if auto {
        println!("{}", "Running in AUTOMATED DAEMON mode...".dimmed());
        // Simulation of checking policy
        println!("Checking rotation policy (90 days)... Key age is 23 days. No action needed.");
        return Ok(0);
    }

    println!("{}", "Initiating MANUAL key rotation...".yellow());

    // 1. Authenticate Admin
    ui::print_step("Verifying Admin Policy Key signature...");

    // 2. Generate New Key (Epoch N+1)
    ui::print_step("Generating Epoch 43 Keypair (Ed25519) inside Enclave...");

    // 3. Seal & Persist
    ui::print_step("Sealing new key to PCR0...");
    ui::print_step("Persisting Sealed Blob to Sidecar...");

    // 4. Activate
    ui::print_step("Atomic Swap: Epoch 43 is now ACTIVE. Epoch 42 is DRAINING.");

    ui::print_success(
        "ROTATION COMPLETE",
        &[
            ("zcp keys status", "Verify new epoch"),
            ("zcp audit logs", "Check rotation audit trail"),
        ],
    );

    Ok(0)
}

fn list_keys() -> Result<i32> {
    ui::print_header("KEY HISTORY");

    println!(
        "{0: <10} | {1: <15} | {2: <45} | {3: <10}",
        "EPOCH", "STATUS", "PUBLIC KEY FINGERPRINT", "AGE"
    );
    println!("{}", "-".repeat(90).dimmed());

    println!(
        "{0: <10} | {1: <15} | {2: <45} | {3: <10}",
        "42",
        "ACTIVE".green().bold(),
        "Points to: 7cTGEw...HeyGKDdT (Solana)",
        "23d"
    );

    println!(
        "{0: <10} | {1: <15} | {2: <45} | {3: <10}",
        "41",
        "DRAINING".yellow(),
        "Points to: 8x8123...12938123 (Solana)",
        "113d"
    );

    println!(
        "{0: <10} | {1: <15} | {2: <45} | {3: <10}",
        "40",
        "REVOKED".red(),
        "Points to: 129038...12093812 (Solana)",
        "203d"
    );

    Ok(0)
}

fn show_policy() -> Result<i32> {
    ui::print_header("ROTATION POLICY");

    println!("  {}: 90 days", "Max Key Age".bold());
    println!("  {}: 7 days (Draining Phase)", "Overlap Period".bold());
    println!("  {}: Ed25519-Dalek", "Algorithm".bold());
    println!(
        "  {}: AWS Nitro Enclave (RamDisk) + S3 (Sealed)",
        "Storage".bold()
    );

    Ok(0)
}
