# ARCHITECTURE

Status: PUBLIC CANDIDATE
Scope: open-core documentation and synthetic fixtures only
Not included: scoring, individual profiling, raw telemetry, private-layer logic
Evidence status: design contract — not a medical, predictive, or performance claim

## Crate boundaries

| Crate | Responsibility | Depends on |
|---|---|---|
| governance-contracts | Record types, statuses, schemas (DTOs only) | none |
| governance-validator | Field/name validation, FSM transition rules | governance-contracts |
| governance-effectlog | Deterministic hash-chain, lineage, append-only log | none |
| governance-cli | intake, validate, publication-check, verify-chain commands | all above |

## Data flow

```
records/inbox/*  --intake-check-->  classification [H|T|E|M]
      |
      v
records/submissions/*  --validate-->  FSM: DRAFT -> OPEN -> WAITING -> DECIDED
      |
      v
records/decisions/*  (OWNER sign-off, correlation_id links back to submission)
      |
      v
governance-effectlog  (hash-chain append, lineage proof)
      |
      v
publications/drafts -> review -> published  (publication-check gate, requires DEC-*)
```

## Trust boundary (RULE-DOM-001)

- This repo is open-core: MIT, no PII, no raw telemetry.
- The private layer MAY import contracts from this repo.
- This repo MUST NEVER import from the private layer.
- Enforced in CI by the `WORLDHALT` rule: any inbound reference to private-layer
  namespaces fails the build.

## Non-goals

No scoring, no fusion, no ranking, no individual profiling, no medical
classification. The validator checks structural completeness and
lineage only — it does not evaluate or rank the content of any record.
