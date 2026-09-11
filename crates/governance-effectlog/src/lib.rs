//! governance-effectlog
//!
//! Status: PUBLIC CANDIDATE
//! Scope: open-core documentation and synthetic fixtures only.
//! Not included: scoring, individual profiling, raw telemetry, or private-layer logic.
//!
//! Deterministic, append-only hash-chain for record lineage. Uses an
//! injected clock (never `Utc::now()` inside chain logic) so replay is
//! bit-for-bit reproducible — this is the "injected clock" contract.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Injected clock trait — production uses system time, tests inject a fixed
/// value so hash-chain output is deterministically replayable.
pub trait Clock {
    fn now_ts(&self) -> i64;
}

pub struct FixedClock(pub i64);
impl Clock for FixedClock {
    fn now_ts(&self) -> i64 {
        self.0
    }
}

pub struct SystemClock;
impl Clock for SystemClock {
    fn now_ts(&self) -> i64 {
        chrono::Utc::now().timestamp()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectLogEntry {
    pub seq: u64,
    pub record_id: String,
    pub ts: i64,
    pub prev_hash: String,
    pub hash: String,
}

/// Computes the next hash-chain entry deterministically from the previous
/// hash, record id, and injected timestamp. Given the same clock, the same
/// output is guaranteed byte-for-byte (MEASURED via replay tests).
pub fn append_entry(
    seq: u64,
    record_id: &str,
    prev_hash: &str,
    clock: &dyn Clock,
) -> EffectLogEntry {
    let ts = clock.now_ts();
    let mut hasher = Sha256::new();
    hasher.update(seq.to_le_bytes());
    hasher.update(record_id.as_bytes());
    hasher.update(ts.to_le_bytes());
    hasher.update(prev_hash.as_bytes());
    let hash = hex::encode(hasher.finalize());

    EffectLogEntry {
        seq,
        record_id: record_id.to_string(),
        ts,
        prev_hash: prev_hash.to_string(),
        hash,
    }
}

/// Verifies a chain is internally consistent: each entry's `prev_hash`
/// matches the prior entry's `hash`.
pub fn verify_chain(entries: &[EffectLogEntry]) -> bool {
    for i in 1..entries.len() {
        if entries[i].prev_hash != entries[i - 1].hash {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replay_with_fixed_clock_is_deterministic() {
        let clock_a = FixedClock(1_700_000_000);
        let clock_b = FixedClock(1_700_000_000);

        let entry_a = append_entry(1, "WN-2026-0001", "GENESIS", &clock_a);
        let entry_b = append_entry(1, "WN-2026-0001", "GENESIS", &clock_b);

        assert_eq!(entry_a.hash, entry_b.hash, "MEASURED: replay match");
    }

    #[test]
    fn chain_verification_detects_tampering() {
        let clock = FixedClock(1_700_000_000);
        let e1 = append_entry(1, "WN-2026-0001", "GENESIS", &clock);
        let e2 = append_entry(2, "WN-2026-0002", &e1.hash, &clock);
        assert!(verify_chain(&[e1.clone(), e2.clone()]));

        let mut tampered = e2.clone();
        tampered.prev_hash = "TAMPERED".into();
        assert!(!verify_chain(&[e1, tampered]));
    }
}
