//! governance-validator
//!
//! Status: PUBLIC CANDIDATE
//! Scope: open-core documentation and synthetic fixtures only.
//! Structural completeness and lineage only — does not rank content.

use governance_contracts::RecordStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawStream;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    RawStreamRefused,
    RestrictedContentRefused,
    MissingField(&'static str),
    IllegalTransition {
        from: RecordStatus,
        to: RecordStatus,
    },
}

pub fn ingest_raw(_stream: RawStream) -> Result<(), ValidationError> {
    Err(ValidationError::RawStreamRefused)
}

pub fn validate_transition(from: RecordStatus, to: RecordStatus) -> Result<(), ValidationError> {
    use RecordStatus::*;
    let ok = matches!(
        (from, to),
        (Draft, Open)
            | (Open, Waiting)
            | (Waiting, Open)
            | (Open, Decided)
            | (Waiting, Decided)
            | (Decided, Closed)
            | (Closed, Archived)
            | (Draft, Archived)
            | (Open, Archived)
            | (Waiting, Archived)
            | (Decided, Archived)
    );
    if ok {
        Ok(())
    } else {
        Err(ValidationError::IllegalTransition { from, to })
    }
}

/// S11 vocabulary lock for PUBLIC text. Tests live in tests/s11.rs.
pub const S11_FORBIDDEN: &[&str] = &[
    "diagnos",
    "treat",
    "therapeutic",
    "clinical",
    "biological age",
    "screening",
];

pub fn contains_forbidden_term(text: &str) -> Option<&'static str> {
    let lower = text.to_lowercase();
    S11_FORBIDDEN
        .iter()
        .copied()
        .find(|term| lower.contains(term))
}

#[cfg(test)]
mod tests {
    use super::*;
    use RecordStatus::*;

    #[test]
    fn raw_stream_ingest_is_refused() {
        assert_eq!(
            ingest_raw(RawStream),
            Err(ValidationError::RawStreamRefused)
        );
    }

    #[test]
    fn skipping_review_is_rejected() {
        assert!(validate_transition(Draft, Decided).is_err());
        assert!(validate_transition(Draft, Closed).is_err());
    }
}
