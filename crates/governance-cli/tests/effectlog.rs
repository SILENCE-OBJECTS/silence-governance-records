//! Status: PUBLIC CANDIDATE — synthetic fixtures only.
use governance_effectlog::{append_entry, verify_chain, FixedClock};

#[test]
fn deterministic_replay_across_two_runs() {
    let clock1 = FixedClock(1_700_000_000);
    let clock2 = FixedClock(1_700_000_000);

    let a = append_entry(1, "WN-2026-0001", "GENESIS", &clock1);
    let b = append_entry(1, "WN-2026-0001", "GENESIS", &clock2);

    assert_eq!(a.hash, b.hash, "MEASURED: replay match");
}

#[test]
fn chain_of_three_verifies() {
    let clock = FixedClock(1_700_000_000);
    let e1 = append_entry(1, "WN-2026-0001", "GENESIS", &clock);
    let e2 = append_entry(2, "KOR-2026-0001", &e1.hash, &clock);
    let e3 = append_entry(3, "DEC-2026-0001", &e2.hash, &clock);
    assert!(verify_chain(&[e1, e2, e3]));
}
