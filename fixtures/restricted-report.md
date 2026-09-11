# WN-2026-0003

Status: DRAFT
Created: 2026-09-11
Updated: 2026-09-11
correlation_id: COR-2026-0003
Owner: ev-silence-owner
Parties: external-reviewer-synth

## Thesis

Synthetic fixture simulating restricted content (raw sensor export
reference) that intake-check must refuse before promotion to submissions/.

## Evidence
- [E] EVI-2026-0003 — SIMULATED raw_sensor_export reference (synthetic, not real data)

## Source classification
[M]

## Expected CLI behavior
intake-check should flag this record: RESTRICTED_CONTENT_REFUSED
The current CLI is a skeleton (NOTE on stdout). Do not treat missing output as PASS.
