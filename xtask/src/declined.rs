//! `samples-declined.toml` — the declined-transcript ledger (bs41).
//!
//! The third member of the family. `samples-pending.toml` answers "not
//! yet, anywhere"; `samples-os.toml` answers "not here, and here is
//! exactly what here says instead". This file answers the third thing
//! the corpus console lane can say about a block: **not replayed at
//! all, and here is what a replay of it would need.**
//!
//! Before this file the runner already named every declined corpus
//! block in its log, with a reason it computed itself (wolf-book#24's
//! second ask, standing in for its first). That is better than silence
//! and it is not a ledger, for two reasons:
//!
//!   * a reason the runner computes is a description of the code, not a
//!     claim by an author. "`lupin` names no solution in this exercise
//!     directory" is what the rule noticed; it does not say that the
//!     block is a REPL session, that the REPL replay lane is
//!     `wolf-interp is08`, or that someone looked; and
//!   * nothing was enforced in either direction. A block could join the
//!     declined set — by an edit, a pin bump, a rule change — and the
//!     only trace would be one more line in a log that already had
//!     forty-eight of them.
//!
//! So each declined block DECLARES itself, and the gate runs both ways,
//! the way the other two ledgers do:
//!
//!   * a corpus block that is declined and carries no row is a hard
//!     error — the book does not grow an unexplained hole; and
//!   * a row whose block is REPLAYED now is stale, and that is a hard
//!     error too, naming the row to delete. A hole that closes is news,
//!     and news is not absorbed quietly.
//!
//! One decline is exempt from both directions, and the exemption is the
//! reason `Decline::host_only` exists: a block the unix lane replays
//! and windows declines (a built binary, a linked `wolf build`) is a
//! third of the matrix standing down, not a hole in the book. Those are
//! reported and never balanced against this file, which is what lets
//! the same twenty rows hold on all three hosts.
//!
//! Rows carry `needs` (what a replay would take, in the author's
//! words), `filed` (the issue that holds the remainder) and `retires`
//! (what is expected to end the row). They are removed in the commit
//! that makes them false.

use anyhow::{bail, Context, Result};
use std::path::Path;

/// The classes wolf-book#29 sorted the remainder into. The `class` on a
/// row is not decoration: it is what makes the report countable, and a
/// spelling outside this set is a typo rather than a new class.
const KNOWN_CLASSES: [&str; 3] = ["file", "typed", "binary-or-shell"];

/// One corpus console block nothing replays, and why.
#[derive(Debug, serde::Deserialize)]
pub struct Declined {
    /// `principles/exercises/ch01/EXERCISES.md:18` — repo-relative path
    /// and the opening fence's line, the same three characters on all
    /// three hosts.
    pub block: String,
    /// `file`, `typed`, or `binary-or-shell` (wolf-book#29 §2/§4/§5).
    pub class: String,
    /// What a replay of this block would need. The author's sentence,
    /// not the rule's.
    pub needs: String,
    /// The issue that holds the remainder.
    pub filed: String,
    /// What is expected to end this row.
    pub retires: String,
}

#[derive(Debug, Default, serde::Deserialize)]
struct Manifest {
    #[serde(default)]
    declined: Vec<Declined>,
}

pub struct Ledger {
    rows: Vec<Declined>,
}

impl Ledger {
    pub fn load(root: &Path) -> Result<Ledger> {
        let path = root.join("samples-declined.toml");
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        let manifest: Manifest =
            toml::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
        let mut seen: Vec<&str> = Vec::new();
        for row in &manifest.declined {
            if !KNOWN_CLASSES.contains(&row.class.as_str()) {
                bail!(
                    "samples-declined.toml: {} names class `{}` — one of {KNOWN_CLASSES:?}",
                    row.block,
                    row.class
                );
            }
            if seen.contains(&row.block.as_str()) {
                bail!(
                    "samples-declined.toml: {} has two rows — one block, one reason",
                    row.block
                );
            }
            seen.push(&row.block);
        }
        Ok(Ledger {
            rows: manifest.declined,
        })
    }

    pub fn row(&self, block: &str) -> Option<&Declined> {
        self.rows.iter().find(|r| r.block == block)
    }

    pub fn rows(&self) -> &[Declined] {
        &self.rows
    }

    /// How many rows carry each class, in the ledger's own order.
    pub fn by_class(&self) -> Vec<(&'static str, usize)> {
        KNOWN_CLASSES
            .iter()
            .map(|c| (*c, self.rows.iter().filter(|r| r.class == *c).count()))
            .collect()
    }
}

/// The gate, both ways.
///
/// `declined` is every corpus console block this run declined for a
/// reason that is not this host's lane; `replayed` is every corpus
/// block key that actually replayed here. Returns the failures, in the
/// samples runner's voice.
pub fn audit(ledger: &Ledger, declined: &[(String, String)], replayed: &[String]) -> Vec<String> {
    let mut failures = Vec::new();
    for (block, why) in declined {
        if ledger.row(block).is_none() {
            failures.push(format!(
                "{block} is a corpus console block nothing replays ({why}) and \
                 samples-declined.toml has no row for it — every hole in the corpus \
                 transcript lane is a sentence someone wrote, never a silence \
                 (wolf-book#29)"
            ));
        }
    }
    for row in ledger.rows() {
        if replayed.iter().any(|k| k == &row.block) {
            failures.push(format!(
                "samples-declined.toml declares {} not replayable ({}) and it REPLAYS \
                 now — the row is stale; delete it in the commit that closed the hole \
                 (it was retiring at {})",
                row.block, row.needs, row.retires
            ));
        }
        // A row neither declined nor replayed HERE is a block that
        // stood down for this lane's own reason (the native half off
        // unix), or one that does not exist at all. The second is
        // caught by the existence check in samples.rs, which runs on
        // every host; the first is exactly what must not fail.
    }
    failures
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ledger(text: &str) -> Result<Ledger> {
        let manifest: Manifest = toml::from_str(text)?;
        for row in &manifest.declined {
            if !KNOWN_CLASSES.contains(&row.class.as_str()) {
                bail!("bad class {}", row.class);
            }
        }
        Ok(Ledger {
            rows: manifest.declined,
        })
    }

    const ONE: &str = r#"
[[declined]]
block = "principles/exercises/ch01/EXERCISES.md:18"
class = "typed"
needs = "an interactive session"
filed = "wolf-book#29"
retires = "wolf-interp is08"
"#;

    #[test]
    fn a_declined_block_with_no_row_is_a_failure() {
        let l = ledger("").unwrap();
        let declined = vec![(
            "principles/exercises/ch01/EXERCISES.md:18".into(),
            "x".into(),
        )];
        let f = audit(&l, &declined, &[]);
        assert_eq!(f.len(), 1);
        assert!(f[0].contains("has no row"), "{}", f[0]);
    }

    #[test]
    fn a_declared_block_that_replays_is_stale() {
        let l = ledger(ONE).unwrap();
        let f = audit(
            &l,
            &[],
            &["principles/exercises/ch01/EXERCISES.md:18".to_string()],
        );
        assert_eq!(f.len(), 1);
        assert!(f[0].contains("stale"), "{}", f[0]);
    }

    #[test]
    fn a_declared_block_that_is_still_declined_is_quiet() {
        let l = ledger(ONE).unwrap();
        let declined = vec![(
            "principles/exercises/ch01/EXERCISES.md:18".into(),
            "x".into(),
        )];
        assert!(audit(&l, &declined, &[]).is_empty());
    }

    #[test]
    fn an_unknown_class_is_a_typo() {
        let bad = ONE.replace("typed", "typoed");
        assert!(ledger(&bad).is_err());
    }

    #[test]
    fn the_classes_are_counted() {
        let l = ledger(ONE).unwrap();
        assert_eq!(
            l.by_class(),
            vec![("file", 0), ("typed", 1), ("binary-or-shell", 0)]
        );
    }
}
