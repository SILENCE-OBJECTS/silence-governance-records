# EXTERNAL REPORT INTAKE

Status: PUBLIC CANDIDATE
Scope: open-core documentation and synthetic fixtures only
Not included: scoring, individual profiling, raw telemetry, private-layer logic
Evidence status: design contract — not a medical, predictive, or performance claim

## Purpose

Defines how unclassified material entering `records/inbox/` is triaged before
it may become a formal submission (`records/submissions/`).

## Intake steps

1. Material lands in `records/inbox/` with no assumed status.
2. `governance-cli intake-check records/inbox/` validates minimal shape:
   - unique identifier present (or assignable)
   - creation timestamp present
   - source classification tag present: `[H]`, `[T]`, `[E]`, or `[M]`
3. If raw/PII/private-layer-scoped content is detected by pattern scan, the
   record is rejected with `RAW_STREAM_REFUSED` or `RESTRICTED_CONTENT_REFUSED`
   and never reaches `submissions/`.
4. Accepted material is assigned a `correlation_id` and may be promoted to
   `records/submissions/` using `records/05_templates/WNIOSEK_TEMPLATE.md`.

## Rejection is not silent

Every rejection at intake is logged (not deleted) under
`records/archive/rejected/` with the reason code, so the intake gate itself
remains auditable.

## Source classification tags

- `[H]` — human-authored
- `[T]` — tool-generated
- `[E]` — evidence artifact (attached proof, link, hash)
- `[M]` — machine-inferred (must cite the inferring process)
