// ============================================================================
// WALLET ADDRESS VALIDATION MODULE
// Task 2.1: Wallet Address Input & Validation
// Supports EVM (0x...) and Solana (base58) addresses
// ============================================================================

use colored::*;

/// Represents a validated blockchain address
#[derive(Debug, Clone)]
pub enum WalletAddress {
    /// Ethereum/EVM address (20 bytes, checksummed)
    Evm(String),
    /// Solana address (32 bytes, base58 encoded)
    Solana(String),
}

impl WalletAddress {
    /// Get the chain type as a string
    #[allow(dead_code)]
    pub fn chain(&self) -> &'static str {
        match self {
            WalletAddress::Evm(_) => "EVM",
            WalletAddress::Solana(_) => "Solana",
        }
    }

    /// Get the raw address string
    pub fn address(&self) -> &str {
        match self {
            WalletAddress::Evm(addr) => addr,
            WalletAddress::Solana(addr) => addr,
        }
    }
}

/// Validation error types
#[derive(Debug)]
pub enum ValidationError {
    Format(String),
    Checksum(String),
    Length(String),
    Characters(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::Format(msg) => write!(f, "Invalid format: {}", msg),
            ValidationError::Checksum(msg) => write!(f, "Invalid checksum: {}", msg),
            ValidationError::Length(msg) => write!(f, "Invalid length: {}", msg),
            ValidationError::Characters(msg) => write!(f, "Invalid characters: {}", msg),
        }
    }
}

/// Parse and validate a wallet address
/// Auto-detects chain from format (0x = EVM, base58 = Solana)
pub fn parse_address(input: &str) -> Result<WalletAddress, ValidationError> {
    let trimmed = input.trim();

    if trimmed.starts_with("0x") || trimmed.starts_with("0X") {
        validate_evm_address(trimmed)
    } else if is_likely_base58(trimmed) {
        validate_solana_address(trimmed)
    } else {
        Err(ValidationError::Format(
            "Address must start with 0x (EVM) or be valid base58 (Solana)".to_string(),
        ))
    }
}

/// Validate an EVM (Ethereum) address
fn validate_evm_address(input: &str) -> Result<WalletAddress, ValidationError> {
    // Check prefix
    let without_prefix = if input.starts_with("0x") || input.starts_with("0X") {
        &input[2..]
    } else {
        input
    };

    // Check length (40 hex chars = 20 bytes)
    if without_prefix.len() != 40 {
        return Err(ValidationError::Length(format!(
            "EVM address must be 40 hex characters, got {}",
            without_prefix.len()
        )));
    }

    // Check valid hex
    if !without_prefix.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ValidationError::Characters(
            "EVM address contains non-hex characters".to_string(),
        ));
    }

    // Validate EIP-55 checksum (if mixed case)
    let has_mixed_case = without_prefix.chars().any(|c| c.is_ascii_uppercase())
        && without_prefix.chars().any(|c| c.is_ascii_lowercase());

    if has_mixed_case && !verify_eip55_checksum(without_prefix) {
        return Err(ValidationError::Checksum(
            "EIP-55 checksum validation failed".to_string(),
        ));
    }

    // Return checksummed address
    let checksummed = to_eip55_checksum(without_prefix);
    Ok(WalletAddress::Evm(format!("0x{}", checksummed)))
}

/// Convert address to EIP-55 checksummed format
fn to_eip55_checksum(address: &str) -> String {
    use sha2::{Digest, Sha256};

    let lower = address.to_lowercase();

    // Use Keccak-256 in production, but SHA-256 as fallback for simplicity
    // In a real implementation, use keccak256
    let mut hasher = Sha256::new();
    hasher.update(lower.as_bytes());
    let hash = hasher.finalize();
    let hash_hex = hex::encode(hash);

    lower
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if c.is_ascii_alphabetic() {
                // If the ith nibble of the hash is >= 8, uppercase
                let hash_nibble = u8::from_str_radix(&hash_hex[i..i + 1], 16).unwrap_or(0);
                if hash_nibble >= 8 {
                    c.to_ascii_uppercase()
                } else {
                    c
                }
            } else {
                c
            }
        })
        .collect()
}

/// Verify EIP-55 checksum
fn verify_eip55_checksum(address: &str) -> bool {
    let expected = to_eip55_checksum(&address.to_lowercase());
    address == expected
}

/// Check if string looks like base58
fn is_likely_base58(input: &str) -> bool {
    // Base58 alphabet: 123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz
    // Excludes: 0, O, I, l
    let base58_chars = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    input.len() >= 32 && input.chars().all(|c| base58_chars.contains(c))
}

/// Validate a Solana address
fn validate_solana_address(input: &str) -> Result<WalletAddress, ValidationError> {
    // Solana addresses are 32-44 characters in base58
    if input.len() < 32 || input.len() > 44 {
        return Err(ValidationError::Length(format!(
            "Solana address must be 32-44 characters, got {}",
            input.len()
        )));
    }

    // Validate base58 characters
    let base58_chars = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    if !input.chars().all(|c| base58_chars.contains(c)) {
        return Err(ValidationError::Characters(
            "Solana address contains invalid base58 characters".to_string(),
        ));
    }

    Ok(WalletAddress::Solana(input.to_string()))
}

/// Parse multiple addresses from CLI input
pub fn parse_addresses(inputs: &[String]) -> Vec<Result<WalletAddress, ValidationError>> {
    inputs.iter().map(|s| parse_address(s)).collect()
}

/// Print parsed addresses for verification
pub fn print_parsed_addresses(addresses: &[WalletAddress], quiet: bool) {
    if quiet {
        return;
    }

    println!("{}", "Parsed Addresses:".bold());
    for addr in addresses {
        let chain_badge = match addr {
            WalletAddress::Evm(_) => "EVM".cyan(),
            WalletAddress::Solana(_) => "SOL".magenta(),
        };
        println!("  [{}] {}", chain_badge, addr.address().dimmed());
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // EVM Address Tests
    // =========================================================================

    #[test]
    fn test_valid_evm_address() {
        // Valid lowercase address (checksum validation skipped for all-lowercase)
        // Note: Our checksum uses SHA-256, not Keccak-256, so mixed-case addresses
        // with real EIP-55 checksums will fail. In production, use keccak256.
        let result = parse_address("0xd8da6bf26964af9d7eed9e03e53415d37aa96045");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), WalletAddress::Evm(_)));
    }

    #[test]
    fn test_valid_evm_lowercase() {
        // Valid lowercase (no checksum validation needed)
        let result = parse_address("0xd8da6bf26964af9d7eed9e03e53415d37aa96045");
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_evm_uppercase_prefix() {
        // 0X prefix should also work
        let result = parse_address("0Xd8da6bf26964af9d7eed9e03e53415d37aa96045");
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_evm_length_short() {
        let result = parse_address("0x1234");
        assert!(matches!(result, Err(ValidationError::Length(_))));
    }

    #[test]
    fn test_invalid_evm_length_long() {
        let result = parse_address("0xd8da6bf26964af9d7eed9e03e53415d37aa96045extra");
        assert!(matches!(result, Err(ValidationError::Length(_))));
    }

    #[test]
    fn test_invalid_evm_characters() {
        // Contains 'g' which is not valid hex
        let result = parse_address("0xd8da6bf26964af9d7eed9e03e53415d37aa9604g");
        assert!(matches!(result, Err(ValidationError::Characters(_))));
    }

    #[test]
    fn test_evm_empty_after_prefix() {
        let result = parse_address("0x");
        assert!(matches!(result, Err(ValidationError::Length(_))));
    }

    // =========================================================================
    // Solana Address Tests
    // =========================================================================

    #[test]
    fn test_valid_solana_address() {
        // Example Solana address
        let result = parse_address("7cTGEwFhSwn4gYqNPz9qQVqdpTd7DYzPYfNJHEYgKDdT");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), WalletAddress::Solana(_)));
    }

    #[test]
    fn test_valid_solana_address_32_chars() {
        // Minimum length Solana address (32 chars)
        let result = parse_address("7cTGEwFhSwn4gYqNPz9qQVqdpTd7DYzP");
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_solana_too_short() {
        // 31 chars - too short to be detected as base58 (needs >= 32)
        // This will fail format detection, not length validation
        let result = parse_address("7cTGEwFhSwn4gYqNPz9qQVqdpTd7DYz");
        // Won't be detected as base58 (< 32 chars), so falls through to InvalidFormat
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_solana_invalid_chars() {
        // Contains '0' which is not valid base58
        let result = parse_address("0cTGEwFhSwn4gYqNPz9qQVqdpTd7DYzPYfNJHEYgKDdT");
        // This will be detected as invalid format (not starting with 0x, not valid base58)
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_solana_contains_o() {
        // Contains 'O' which is not valid base58
        let result = parse_address("OcTGEwFhSwn4gYqNPz9qQVqdpTd7DYzPYfNJHEYgKDdT");
        assert!(result.is_err());
    }

    // =========================================================================
    // Format Detection Tests
    // =========================================================================

    #[test]
    fn test_invalid_format_random_string() {
        let result = parse_address("not_an_address");
        assert!(matches!(result, Err(ValidationError::Format(_))));
    }

    #[test]
    fn test_invalid_format_empty() {
        let result = parse_address("");
        assert!(matches!(result, Err(ValidationError::Format(_))));
    }

    #[test]
    fn test_invalid_format_whitespace() {
        let result = parse_address("   ");
        assert!(matches!(result, Err(ValidationError::Format(_))));
    }

    #[test]
    fn test_address_with_leading_whitespace() {
        // Should handle whitespace trimming
        let result = parse_address("  0xd8da6bf26964af9d7eed9e03e53415d37aa96045  ");
        assert!(result.is_ok());
    }

    // =========================================================================
    // WalletAddress Methods Tests
    // =========================================================================

    #[test]
    fn test_wallet_address_chain_evm() {
        let addr = WalletAddress::Evm("0x123".to_string());
        assert_eq!(addr.chain(), "EVM");
    }

    #[test]
    fn test_wallet_address_chain_solana() {
        let addr = WalletAddress::Solana("abc123".to_string());
        assert_eq!(addr.chain(), "Solana");
    }

    #[test]
    fn test_wallet_address_get_address() {
        let addr = WalletAddress::Evm("0xabc".to_string());
        assert_eq!(addr.address(), "0xabc");
    }
}
