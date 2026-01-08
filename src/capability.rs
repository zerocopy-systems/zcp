// ============================================================================
// CAPABILITY DECLARATION MODULE
// Task 1.2: Transparency Banner - Builds trust with paranoid users
// ============================================================================

use colored::*;
use std::io::{IsTerminal, Write};

/// Capabilities that the ZCP audit tool has
pub struct Capabilities {
    pub read_system_config: bool,
    pub read_public_chain: bool,
    pub write_report: bool,
    pub network_calls: bool,
    #[allow(dead_code)]
    pub access_secrets: bool,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            read_system_config: true,
            read_public_chain: false, // Only if --fetch-rpc is provided
            write_report: true,
            network_calls: false,  // Only if --submit or --publish is provided
            access_secrets: false, // Never
        }
    }
}

impl Capabilities {
    /// Create capabilities from CLI args
    pub fn from_args(submit: bool, publish: bool, fetch_rpc: bool) -> Self {
        Self {
            read_system_config: true,
            read_public_chain: fetch_rpc,
            write_report: true,
            network_calls: submit || publish,
            access_secrets: false,
        }
    }
}

/// Display the capability declaration banner
/// Returns true if user accepted, false if declined
pub fn show_capability_banner(caps: &Capabilities, accept_flag: bool, quiet: bool) -> bool {
    if quiet {
        return true; // Auto-accept in quiet mode
    }

    if accept_flag {
        return true; // User already accepted via flag
    }

    if !std::io::stdout().is_terminal() {
        return true; // Non-interactive mode, auto-accept
    }

    println!();
    println!(
        "{}",
        "┌─────────────────────────────────────────────────────────┐".cyan()
    );
    println!(
        "{}",
        "│        ZCP AUDIT - CAPABILITY DECLARATION               │"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "├─────────────────────────────────────────────────────────┤".cyan()
    );

    // READ capabilities
    if caps.read_system_config {
        println!(
            "{}",
            format!(
                "│  {} READ: System config (CPU, OS, kernel params)       │",
                "✓".green()
            )
            .cyan()
        );
    }
    if caps.read_public_chain {
        println!(
            "{}",
            format!(
                "│  {} READ: Public chain data (via RPC)                  │",
                "✓".green()
            )
            .cyan()
        );
    } else {
        println!(
            "{}",
            format!(
                "│  {} NETWORK: No RPC calls (use --fetch-rpc to enable) │",
                "✗".red()
            )
            .cyan()
        );
    }

    // WRITE capabilities
    if caps.write_report {
        println!(
            "{}",
            format!(
                "│  {} WRITE: Report file only (if -o specified)          │",
                "✓".green()
            )
            .cyan()
        );
    }

    // NETWORK capabilities
    if caps.network_calls {
        println!(
            "{}",
            format!(
                "│  {} NETWORK: Submit to ZeroCopy API                    │",
                "⚠".yellow()
            )
            .cyan()
        );
    } else {
        println!(
            "{}",
            format!(
                "│  {} NETWORK: No external calls (air-gapped safe)       │",
                "✗".red()
            )
            .cyan()
        );
    }

    // NEVER
    println!(
        "{}",
        format!(
            "│  {} SECRETS: Does NOT access keystore/wallets          │",
            "✗".red()
        )
        .cyan()
    );
    println!(
        "{}",
        format!(
            "│  {} MODIFY: Does NOT modify system files               │",
            "✗".red()
        )
        .cyan()
    );

    println!(
        "{}",
        "├─────────────────────────────────────────────────────────┤".cyan()
    );
    println!(
        "{}",
        "│  Bypass this prompt: --accept                           │".dimmed()
    );
    println!(
        "{}",
        "└─────────────────────────────────────────────────────────┘".cyan()
    );
    println!();

    // Interactive confirmation
    print!("{}", "Proceed? [Y/n] ".bold());
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let response = input.trim().to_lowercase();
            if response.is_empty() || response == "y" || response == "yes" {
                println!();
                true
            } else {
                println!("{}", "Audit cancelled by user.".yellow());
                false
            }
        }
        Err(_) => {
            // If we can't read input, auto-accept (non-interactive)
            true
        }
    }
}

/// Print a compact capability summary (for verbose mode)
#[allow(dead_code)]
pub fn print_capability_summary(caps: &Capabilities) {
    let mut perms = Vec::new();
    if caps.read_system_config {
        perms.push("sys-read");
    }
    if caps.read_public_chain {
        perms.push("rpc-read");
    }
    if caps.network_calls {
        perms.push("network");
    }

    println!(
        "{}: [{}]",
        "Capabilities".dimmed(),
        perms.join(", ").dimmed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_capabilities() {
        let caps = Capabilities::default();
        assert!(caps.read_system_config);
        assert!(!caps.read_public_chain);
        assert!(caps.write_report);
        assert!(!caps.network_calls);
        assert!(!caps.access_secrets);
    }

    #[test]
    fn test_capabilities_from_args() {
        let caps = Capabilities::from_args(true, false, true);
        assert!(caps.network_calls);
        assert!(caps.read_public_chain);
    }
}
