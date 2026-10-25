# silence-governance-records

Status: PUBLIC CANDIDATE
Scope: open-core documentation and synthetic fixtures only
Not included: scoring, individual profiling, raw telemetry, private-layer logic
Evidence status: design contract — not a medical, predictive, or performance claim

## What this is

Governance registry for SILENCE decisions, submissions, correspondence, and
evidence lineage. A deterministic Rust CLI validates record schemas, enforces
FSM lifecycle transitions, and verifies an append-only hash-chain (EffectLog).

## What this is not

Not a scoring engine. Not a medical device. Not a repository for PII, raw
telemetry, or private-layer logic. Trust boundary is one-directional: the
private layer may depend on open-core; open-core never depends on the private
layer (RULE-DOM-001, enforced in CI via WORLDHALT).

## Structure

- `records/` — inbox, submissions, correspondence, decisions, evidence, archive
- `publications/` — drafts, review, published (gated by `publication-check`)
- `crates/` — Rust workspace: contracts, validator, effectlog, cli
- `fixtures/` — synthetic test records only
- `docs/` — architecture, intake policy, publication policy, record lifecycle

## Quickstart

```
cargo test --workspace --locked
cargo run -p governance-cli -- validate records/
cargo run -p governance-cli -- intake-check records/inbox/
cargo run -p governance-cli -- publication-check publications/
cargo run -p governance-cli -- verify-chain records/
```

CLI `intake-check` and `publication-check` are skeletons (NOTE on stdout).
Do not treat missing output as PASS. File-backed Markdown parse is next.

Cargo.lock is SSoT. rustc 1.98.1.

## License

Apache-2.0 for `crates/` and this documentation; see [LICENSE](LICENSE).
Records under `records/` are governance artifacts, not software, and are
versioned but not licensed as code.
