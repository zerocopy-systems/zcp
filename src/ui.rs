//! CLI UI/UX Utilities for ZeroCopy
//!
//! Best practices for B2B CLI design:
//! - Branded headers for major commands
//! - "Next Steps" guidance after every action
//! - Structured error messages with recovery hints
//! - Consistent --json support

use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

/// Print the branded ZeroCopy header
pub fn print_header(title: &str) {
    println!();
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════╗".bright_green()
    );
    println!(
        "{} {:<57} {}",
        "║".bright_green(),
        "▓▓ ZEROCOPY".bold().white(),
        "║".bright_green()
    );
    println!(
        "{} {:<57} {}",
        "║".bright_green(),
        "   Sovereign Infrastructure for Autonomous Capital".dimmed(),
        "║".bright_green()
    );
    println!(
        "{}",
        "╠═══════════════════════════════════════════════════════════╣".bright_green()
    );
    // Truncate title if too long
    let display_title = if title.len() > 55 {
        &title[..55]
    } else {
        title
    };
    let padding = (57 - display_title.len()) / 2;
    println!(
        "{} {:>width$}{}{:<rest$} {}",
        "║".bright_green(),
        "",
        display_title.bold().cyan(),
        "",
        "║".bright_green(),
        width = padding,
        rest = 57 - padding - display_title.len()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════╝".bright_green()
    );
    println!();
}

/// Print a success message with next steps
pub fn print_success(message: &str, next_steps: &[(&str, &str)]) {
    println!();
    println!("{} {}", "✓".green().bold(), message.green().bold());

    if !next_steps.is_empty() {
        println!();
        println!("{}", "NEXT STEPS:".bold());
        for (i, (cmd, desc)) in next_steps.iter().enumerate() {
            println!(
                "  {}. {}  {}",
                (i + 1).to_string().dimmed(),
                cmd.cyan(),
                format!("# {}", desc).dimmed()
            );
        }
    }

    println!();
    println!(
        "{}: {}",
        "DOCS".dimmed(),
        "https://zerocopy.systems/docs".dimmed().underline()
    );
    println!();
}

/// Print an error with cause and recovery hints
pub fn print_error(error: &str, cause: Option<&str>, fixes: &[&str], docs_link: Option<&str>) {
    println!();
    println!("{} {}", "✗ ERROR:".red().bold(), error.red());

    if let Some(c) = cause {
        println!();
        println!("  {}: {}", "CAUSE".yellow(), c);
    }

    if !fixes.is_empty() {
        println!();
        println!("  {}:", "FIX".green());
        for fix in fixes {
            println!("    {}", fix.white());
        }
    }

    if let Some(docs) = docs_link {
        println!();
        println!("  {}: {}", "DOCS".dimmed(), docs.dimmed().underline());
    }

    println!();
}

/// Print a warning message
#[allow(dead_code)]
pub fn print_warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message.yellow());
}

/// Print an info message
#[allow(dead_code)]
pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue(), message);
}

/// Create and return a new spinner
#[allow(dead_code)]
pub fn create_spinner(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));
    spinner
}

/// Print a step in progress
pub fn print_step(step: &str) {
    println!("{} {}", "▶".green(), step);
}

/// Print a key-value pair
pub fn print_kv(key: &str, value: &str) {
    println!("  {}: {}", key.dimmed(), value.white());
}

/// Print a divider line
#[allow(dead_code)]
pub fn print_divider() {
    println!(
        "{}",
        "───────────────────────────────────────────────────────────".dimmed()
    );
}

/// Print a section header
#[allow(dead_code)]
pub fn print_section(title: &str) {
    println!();
    println!("{}", title.bold().underline());
}

/// Format a duration in a human-readable way
#[allow(dead_code)]
pub fn format_duration_us(us: u64) -> String {
    if us < 1000 {
        format!("{} µs", us)
    } else if us < 1_000_000 {
        format!("{:.1} ms", us as f64 / 1000.0)
    } else {
        format!("{:.2} s", us as f64 / 1_000_000.0)
    }
}

/// Format bytes in a human-readable way
#[allow(dead_code)]
pub fn format_bytes(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration_us(42), "42 µs");
        assert_eq!(format_duration_us(1500), "1.5 ms");
        assert_eq!(format_duration_us(1_500_000), "1.50 s");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1536), "1.5 KB");
    }
}
