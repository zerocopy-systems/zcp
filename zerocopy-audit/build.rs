use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Determine the target directory for the eBPF bytecode
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(out_dir).join("zerocopy_audit_ebpf");

    println!("cargo:rerun-if-changed=../zerocopy-audit-ebpf/src");

    // Build the eBPF program
    let mut cmd = Command::new("cargo");
    for (key, _) in env::vars() {
        if key.starts_with("CARGO") || key.starts_with("RUST") {
            cmd.env_remove(&key);
        }
    }

    let status = cmd
        .current_dir("../zerocopy-audit-ebpf")
        .args([
            "+nightly",
            "build",
            "--target",
            "bpfel-unknown-none",
            "--release",
            "-Z",
            "build-std=core",
        ])
        .status()
        .expect("Failed to build eBPF program");

    if !status.success() {
        panic!("eBPF build failed");
    }

    let target1 = PathBuf::from("../target/bpfel-unknown-none/release/zerocopy-audit-ebpf");
    let target2 = PathBuf::from("../zerocopy-audit-ebpf/target/bpfel-unknown-none/release/zerocopy-audit-ebpf");

    let ebpf_artifact = if target1.exists() {
        target1
    } else if target2.exists() {
        target2
    } else {
        panic!("eBPF artifact not found at expected workspace/crate target paths");
    };

    std::fs::copy(&ebpf_artifact, &out_path).expect("Failed to copy eBPF artifact to OUT_DIR");
}
