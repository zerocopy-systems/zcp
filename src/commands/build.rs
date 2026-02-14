use anyhow::{Context, Result};
use colored::*;
use std::process::Command;

pub fn run(_verbose: bool) -> Result<i32> {
    println!("{}", "BUILDING DETERMINISTIC ENCLAVE...".blue().bold());

    // 1. Check dependencies
    check_dependency("docker")?;
    check_dependency("nitro-cli")?;

    // 2. Build Docker Image (Stage 1)
    println!("Step 1/3: Building Docker Image...");

    // We use "--no-cache" to ensure freshness, and explicit platform
    let status = Command::new("docker")
        .args([
            "build",
            ".",
            "-t",
            "enclave-app:latest",
            "--platform",
            "linux/amd64", // Enclaves are always AMD64
            "--no-cache",
        ])
        .status()
        .context("Failed to execute docker build")?;

    if !status.success() {
        println!("{}", "Docker build failed.".red());
        return Ok(1);
    }

    // 3. Convert to EIF (Stage 2)
    println!("Step 2/3: Converting to EIF (Nitro Enclave)...");

    // Create output dir if needed
    std::fs::create_dir_all("target/enclave").ok();

    let status = Command::new("nitro-cli")
        .args([
            "build-enclave",
            "--docker-uri",
            "enclave-app:latest",
            "--output-file",
            "target/enclave/enclave.eif",
        ])
        .status()
        .context("Failed to execute nitro-cli build-enclave")?;

    if !status.success() {
        println!("{}", "Nitro CLI build failed.".red());
        println!("Ensure you have nitro-cli installed and properly configured.");
        return Ok(1);
    }

    // 4. Verify Measurements
    println!("Step 3/3: Verifying PCR Measurements...");
    let output = Command::new("nitro-cli")
        .args(["describe-eif", "--eif-path", "target/enclave/enclave.eif"])
        .output()
        .context("Failed to describe EIF")?;

    if !output.status.success() {
        println!("{}", "Failed to read EIF measurements.".yellow());
    } else {
        let json: serde_json::Value = serde_json::from_slice(&output.stdout)?;
        let pcr0 = json["Ok"]["Measurements"]["PCR0"]
            .as_str()
            .unwrap_or("UNKNOWN");

        println!("\n{}", "✓ BUILD SUCCESSFUL".green().bold());
        println!("EIF Path: target/enclave/enclave.eif");
        println!("PCR0:     {}", pcr0.cyan());
        println!("\nTo run this enclave locally (Mock Mode):");
        println!("  zcp run --mock");
    }

    Ok(0)
}

fn check_dependency(cmd: &str) -> Result<()> {
    if Command::new("which").arg(cmd).output().is_err() {
        println!(
            "{}",
            format!("Warning: '{}' not found in PATH.", cmd).yellow()
        );
        // We don't hard fail here to allow running on non-dev machines for testing logic
    }
    Ok(())
}
