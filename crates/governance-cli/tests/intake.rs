//! Status: PUBLIC CANDIDATE — synthetic fixtures only.
use governance_validator::{ingest_raw, RawStream, ValidationError};

#[test]
fn raw_stream_ingest_is_refused() {
    let result = ingest_raw(RawStream);
    assert_eq!(result, Err(ValidationError::RawStreamRefused));
}
