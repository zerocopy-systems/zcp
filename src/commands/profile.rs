use anyhow::Result;
use colored::*;
use std::process::Command;

pub fn run_perf_stat(cmd_args: &[String]) -> Result<i32> {
    if cmd_args.is_empty() {
        eprintln!("{}", "Error: No command provided to profile.".red());
        return Ok(1);
    }

    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║              DYNAMIC PROFILING (PERF STAT)                ║"
            .cyan()
            .bold()
    );
    println!(
        "{}",
        "╠═══════════════════════════════════════════════════════════╣".cyan()
    );
    println!(
        "{}",
        "║ Tracks: cache-misses, L1-dcache-load-misses, dTLB-load-misses ║".cyan()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════════╝".cyan()
    );
    println!();

    #[cfg(target_os = "linux")]
    {
        let events = "cache-misses,L1-dcache-load-misses,dTLB-load-misses";
        let mut child = Command::new("perf")
            .arg("stat")
            .arg("-e")
            .arg(events)
            .args(cmd_args)
            .spawn()
            .map_err(|e| {
                anyhow::anyhow!(
                    "Failed to run perf stat (is linux 'perf' installed?): {}",
                    e
                )
            })?;

        let status = child.wait()?;
        if status.success() {
            println!("\n{}", "✓ Profiling completed successfully".green().bold());
            Ok(0)
        } else {
            eprintln!(
                "\n{}",
                "✗ Profiling failed or command returned non-zero"
                    .red()
                    .bold()
            );
            Ok(status.code().unwrap_or(1))
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        eprintln!(
            "{}",
            "Hardware profiling via 'perf' is only supported on Linux.".yellow()
        );
        // Fallback to just running the command
        let mut child = Command::new(&cmd_args[0])
            .args(&cmd_args[1..])
            .spawn()
            .map_err(|e| anyhow::anyhow!("Failed to run command: {}", e))?;
        let status = child.wait()?;
        Ok(status.code().unwrap_or(1))
    }
}
