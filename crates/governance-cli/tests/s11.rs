//! Status: PUBLIC CANDIDATE — synthetic fixtures only.
//! S11 vocabulary lock: forbidden medical/treatment terms in PUBLIC text.

const FORBIDDEN_TERMS: [&str; 6] = [
    "diagnos",
    "treat",
    "therapeutic",
    "clinical",
    "biological age",
    "screening",
];

fn contains_forbidden_term(text: &str) -> Option<&'static str> {
    let lower = text.to_lowercase();
    FORBIDDEN_TERMS
        .iter()
        .find(|term| lower.contains(*term))
        .copied()
}

#[test]
fn clean_text_passes() {
    let text = "Structural ritual continuity vectors captured at the edge.";
    assert_eq!(contains_forbidden_term(text), None);
}

#[test]
fn forbidden_term_is_detected() {
    let text = "This tool provides a diagnostic screening for biological age.";
    assert!(contains_forbidden_term(text).is_some());
}
