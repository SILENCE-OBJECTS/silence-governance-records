# RECORD LIFECYCLE

Status: PUBLIC CANDIDATE
Scope: open-core documentation and synthetic fixtures only
Not included: scoring, individual profiling, raw telemetry, private-layer logic
Evidence status: design contract — not a medical, predictive, or performance claim

## Statuses (FSM)

```
DRAFT -> OPEN -> WAITING -> DECIDED -> CLOSED
                     |
                     v
                 ARCHIVED (from any state, terminal)
```

## Transition rules

| From | To | Allowed when |
|---|---|---|
| DRAFT | OPEN | Minimal required fields present (id, dates, correlation_id, owner, tag) |
| OPEN | WAITING | Correspondence sent, awaiting counterparty or evidence |
| WAITING | OPEN | Response received, re-entering active review |
| OPEN / WAITING | DECIDED | OWNER records a decision with rationale + evidence |
| DECIDED | CLOSED | No further action pending; record frozen |
| any | ARCHIVED | Superseded, withdrawn, or retention period reached |

Disallowed transitions (e.g. DRAFT -> DECIDED, skipping review) are rejected
by `governance-validator` with a named FSM error, not a warning.

## Required fields (every record)

- Unique identifier (e.g. `WN-2026-0001`, `KOR-2026-0001`, `DEC-2026-0001`)
- Creation date and last-updated date
- `correlation_id` linking submission, correspondence, evidence, decision
- Status (one of the FSM states above)
- Owner and correspondence parties
- Thesis/claim, evidence references, link to decision (if any)
- Source classification: `[H]`, `[T]`, `[E]`, or `[M]`
- Optional: hash/lineage reference for EffectLog compliance
