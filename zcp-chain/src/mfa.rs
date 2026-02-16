use anyhow::Result;
use qr2term::print_qr;
use std::io::Write;
use totp_rs::{Algorithm, Secret, TOTP};

pub fn setup_mfa() -> Result<()> {
    let mfa_file = get_mfa_file();
    if mfa_file.exists() {
        println!("MFA is already configured at {:?}", mfa_file);
        println!("Delete the file to reset.");
        return Ok(());
    }

    let secret = Secret::generate_secret();
    let secret_bytes = secret.to_bytes().unwrap();
    let secret_str = secret.to_encoded().to_string();

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret_bytes,
        Some("ZeroCopy".to_string()),
        "user@zerocopy".to_string(),
    )
    .unwrap();

    let qr = totp.get_url();
    println!("Scan this QR code with your authenticator app:");
    print_qr(&qr).unwrap();
    println!("Secret Key: {}", secret_str);

    print!("\nEnter the 6-digit code to confirm: ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let code = input.trim();

    if totp.check_current(code).unwrap_or(false) {
        std::fs::write(mfa_file, secret_str)?;
        println!("MFA Configured Successfully! ✅");
    } else {
        println!("Invalid code. MFA setup aborted. ❌");
    }

    Ok(())
}

pub fn verify_mfa_interactive() -> Result<()> {
    let mfa_file = get_mfa_file();
    if !mfa_file.exists() {
        return Ok(()); // MFA Not forced if not setup
    }

    print!("🔒 MFA Enabled. Enter 6-digit code: ");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let code = input.trim();

    if verify_mfa_code(code)? {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Invalid MFA Code"))
    }
}

pub fn verify_mfa_code(code: &str) -> Result<bool> {
    let mfa_file = get_mfa_file();
    if !mfa_file.exists() {
        return Ok(false);
    }

    let secret_str = std::fs::read_to_string(mfa_file)?;
    let secret = Secret::Encoded(secret_str.trim().to_string());
    let secret_bytes = secret.to_bytes().unwrap();

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret_bytes,
        Some("ZeroCopy".to_string()),
        "user@zerocopy".to_string(),
    )
    .unwrap();

    Ok(totp.check_current(code).unwrap_or(false))
}

fn get_mfa_file() -> std::path::PathBuf {
    let home = std::env::var("HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("."));
    let mut path = home;
    path.push(".zerocopy");
    std::fs::create_dir_all(&path).ok();
    path.push("mfa.secret");
    path
}
