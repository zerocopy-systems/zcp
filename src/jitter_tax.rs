// ============================================================================
// LATENCY INFERENCE MODULE
// Task 2.4: Infer signing latency from user's setup
// Task 3.1: Jitter Tax Formula Implementation
// ============================================================================

use colored::*;
use std::io::{IsTerminal, Write};

/// Signing provider with associated latency assumptions
#[derive(Debug, Clone, Copy)]
pub enum SigningProvider {
    /// AWS KMS - network HSM (150ms average latency)
    AwsKms,
    /// Fireblocks / MPC providers (350ms average latency)
    Mpc,
    /// Local HSM or Enclave (5ms latency)
    LocalHsm,
    /// ZeroCopy Sentinel (0.042ms = 42µs latency)
    Sentinel,
    /// Custom latency from log file or user input
    Custom(u64),
}

impl SigningProvider {
    /// Get the assumed P99 latency in milliseconds
    pub fn latency_ms(&self) -> u64 {
        match self {
            SigningProvider::AwsKms => 150,
            SigningProvider::Mpc => 350,
            SigningProvider::LocalHsm => 5,
            SigningProvider::Sentinel => 0, // ~42µs, rounds to 0ms for calculation
            SigningProvider::Custom(ms) => *ms,
        }
    }

    /// Get the display name
    pub fn name(&self) -> &'static str {
        match self {
            SigningProvider::AwsKms => "AWS KMS",
            SigningProvider::Mpc => "Fireblocks / MPC",
            SigningProvider::LocalHsm => "Local HSM / Enclave",
            SigningProvider::Sentinel => "ZeroCopy Sentinel",
            SigningProvider::Custom(_) => "Custom (User Provided)",
        }
    }

    /// Get the source citation for the latency assumption
    pub fn source(&self) -> &'static str {
        match self {
            SigningProvider::AwsKms => "AWS Re:Post Community Benchmarks (2024)",
            SigningProvider::Mpc => "Fireblocks Performance Docs (2024)",
            SigningProvider::LocalHsm => "Industry Standard HSM Benchmarks",
            SigningProvider::Sentinel => "ZeroCopy Internal Benchmarks (42µs P99)",
            SigningProvider::Custom(_) => "User-provided measurement",
        }
    }
}

/// Jitter Tax calculation parameters
#[derive(Debug, Clone)]
pub struct JitterTaxParams {
    /// Signing provider (determines latency)
    pub provider: SigningProvider,
    /// Daily trading volume in USD
    pub daily_volume_usd: u64,
    /// Slippage rate per 100ms of latency (default: 1 bps = 0.0001)
    pub slippage_rate: f64,
    /// Trading days per year (default: 365 for crypto)
    pub trading_days: u32,
}

impl Default for JitterTaxParams {
    fn default() -> Self {
        Self {
            provider: SigningProvider::AwsKms,
            daily_volume_usd: 10_000_000, // $10M default
            slippage_rate: 0.0001,        // 1 bps per 100ms
            trading_days: 365,            // crypto is 24/7
        }
    }
}

/// Result of Jitter Tax calculation
#[derive(Debug, Clone)]
pub struct JitterTaxResult {
    pub provider: SigningProvider,
    pub latency_ms: u64,
    pub daily_volume_usd: u64,
    #[allow(dead_code)]
    pub annual_volume_usd: u64,
    pub slippage_rate: f64,
    pub trading_days: u32,
    pub annual_loss_usd: u64,
    pub daily_loss_usd: u64,
    /// Potential savings if switching to Sentinel
    pub potential_savings_usd: u64,
}

impl JitterTaxResult {
    /// Format the annual loss as a string
    #[allow(dead_code)]
    pub fn format_annual_loss(&self) -> String {
        format_currency(self.annual_loss_usd)
    }

    /// Format potential savings as a string
    #[allow(dead_code)]
    pub fn format_savings(&self) -> String {
        format_currency(self.potential_savings_usd)
    }
}

/// Calculate the Jitter Tax
/// Formula: AnnualLoss = (Jitter_ms / 1000) * SlippageRate * DailyVolume * TradingDays
pub fn calculate_jitter_tax(params: &JitterTaxParams) -> JitterTaxResult {
    let latency_ms = params.provider.latency_ms();
    let annual_volume = params.daily_volume_usd as f64 * params.trading_days as f64;

    // Core formula: (latency_seconds) * slippage_rate * annual_volume
    let latency_seconds = latency_ms as f64 / 1000.0;
    let annual_loss = latency_seconds * params.slippage_rate * annual_volume;
    let daily_loss = annual_loss / params.trading_days as f64;

    // Calculate savings if switching to Sentinel (42µs = ~0ms)
    let _sentinel_params = JitterTaxParams {
        provider: SigningProvider::Sentinel,
        ..params.clone()
    };
    let sentinel_latency_seconds = 0.000042; // 42µs
    let sentinel_loss = sentinel_latency_seconds * params.slippage_rate * annual_volume;
    let potential_savings = annual_loss - sentinel_loss;

    JitterTaxResult {
        provider: params.provider,
        latency_ms,
        daily_volume_usd: params.daily_volume_usd,
        annual_volume_usd: annual_volume as u64,
        slippage_rate: params.slippage_rate,
        trading_days: params.trading_days,
        annual_loss_usd: annual_loss as u64,
        daily_loss_usd: daily_loss as u64,
        potential_savings_usd: potential_savings as u64,
    }
}

/// Interactive prompt to select signing provider
pub fn prompt_signing_provider(quiet: bool) -> SigningProvider {
    if quiet || !std::io::stdout().is_terminal() {
        // Default to AWS KMS in non-interactive mode
        return SigningProvider::AwsKms;
    }

    println!();
    println!("{}", "What signing provider do you currently use?".bold());
    println!();
    println!("  {}  {}", "1.".cyan(), "AWS KMS (Cloud HSM)".white());
    println!("      {}", "~150ms latency".dimmed());
    println!();
    println!("  {}  {}", "2.".cyan(), "Fireblocks / MPC Provider".white());
    println!(
        "      {}",
        "~350ms latency (multi-party computation overhead)".dimmed()
    );
    println!();
    println!(
        "  {}  {}",
        "3.".cyan(),
        "Local HSM / Hardware Enclave".white()
    );
    println!("      {}", "~5ms latency".dimmed());
    println!();
    println!("  {}  {}", "4.".cyan(), "Other / Custom Latency".white());
    println!("      {}", "Enter your measured latency in ms".dimmed());
    println!();

    print!("{}", "Enter choice [1-4]: ".bold());
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_err() {
        return SigningProvider::AwsKms;
    }

    match input.trim() {
        "1" => SigningProvider::AwsKms,
        "2" => SigningProvider::Mpc,
        "3" => SigningProvider::LocalHsm,
        "4" => {
            print!("{}", "Enter latency in ms: ".bold());
            std::io::stdout().flush().unwrap();
            let mut latency_input = String::new();
            if std::io::stdin().read_line(&mut latency_input).is_ok() {
                if let Ok(ms) = latency_input.trim().parse::<u64>() {
                    return SigningProvider::Custom(ms);
                }
            }
            SigningProvider::AwsKms // Fallback
        }
        _ => SigningProvider::AwsKms, // Default
    }
}

/// Print detailed calculation breakdown (--explain flag)
pub fn print_explain_breakdown(result: &JitterTaxResult) {
    println!();
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║           JITTER TAX CALCULATION BREAKDOWN                ║"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════╝".cyan()
    );
    println!();

    println!("{}", "[INPUTS]".yellow().bold());
    println!(
        "  Detected Provider:   {} ",
        result.provider.name().white().bold()
    );
    println!(
        "  Assumed Latency:     {} ms (P99)",
        format!("{}", result.latency_ms).red()
    );
    println!(
        "  Daily Volume:        {}",
        format_currency(result.daily_volume_usd).white()
    );
    println!("  Trading Days/Year:   {}", result.trading_days);
    println!(
        "  Slippage Model:      {} per 100ms",
        format!("{:.4}%", result.slippage_rate * 100.0).yellow()
    );
    println!();

    println!("{}", "[FORMULA]".yellow().bold());
    println!("  AnnualLoss = (Latency_ms / 1000) × SlippageRate × DailyVolume × TradingDays");
    println!();

    println!("{}", "[CALCULATION]".yellow().bold());
    let latency_s = result.latency_ms as f64 / 1000.0;
    println!(
        "  = ({} / 1000) × {} × {} × {}",
        result.latency_ms,
        result.slippage_rate,
        format_currency(result.daily_volume_usd),
        result.trading_days
    );
    println!(
        "  = {} × {} × {} × {}",
        latency_s,
        result.slippage_rate,
        format_currency(result.daily_volume_usd),
        result.trading_days
    );
    println!();

    println!("{}", "[RESULT]".green().bold());
    println!(
        "  Annual Jitter Tax:   {}",
        format_currency(result.annual_loss_usd).red().bold()
    );
    println!(
        "  Daily Jitter Tax:    {}",
        format_currency(result.daily_loss_usd).red()
    );
    println!();

    println!("{}", "[POTENTIAL SAVINGS WITH ZEROCOPY]".green().bold());
    println!("  ZeroCopy Latency:    42 µs (0.000042s)");
    println!(
        "  Annual Savings:      {}",
        format_currency(result.potential_savings_usd).green().bold()
    );
    println!();

    println!("{}", "[SOURCES]".dimmed());
    println!("  • Latency: {}", result.provider.source());
    println!("  • Slippage Model: BIS Working Paper 955 (2021)");
    println!();
}

/// Print summary result (compact form)
#[allow(dead_code)]
pub fn print_summary(result: &JitterTaxResult, quiet: bool) {
    if quiet {
        return;
    }

    println!();
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════╗".red()
    );
    println!(
        "{}",
        format!(
            "║   YOUR ANNUAL JITTER TAX: {:>32} ║",
            format_currency(result.annual_loss_usd)
        )
        .red()
        .bold()
    );
    println!(
        "{}",
        "╠═══════════════════════════════════════════════════════════╣".red()
    );
    println!(
        "{}",
        format!("║   Provider: {:>44} ║", result.provider.name()).red()
    );
    println!(
        "{}",
        format!("║   Latency:  {:>41} ms ║", result.latency_ms).red()
    );
    println!(
        "{}",
        format!(
            "║   Savings with ZeroCopy: {:>28} ║",
            format_currency(result.potential_savings_usd)
        )
        .green()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════╝".red()
    );
    println!();
}

/// Format a number as USD currency
fn format_currency(amount: u64) -> String {
    if amount >= 1_000_000 {
        format!("${:.1}M", amount as f64 / 1_000_000.0)
    } else if amount >= 1_000 {
        format!("${:.1}K", amount as f64 / 1_000.0)
    } else {
        format!("${}", amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Provider Latency Tests
    // =========================================================================

    #[test]
    fn test_aws_kms_jitter_tax() {
        let params = JitterTaxParams {
            provider: SigningProvider::AwsKms,
            daily_volume_usd: 10_000_000, // $10M
            slippage_rate: 0.0001,
            trading_days: 365,
        };

        let result = calculate_jitter_tax(&params);

        // Expected: 0.15 * 0.0001 * 10M * 365 = $54,750
        assert!(result.annual_loss_usd > 50_000);
        assert!(result.annual_loss_usd < 60_000);
        assert_eq!(result.latency_ms, 150);
    }

    #[test]
    fn test_mpc_jitter_tax() {
        let params = JitterTaxParams {
            provider: SigningProvider::Mpc,
            daily_volume_usd: 10_000_000,
            slippage_rate: 0.0001,
            trading_days: 365,
        };

        let result = calculate_jitter_tax(&params);

        // MPC (350ms) should have higher loss than AWS KMS (150ms)
        let kms_params = JitterTaxParams {
            provider: SigningProvider::AwsKms,
            ..params.clone()
        };
        let kms_result = calculate_jitter_tax(&kms_params);

        assert!(result.annual_loss_usd > kms_result.annual_loss_usd);
        assert_eq!(result.latency_ms, 350);
    }

    #[test]
    fn test_sentinel_minimal_loss() {
        let params = JitterTaxParams {
            provider: SigningProvider::Sentinel,
            daily_volume_usd: 10_000_000,
            slippage_rate: 0.0001,
            trading_days: 365,
        };

        let result = calculate_jitter_tax(&params);

        // Sentinel at 0ms should have ~$0 loss
        assert!(result.annual_loss_usd < 100);
        assert_eq!(result.latency_ms, 0);
    }

    #[test]
    fn test_local_hsm_jitter_tax() {
        let params = JitterTaxParams {
            provider: SigningProvider::LocalHsm,
            daily_volume_usd: 10_000_000,
            slippage_rate: 0.0001,
            trading_days: 365,
        };

        let result = calculate_jitter_tax(&params);

        // LocalHsm (5ms) should have low loss
        assert!(result.annual_loss_usd < 5_000);
        assert_eq!(result.latency_ms, 5);
    }

    #[test]
    fn test_custom_provider() {
        let params = JitterTaxParams {
            provider: SigningProvider::Custom(200),
            daily_volume_usd: 10_000_000,
            slippage_rate: 0.0001,
            trading_days: 365,
        };

        let result = calculate_jitter_tax(&params);

        // Custom 200ms should be between AWS KMS (150ms) and MPC (350ms)
        assert_eq!(result.latency_ms, 200);
        assert!(result.annual_loss_usd > 50_000);
        assert!(result.annual_loss_usd < 130_000);
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_zero_volume() {
        let params = JitterTaxParams {
            provider: SigningProvider::AwsKms,
            daily_volume_usd: 0,
            slippage_rate: 0.0001,
            trading_days: 365,
        };

        let result = calculate_jitter_tax(&params);
        assert_eq!(result.annual_loss_usd, 0);
        assert_eq!(result.potential_savings_usd, 0);
    }

    #[test]
    fn test_very_large_volume() {
        let params = JitterTaxParams {
            provider: SigningProvider::AwsKms,
            daily_volume_usd: 1_000_000_000, // $1B daily
            slippage_rate: 0.0001,
            trading_days: 365,
        };

        let result = calculate_jitter_tax(&params);

        // Expected: 0.15 * 0.0001 * 1B * 365 = $5,475,000
        assert!(result.annual_loss_usd > 5_000_000);
        assert!(result.annual_loss_usd < 6_000_000);
    }

    #[test]
    fn test_one_trading_day() {
        let params = JitterTaxParams {
            provider: SigningProvider::AwsKms,
            daily_volume_usd: 10_000_000,
            slippage_rate: 0.0001,
            trading_days: 1,
        };

        let result = calculate_jitter_tax(&params);

        // Expected: 0.15 * 0.0001 * 10M * 1 = $150
        assert!(result.annual_loss_usd > 100);
        assert!(result.annual_loss_usd < 200);
        assert_eq!(result.trading_days, 1);
    }

    // =========================================================================
    // Provider Metadata Tests
    // =========================================================================

    #[test]
    fn test_provider_names() {
        assert_eq!(SigningProvider::AwsKms.name(), "AWS KMS");
        assert_eq!(SigningProvider::Mpc.name(), "Fireblocks / MPC");
        assert_eq!(SigningProvider::LocalHsm.name(), "Local HSM / Enclave");
        assert_eq!(SigningProvider::Sentinel.name(), "ZeroCopy Sentinel");
        assert_eq!(
            SigningProvider::Custom(100).name(),
            "Custom (User Provided)"
        );
    }

    #[test]
    fn test_provider_latencies() {
        assert_eq!(SigningProvider::AwsKms.latency_ms(), 150);
        assert_eq!(SigningProvider::Mpc.latency_ms(), 350);
        assert_eq!(SigningProvider::LocalHsm.latency_ms(), 5);
        assert_eq!(SigningProvider::Sentinel.latency_ms(), 0);
        assert_eq!(SigningProvider::Custom(42).latency_ms(), 42);
    }

    #[test]
    fn test_provider_sources() {
        assert!(SigningProvider::AwsKms.source().contains("AWS"));
        assert!(SigningProvider::Mpc.source().contains("Fireblocks"));
        assert!(SigningProvider::Sentinel.source().contains("ZeroCopy"));
    }

    // =========================================================================
    // Result Formatting Tests
    // =========================================================================

    #[test]
    fn test_format_currency_small() {
        assert_eq!(format_currency(500), "$500");
        assert_eq!(format_currency(0), "$0");
    }

    #[test]
    fn test_format_currency_thousands() {
        assert_eq!(format_currency(1_500), "$1.5K");
        assert_eq!(format_currency(54_750), "$54.8K");
    }

    #[test]
    fn test_format_currency_millions() {
        assert_eq!(format_currency(1_500_000), "$1.5M");
        assert_eq!(format_currency(5_475_000), "$5.5M");
    }

    #[test]
    fn test_default_params() {
        let params = JitterTaxParams::default();
        assert_eq!(params.daily_volume_usd, 10_000_000);
        assert_eq!(params.slippage_rate, 0.0001);
        assert_eq!(params.trading_days, 365);
    }

    // =========================================================================
    // Savings Calculation Tests
    // =========================================================================

    #[test]
    fn test_savings_calculation() {
        let params = JitterTaxParams {
            provider: SigningProvider::AwsKms,
            daily_volume_usd: 10_000_000,
            slippage_rate: 0.0001,
            trading_days: 365,
        };

        let result = calculate_jitter_tax(&params);

        // Savings should be approximately equal to annual loss (since Sentinel ~= $0)
        assert!(result.potential_savings_usd > result.annual_loss_usd - 100);
        assert!(result.potential_savings_usd <= result.annual_loss_usd);
    }

    #[test]
    fn test_sentinel_no_savings() {
        let params = JitterTaxParams {
            provider: SigningProvider::Sentinel,
            daily_volume_usd: 10_000_000,
            slippage_rate: 0.0001,
            trading_days: 365,
        };

        let result = calculate_jitter_tax(&params);

        // Already at Sentinel, so savings should be minimal
        assert!(result.potential_savings_usd < 100);
    }
}
