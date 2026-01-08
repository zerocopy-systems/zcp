// ============================================================================
// RICH OUTPUT MODULE
// Task 4.1: Rich Terminal Output
// Task 4.2: Dramatic Reveal
// Task 4.3: Comparison Table
// ============================================================================

use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::IsTerminal;
use std::time::Duration;

/// ASCII art logo for ZCP Audit
pub const ASCII_LOGO: &str = r#"
╔═══════════════════════════════════════════════════════════════╗
║  _______ ______ _____                                         ║
║ |___  / / ____||  __ \                                        ║
║    / / | |     | |__) |                                       ║
║   / /  | |     |  ___/                                        ║
║  / /__ | |____ | |                                            ║
║ /_____| \_____||_|     AUDIT                                  ║
║                                                               ║
║  Revenue Leakage Detector                                     ║
╚═══════════════════════════════════════════════════════════════╝
"#;

/// Print the ASCII logo
pub fn print_logo(quiet: bool) {
    if quiet || !std::io::stdout().is_terminal() {
        return;
    }
    println!("{}", ASCII_LOGO.cyan());
}

/// Create a spinner for long-running operations
#[allow(dead_code)]
pub fn create_spinner(message: &str) -> Option<ProgressBar> {
    if !std::io::stdout().is_terminal() {
        return None;
    }

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    Some(pb)
}

/// The dramatic reveal - builds suspense before showing the loss
pub fn dramatic_reveal(annual_loss: u64, provider_name: &str, latency_ms: u64, quiet: bool) {
    if quiet || !std::io::stdout().is_terminal() {
        // Just print the result in quiet mode
        println!("JITTER_TAX: ${}", annual_loss);
        return;
    }

    println!();

    // Phase 1: "Calculating..." spinner
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("▱▰▱▰▱▰▱▰")
            .template("{spinner:.yellow} {msg}")
            .unwrap(),
    );
    pb.set_message("Analyzing infrastructure configuration...");
    pb.enable_steady_tick(Duration::from_millis(100));
    std::thread::sleep(Duration::from_millis(800));

    pb.set_message("Calculating latency impact...");
    std::thread::sleep(Duration::from_millis(600));

    pb.set_message("Computing annual revenue leakage...");
    std::thread::sleep(Duration::from_millis(600));

    pb.finish_and_clear();

    // Phase 2: The Reveal
    println!();

    // Animate the box appearing
    let loss_str = format_currency(annual_loss);

    if annual_loss > 100_000 {
        // Critical - RED
        println!(
            "{}",
            "╔════════════════════════════════════════════════════════════╗"
                .red()
                .bold()
        );
        println!(
            "{}",
            "║             ⚠  CRITICAL: JITTER TAX DETECTED               ║"
                .red()
                .bold()
        );
        println!(
            "{}",
            "╠════════════════════════════════════════════════════════════╣"
                .red()
                .bold()
        );
        println!(
            "{}",
            format!("║  Provider:              {:>35} ║", provider_name).red()
        );
        println!(
            "{}",
            format!("║  Signing Latency:       {:>32} ms ║", latency_ms).red()
        );
        println!(
            "{}",
            "║                                                            ║".red()
        );
        println!(
            "{}",
            format!("║  ESTIMATED ANNUAL LOSS: {:>35} ║", loss_str)
                .red()
                .bold()
        );
        println!(
            "{}",
            "╚════════════════════════════════════════════════════════════╝"
                .red()
                .bold()
        );
    } else if annual_loss > 10_000 {
        // Warning - YELLOW
        println!(
            "{}",
            "╔════════════════════════════════════════════════════════════╗"
                .yellow()
                .bold()
        );
        println!(
            "{}",
            "║             ⚠  WARNING: JITTER TAX DETECTED                ║"
                .yellow()
                .bold()
        );
        println!(
            "{}",
            "╠════════════════════════════════════════════════════════════╣".yellow()
        );
        println!(
            "{}",
            format!("║  Provider:              {:>35} ║", provider_name).yellow()
        );
        println!(
            "{}",
            format!("║  Signing Latency:       {:>32} ms ║", latency_ms).yellow()
        );
        println!(
            "{}",
            "║                                                            ║".yellow()
        );
        println!(
            "{}",
            format!("║  ESTIMATED ANNUAL LOSS: {:>35} ║", loss_str)
                .yellow()
                .bold()
        );
        println!(
            "{}",
            "╚════════════════════════════════════════════════════════════╝"
                .yellow()
                .bold()
        );
    } else {
        // Good - GREEN
        println!(
            "{}",
            "╔════════════════════════════════════════════════════════════╗"
                .green()
                .bold()
        );
        println!(
            "{}",
            "║             ✓  LOW JITTER TAX                              ║"
                .green()
                .bold()
        );
        println!(
            "{}",
            "╠════════════════════════════════════════════════════════════╣".green()
        );
        println!(
            "{}",
            format!("║  Provider:              {:>35} ║", provider_name).green()
        );
        println!(
            "{}",
            format!("║  Signing Latency:       {:>32} ms ║", latency_ms).green()
        );
        println!(
            "{}",
            "║                                                            ║".green()
        );
        println!(
            "{}",
            format!("║  ESTIMATED ANNUAL LOSS: {:>35} ║", loss_str)
                .green()
                .bold()
        );
        println!(
            "{}",
            "╚════════════════════════════════════════════════════════════╝"
                .green()
                .bold()
        );
    }
}

/// Print comparison table (You vs. ZeroCopy)
pub fn print_comparison_table(
    current_latency_ms: u64,
    current_loss: u64,
    provider_name: &str,
    quiet: bool,
) {
    if quiet || !std::io::stdout().is_terminal() {
        return;
    }

    println!();
    println!(
        "{}",
        "┌────────────────────────┬──────────────────┬──────────────────┐".white()
    );
    println!(
        "{}",
        format!(
            "│ {:^22} │ {:^16} │ {:^16} │",
            "Metric", "You (Current)", "ZeroCopy"
        )
        .white()
        .bold()
    );
    println!(
        "{}",
        "├────────────────────────┼──────────────────┼──────────────────┤".white()
    );

    // Provider row
    println!(
        "{}",
        format!(
            "│ {:^22} │ {:^16} │ {:^16} │",
            "Signing Provider", provider_name, "Sentinel"
        )
        .white()
    );

    // Latency row
    let current_latency_str = format!("{} ms", current_latency_ms);
    println!(
        "{}{}{}",
        format!("│ {:^22} │ ", "Time-to-Sign (P99)").white(),
        format!("{:^16}", current_latency_str).red(),
        format!(" │ {:^16} │", "42 µs").green()
    );

    // Loss row
    let current_loss_str = format_currency(current_loss);
    println!(
        "{}{}{}",
        format!("│ {:^22} │ ", "Annual Jitter Tax").white(),
        format!("{:^16}", current_loss_str).red(),
        format!(" │ {:^16} │", "$0").green()
    );

    // Savings row
    let savings_str = format_currency(current_loss);
    println!(
        "{}{}{}",
        format!("│ {:^22} │ ", "Potential Savings").white(),
        format!("{:^16}", "-").dimmed(),
        format!(" │ {:^16} │", savings_str).green().bold()
    );

    println!(
        "{}",
        "└────────────────────────┴──────────────────┴──────────────────┘".white()
    );
}

/// Print the CTA (Call to Action)
pub fn print_cta(quiet: bool) {
    if quiet || !std::io::stdout().is_terminal() {
        return;
    }

    println!();
    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan()
    );
    println!("{}", "  Ready to eliminate your Jitter Tax?".cyan().bold());
    println!();
    println!(
        "  → Book a 15-min demo:  {}",
        "https://zerocopy.systems/demo".blue().underline()
    );
    println!(
        "  → Read the docs:       {}",
        "https://docs.zerocopy.systems".blue().underline()
    );
    println!(
        "{}",
        "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".cyan()
    );
    println!();
}

/// Print section header
#[allow(dead_code)]
pub fn print_section(title: &str, quiet: bool) {
    if quiet || !std::io::stdout().is_terminal() {
        return;
    }
    println!();
    println!("{}", format!("▸ {}", title).cyan().bold());
    println!("{}", "─".repeat(60).dimmed());
}

/// Print a check result with nice formatting
#[allow(dead_code)]
pub fn print_check(name: &str, passed: bool, details: &str, quiet: bool) {
    if quiet {
        return;
    }

    let status = if passed {
        "PASS".green().bold()
    } else {
        "FAIL".red().bold()
    };

    println!("[{}] {}", status, name.white());
    println!("      {}", details.dimmed());
}

/// Format currency with K/M suffixes
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

    #[test]
    fn test_format_currency() {
        assert_eq!(format_currency(500), "$500");
        assert_eq!(format_currency(1_500), "$1.5K");
        assert_eq!(format_currency(150_000), "$150.0K");
        assert_eq!(format_currency(1_500_000), "$1.5M");
    }
}
