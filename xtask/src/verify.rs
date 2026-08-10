//! `cargo xtask verify-docs` — doc-truth checks. The documents that
//! make claims about the corpus, the pins, and the structure are held
//! to those claims by CI, in the house style: a stale count is a build
//! failure, not a footnote.

use anyhow::{bail, Context, Result};
use std::path::Path;

pub fn run(root: &Path) -> Result<()> {
    let mut failures: Vec<String> = Vec::new();

    // 1. The pins parse (and grammar-sync's pin field is a full sha).
    let pins = crate::load_pins(root)?;
    if pins.wolf_lsp_rev.len() != 40 || !pins.wolf_lsp_rev.chars().all(|c| c.is_ascii_hexdigit()) {
        failures.push("wolf-toolchain.toml: [wolf-lsp].rev is not a full commit sha".into());
    }

    // 2. The vendored grammars load in the interpreter.
    for name in crate::highlight::GRAMMARS {
        let path = root.join("highlight").join(name);
        let json = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        let wolf_json = std::fs::read_to_string(root.join("highlight/wolf.tmLanguage.json"))
            .unwrap_or_default();
        if let Err(e) = crate::tm::Grammar::load_with(&json, &[&wolf_json]) {
            failures.push(format!("{name}: does not load: {e:#}"));
        }
    }

    // 3. The corpus count matches the count the docs claim.
    let claimed = claimed_corpus_count(root)?;
    let actual = count_lu_files(root)?;
    if claimed != actual {
        failures.push(format!(
            "EXERCISES-PENDING.md claims {claimed} directive-headed .lu files; \
             the corpus has {actual} — update the claim or the corpus"
        ));
    }

    // 4. Pending-manifest rows must name exercises the pending doc knows.
    let pending_doc = std::fs::read_to_string(root.join("principles/EXERCISES-PENDING.md"))?;
    let manifest = std::fs::read_to_string(root.join("samples-pending.toml"))?;
    let parsed: toml::Value = manifest.parse()?;
    if let Some(rows) = parsed.get("pending").and_then(|p| p.as_array()) {
        for row in rows {
            let id = row.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let file = root.join("principles/exercises").join(format!("{id}.lu"));
            if !file.is_file() {
                failures.push(format!("samples-pending.toml: `{id}` has no .lu file"));
            }
            // ch13/ex13-1 → "13-1" must appear in the pending doc.
            if let Some(ex) = id.rsplit('/').next().and_then(|f| f.strip_prefix("ex")) {
                if !pending_doc.contains(ex) {
                    failures.push(format!(
                        "samples-pending.toml: `{id}` is not in EXERCISES-PENDING.md — \
                         the manifest and the doc must agree"
                    ));
                }
            }
        }
    }

    // 5. TOC ↔ SUMMARY ↔ stubs: chapters exist, numbered headings match.
    verify_toc(root, &mut failures)?;

    // 6. The colophon points at the pin file.
    let colophon = std::fs::read_to_string(root.join("book/back/colophon.md"))?;
    if !colophon.contains("wolf-toolchain.toml") {
        failures.push("book/back/colophon.md no longer names wolf-toolchain.toml".into());
    }

    if failures.is_empty() {
        println!(
            "verify-docs: ok (corpus count {actual}, pins well-formed, TOC/stub numbering agrees)"
        );
        Ok(())
    } else {
        for f in &failures {
            eprintln!("verify-docs: FAIL {f}");
        }
        bail!("{} doc-truth failure(s)", failures.len());
    }
}

fn claimed_corpus_count(root: &Path) -> Result<usize> {
    let text = std::fs::read_to_string(root.join("principles/EXERCISES-PENDING.md"))?;
    for line in text.lines() {
        if let Some(idx) = line.find(" directive-headed") {
            let head = &line[..idx];
            let num: String = head
                .chars()
                .rev()
                .take_while(|c| c.is_ascii_digit())
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();
            if !num.is_empty() {
                return Ok(num.parse()?);
            }
        }
    }
    bail!("EXERCISES-PENDING.md no longer states the corpus file count");
}

fn count_lu_files(root: &Path) -> Result<usize> {
    let mut count = 0usize;
    fn walk(dir: &Path, count: &mut usize) -> Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                walk(&path, count)?;
            } else if path.extension().and_then(|e| e.to_str()) == Some("lu") {
                *count += 1;
            }
        }
        Ok(())
    }
    walk(&root.join("principles/exercises"), &mut count)?;
    Ok(count)
}

fn verify_toc(root: &Path, failures: &mut Vec<String>) -> Result<()> {
    let toc = std::fs::read_to_string(root.join("principles/TOC.md"))?;
    let summary = std::fs::read_to_string(root.join("book/SUMMARY.md"))?;

    let mut current_chapter: Option<u32> = None;
    for line in toc.lines() {
        if let Some(rest) = line.strip_prefix("### Chapter ") {
            let num: u32 = rest
                .split_whitespace()
                .next()
                .unwrap_or("")
                .parse()
                .with_context(|| format!("TOC chapter line: {line}"))?;
            current_chapter = Some(num);
            let stub = root.join(format!("book/ch{num:02}.md"));
            if !stub.is_file() {
                failures.push(format!("TOC chapter {num} has no stub book/ch{num:02}.md"));
                continue;
            }
            let content = std::fs::read_to_string(&stub)?;
            if !content.starts_with(&format!("# {num}. ")) {
                failures.push(format!(
                    "book/ch{num:02}.md: first heading does not carry its number `# {num}. …`"
                ));
            }
            if !summary.contains(&format!("(ch{num:02}.md)")) {
                failures.push(format!("SUMMARY.md does not list ch{num:02}.md"));
            }
        } else if let Some(rest) = line.strip_prefix("- ") {
            // `- 8.4 Cycles are fine here — …`
            let Some(chapter) = current_chapter else {
                continue;
            };
            let first = rest.split_whitespace().next().unwrap_or("");
            if !first.contains('.')
                || !first
                    .split('.')
                    .all(|p| p.chars().all(|c| c.is_ascii_digit()) && !p.is_empty())
            {
                continue;
            }
            let stub = root.join(format!("book/ch{chapter:02}.md"));
            let Ok(content) = std::fs::read_to_string(&stub) else {
                continue;
            };
            if !content.contains(&format!("## {first} ")) {
                failures.push(format!(
                    "book/ch{chapter:02}.md: TOC section {first} has no `## {first} …` heading"
                ));
            }
        }
    }
    Ok(())
}
