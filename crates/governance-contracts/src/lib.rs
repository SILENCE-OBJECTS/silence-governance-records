//! governance-contracts
//!
//! Status: PUBLIC CANDIDATE
//! Scope: open-core documentation and synthetic fixtures only.
//! Not included: scoring, individual profiling, raw telemetry, or private-layer logic.

use serde::{Deserialize, Serialize};

/// Record lifecycle FSM. Illegal transitions are rejected by the validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RecordStatus {
    Draft,
    Open,
    Waiting,
    Decided,
    Closed,
    Archived,
}

impl RecordStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "DRAFT",
            Self::Open => "OPEN",
            Self::Waiting => "WAITING",
            Self::Decided => "DECIDED",
            Self::Closed => "CLOSED",
            Self::Archived => "ARCHIVED",
        }
    }
}

/// Source classification tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SourceClass {
    #[serde(rename = "H")]
    Human,
    #[serde(rename = "T")]
    Tool,
    #[serde(rename = "E")]
    Evidence,
    #[serde(rename = "M")]
    Machine,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_roundtrip_labels() {
        assert_eq!(RecordStatus::Draft.as_str(), "DRAFT");
        assert_eq!(RecordStatus::Archived.as_str(), "ARCHIVED");
    }
}
