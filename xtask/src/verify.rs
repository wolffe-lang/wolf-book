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

    // 7. Part 5's line-count claims are measured, not remembered.
    verify_wc_claims(root, &mut failures)?;

    // 8. Each project's on-disk program is the program its chapter prints.
    verify_projects(root, &mut failures)?;

    // 9. Clause tags, diagnostic codes, trap kinds: the back matter and
    //    the chapters against the vendored spec artifacts.
    verify_clause_tags(root, &mut failures)?;
    verify_diagnostic_codes(root, &mut failures)?;
    verify_traps(root, &mut failures)?;

    // 10. Every printed exercise has a published solution.
    verify_solutions(root, &mut failures)?;

    // 11. The tense discipline, as a grep CI owns (TONE.md §Tense
    //     discipline). Prose only: fence content is the tools' voice and
    //     fence directives are build metadata.
    verify_tense(root, &mut failures)?;

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

/// Reader-facing prose of one page: HTML comments stripped, fenced
/// blocks dropped, so the checks below never read the tools' voice or a
/// fence's build directives as if the book had written them.
fn reader_prose(text: &str) -> String {
    let stripped = crate::render::strip_html_comments(text);
    let mut out = String::new();
    let mut in_fence = false;
    for line in stripped.lines() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if !in_fence {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

fn book_pages(root: &Path) -> Result<Vec<(String, String)>> {
    let mut pages = Vec::new();
    let mut dirs = vec![root.join("book")];
    while let Some(dir) = dirs.pop() {
        for entry in std::fs::read_dir(&dir)? {
            let path = entry?.path();
            if path.is_dir() {
                dirs.push(path);
            } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
                let rel = path
                    .strip_prefix(root)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .replace('\\', "/");
                pages.push((rel, std::fs::read_to_string(&path)?));
            }
        }
    }
    pages.sort();
    Ok(pages)
}

/// Clause tags the tools print that the spec has not registered.
/// Appendix D lists them for the reader; this list is what CI holds to
/// its length, in both directions: a new one fails the build, and one
/// that gains an anchor upstream has to leave the appendix.
const UNANCHORED_TAGS: &[&str] = &[
    "arith.checked",
    "mod.cycle",
    "mod.dup",
    "mod.use.unused",
    "mod.vis.private",
    "repl.trap.alive",
];

fn verify_clause_tags(root: &Path, failures: &mut Vec<String>) -> Result<()> {
    let anchors = std::fs::read_to_string(root.join("vendor/spec/anchors.json"))
        .context("reading vendor/spec/anchors.json")?;
    let anchors: toml::Value = serde_json::from_str::<serde_json::Value>(&anchors)
        .map(|v| toml::Value::String(v.to_string()))
        .context("parsing vendor/spec/anchors.json")?;
    let anchor_text = anchors.as_str().unwrap_or_default().to_string();
    let appendix = std::fs::read_to_string(root.join("book/back/appendix-d.md"))?;
    let mut seen_unanchored: Vec<&str> = Vec::new();
    for (rel, text) in book_pages(root)? {
        for tag in clause_tags(&reader_prose(&text)) {
            let quoted = format!("\"{tag}\":");
            if anchor_text.contains(&quoted) {
                continue;
            }
            match UNANCHORED_TAGS.iter().find(|t| **t == tag) {
                Some(known) => {
                    if !seen_unanchored.contains(known) {
                        seen_unanchored.push(known);
                    }
                }
                None => failures.push(format!(
                    "{rel}: clause tag `[{tag}]` is in no spec anchor and not in \
                     Appendix D's list of tags without one"
                )),
            }
        }
    }
    for tag in UNANCHORED_TAGS {
        if !appendix.contains(tag) {
            failures.push(format!(
                "book/back/appendix-d.md: `{tag}` is listed as unanchored but the \
                 appendix does not name it"
            ));
        }
        if !seen_unanchored.contains(tag) && anchor_text.contains(&format!("\"{tag}\":")) {
            failures.push(format!(
                "vendor/spec/anchors.json now anchors `{tag}` — drop its row from \
                 Appendix D and from UNANCHORED_TAGS"
            ));
        }
    }
    Ok(())
}

/// `[a.b.c]` tags in prose. Diagnostic codes, markdown links and
/// generic-argument brackets are not clause tags.
fn clause_tags(prose: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = prose.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'[' {
            if let Some(end) = prose[i + 1..].find(']') {
                let inner = &prose[i + 1..i + 1 + end];
                // A tag is dotted lowercase segments: `mem.ub.defined`.
                // Slice ranges (`[2..]`, `[t.0..t.1]`) and format specs
                // (`[.precision]`) are not tags.
                let segments: Vec<&str> = inner.split('.').collect();
                let dotted = segments.len() > 1
                    && !inner.contains("..")
                    && segments.iter().all(|s| {
                        s.starts_with(|c: char| c.is_ascii_lowercase())
                            && s.chars()
                                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
                    });
                if dotted && !out.contains(&inner.to_string()) {
                    out.push(inner.to_string());
                }
                i += end + 2;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn verify_diagnostic_codes(root: &Path, failures: &mut Vec<String>) -> Result<()> {
    let catalog = std::fs::read_to_string(root.join("vendor/spec/diagnostic-codes.txt"))
        .context("reading vendor/spec/diagnostic-codes.txt")?;
    let known: Vec<&str> = catalog
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    let appendix = std::fs::read_to_string(root.join("book/back/appendix-c.md"))?;
    for code in codes_in(&appendix) {
        if !known.contains(&code.as_str()) {
            failures.push(format!(
                "book/back/appendix-c.md: `{code}` is not in the compiler's catalog"
            ));
        }
    }
    for (rel, text) in book_pages(root)? {
        if rel.ends_with("appendix-c.md") || rel.ends_with("book-index.md") {
            continue;
        }
        for code in codes_in(&reader_prose(&text)) {
            if !appendix.contains(&code) {
                failures.push(format!(
                    "{rel}: shows `{code}`, which Appendix C does not list"
                ));
            }
        }
    }
    Ok(())
}

fn codes_in(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    for (i, b) in bytes.iter().enumerate() {
        if !(*b == b'E' || *b == b'W') {
            continue;
        }
        if i > 0 && (bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_') {
            continue;
        }
        let rest = &text[i + 1..];
        let digits: String = rest.chars().take(4).collect();
        if digits.len() == 4 && digits.chars().all(|c| c.is_ascii_digit()) {
            let after = rest.chars().nth(4);
            if after.map(|c| c.is_ascii_digit()).unwrap_or(false) {
                continue;
            }
            let code = format!("{}{digits}", *b as char);
            if !out.contains(&code) {
                out.push(code);
            }
        }
    }
    out
}

/// The twelve kinds `[conf.trap.set]` closes. Appendix B is the book's
/// copy of that set, and a copy that drifts is worse than no copy.
const TRAP_KINDS: &[&str] = &[
    "overflow",
    "div-zero",
    "bounds",
    "use-after-move",
    "exclusivity",
    "region-fault",
    "stale-handle",
    "alloc-contract",
    "assert",
    "race",
    "ub",
    "deadlock",
];

fn verify_traps(root: &Path, failures: &mut Vec<String>) -> Result<()> {
    let appendix = std::fs::read_to_string(root.join("book/back/appendix-b.md"))?;
    for kind in TRAP_KINDS {
        if !appendix.contains(&format!("`{kind}`")) {
            failures.push(format!(
                "book/back/appendix-b.md: trap kind `{kind}` is missing from the table"
            ));
        }
    }
    if !appendix.contains("twelve") {
        failures.push("book/back/appendix-b.md: no longer states the set's size".into());
    }
    Ok(())
}

fn verify_solutions(root: &Path, failures: &mut Vec<String>) -> Result<()> {
    let page = std::fs::read_to_string(root.join("book/back/solutions.md"))?;
    for ((ch, num), _) in crate::backmatter::printed_exercises(root)? {
        let marker = format!("<summary>Exercise {ch}-{num}.");
        if !page.contains(&marker) {
            failures.push(format!(
                "book/back/solutions.md: exercise {ch}-{num} is printed in the chapter \
                 and has no published solution"
            ));
        }
    }
    Ok(())
}

/// Deferral vocabulary and repo apparatus, in reader-facing prose only.
const FORBIDDEN_PROSE: &[&str] = &[
    "at this pin",
    "a feature away",
    "does not exist yet",
    "when it lands",
    "the pinned interpreter",
    "the pinned toolchain",
    "blocker:",
    "owner:",
    "audit ledger",
    "young toolchain",
];

fn verify_tense(root: &Path, failures: &mut Vec<String>) -> Result<()> {
    for (rel, text) in book_pages(root)? {
        let prose = reader_prose(&text);
        for needle in FORBIDDEN_PROSE {
            if prose.contains(needle) {
                failures.push(format!(
                    "{rel}: reader-facing prose says \"{needle}\" (TONE.md \
                     §Tense discipline: the ledger is where that lives)"
                ));
            }
        }
        for word in prose.split(|c: char| !(c.is_ascii_alphanumeric())) {
            let is_sprint = matches!(word.len(), 3 | 4)
                && (word.starts_with('s') || word.starts_with("is") || word.starts_with("bs"))
                && word[1..].chars().any(|c| c.is_ascii_digit())
                && word.chars().skip(1).all(|c| c.is_ascii_digit() || c == 's')
                && word
                    .chars()
                    .last()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false);
            if is_sprint {
                failures.push(format!(
                    "{rel}: reader-facing prose names `{word}` — sprint identifiers are \
                     CI's vocabulary, never a page's"
                ));
            }
        }
    }
    Ok(())
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

/// Part 5's honesty rule says line counts come from `wc`, not from vibes
/// (`principles/TOC.md`, Part 5). A chapter that makes such a claim carries
/// the numbers in a machine-readable comment beside the prose:
///
/// ```text
/// <!-- WC (verify-docs): samples/contrast/count.c=106 samples/… .lu=57 -->
/// ```
///
/// This recomputes every one of them. A program that grows by a line fails
/// the book's build until the page that measured it is corrected, which is
/// the only way a printed measurement stays a measurement.
fn verify_wc_claims(root: &Path, failures: &mut Vec<String>) -> Result<()> {
    let mut claims = 0usize;
    let mut mds: Vec<std::path::PathBuf> = Vec::new();
    collect_md(&root.join("book"), &mut mds)?;
    mds.sort();
    for md in &mds {
        let text = std::fs::read_to_string(md)?;
        for line in text.lines() {
            let Some(rest) = line.trim().strip_prefix("<!-- WC (verify-docs):") else {
                continue;
            };
            let rest = rest.trim_end().trim_end_matches("-->").trim();
            for pair in rest.split_whitespace() {
                let Some((rel, claimed)) = pair.rsplit_once('=') else {
                    failures.push(format!(
                        "{}: WC claim `{pair}` is not `<path>=<count>`",
                        md.display()
                    ));
                    continue;
                };
                let claimed: usize = match claimed.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        failures.push(format!(
                            "{}: WC claim `{pair}` has a non-numeric count",
                            md.display()
                        ));
                        continue;
                    }
                };
                let path = root.join(rel);
                let Ok(body) = std::fs::read_to_string(&path) else {
                    failures.push(format!(
                        "{}: WC claim names `{rel}`, which does not exist",
                        md.display()
                    ));
                    continue;
                };
                // `wc -l` counts newlines; every file here ends with one.
                let actual = body.lines().count();
                if actual != claimed {
                    failures.push(format!(
                        "{}: WC claim says `{rel}` is {claimed} lines; `wc -l` says \
                         {actual} — the page's measurement is stale",
                        md.display()
                    ));
                }
                claims += 1;
            }
        }
    }
    println!("verify-docs: {claims} line-count claim(s) recomputed");
    Ok(())
}

/// Each guided project keeps its finished program on disk under
/// `samples/projects/<name>/<name>.lu`, so a reader can take the whole
/// file and so `wc` has something stable to measure. That copy is only
/// worth having if it is the same bytes the chapter prints, and a second
/// copy of a program is exactly the kind of thing that drifts — so the
/// file has to appear, contiguously, in the concatenation of its
/// chapter's wolf listings.
fn verify_projects(root: &Path, failures: &mut Vec<String>) -> Result<()> {
    let base = root.join("samples/projects");
    if !base.is_dir() {
        return Ok(());
    }
    // Every wolf listing in the book, joined per chapter in reading order.
    let mut mds: Vec<std::path::PathBuf> = Vec::new();
    collect_md(&root.join("book"), &mut mds)?;
    mds.sort();
    let mut listings: Vec<String> = Vec::new();
    for md in &mds {
        let text = std::fs::read_to_string(md)?;
        let mut joined = String::new();
        for seg in crate::fence::segments(&text) {
            if let crate::fence::Segment::Fence(f) = seg {
                if f.info.trim() == "wolf" || f.info.trim_start().starts_with("wolf,") {
                    joined.push_str(&f.content);
                }
            }
        }
        listings.push(joined);
    }

    let mut dirs: Vec<std::path::PathBuf> = std::fs::read_dir(&base)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    let mut checked = 0usize;
    for dir in &dirs {
        let name = dir.file_name().unwrap().to_string_lossy().into_owned();
        let program = dir.join(format!("{name}.lu"));
        let Ok(body) = std::fs::read_to_string(&program) else {
            failures.push(format!(
                "samples/projects/{name}/ has no {name}.lu — a project directory is \
                 its finished program"
            ));
            continue;
        };
        if !listings.iter().any(|l| l.contains(body.trim_end())) {
            failures.push(format!(
                "samples/projects/{name}/{name}.lu is not printed verbatim by any \
                 chapter — the on-disk program and the printed program have drifted"
            ));
            continue;
        }
        checked += 1;
    }
    println!("verify-docs: {checked} project program(s) match the chapters that print them");
    Ok(())
}

fn collect_md(dir: &Path, out: &mut Vec<std::path::PathBuf>) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_md(&path, out)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("md")
            && path.file_name().and_then(|n| n.to_str()) != Some("SUMMARY.md")
        {
            out.push(path);
        }
    }
    Ok(())
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
