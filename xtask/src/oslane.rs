//! `samples-os.toml` — the per-host expectation ledger (bs25).
//!
//! The samples lane runs on three hosts. Most of what the book claims is
//! the same on all three, and the rig's default posture is that a sample
//! passes everywhere. Two kinds of thing are not the same everywhere:
//!
//!   * a program the pinned toolchain **refuses by name** on one host,
//!     because a runtime the language needs is not built there yet; and
//!   * a console transcript whose **tool text** differs on one host.
//!
//! Before bs25 the rig had exactly one answer for both — a blanket skip,
//! which is the answer the book does not accept anywhere else. A skip
//! says "not checked". What is true here is stronger and worth writing
//! down: *this host prints exactly this, at this pin, and here is when
//! that stops being true*. So each row DECLARES the outcome, byte for
//! byte, and the rig enforces the declaration in both directions:
//!
//!   * the declared text must be what the host actually produces — a
//!     refusal that changes its wording is a FAIL, not a shrug; and
//!   * the moment the sample starts passing (or the transcript starts
//!     matching the book), the row is a FLIP — a hard error naming the
//!     row to delete, exactly as `samples-pending.toml` behaves when a
//!     feature lands.
//!
//! Rows carry `retires`: the release that is expected to end them. They
//! are removed in the pin-bump commit that makes them false, never
//! silently absorbed.

use anyhow::{bail, Context, Result};
use std::path::Path;

/// The host this run is executing on, in the ledger's spelling.
pub fn host_os() -> &'static str {
    if cfg!(windows) {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    }
}

const KNOWN_OS: [&str; 3] = ["windows", "macos", "linux"];

/// A sample the pinned toolchain refuses by name on one host.
#[derive(Debug, serde::Deserialize)]
pub struct Refusal {
    /// Sample id, e.g. `ch30/ex30-3`, `book/ch30/part-pargrep`.
    pub id: String,
    pub os: String,
    /// The exit status the refusal carries.
    pub exit: i32,
    /// The refusal sentence, verbatim — the whole of stderr.
    pub stderr: String,
    /// The release expected to end this row.
    pub retires: String,
    /// Why, dated, in the book's voice.
    pub note: String,
}

/// A console block whose replay differs on one host.
#[derive(Debug, serde::Deserialize)]
pub struct Transcript {
    /// `book/ch23.md:117` — repo-relative path and the opening fence's line.
    pub block: String,
    pub os: String,
    /// The transcript this host really produces, verbatim.
    pub expect: String,
    /// The issue that would end this row.
    pub filed: String,
    pub retires: String,
    pub note: String,
}

#[derive(Debug, Default, serde::Deserialize)]
struct Manifest {
    #[serde(default)]
    refusal: Vec<Refusal>,
    #[serde(default)]
    transcript: Vec<Transcript>,
}

/// The ledger, already narrowed to this host: a row for another host is
/// inert here, and `ids()` still reports every row's subject so a typo
/// on any lane is caught on every lane.
#[derive(Debug, Default)]
pub struct Ledger {
    refusals: Vec<Refusal>,
    transcripts: Vec<Transcript>,
    /// Every `id` named by a `[[refusal]]` row, this host's or not.
    all_refusal_ids: Vec<String>,
    /// Every `block` named by a `[[transcript]]` row, this host's or not.
    all_transcript_blocks: Vec<String>,
}

impl Ledger {
    pub fn load(root: &Path) -> Result<Ledger> {
        let path = root.join("samples-os.toml");
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        let manifest: Manifest =
            toml::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
        for r in &manifest.refusal {
            if !KNOWN_OS.contains(&r.os.as_str()) {
                bail!(
                    "samples-os.toml: [[refusal]] {} names os `{}` — one of {KNOWN_OS:?}",
                    r.id,
                    r.os
                );
            }
        }
        for t in &manifest.transcript {
            if !KNOWN_OS.contains(&t.os.as_str()) {
                bail!(
                    "samples-os.toml: [[transcript]] {} names os `{}` — one of {KNOWN_OS:?}",
                    t.block,
                    t.os
                );
            }
        }
        let host = host_os();
        let all_refusal_ids = manifest.refusal.iter().map(|r| r.id.clone()).collect();
        let all_transcript_blocks = manifest
            .transcript
            .iter()
            .map(|t| t.block.clone())
            .collect();
        Ok(Ledger {
            refusals: manifest
                .refusal
                .into_iter()
                .filter(|r| r.os == host)
                .collect(),
            transcripts: manifest
                .transcript
                .into_iter()
                .filter(|t| t.os == host)
                .collect(),
            all_refusal_ids,
            all_transcript_blocks,
        })
    }

    /// This host's declared refusal for a sample, if any.
    pub fn refusal(&self, id: &str) -> Option<&Refusal> {
        self.refusals.iter().find(|r| r.id == id)
    }

    /// This host's declared transcript for a console block, if any.
    pub fn transcript(&self, block: &str) -> Option<&Transcript> {
        self.transcripts.iter().find(|t| t.block == block)
    }

    pub fn all_refusal_ids(&self) -> &[String] {
        &self.all_refusal_ids
    }

    pub fn all_transcript_blocks(&self) -> &[String] {
        &self.all_transcript_blocks
    }

    /// How many of this host's rows there are, for the report line.
    pub fn here(&self) -> usize {
        self.refusals.len() + self.transcripts.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ledger_from(text: &str) -> Result<Manifest> {
        Ok(toml::from_str(text)?)
    }

    #[test]
    fn the_host_name_is_one_of_the_three() {
        assert!(KNOWN_OS.contains(&host_os()));
    }

    #[test]
    fn a_refusal_row_round_trips() {
        let m = ledger_from(
            r#"
[[refusal]]
id = "ch30/ex30-3"
os = "windows"
exit = 1
stderr = '''wolf run: cannot compile this yet — no channels'''
retires = "wolf 0.2.3"
note = "s60b"
"#,
        )
        .unwrap();
        assert_eq!(m.refusal.len(), 1);
        assert_eq!(m.refusal[0].exit, 1);
        assert_eq!(
            m.refusal[0].stderr,
            "wolf run: cannot compile this yet — no channels"
        );
    }

    #[test]
    fn a_transcript_row_keeps_its_backslashes() {
        // A literal string is what makes a windows path writable here at
        // all: `app\wolf.pkg` must not become an escape.
        let m = ledger_from(
            r#"
[[transcript]]
block = "book/ch23.md:117"
os = "windows"
filed = "wolf-lang#1"
retires = "the fix"
note = "n"
expect = '''
$ wolf add rows --path ../rows --dir app
wolf add: created app\wolf.pkg (minimal manifest)'''
"#,
        )
        .unwrap();
        assert!(m.transcript[0].expect.contains(r"app\wolf.pkg"));
        // The leading newline after ''' is TOML's to trim; the rig
        // compares whole lines, so a stray blank first line would break
        // every row.
        assert!(m.transcript[0].expect.starts_with("$ wolf add"));
    }

    #[test]
    fn an_unknown_os_is_refused() {
        let dir = std::env::temp_dir().join(format!("bs25-oslane-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("samples-os.toml"),
            "[[refusal]]\nid = \"x\"\nos = \"plan9\"\nexit = 1\nstderr = \"s\"\n\
             retires = \"r\"\nnote = \"n\"\n",
        )
        .unwrap();
        let err = Ledger::load(&dir).unwrap_err().to_string();
        assert!(err.contains("plan9"), "{err}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn rows_for_another_host_are_inert_but_still_named() {
        let dir = std::env::temp_dir().join(format!("bs25-oslane-inert-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let other = if host_os() == "windows" {
            "linux"
        } else {
            "windows"
        };
        std::fs::write(
            dir.join("samples-os.toml"),
            format!(
                "[[refusal]]\nid = \"ch30/ex30-3\"\nos = \"{other}\"\nexit = 1\n\
                 stderr = \"s\"\nretires = \"r\"\nnote = \"n\"\n"
            ),
        )
        .unwrap();
        let l = Ledger::load(&dir).unwrap();
        assert!(l.refusal("ch30/ex30-3").is_none());
        assert_eq!(l.here(), 0);
        // Still named, so a typo is caught on every lane, not only the
        // one lane the row applies to.
        assert_eq!(l.all_refusal_ids(), ["ch30/ex30-3"]);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
