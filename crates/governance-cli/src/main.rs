//! governance-cli
//!
//! Status: PUBLIC CANDIDATE
//! Scope: open-core documentation and synthetic fixtures only.
//! Not included: scoring, individual profiling, raw telemetry, or private-layer logic.

use clap::{Parser, Subcommand};
use governance_effectlog::{append_entry, verify_chain, EffectLogEntry, SystemClock};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "governance-cli", about = "SILENCE governance record CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate structural completeness of records under a directory.
    Validate { path: PathBuf },
    /// Check inbox records for minimal shape and classification tags.
    IntakeCheck { path: PathBuf },
    /// Verify publications/ meet the publication gate before publishing.
    PublicationCheck { path: PathBuf },
    /// Verify the EffectLog hash-chain integrity for records/.
    VerifyChain { path: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { path } => {
            let count = walk_markdown(&path);
            println!("validate: scanned {count} record file(s) under {path:?}");
            println!("NOTE: structural scan only in this skeleton — wire governance-validator per-file parsing next.");
        }
        Commands::IntakeCheck { path } => {
            let count = walk_markdown(&path);
            println!("intake-check: {count} file(s) in inbox {path:?}");
            println!("NOTE: raw-stream / restricted-content pattern scan not yet wired — see governance-validator::ingest_raw for the refusal contract.");
        }
        Commands::PublicationCheck { path } => {
            let count = walk_markdown(&path);
            println!("publication-check: {count} draft/review file(s) under {path:?}");
            println!("BLOCKED BY DEFAULT: publication requires evidence + source_tag + license basis + OWNER DEC-* record. Not implemented in this skeleton — do not treat missing output as PASS.");
        }
        Commands::VerifyChain { path } => {
            let clock = SystemClock;
            let e1 = append_entry(1, "GENESIS", "GENESIS", &clock);
            let e2 = append_entry(2, "EXAMPLE", &e1.hash, &clock);
            let entries: Vec<EffectLogEntry> = vec![e1, e2];
            let ok = verify_chain(&entries);
            println!("verify-chain: example chain over {path:?} valid = {ok}");
            println!("NOTE: this skeleton verifies an in-memory example chain, not yet the on-disk records/ ledger — wire file-backed EffectLog storage next.");
        }
    }

    Ok(())
}

fn walk_markdown(path: &PathBuf) -> usize {
    if !path.exists() {
        return 0;
    }
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "md").unwrap_or(false))
        .count()
}
