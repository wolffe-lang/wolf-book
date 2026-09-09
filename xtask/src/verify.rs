//! `cargo xtask verify-docs` — doc-truth checks. The documents that
//! make claims about the corpus, the pins, and the structure are held
//! to those claims by CI, in the house style: a stale count is a build
//! failure, not a footnote.

use anyhow::{bail, Context, Result};
use std::collections::{BTreeMap, BTreeSet};
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
    verify_spec_shape(root, &mut failures)?;
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

/// Appendix D's shape claims, derived from `vendor/spec/anchors.json`
/// rather than remembered (wolf-book#7).
///
/// The page states three counted facts about the specification — how
/// many documents it is, how many anchor namespaces they publish, and
/// which namespaces belong to which document — and until this check
/// none of them was held to anything. "The specification is seven
/// documents" survived the whole 0.2 line, and the table's Anchors
/// column showed one namespace for document 01 while `[conf.anchor.ns]`
/// gives it two. Both were found by reading, one sprint apart, and a
/// third instance was a matter of time: `anchors.json` already carries
/// the owning file of every anchor, so the table is derivable and the
/// prose counts are arithmetic on it.
///
/// Every comparison here runs BOTH WAYS. A namespace the page lists and
/// the spec does not publish is as wrong as one the spec publishes and
/// the page omits, and the second is the one that leaves a reader with a
/// tag and nowhere to take it.
fn verify_spec_shape(root: &Path, failures: &mut Vec<String>) -> Result<()> {
    let raw = std::fs::read_to_string(root.join("vendor/spec/anchors.json"))
        .context("reading vendor/spec/anchors.json")?;
    let parsed: serde_json::Value =
        serde_json::from_str(&raw).context("parsing vendor/spec/anchors.json")?;
    let anchors = parsed
        .get("anchors")
        .and_then(|a| a.as_object())
        .context("vendor/spec/anchors.json has no `anchors` object")?;

    // document number -> the namespaces owning anchors in that document.
    let mut spec: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (tag, file) in anchors {
        let file = file
            .as_str()
            .with_context(|| format!("anchors.json: `{tag}` does not name a file"))?;
        let (number, _) = file
            .split_once('-')
            .with_context(|| format!("anchors.json: `{file}` is not `NN-name.md`"))?;
        let namespace = tag.split('.').next().unwrap_or_default();
        if namespace.is_empty() {
            failures.push(format!(
                "vendor/spec/anchors.json: `{tag}` has no namespace"
            ));
            continue;
        }
        spec.entry(number.to_string())
            .or_default()
            .insert(namespace.to_string());
    }

    let page = std::fs::read_to_string(root.join("book/back/appendix-d.md"))?;
    failures.extend(spec_shape_failures(&spec, &page));
    Ok(())
}

/// The comparison itself, with no filesystem in it, so the two defects
/// that motivated this check can be planted in a test rather than
/// described in a comment.
fn spec_shape_failures(spec: &BTreeMap<String, BTreeSet<String>>, page: &str) -> Vec<String> {
    let mut failures = Vec::new();
    let table = appendix_d_namespace_table(page);

    for (number, namespaces) in spec {
        match table.get(number) {
            None => failures.push(format!(
                "book/back/appendix-d.md: the spec publishes anchors in document {number} \
                 and the table has no row for it"
            )),
            Some(listed) if listed != namespaces => failures.push(format!(
                "book/back/appendix-d.md: document {number} owns {} in the spec and the \
                 table lists {}",
                joined(namespaces),
                joined(listed)
            )),
            Some(_) => {}
        }
    }
    for number in table.keys() {
        if !spec.contains_key(number) {
            failures.push(format!(
                "book/back/appendix-d.md: the table lists document {number}, which owns no \
                 anchor in vendor/spec/anchors.json"
            ));
        }
    }

    // The prose counts. Both are arithmetic on the map above, and both
    // are written as words on the page, which is why they rotted
    // silently: no digit for a grep to catch.
    let documents = spec.len();
    let namespaces: BTreeSet<&String> = spec.values().flatten().collect();
    // The page wraps at 72 columns, so a claim like "twelve anchor
    // namespaces" reaches this check with a newline inside it. Collapse
    // whitespace before looking for a sentence, or the check finds
    // nothing and passes a page that says the wrong number.
    let prose = reader_prose(page)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    for (count, phrase) in [
        (documents, "documents"),
        (namespaces.len(), "anchor namespaces"),
    ] {
        let Some(word) = number_word(count) else {
            failures.push(format!(
                "book/back/appendix-d.md: {count} {phrase} is outside the range this check \
                 spells; extend `number_word`"
            ));
            continue;
        };
        let claim = format!("{word} {phrase}");
        if !prose.contains(&claim) {
            failures.push(format!(
                "book/back/appendix-d.md: the spec has {count} {phrase} and the page does \
                 not say `{claim}`"
            ));
        }
    }

    // How many of those documents the section table actually cites.
    let cited = appendix_d_cited_documents(page);
    if let Some(word) = number_word(cited.len()) {
        let claim = format!("cites {word} of the documents");
        if !prose.contains(&claim) {
            failures.push(format!(
                "book/back/appendix-d.md: the section table cites {} document(s) and the \
                 page does not say `{claim}`",
                cited.len()
            ));
        }
    }
    failures
}

/// `| 01. Surface Grammar | `gram.*`, `diag.*` |` → `01` → {gram, diag}.
fn appendix_d_namespace_table(page: &str) -> BTreeMap<String, BTreeSet<String>> {
    let mut out: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for line in page.lines() {
        let line = line.trim();
        if !line.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        if cells.len() != 2 {
            continue;
        }
        let Some((number, _)) = cells[0].split_once(". ") else {
            continue;
        };
        if number.len() != 2 || !number.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        let mut namespaces = BTreeSet::new();
        for item in cells[1].split(',') {
            let item = item.trim().trim_matches('`');
            if let Some(ns) = item.strip_suffix(".*") {
                namespaces.insert(ns.to_string());
            }
        }
        if !namespaces.is_empty() {
            out.insert(number.to_string(), namespaces);
        }
    }
    out
}

/// The document numbers the "Book section to clause" table cites.
fn appendix_d_cited_documents(page: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for line in page.lines() {
        let line = line.trim();
        if !line.starts_with('|') {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        if cells.len() != 3 {
            continue;
        }
        for item in cells[2].split(',') {
            let item = item.trim();
            if item.len() == 2 && item.chars().all(|c| c.is_ascii_digit()) {
                out.insert(item.to_string());
            }
        }
    }
    out
}

fn joined(set: &BTreeSet<String>) -> String {
    set.iter()
        .map(|n| format!("`{n}.*`"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The page writes its counts as words, so the check has to as well.
fn number_word(n: usize) -> Option<&'static str> {
    const WORDS: &[&str] = &[
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
        "twenty",
    ];
    WORDS.get(n).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TABLE: &str = "\
| Document | Anchors |
|----------|---------|
| 01. Surface Grammar | `gram.*`, `diag.*` |
| 02. Memory Model | `mem.*` |
";

    #[test]
    fn namespace_table_reads_two_namespaces_in_one_row() {
        let t = appendix_d_namespace_table(TABLE);
        assert_eq!(t.len(), 2);
        assert_eq!(
            t["01"],
            ["diag".to_string(), "gram".to_string()]
                .into_iter()
                .collect::<BTreeSet<_>>()
        );
        assert_eq!(
            t["02"],
            ["mem".to_string()].into_iter().collect::<BTreeSet<_>>()
        );
    }

    #[test]
    fn namespace_table_ignores_the_other_tables_on_the_page() {
        // Three-column section rows and the unanchored-tag table must
        // not be read as document rows.
        let page = format!(
            "{TABLE}\n| 1.4 The REPL | `[mem.ub.defined]` | 02 |\n\
             | `arith.checked` | the `trap(overflow)` line |\n"
        );
        let t = appendix_d_namespace_table(&page);
        assert_eq!(t.keys().collect::<Vec<_>>(), vec!["01", "02"]);
    }

    #[test]
    fn cited_documents_come_from_the_section_table_only() {
        let page = format!(
            "{TABLE}\n| 1.4 The REPL | `[mem.ub.defined]` | 02 |\n\
             | 1.5 What `run` did | `[gram.expr.block]` | 01, 02 |\n"
        );
        let cited = appendix_d_cited_documents(&page);
        assert_eq!(cited.len(), 2);
        assert!(cited.contains("01") && cited.contains("02"));
    }

    #[test]
    fn the_real_page_agrees_with_the_vendored_anchors() {
        // The check as CI runs it, against the repository's own files:
        // this is the assertion that would have failed on "seven
        // documents" and on document 01's missing `diag.*`.
        let root = crate::repo_root().expect("repo root");
        let mut failures = Vec::new();
        verify_spec_shape(&root, &mut failures).expect("check runs");
        assert!(failures.is_empty(), "{failures:#?}");
    }

    /// Two documents, three namespaces, and a page that gets all of it
    /// right — the control every negative test below deviates from.
    fn good_page() -> String {
        format!(
            "The specification is two documents, and they publish three \
             anchor namespaces between them. The table below this one \
             cites two of the documents.\n\n{TABLE}\n\
             | 1.4 The REPL | `[mem.ub.defined]` | 02 |\n\
             | 1.5 What `run` did | `[gram.expr.block]` | 01 |\n"
        )
    }

    fn good_spec() -> BTreeMap<String, BTreeSet<String>> {
        [
            (
                "01".to_string(),
                ["gram".to_string(), "diag".to_string()]
                    .into_iter()
                    .collect(),
            ),
            (
                "02".to_string(),
                ["mem".to_string()].into_iter().collect::<BTreeSet<_>>(),
            ),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn the_control_passes() {
        assert!(spec_shape_failures(&good_spec(), &good_page()).is_empty());
    }

    #[test]
    fn the_missing_diag_row_is_caught() {
        // bs30's defect: document 01 owns `gram.*` AND `diag.*`, and the
        // table showed only `gram.*`, so a reader who met `[diag.sev.teach]`
        // in a tool's output had no row to follow.
        let page = good_page().replace("`gram.*`, `diag.*`", "`gram.*`");
        let f = spec_shape_failures(&good_spec(), &page);
        assert_eq!(f.len(), 1, "{f:#?}");
        assert!(f[0].contains("document 01 owns"), "{f:#?}");
    }

    #[test]
    fn a_wrong_document_count_in_prose_is_caught() {
        // bs29's defect: "The specification is seven documents", true
        // once and wrong for the whole 0.2 line after it.
        let page = good_page().replace("is two documents", "is seven documents");
        let f = spec_shape_failures(&good_spec(), &page);
        assert_eq!(f.len(), 1, "{f:#?}");
        assert!(f[0].contains("`two documents`"), "{f:#?}");
    }

    #[test]
    fn a_wrong_namespace_count_in_prose_is_caught() {
        let page = good_page().replace("three anchor namespaces", "four anchor namespaces");
        let f = spec_shape_failures(&good_spec(), &page);
        assert_eq!(f.len(), 1, "{f:#?}");
        assert!(f[0].contains("`three anchor namespaces`"), "{f:#?}");
    }

    #[test]
    fn a_row_for_a_document_the_spec_does_not_publish_is_caught() {
        // The #246 shape from the book's side: a table row implying a
        // prefix that cannot be looked up.
        let page = good_page().replace(
            "| 02. Memory Model | `mem.*` |",
            "| 02. Memory Model | `mem.*` |\n| 07. Schedule Points | `sched.*` |",
        );
        let f = spec_shape_failures(&good_spec(), &page);
        assert_eq!(f.len(), 1, "{f:#?}");
        assert!(f[0].contains("lists document 07"), "{f:#?}");
    }

    #[test]
    fn a_document_the_page_forgot_is_caught() {
        let page = good_page().replace("| 02. Memory Model | `mem.*` |\n", "");
        let f = spec_shape_failures(&good_spec(), &page);
        assert!(f.iter().any(|m| m.contains("no row for it")), "{f:#?}");
    }

    #[test]
    fn a_stale_cited_count_is_caught() {
        let page = good_page().replace("cites two of the documents", "cites five of the documents");
        let f = spec_shape_failures(&good_spec(), &page);
        assert_eq!(f.len(), 1, "{f:#?}");
        assert!(f[0].contains("cites two of the documents"), "{f:#?}");
    }

    #[test]
    fn a_wrapped_prose_claim_is_still_found() {
        // The defect this test pins: the page wraps at 72 columns, so
        // the first version of the check missed `twelve anchor\nnamespaces`
        // and reported a correct page as wrong.
        let wrapped = "they publish twelve anchor\nnamespaces between them";
        let flat = wrapped.split_whitespace().collect::<Vec<_>>().join(" ");
        assert!(flat.contains("twelve anchor namespaces"));
        assert!(!wrapped.contains("twelve anchor namespaces"));
    }

    #[test]
    fn number_words_cover_the_counts_the_page_uses() {
        assert_eq!(number_word(11), Some("eleven"));
        assert_eq!(number_word(12), Some("twelve"));
        assert_eq!(number_word(21), None);
    }
}
