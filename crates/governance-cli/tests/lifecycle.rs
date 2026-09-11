//! Status: PUBLIC CANDIDATE — synthetic fixtures only.
use governance_contracts::RecordStatus::*;
use governance_validator::validate_transition;

#[test]
fn allowed_transitions_pass() {
    assert!(validate_transition(Draft, Open).is_ok());
    assert!(validate_transition(Open, Waiting).is_ok());
    assert!(validate_transition(Waiting, Open).is_ok());
    assert!(validate_transition(Open, Decided).is_ok());
    assert!(validate_transition(Decided, Closed).is_ok());
    assert!(validate_transition(Closed, Archived).is_ok());
}

#[test]
fn skipping_review_is_rejected() {
    assert!(validate_transition(Draft, Decided).is_err());
    assert!(validate_transition(Draft, Closed).is_err());
}
