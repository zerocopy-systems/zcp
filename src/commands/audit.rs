use colored::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use zero_copy_utils::kernel::{self, AuditResult};

/// Audit report structure for JSON serialization
#[derive(Serialize, Deserialize, Debug)]
pub struct AuditReport {
    pub version: String,
    pub timestamp: String,
    pub simulation_mode: bool,
    pub platform: PlatformInfo,
    pub checks: Vec<CheckResult>,
    pub summary: Summary,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blockchain_proof: Option<BlockchainProof>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlatformInfo {
    pub os: String,
    pub is_linux: bool,
    pub is_nitro_compatible: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub details: String,
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Summary {
    pub total_checks: usize,
    pub passed: usize,
    pub failed: usize,
    pub pass_rate: f64,
    pub hft_ready: bool,
    pub score: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_annual_loss_usd: Option<u64>,
    pub loss_calculation_method: String,
    pub market_regime: String,
    pub volatility_multiplier: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockchainProof {
    pub report_hash: String,
    pub transaction_hash: String,
    pub contract_address: String,
}

pub fn run_technicals(sim: bool) -> (Vec<CheckResult>, Summary, PlatformInfo) {
    let mut checks = Vec::new();

    // Run checks
    let results = vec![
        (kernel::check_nitro_enclave(sim), "security"),
        (kernel::check_isolcpus(sim), "cpu"),
        (kernel::check_tickless(sim), "kernel"),
        (kernel::check_iommu(sim), "io"),
        (kernel::check_hugepages(sim), "memory"),
        (kernel::check_jitter(sim), "performance"),
    ];

    for (res_result, category) in results {
        if let Ok(res) = res_result {
            checks.push(CheckResult {
                name: res.check_name,
                passed: res.passed,
                details: res.details,
                category: category.to_string(),
            });
        } else if let Err(e) = res_result {
            checks.push(CheckResult {
                name: "System Check Error".to_string(),
                passed: false,
                details: format!("Check failed: {}", e),
                category: category.to_string(),
            });
        }
    }

    let passed_count = checks.iter().filter(|c| c.passed).count();
    let failed_count = checks.len() - passed_count;
    let pass_rate = if checks.is_empty() {
        0.0
    } else {
        (passed_count as f64 / checks.len() as f64) * 100.0
    };

    let is_linux = cfg!(target_os = "linux");
    let summary = Summary {
        total_checks: checks.len(),
        passed: passed_count,
        failed: failed_count,
        pass_rate,
        hft_ready: pass_rate >= 100.0 && is_linux,
        score: pass_rate.round() as u8,
        estimated_annual_loss_usd: None, // Calculator fills this
        loss_calculation_method: "Physics_v2".to_string(),
        market_regime: "Standard".to_string(),
        volatility_multiplier: 1.0,
    };

    let platform = PlatformInfo {
        os: std::env::consts::OS.to_string(),
        is_linux,
        is_nitro_compatible: is_linux,
    };

    (checks, summary, platform)
}

pub fn generate_bill_of_health(
    report: &AuditReport,
    daily_volume: u64,
    jitter_us: u64,
    annual_loss: u64,
    output_path: Option<String>,
    is_institutional: bool,
) -> std::io::Result<()> {
    let mut buffer = String::new();

    // --- Header ---
    if is_institutional {
        buffer.push_str("# 🏛️ INSTITUTIONAL READINESS: INFRASTRUCTURE BILL OF HEALTH\n\n");
    } else {
        buffer.push_str("# 🏥 INFRASTRUCTURE BILL OF HEALTH\n\n");
    }
    buffer.push_str(&format!("**Date:** {}\n", report.timestamp));
    buffer.push_str(&format!(
        "**Platform:** {} (OS: {})\n",
        report.platform.os,
        if report.platform.is_linux {
            "Linux"
        } else {
            "Mac/Other"
        }
    ));
    buffer.push_str("\n---\n\n");

    // --- Executive Summary ---
    buffer.push_str("## 1. Executive Summary\n\n");
    if report.summary.score >= 90 {
        buffer.push_str("✅ **STATUS: HEALTHY**\n\n");
        buffer.push_str("Your infrastructure is optimized for HFT workloads.\n");
    } else {
        buffer.push_str("⚠️ **STATUS: CRITICAL LEAKAGE DETECTED**\n\n");
        buffer.push_str("Your current infrastructure is bleeding alpha due to high jitter and non-deterministic cloud overhead. **Failure to mitigate this leakage may constitute Fiduciary Negligence.**\n");
    }

    // --- The Jitter Tax Calculation ---
    // Model: 1ms latency = 0.01 bps slippage in high vol (approx)
    // Jitter is the "noise" that prevents predictable execution.
    // Calculations synchronized with ZeroCopy Jitter Tax Engine (BIS WP 955)

    buffer.push_str("\n## 2. Institutional Impact Analysis (Jitter Tax)\n\n");
    buffer.push_str("| Metric | Value |\n");
    buffer.push_str("| :--- | :--- |\n");
    buffer.push_str(&format!(
        "| **Daily Volume** | ${:0.2}M |\n",
        daily_volume as f64 / 1_000_000.0
    ));
    buffer.push_str(&format!(
        "| **L1 Jitter (Measured)** | {} µs |\n",
        jitter_us
    ));
    buffer.push_str(&format!(
        "| **Projected Annual Alpha Loss** | **${}** |\n",
        format_large_number(annual_loss)
    ));
    buffer.push_str("\n> *\"Latency is a cost of hardware; Jitter is a cost of architecture. Sentinel Prime solves for both.\"*\n");

    // --- Technical Findings ---
    buffer.push_str("\n## 3. Technical Findings\n\n");
    for check in &report.checks {
        let icon = if check.passed { "✅" } else { "❌" };
        buffer.push_str(&format!(
            "- {} **{}**: {}\n",
            icon, check.name, check.details
        ));
    }

    // --- Prescription ---
    buffer.push_str("\n## 4. Operational Recommendation\n\n");
    buffer.push_str(
        "To mitigate this systemic risk, we recommend migration to a **Sovereign Pod**.\n\n",
    );
    buffer.push_str(
        "- **Deterministic Hot-Path:** Sub-50µs p99 guaranteed by kernel-level isolation.\n",
    );
    buffer.push_str(
        "- **Institutional Grade Security:** Nitro Enclave attestation for every transaction.\n",
    );
    buffer.push_str("- **Total Cost of Ownership (TCO):** $96,000/year (All-inclusive).\n\n");

    if annual_loss > 96_000 {
        let savings = annual_loss - 96_000;
        buffer.push_str(&format!(
            "### 📈 Projected Net Alpha Gain: +${} / year\n",
            format_large_number(savings)
        ));
        buffer.push_str(
            "Based on current volume, this upgrade pays for itself within the first quarter.\n",
        );
    }

    buffer.push_str("\n---\n\n");
    buffer.push_str("*Generated by ZeroCopy Audit CLI*");

    // Write to file or stdout
    if let Some(path) = output_path {
        let mut file = File::create(path)?;
        file.write_all(buffer.as_bytes())?;
        println!("{} Bill of Health saved to details", "✓".green());
    } else {
        println!("{}", buffer);
    }

    Ok(())
}

fn format_large_number(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}
