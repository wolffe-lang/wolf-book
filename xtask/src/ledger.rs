//! `cargo xtask ledger` — the audit-ledger gate (bs12).
//!
//! bs01 defined the book-audit loop: every ledger finding row becomes a
//! wolf-lang issue (labels `book-audit` + one `ba:*` severity +
//! `from:bsNN`) or is explicitly waived in review. Nothing enforced it,
//! so 169 rows rotted unfiled across 28 chapters until bs12 swept them.
//! This command is the enforcement: it parses every `AUDIT LEDGER`
//! comment block under `book/`, prints the per-chapter state so it is
//! visible in CI output instead of buried in an HTML comment, and — with
//! `--check` — fails when any open `ba:*` row is neither filed
//! (`wolf-lang#N` in the row) nor waived (`waived(...)` in the row, the
//! reviewed escape hatch bs01 allows).

use anyhow::{bail, Context, Result};
use std::path::Path;

#[derive(Debug, Default, Clone)]
struct ChapterState {
    name: String,
    closed: usize,
    filed: usize,
    waived: usize,
    unfiled: Vec<String>, // first line of each offending row
}

pub fn run(root: &Path, args: &[String]) -> Result<()> {
    let mut check = false;
    for a in args {
        match a.as_str() {
            "--check" => check = true,
            other => bail!("unknown argument `{other}` — `cargo xtask ledger [--check]`"),
        }
    }

    let book = root.join("book");
    let mut chapters: Vec<ChapterState> = Vec::new();
    let mut entries: Vec<_> = std::fs::read_dir(&book)
        .with_context(|| format!("reading {}", book.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .map(|n| n.starts_with("ch") && n.ends_with(".md"))
                .unwrap_or(false)
        })
        .collect();
    entries.sort();

    for path in entries {
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        if let Some(state) = scan_chapter(&name, &text) {
            chapters.push(state);
        }
    }

    let (mut open, mut filed, mut waived, mut closed, mut unfiled) = (0, 0, 0, 0, 0);
    for ch in &chapters {
        let ch_open = ch.filed + ch.waived + ch.unfiled.len();
        open += ch_open;
        filed += ch.filed;
        waived += ch.waived;
        closed += ch.closed;
        unfiled += ch.unfiled.len();
        eprintln!(
            "ledger: {:<10} open {:>3} (filed {:>3}, waived {}, UNFILED {}) closed {:>3}",
            ch.name,
            ch_open,
            ch.filed,
            ch.waived,
            ch.unfiled.len(),
            ch.closed
        );
        for row in &ch.unfiled {
            eprintln!("ledger:   UNFILED {row}");
        }
    }
    eprintln!(
        "ledger: {} chapter ledger(s) — {open} open ({filed} filed, {waived} waived, \
         {unfiled} unfiled), {closed} closed",
        chapters.len()
    );

    if unfiled > 0 {
        if check {
            bail!(
                "{unfiled} open ledger row(s) are neither filed (wolf-lang#N) nor \
                 waived (waived(...)) — bs01's loop: every finding row becomes an \
                 issue or is explicitly waived in review"
            );
        }
        eprintln!("ledger: NOTE: {unfiled} row(s) above would fail `--check`");
    }
    Ok(())
}

/// Parse one chapter's `AUDIT LEDGER` comment blocks. Returns None when
/// the chapter has no ledger (held chapters, stubs).
fn scan_chapter(name: &str, text: &str) -> Option<ChapterState> {
    let mut state = ChapterState {
        name: name.to_string(),
        ..Default::default()
    };
    let mut in_ledger = false;
    let mut saw_ledger = false;
    // Current open row accumulator: (first line, block text so far).
    let mut open_row: Option<(String, String)> = None;

    let finish = |row: Option<(String, String)>, state: &mut ChapterState| {
        if let Some((first, block)) = row {
            if block.contains("waived(") {
                state.waived += 1;
            } else if is_filed(&block) {
                state.filed += 1;
            } else {
                state.unfiled.push(first);
            }
        }
    };

    for line in text.lines() {
        if !in_ledger {
            if line.contains("<!-- AUDIT LEDGER") {
                in_ledger = true;
                saw_ledger = true;
            }
            continue;
        }
        let trimmed = line.trim_start();
        let row_start = trimmed.starts_with("- [ ]") || trimmed.starts_with("- [x]");
        if row_start || trimmed.starts_with("-->") {
            finish(open_row.take(), &mut state);
        }
        if trimmed.starts_with("-->") {
            in_ledger = false;
            continue;
        }
        if row_start {
            let is_ba = trimmed.starts_with("- [ ] ba:") || trimmed.starts_with("- [x] ba:");
            if !is_ba {
                continue; // accounting rows (running example, contract deltas, …)
            }
            if trimmed.starts_with("- [x]") {
                state.closed += 1;
            } else {
                open_row = Some((trimmed.to_string(), String::new()));
            }
        } else if let Some((_, block)) = open_row.as_mut() {
            block.push_str(trimmed);
            block.push('\n');
        }
    }
    finish(open_row.take(), &mut state);
    saw_ledger.then_some(state)
}

/// A row counts as filed when it cites a wolf-lang issue: `wolf-lang#123`.
fn is_filed(block: &str) -> bool {
    for (idx, _) in block.match_indices("wolf-lang#") {
        let rest = &block[idx + "wolf-lang#".len()..];
        if rest.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const LEDGER: &str = "\
prose above
<!-- AUDIT LEDGER (unpublished) — bsNN, chapter N.
     - [ ] ba:blocker — a finding that was filed.
           → filed wolf-lang#150 (bs12 theme batch).
     - [ ] ba:papercut — a finding that was waived.
           → waived(bs12 review): editorial marker.
     - [ ] ba:doc-only — a finding nobody filed.
     - [x] ba:diagnostic — a closed finding. HEALED.
     - [ ] Recorded as good news — not a ba row at all.
-->
prose below";

    #[test]
    fn counts_filed_waived_unfiled_closed() {
        let st = scan_chapter("chNN.md", LEDGER).expect("ledger found");
        assert_eq!(st.filed, 1);
        assert_eq!(st.waived, 1);
        assert_eq!(st.unfiled.len(), 1);
        assert!(st.unfiled[0].contains("ba:doc-only"));
        assert_eq!(st.closed, 1);
    }

    #[test]
    fn chapter_without_ledger_is_none() {
        assert!(scan_chapter("ch99.md", "no ledger here").is_none());
    }

    #[test]
    fn filed_requires_a_number() {
        assert!(is_filed("→ filed wolf-lang#7."));
        assert!(!is_filed("see wolf-lang# someday"));
        assert!(!is_filed("no citation at all"));
    }

    #[test]
    fn last_row_before_close_is_counted() {
        let text = "<!-- AUDIT LEDGER\n     - [ ] ba:blocker — trailing unfiled row.\n-->";
        let st = scan_chapter("ch.md", text).unwrap();
        assert_eq!(st.unfiled.len(), 1);
    }
}
