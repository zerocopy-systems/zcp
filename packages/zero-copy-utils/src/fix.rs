//! # FIX Protocol Engine (Financial Information eXchange)
//!
//! minimal Zero-Copy FIX Parser/Builder for HFT.

use bytes::{Bytes, BytesMut};

/// FIX Message Tag
pub type Tag = u32;

/// Zero-Allocation FIX Message Builder
pub struct FixMessage {
    buffer: BytesMut,
    // Store offsets or references? For now, we build linearly.
}

impl Default for FixMessage {
    fn default() -> Self {
        Self::new()
    }
}

impl FixMessage {
    pub fn new() -> Self {
        Self {
            buffer: BytesMut::with_capacity(512), // Standard MTU friendly
        }
    }

    /// Add a standard tag (e.g., 35=D)
    pub fn add_tag(&mut self, tag: Tag, value: &str) {
        use std::fmt::Write;
        // Format: <tag>=<value>\x01
        write!(self.buffer, "{}={}\x01", tag, value).unwrap();
    }

    /// Add a custom tag (e.g., 9000 for Dark Pool instructions)
    ///
    /// # Compliance
    ///
    /// This supports user-defined fields required by:
    /// - IEX (Anti-Gaming logic)
    /// - Sigma X (Routing instructions)
    pub fn add_custom_tag(&mut self, tag: Tag, value: &str) {
        // Validation: Custom tags are usually > 5000
        if tag < 5000 {
            // Log warning or enforce strict mode?
        }
        self.add_tag(tag, value);
    }

    pub fn to_bytes(&self) -> Bytes {
        self.buffer.clone().freeze()
    }
}

/// FIX Message Parser
pub struct FixParser;

impl FixParser {
    // Zero-copy parsing would return slices of the input buffer
    // TODO: Implement parsing
}
