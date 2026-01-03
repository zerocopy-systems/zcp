// Build script for zerocopy-audit-cli
//
// This script generates a man page during the build process.
// The man page is output to OUT_DIR and can be installed separately.
//
// To generate the man page manually:
//   cargo build --release
//   cp target/release/build/zerocopy-audit-cli-*/out/man/zerocopy-audit.1 /usr/local/share/man/man1/

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Only generate man page during release builds to speed up dev builds
    let profile = env::var("PROFILE").unwrap_or_default();
    if profile != "release" {
        return;
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap_or_else(|_| ".".to_string()));
    let man_dir = out_dir.join("man");

    // Create man directory
    if let Err(e) = fs::create_dir_all(&man_dir) {
        println!("cargo:warning=Could not create man dir: {}", e);
        return;
    }

    // Write a basic man page template
    // In production, use clap_mangen with proper Args struct
    let man_content = r#".TH ZEROCOPY-AUDIT 1 "December 2024" "v1.0.0" "ZeroCopy Systems"
.SH NAME
zerocopy-audit \- HFT infrastructure latency audit tool
.SH SYNOPSIS
.B zerocopy-audit
[\fIOPTIONS\fR]
.SH DESCRIPTION
Performs comprehensive latency audits for high-frequency trading infrastructure.
Checks kernel configuration, memory settings, and hardware isolation.
.SH OPTIONS
.TP
.BR \-v ", " \-\-verbose
Enable verbose output with detailed explanations.
.TP
.BR \-\-sim
Run in simulation mode (mocks successful checks for testing).
.TP
.BR \-\-publish
Hash report and publish proof to Ethereum blockchain.
.TP
.BR \-\-json
Output results in JSON format (machine-readable).
.TP
.BR \-o ", " \-\-output " " \fIFILE\fR
Write report to specified file path.
.TP
.BR \-q ", " \-\-quiet
Minimal output - only show pass/fail status.
.SH EXIT STATUS
.TP
.B 0
All checks passed.
.TP
.B 1
At least one check failed.
.TP
.B 2
Error occurred (couldn't run checks).
.SH EXAMPLES
.TP
Run audit in simulation mode:
.B zerocopy-audit --sim
.TP
Run audit and save JSON report:
.B zerocopy-audit --json --output report.json
.TP
Run audit and publish to blockchain:
.B zerocopy-audit --publish
.SH AUTHOR
ZeroCopy Systems <engineering@zerocopy.systems>
.SH SEE ALSO
.BR isolcpus (7),
.BR hugepages (7)
"#;

    let man_path = man_dir.join("zerocopy-audit.1");
    if let Err(e) = fs::write(&man_path, man_content) {
        println!("cargo:warning=Could not write man page: {}", e);
        return;
    }

    println!("cargo:warning=Man page generated at {:?}", man_path);
}
