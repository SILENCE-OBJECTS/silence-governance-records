# PUBLICATION POLICY

Status: PUBLIC CANDIDATE
Scope: open-core documentation and synthetic fixtures only
Not included: scoring, individual profiling, raw telemetry, private-layer logic
Evidence status: design contract — not a medical, predictive, or performance claim

## Gate

A record may move from `publications/drafts/` to `publications/review/` to
`publications/published/` only if `governance-cli publication-check` confirms
ALL of the following:

1. **Evidence attached** — at least one `[E]` classified item linked via
   `correlation_id`.
2. **Source classification present** on every referenced record.
3. **Legal/licensing basis stated** — explicit license or DUA reference.
4. **OWNER decision exists** — a `DEC-*` record in `records/decisions/` with
   status `DECIDED`, referencing the same `correlation_id`.
5. **S11 vocabulary lock passes** — no medical-device or treatment language
   in the publication text.

Failing any check blocks the move; the CLI returns a named error, never a
silent skip.

The current CLI prints `BLOCKED BY DEFAULT` and is a skeleton. Do not treat
missing output as PASS.

## What may never be published from this repo

- Raw telemetry, PII, or individual-level profiles.
- Private-layer business logic, scoring formulas, or model internals.
- Any claim phrased as medical, treatment, or predictive-performance.

## Register

Every published item is logged in `publications/PUBLICATION_REGISTER.md`
with: identifier, `correlation_id`, publish date, DEC-* reference, and a
content hash from `governance-effectlog`.
