use std::collections::HashMap;

/// Gap Detector for UDP Market Data Feeds
///
/// Tracks sequence numbers and alerts on gaps.
pub struct GapDetector {
    expected_seq: HashMap<u32, u64>, // StreamID -> NextSeqNum
    gaps_detected: u64,
}

impl Default for GapDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl GapDetector {
    pub fn new() -> Self {
        Self {
            expected_seq: HashMap::new(),
            gaps_detected: 0,
        }
    }

    /// Process a packet sequence number for a given stream
    /// Returns true if a gap was detected.
    pub fn check(&mut self, stream_id: u32, seq_num: u64) -> bool {
        let expected = self.expected_seq.entry(stream_id).or_insert(seq_num);

        match seq_num.cmp(expected) {
            std::cmp::Ordering::Greater => {
                // Gap detected!
                let gap_size = seq_num - *expected;
                println!(
                    "CRITICAL: Gap detected on stream {}. lost {} packets (Exp: {}, Got: {})",
                    stream_id, gap_size, *expected, seq_num
                );
                self.gaps_detected += 1;
                *expected = seq_num + 1;
                return true;
            }
            std::cmp::Ordering::Less => {
                // Out of order or duplicate
                // Ignored for this simple check
            }
            std::cmp::Ordering::Equal => {
                *expected = seq_num + 1;
            }
        }

        false
    }
}
