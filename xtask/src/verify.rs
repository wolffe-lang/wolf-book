//! `cargo xtask verify-docs` — doc-truth checks. The documents that
//! make claims about the corpus, the pins, and the structure are held
//! to those claims by CI, in the house style: a stale count is a build
//! failure, not a footnote.

use anyhow::{bail, Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

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

    // 12. Version literals the console replay cannot see (wolf-book#7,
    //     part 2). Every version-shaped literal in reader-facing text is
    //     enumerated with the pin that audited it, and a pin that has
    //     moved past an audit fails the build with the sentence named.
    verify_version_literals(root, &mut failures)?;

    // 13. The one conditional wolf-book#5 states and nothing checked
    //     (bs44). The vendored grammar paints a nested string literal
    //     inside an interpolation hole with a CLOSED match, because
    //     `tm.rs` expands `#include`s eagerly and a recursive
    //     `#strings` is a load error rather than a wrong paint. The
    //     closed form cannot hold a nested literal carrying a brace or
    //     an escaped quote, and such a block stays body ink. Measured
    //     today, none does — this is the check that keeps that a fact
    //     rather than a memory.
    verify_paintable_interpolation(root, &mut failures)?;

    // 14. EXERCISES-INDEX.md's figures against its own table, and that
    //     table against the corpus it ledgers (wolf-book#40). The file
    //     said "generated view" and was typed; it is hand-kept now and
    //     says so, and every number on it is arithmetic CI owns.
    verify_exercises_index(root, &mut failures)?;

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

/// Nested string literals inside an interpolation hole that the
/// vendored grammar's CLOSED form cannot paint (wolf-book#5).
///
/// wolf-lsp le06 ships the nested literal as
/// `"interpolated-string": { "match": "\"[^\"{}\\r\\n]*\"" }` rather
/// than as an include back into `#strings`, because the include is a
/// cycle and `tm::Grammar::load` refuses cycles by design — a grammar
/// that outgrows the interpreter fails loudly instead of highlighting
/// wrong. The closed form is exact for the literals the book prints
/// today, and it has one edge: a nested literal whose body carries a
/// `{`, a `}` or an escaped quote does not match it. When that happens
/// nothing is merely under-painted — the enclosing hole's `end`
/// pattern fires on the wrong brace and the rest of the line leaves
/// the string scope entirely.
///
/// This function emulates the closed form rather than describing it,
/// so the check and the grammar cannot drift apart in words: at each
/// `"` inside a hole it scans the way `[^"{}\r\n]*"` scans, and
/// reports the site when that scan does not reach a clean close.
///
/// A plain escape (`"{audit("340\n275")}"`) is NOT reported: the
/// character class admits a backslash, the literal matches whole and
/// paints as a string. Only the escape sequence inside it goes
/// unpainted, which is a shade, not a scope.
///
/// Pure, so the defect can be planted in a test rather than in the book.
pub fn unpaintable_nested_literals(code: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for (i, line) in code.lines().enumerate() {
        for why in unpaintable_in_line(line) {
            out.push((i + 1, why));
        }
    }
    out
}

/// One line of a wolf listing. Raw strings (`r"…"`, `r#"…"#`) and block
/// strings (`"""`) are skipped: neither interpolates, and the grammar
/// paints both with their own rules.
fn unpaintable_in_line(line: &str) -> Vec<String> {
    let b: Vec<char> = line.chars().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    // Depth of `{…}` holes inside the string we are in, 0 = not in a hole.
    let mut hole = 0usize;
    let mut in_str = false;
    while i < b.len() {
        let c = b[i];
        if !in_str {
            // A raw string opener: `r"` or `r#…#"`. Skip to its close.
            if c == 'r' && i + 1 < b.len() && (b[i + 1] == '"' || b[i + 1] == '#') {
                let mut j = i + 1;
                let mut hashes = 0usize;
                while j < b.len() && b[j] == '#' {
                    hashes += 1;
                    j += 1;
                }
                if j < b.len() && b[j] == '"' {
                    i = skip_raw(&b, j + 1, hashes);
                    continue;
                }
            }
            if c == '"' {
                // A block string opener swallows the rest of the line.
                if b.get(i + 1) == Some(&'"') && b.get(i + 2) == Some(&'"') {
                    break;
                }
                in_str = true;
                i += 1;
                continue;
            }
            if c == '/' && b.get(i + 1) == Some(&'/') {
                break; // a line comment: nothing after it is code
            }
            i += 1;
            continue;
        }
        // Inside a string.
        if hole == 0 {
            match c {
                '\\' => i += 2,
                '{' => {
                    hole = 1;
                    i += 1;
                }
                '"' => {
                    in_str = false;
                    i += 1;
                }
                _ => i += 1,
            }
            continue;
        }
        // Inside a `{…}` hole.
        match c {
            '}' => {
                hole -= 1;
                i += 1;
            }
            '{' => {
                hole += 1;
                i += 1;
            }
            '"' => {
                // The closed form, walked: `"[^"{}\r\n]*"`.
                let start = i;
                let mut j = i + 1;
                let mut blocked: Option<char> = None;
                while j < b.len() {
                    let d = b[j];
                    if d == '"' {
                        break;
                    }
                    if d == '{' || d == '}' {
                        blocked = Some(d);
                        break;
                    }
                    j += 1;
                }
                if let Some(d) = blocked {
                    out.push(format!(
                        "a nested string literal in an interpolation hole carries `{d}` \
                         (`{}`) — the vendored grammar's closed form `\"[^\"{{}}\\r\\n]*\"` \
                         cannot match it and the hole's own `}}` closes on the wrong brace",
                        snippet(&b, start)
                    ));
                    // Give up on this line: the scope is already wrong.
                    return out;
                }
                if j >= b.len() {
                    out.push(format!(
                        "a nested string literal in an interpolation hole never closes on \
                         this line (`{}`)",
                        snippet(&b, start)
                    ));
                    return out;
                }
                if j > start + 1 && b[j - 1] == '\\' {
                    out.push(format!(
                        "a nested string literal in an interpolation hole carries an escaped \
                         quote (`{}`) — the closed form's character class excludes `\"`, so \
                         it terminates at the escape and the rest of the line leaves the \
                         string scope",
                        snippet(&b, start)
                    ));
                    return out;
                }
                i = j + 1;
            }
            _ => i += 1,
        }
    }
    out
}

/// Past a raw string's closing `"` + `hashes` hashes.
fn skip_raw(b: &[char], from: usize, hashes: usize) -> usize {
    let mut i = from;
    while i < b.len() {
        if b[i] == '"' {
            let mut k = 0usize;
            while k < hashes && b.get(i + 1 + k) == Some(&'#') {
                k += 1;
            }
            if k == hashes {
                return i + 1 + hashes;
            }
        }
        i += 1;
    }
    b.len()
}

/// Enough of the site to find it by eye, from the offending `"`.
fn snippet(b: &[char], start: usize) -> String {
    let end = (start + 24).min(b.len());
    b[start..end].iter().collect::<String>()
}

/// Every wolf-family listing in the book and in the exercise corpus.
/// Both, deliberately: `book/back/solutions.md` is generated from the
/// corpus pages, so a corpus listing reaches the renderer too, and a
/// failure that names only the generated copy sends the reader to a
/// file they must not edit.
fn verify_paintable_interpolation(root: &Path, failures: &mut Vec<String>) -> Result<()> {
    let mut mds: Vec<std::path::PathBuf> = Vec::new();
    collect_md(&root.join("book"), &mut mds)?;
    collect_md(&root.join("principles"), &mut mds)?;
    mds.sort();
    let mut scanned = 0usize;
    for md in &mds {
        let text = std::fs::read_to_string(md)?;
        for seg in crate::fence::segments(&text) {
            let crate::fence::Segment::Fence(f) = seg else {
                continue;
            };
            let lang = f.info.trim().split(',').next().unwrap_or("").trim();
            if !matches!(lang, "wolf" | "wolfi" | "wolf-pkg") {
                continue;
            }
            scanned += 1;
            for (line, why) in unpaintable_nested_literals(&f.content) {
                failures.push(format!(
                    "{}:{}: {why} — wolf-book#5. Bind the inner string to a `let` and print \
                     the binding, the way exercise 2-9 does",
                    md.strip_prefix(root).unwrap_or(md).display(),
                    f.open_line + 1 + line,
                ));
            }
        }
    }
    println!(
        "verify-docs: {scanned} wolf listing(s) scanned for interpolation the vendored \
         grammar cannot paint (wolf-book#5)"
    );
    Ok(())
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
    failures.extend(appendix_c_count_failures(&known, &appendix));
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

/// Appendix C's opening sentence counts two things and until bs35 no
/// gate read either: how many codes the compiler's catalog holds, and
/// how many of them this edition names. Both are arithmetic on files
/// that are already in the tree, and both were moved by hand at every
/// pin bump that touched them (wolf-book#7). This is Appendix D's
/// pattern — the comparison is pure so the defect can be planted in a
/// test — with one difference: Appendix C writes its counts as DIGITS,
/// so there is no `number_word` in it.
fn appendix_c_count_failures(known: &[&str], page: &str) -> Vec<String> {
    let mut failures = Vec::new();
    // The page wraps at 72 columns, so the sentence reaches this check
    // with a newline inside it. Collapse whitespace first or the check
    // finds nothing and passes a page that says the wrong number.
    let prose = reader_prose(page)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let shown = appendix_c_table_codes(page);
    for (count, claim) in [
        (
            known.len(),
            format!("The catalog holds {} codes", known.len()),
        ),
        (
            shown.len(),
            format!(
                "these {} are the ones a page in this edition names",
                shown.len()
            ),
        ),
    ] {
        if !prose.contains(&claim) {
            failures.push(format!(
                "book/back/appendix-c.md: the count is {count} and the page does not say \
                 `{claim}`"
            ));
        }
    }
    failures
}

/// The codes Appendix C's own table lists, one row each. The table is
/// the page's answer to "which codes does this edition name", so the
/// second half of the count sentence is arithmetic on it rather than a
/// number somebody kept in their head.
fn appendix_c_table_codes(page: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for line in page.lines() {
        let line = line.trim();
        if !line.starts_with('|') {
            continue;
        }
        let first = line.trim_matches('|').split('|').next().unwrap_or_default();
        let cell = first.trim().trim_matches('`');
        let mut chars = cell.chars();
        if matches!(chars.next(), Some('E') | Some('W'))
            && cell.len() == 5
            && chars.all(|c| c.is_ascii_digit())
        {
            out.insert(cell.to_string());
        }
    }
    out
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

/// wolf-book#7, part 2. Five pin bumps running healed the same version
/// literals by hand and called them "residue the runner cannot see":
/// §1.2's two Windows archive names, §1.2's `+dev.unknown` sentence,
/// and the stamp paragraphs that reverse at every change of build kind.
/// They are literals in prose with no run behind them, so the console
/// replay — which is a byte compare against what the tools print — has
/// nothing to compare. This is wolf-web's `check-version-prose.py` for
/// the book: every version-shaped literal in reader-facing text is
/// enumerated in `audit/version-literals.txt` with a count and the pin
/// that audited it, and a pin that has moved past an audit fails and
/// names the sentence to re-read.
///
/// Two clocks, because two projects release on their own: an entry says
/// `audited-at-wolf=` (re-read when the compiler pin moves) or
/// `audited-at-lupin=` (when the interpreter pin does). The third
/// spelling, `not-a-toolchain-version`, is for the version-shaped
/// literals that are not claims about wolf or lupin at all — a package
/// version in chapter 23, a published record's key in chapter 25. Those
/// still need an entry, so a NEW literal anywhere fails until somebody
/// classifies it; they just do not ride a toolchain clock, because
/// re-reading `rows 1.4.0` at every bump is the noise that gets a gate
/// rubber-stamped.
fn verify_version_literals(root: &Path, failures: &mut Vec<String>) -> Result<()> {
    let pins = toolchain_versions(root)?;
    let mut found: BTreeMap<(String, String), usize> = BTreeMap::new();
    for (rel, text) in book_pages(root)? {
        for literal in version_literals(&auditable_text(&text)) {
            *found.entry((rel.clone(), literal)).or_default() += 1;
        }
    }
    let allow_path = root.join("audit/version-literals.txt");
    let allow = std::fs::read_to_string(&allow_path)
        .with_context(|| format!("reading {}", allow_path.display()))?;
    let before = failures.len();
    failures.extend(version_literal_failures(&found, &allow, &pins));
    if failures.len() == before {
        println!(
            "verify-docs: {} version literal(s) in reader-facing text, each audited against \
             wolf {} / lupin {}",
            found.values().sum::<usize>(),
            pins.0,
            pins.1
        );
    }
    Ok(())
}

/// `[wolf].impl_version` and the version number inside `[lupin].version`
/// (`lupin 0.1.30 (wolf-interp, …)` is the whole stamped line; the audit
/// clock is the `0.1.30` in it).
fn toolchain_versions(root: &Path) -> Result<(String, String)> {
    let text = std::fs::read_to_string(root.join("wolf-toolchain.toml"))
        .context("reading wolf-toolchain.toml")?;
    let value: toml::Value = text.parse().context("parsing wolf-toolchain.toml")?;
    let wolf = value
        .get("wolf")
        .and_then(|t| t.get("impl_version"))
        .and_then(|v| v.as_str())
        .context("wolf-toolchain.toml: [wolf].impl_version missing")?
        .to_string();
    let stamped = value
        .get("lupin")
        .and_then(|t| t.get("version"))
        .and_then(|v| v.as_str())
        .context("wolf-toolchain.toml: [lupin].version missing")?;
    let lupin = stamped
        .split_whitespace()
        .find(|w| w.starts_with(|c: char| c.is_ascii_digit()) && w.contains('.'))
        .context("wolf-toolchain.toml: [lupin].version carries no version number")?
        .to_string();
    Ok((wolf, lupin))
}

/// The text this check reads: HTML comments dropped (the ledgers are not
/// reader-facing), `console`, `diagnostic` and program fences dropped
/// (those the replay and the samples runner own, byte for byte), and
/// `text` fences KEPT — which is the point, because §1.2's two Windows
/// archive names live in one and nothing executes them.
fn auditable_text(text: &str) -> String {
    let stripped = crate::render::strip_html_comments(text);
    let mut out = String::new();
    let mut fence: Option<String> = None;
    for line in stripped.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            fence = match fence {
                None => Some(trimmed.trim_start_matches('`').to_string()),
                Some(_) => None,
            };
            continue;
        }
        if fence
            .as_deref()
            .map(|f| f.starts_with("text"))
            .unwrap_or(true)
        {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

/// Version-shaped literals: `0.2.9`, `v1.4.0`, `0.2.9+dev.e0ce018`.
/// Three components, because two would make every `0.5 seconds` a
/// version claim; and neither end may touch another digit or dot, which
/// is what keeps `127.0.0.1` from reading as one.
fn version_literals(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < chars.len() {
        // A literal starts at a digit, optionally behind one `v`, and
        // nothing wordish may sit to the left of either.
        let start = i;
        let mut j = i;
        if chars[j] == 'v' {
            j += 1;
        }
        let left_ok = start == 0
            || !(chars[start - 1].is_ascii_digit()
                || chars[start - 1] == '.'
                || chars[start - 1].is_alphanumeric()
                || chars[start - 1] == '_');
        // Exactly three dot-separated runs of digits. `..` ends a run
        // with no digits after the dot, which is what keeps a slice
        // range (`318..322`) from reading as a version; a fourth run
        // is what keeps `127.0.0.1` out.
        let mut runs = 0usize;
        let mut k = j;
        loop {
            let digits = chars[k..].iter().take_while(|c| c.is_ascii_digit()).count();
            if digits == 0 {
                break;
            }
            k += digits;
            runs += 1;
            if k < chars.len() && chars[k] == '.' {
                k += 1;
            } else {
                break;
            }
        }
        let mut end = k;
        let shaped = runs == 3 && left_ok;
        if shaped {
            // `+dev.<sha>` rides along: `0.2.9+dev.e0ce018` is one
            // claim, and allowlisting half a stamp is not a check.
            let tail: String = chars[end..].iter().collect();
            if let Some(rest) = tail.strip_prefix("+dev.") {
                let n = rest
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric())
                    .count();
                if n > 0 {
                    end += 5 + n;
                }
            }
        }
        let right_ok = end >= chars.len()
            || !(chars[end].is_ascii_digit() || chars[end] == '.' || chars[end].is_alphanumeric());
        if shaped && right_ok {
            out.push(chars[start..end].iter().collect::<String>());
            i = end;
            continue;
        }
        i = if k > start { k } else { start + 1 };
    }
    out
}

/// Pure, so the defect can be planted in a test rather than in the book.
fn version_literal_failures(
    found: &BTreeMap<(String, String), usize>,
    allow: &str,
    pins: &(String, String),
) -> Vec<String> {
    /// One entry: the count the page must carry, and the clocks it
    /// rides (`("wolf", "0.2.10")`), at most one per project.
    type Entry = (usize, Vec<(String, String)>);

    let mut failures = Vec::new();
    let mut allowed: BTreeMap<(String, String), Entry> = BTreeMap::new();
    for (n, line) in allow.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        let bad = |why: &str| {
            format!(
                "audit/version-literals.txt:{}: {why} — an entry is \
                 `<path> <literal> <count> audited-at-wolf=<v>` (or audited-at-lupin=, or \
                 BOTH clocks on one entry, or not-a-toolchain-version)",
                n + 1
            )
        };
        if parts.len() < 4 {
            failures.push(bad("cannot parse"));
            continue;
        }
        let Ok(count) = parts[2].parse::<usize>() else {
            failures.push(bad("the third field is not a count"));
            continue;
        };
        // One (page, literal) pair may be a claim about BOTH projects —
        // §1.2 says `0.1.31` once for the interpreter the COMPILER
        // names and once for the interpreter this book PINS — so the
        // clock fields are a list, not a field. bs38: with one clock
        // per entry, the pair could only ride one of them, and whichever
        // was chosen the other project's bump would leave the sentence
        // unread, which is the exact defect this file exists to stop.
        let mut clocks: Vec<(String, String)> = Vec::new();
        let mut bogus = false;
        for field in &parts[3..] {
            if *field == "not-a-toolchain-version" {
                if parts.len() != 4 {
                    failures.push(bad("`not-a-toolchain-version` stands alone"));
                    bogus = true;
                }
            } else if let Some(v) = field.strip_prefix("audited-at-wolf=") {
                clocks.push(("wolf".to_string(), v.to_string()));
            } else if let Some(v) = field.strip_prefix("audited-at-lupin=") {
                clocks.push(("lupin".to_string(), v.to_string()));
            } else {
                failures.push(bad("a field after the count names no clock"));
                bogus = true;
            }
        }
        if bogus {
            continue;
        }
        if clocks.len() == 2 && clocks[0].0 == clocks[1].0 {
            failures.push(bad("the same clock is named twice"));
            continue;
        }
        if clocks.len() > 2 {
            failures.push(bad("there are only two clocks"));
            continue;
        }
        allowed.insert(
            (parts[0].to_string(), parts[1].to_string()),
            (count, clocks),
        );
    }
    for ((path, literal), count) in found {
        let Some((want, clocks)) = allowed.get(&(path.clone(), literal.clone())) else {
            failures.push(format!(
                "{path}: `{literal}` x{count} is not in audit/version-literals.txt — a version \
                 literal in reader-facing text gets an entry and the pin that audited it"
            ));
            continue;
        };
        if count != want {
            failures.push(format!(
                "{path}: `{literal}` appears x{count}, audit/version-literals.txt says x{want}"
            ));
        }
        for (clock, audited) in clocks {
            let pin = match clock.as_str() {
                "wolf" => &pins.0,
                "lupin" => &pins.1,
                _ => continue,
            };
            if audited != pin {
                failures.push(format!(
                    "{path}: `{literal}` was audited at {clock} {audited}, the {clock} pin is \
                     now {pin} — re-read the sentence, then re-stamp its audited-at-{clock}"
                ));
            }
        }
    }
    for (path, literal) in allowed.keys() {
        if !found.contains_key(&(path.clone(), literal.clone())) {
            failures.push(format!(
                "audit/version-literals.txt lists `{literal}` in {path}, which no longer carries it"
            ));
        }
    }
    failures
}

// ---------------------------------------------------------------------
// EXERCISES-INDEX.md (wolf-book#40)
//
// The file called itself a "generated view" and nothing generated it.
// It cannot be generated: its tier column is not a function of anything
// on disk (a `run(…)` corpus file executes on BOTH machines, and 136
// rows spelling that directive are tiered `run (lupin)` anyway), and
// four fifths of the page is editorial prose about which batches are
// held and why. So it stays hand-kept and says so — and every number on
// it is arithmetic over its own table, held in both directions here.
//
// What that catches, measured at this head: chapter 33 shipped seven
// exercises in `673183b` and the index never grew a section for them,
// so the table was seven short of the corpus while the totals line was
// separately seven short of the table. Two different sevens, which is
// how the issue read one of them as a single drifted figure.
// ---------------------------------------------------------------------

/// The five kinds of `principles/EXERCISES.md` §3. A vein (§8) may
/// qualify a kind — in parentheses, or joined with `+`, which §3 says to
/// read as the parenthetical form — and is not itself counted.
const EXERCISE_KINDS: [&str; 5] = [
    "fingers",
    "comprehension",
    "extension",
    "spelunking",
    "design",
];

/// One row of the index table.
struct IndexRow {
    id: String,
    kinds: BTreeSet<String>,
    tier: String,
    /// Tokens in the type cell that are neither a kind nor a known vein.
    unknown: Vec<String>,
}

/// Split an index table row from the RIGHT. The section cell is the only
/// one that can carry a `|` of its own — a heading like
/// ``§6.2 — `?`, `else`, `else |err|` `` does, on two rows — so the three
/// cells that matter are counted back from the end rather than forward
/// from the start, which is the bug that made a hand count of this table
/// come out at 331 instead of 334.
fn index_row_cells(line: &str) -> Option<(String, String, String)> {
    let t = line.trim();
    if !t.starts_with('|') || !t.ends_with('|') {
        return None;
    }
    let cells: Vec<&str> = t.trim_matches('|').split('|').map(str::trim).collect();
    if cells.len() < 4 {
        return None;
    }
    let n = cells.len();
    let (id, ty, tier) = (cells[n - 3], cells[n - 2], cells[n - 1]);
    // `13-2 → printed in §13.2` is an id cell with an annotation.
    let id = id.split_whitespace().next().unwrap_or("");
    if id.is_empty() || !id.contains('-') {
        return None;
    }
    let (a, b) = id.split_once('-')?;
    if a.is_empty() || b.is_empty() || !b.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if !a.chars().all(|c| c.is_ascii_alphanumeric()) {
        return None;
    }
    Some((id.to_string(), ty.to_string(), tier.to_string()))
}

/// The taxonomy kinds a type cell names, hybrids counted once per kind.
/// Parenthetical veins are stripped; `+`-joined veins are dropped by the
/// same rule (§3). Anything left that is neither a kind nor a vein is
/// reported rather than silently ignored.
fn row_kinds(ty: &str, veins: &BTreeSet<String>) -> (BTreeSet<String>, Vec<String>) {
    let mut kinds = BTreeSet::new();
    let mut unknown = Vec::new();
    // Drop the checker half (`fingers · lupin`) and any parenthetical.
    let head = ty.split(" · ").next().unwrap_or("");
    let mut flat = String::new();
    let mut depth = 0usize;
    for c in head.chars() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            _ if depth == 0 => flat.push(c),
            _ => {}
        }
    }
    for tok in flat.split('+') {
        let tok = tok.trim();
        if tok.is_empty() {
            continue;
        }
        let norm = tok.to_ascii_lowercase();
        if EXERCISE_KINDS.contains(&norm.as_str()) {
            kinds.insert(norm);
        } else if !veins.contains(&norm) {
            unknown.push(tok.to_string());
        }
    }
    (kinds, unknown)
}

/// Every row of every `| section | id | type · checker | tier |` table on
/// the page, in order.
fn index_rows(page: &str, veins: &BTreeSet<String>) -> Vec<IndexRow> {
    let mut out = Vec::new();
    for line in page.lines() {
        let Some((id, ty, tier)) = index_row_cells(line) else {
            continue;
        };
        let (kinds, unknown) = row_kinds(&ty, veins);
        out.push(IndexRow {
            id,
            kinds,
            tier,
            unknown,
        });
    }
    out
}

/// The bolded vein names of `principles/EXERCISES.md` §8, so a stem may
/// name a vein this check has never heard of without the build going red
/// on a word the doctrine does in fact define.
fn taxonomy_veins(exercises_md: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for line in exercises_md.lines() {
        let t = line.trim_start();
        let Some(rest) = t.strip_prefix("- **") else {
            continue;
        };
        let Some(name) = rest.split("**").next() else {
            continue;
        };
        let name = name.trim().to_ascii_lowercase();
        if !name.is_empty() {
            out.insert(name.clone());
            // §8 writes "The coreutils vein"; a stem would say
            // "coreutils". Index the bare noun too.
            if let Some(bare) = name.strip_prefix("the ") {
                out.insert(bare.trim_end_matches(" vein").to_string());
            }
        }
    }
    out
}

/// `## ch05 — 16 exercises` → ("ch05", 16).
fn index_chapter_heading(line: &str) -> Option<(String, usize)> {
    let rest = line.strip_prefix("## ")?;
    let (name, tail) = rest.split_once(" — ")?;
    let n = tail.strip_suffix(" exercises")?.trim().parse().ok()?;
    Some((name.trim().to_string(), n))
}

/// One `a · b · c` tally sentence → the figure each label carries. The
/// page is flattened first: these sentences wrap to the page width, and
/// reading one line of a wrapped tally finds only its first figure and
/// calls every other one missing.
fn tally_after(page: &str, prefix: &str) -> Option<BTreeMap<String, usize>> {
    let flat = page.replace('\n', " ");
    let at = flat.find(prefix)? + prefix.len();
    let body = &flat[at..];
    let body = body.split(". ").next().unwrap_or(body);
    let mut out = BTreeMap::new();
    for part in body.split(" · ") {
        let part = part.trim().trim_end_matches('.');
        // Either "186 run (lupin)" or "fingers 55".
        if let Some((n, label)) = part.split_once(' ') {
            if let Ok(v) = n.parse::<usize>() {
                out.insert(label.trim().to_string(), v);
                continue;
            }
            if let Ok(v) = label.trim().parse::<usize>() {
                out.insert(n.trim().to_string(), v);
            }
        }
    }
    Some(out)
}

/// `**292 of the 341 are printed**` → (292, 341). Markdown emphasis and
/// newlines are stripped first, because the sentence wraps to the page
/// width and has carried a `**` mid-phrase before.
fn printed_claim(page: &str) -> (Option<usize>, Option<usize>) {
    let flat = page.replace('\n', " ").replace("**", "");
    let Some(at) = flat.find(" are printed") else {
        return (None, None);
    };
    let words: Vec<&str> = flat[..at].split_whitespace().collect();
    let n = words.len();
    // … <a> of the <b> [are printed]
    if n >= 4 && words[n - 2] == "the" && words[n - 3] == "of" {
        return (words[n - 4].parse().ok(), words[n - 1].parse().ok());
    }
    (None, None)
}

fn first_number_after(page: &str, needle: &str) -> Option<usize> {
    let at = page.find(needle)? + needle.len();
    let rest = &page[at..];
    let digits: String = rest
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect();
    digits.parse().ok()
}

/// Every figure on the page against the table under it, and the table
/// against the corpus it claims to ledger.
fn exercises_index_failures(
    page: &str,
    corpus: &BTreeSet<String>,
    printed: &BTreeSet<String>,
    veins: &BTreeSet<String>,
) -> Vec<String> {
    let mut f = Vec::new();
    let rows = index_rows(page, veins);
    if rows.is_empty() {
        f.push("EXERCISES-INDEX.md: no table rows parsed — the table shape moved".into());
        return f;
    }

    // 1. The table is the corpus, both directions.
    let ids: BTreeSet<String> = rows.iter().map(|r| r.id.clone()).collect();
    if ids.len() != rows.len() {
        let mut seen = BTreeSet::new();
        for r in &rows {
            if !seen.insert(r.id.clone()) {
                f.push(format!("EXERCISES-INDEX.md: `{}` has two rows", r.id));
            }
        }
    }
    for missing in corpus.difference(&ids) {
        f.push(format!(
            "EXERCISES-INDEX.md: the corpus has `{missing}` and the index has no row for it"
        ));
    }
    for extra in ids.difference(corpus) {
        f.push(format!(
            "EXERCISES-INDEX.md: a row claims `{extra}`, which no master defines"
        ));
    }

    // 2. Unknown taxonomy tokens.
    for r in &rows {
        for u in &r.unknown {
            f.push(format!(
                "EXERCISES-INDEX.md: `{}` is tagged `{u}`, which is neither a kind \
                 (EXERCISES.md §3) nor a vein (§8)",
                r.id
            ));
        }
    }

    // 3. Each `## chNN — N exercises` heading against its own rows.
    let mut heading: Option<(String, usize)> = None;
    let mut counted = 0usize;
    let close = |h: &Option<(String, usize)>, counted: usize, f: &mut Vec<String>| {
        if let Some((name, claimed)) = h {
            if *claimed != counted {
                f.push(format!(
                    "EXERCISES-INDEX.md: `## {name}` says {claimed} exercises and has \
                     {counted} rows"
                ));
            }
        }
    };
    for line in page.lines() {
        if let Some(h) = index_chapter_heading(line) {
            close(&heading, counted, &mut f);
            heading = Some(h);
            counted = 0;
        } else if index_row_cells(line).is_some() {
            counted += 1;
        }
    }
    close(&heading, counted, &mut f);

    // 4. The header total.
    match first_number_after(page, "in the corpus:") {
        Some(n) if n == rows.len() => {}
        Some(n) => f.push(format!(
            "EXERCISES-INDEX.md: the header says {n} total and the table has {}",
            rows.len()
        )),
        None => f.push("EXERCISES-INDEX.md: the header no longer states a total".into()),
    }

    // 5. The tier totals line, figure by figure, and its stated sum.
    let mut tiers: BTreeMap<String, usize> = BTreeMap::new();
    for r in &rows {
        *tiers.entry(r.tier.clone()).or_default() += 1;
    }
    match tally_after(page, "Tier totals: ") {
        None => f.push("EXERCISES-INDEX.md: the tier totals line is gone".into()),
        Some(claimed) => {
            for (tier, n) in &tiers {
                match claimed.get(tier) {
                    Some(c) if c == n => {}
                    Some(c) => f.push(format!(
                        "EXERCISES-INDEX.md: the totals line says {c} `{tier}` and the \
                         table has {n}"
                    )),
                    None => f.push(format!(
                        "EXERCISES-INDEX.md: the totals line omits `{tier}` ({n} rows)"
                    )),
                }
            }
            for tier in claimed.keys() {
                if !tiers.contains_key(tier) {
                    f.push(format!(
                        "EXERCISES-INDEX.md: the totals line names `{tier}`, which no row uses"
                    ));
                }
            }
            match first_number_after(page, "That is ") {
                Some(n) if n == rows.len() => {}
                Some(n) => f.push(format!(
                    "EXERCISES-INDEX.md: the totals line sums to {n} and the table has {}",
                    rows.len()
                )),
                None => f.push("EXERCISES-INDEX.md: the totals line states no sum".into()),
            }
        }
    }

    // 6. The taxonomy spread.
    let mut spread: BTreeMap<String, usize> = BTreeMap::new();
    for r in &rows {
        for k in &r.kinds {
            *spread.entry(k.clone()).or_default() += 1;
        }
    }
    match tally_after(
        page,
        "Taxonomy spread (tags, hybrids counted once per kind): ",
    ) {
        None => f.push("EXERCISES-INDEX.md: the taxonomy spread line is gone".into()),
        Some(claimed) => {
            for kind in EXERCISE_KINDS {
                let have = spread.get(kind).copied().unwrap_or(0);
                match claimed.get(kind) {
                    Some(c) if *c == have => {}
                    Some(c) => f.push(format!(
                        "EXERCISES-INDEX.md: the taxonomy spread says {c} `{kind}` and the \
                         table has {have}"
                    )),
                    None => f.push(format!(
                        "EXERCISES-INDEX.md: the taxonomy spread omits `{kind}` ({have} rows)"
                    )),
                }
            }
        }
    }

    // 7. The printed claim, against the chapters themselves. The
    //     sentence is bolded and wraps, so it is read backwards from
    //     "are printed" rather than forwards from a `**` that belongs to
    //     a different sentence three paragraphs up.
    let printed_here = ids.intersection(printed).count();
    let (claimed_printed, claimed_of) = printed_claim(page);
    match (claimed_printed, claimed_of) {
        (Some(p), Some(total)) => {
            if p != printed_here {
                f.push(format!(
                    "EXERCISES-INDEX.md: the page says {p} are printed and the chapters \
                     print {printed_here} of its rows"
                ));
            }
            if total != rows.len() {
                f.push(format!(
                    "EXERCISES-INDEX.md: the printed sentence is out of {total} and the \
                     table has {}",
                    rows.len()
                ));
            }
        }
        _ => f.push("EXERCISES-INDEX.md: the printed-count sentence is gone".into()),
    }

    f
}

/// Every `**Exercise N-M**` the masters define, as the id is spelled.
fn corpus_exercise_ids(root: &Path) -> Result<BTreeSet<String>> {
    let mut out = BTreeSet::new();
    let mut files = vec![root.join("principles/EXERCISES.md")];
    let dir = root.join("principles/exercises");
    let mut subs: Vec<PathBuf> = std::fs::read_dir(&dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    subs.sort();
    files.extend(subs.into_iter().map(|d| d.join("EXERCISES.md")));
    for path in files {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        for line in text.lines() {
            if let Some(rest) = line.trim_start().strip_prefix("**Exercise ") {
                if let Some(id) = rest.split("**").next() {
                    let id = id.trim();
                    if id.contains('-') {
                        out.insert(id.to_string());
                    }
                }
            }
        }
    }
    if out.is_empty() {
        bail!("no exercise markers found under principles/ — the corpus moved");
    }
    Ok(out)
}

fn verify_exercises_index(root: &Path, failures: &mut Vec<String>) -> Result<()> {
    let page = std::fs::read_to_string(root.join("principles/EXERCISES-INDEX.md"))
        .context("reading principles/EXERCISES-INDEX.md")?;
    let corpus = corpus_exercise_ids(root)?;
    let printed: BTreeSet<String> = crate::backmatter::printed_exercises(root)?
        .keys()
        .map(|(ch, n)| format!("{ch}-{n}"))
        .collect();
    let exercises_md = std::fs::read_to_string(root.join("principles/EXERCISES.md"))
        .context("reading principles/EXERCISES.md")?;
    let veins = taxonomy_veins(&exercises_md);
    failures.extend(exercises_index_failures(&page, &corpus, &printed, &veins));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- EXERCISES-INDEX.md (wolf-book#40) ------------------------------

    fn veins() -> BTreeSet<String> {
        taxonomy_veins("- **Schedule play** — seeded interleaving predictions.\n")
    }

    /// A page with two chapters, every figure correct.
    fn index_page() -> String {
        "# EXERCISES-INDEX.md — the corpus ledger\n\
         \n\
         Ledger of every exercise in the corpus: 3 total.\n\
         \n\
         **2 of the 3 are printed**, and the build fails otherwise.\n\
         \n\
         Tier totals: 2 run (lupin) · 1 prose. That is 3.\n\
         Taxonomy spread (tags, hybrids counted once per kind): fingers 1 ·\n\
         comprehension 1 · extension 1 · spelunking 0 · design 1.\n\
         \n\
         ## ch01 — 2 exercises\n\
         \n\
         | section | exercise | type · checker | tier |\n\
         |---|---|---|---|\n\
         | §1.1 — One | 1-1 | fingers · lupin | run (lupin) |\n\
         | §1.2 — Two | 1-2 | comprehension + extension · lupin | run (lupin) |\n\
         \n\
         ## ch02 — 1 exercises\n\
         \n\
         | section | exercise | type · checker | tier |\n\
         |---|---|---|---|\n\
         | Chapter batch | 2-1 | design | prose |\n"
            .to_string()
    }

    fn corpus3() -> BTreeSet<String> {
        ["1-1", "1-2", "2-1"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn printed2() -> BTreeSet<String> {
        ["1-1", "1-2"].iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn the_index_control_passes() {
        let f = exercises_index_failures(&index_page(), &corpus3(), &printed2(), &veins());
        assert!(f.is_empty(), "{f:#?}");
    }

    #[test]
    fn a_corpus_exercise_with_no_row_is_caught() {
        // Chapter 33's shape: the master defines it, the index never grew
        // a section for it.
        let mut corpus = corpus3();
        corpus.insert("33-1".into());
        let f = exercises_index_failures(&index_page(), &corpus, &printed2(), &veins());
        assert!(
            f.iter()
                .any(|m| m.contains("`33-1`") && m.contains("no row")),
            "{f:#?}"
        );
    }

    #[test]
    fn a_row_naming_an_exercise_no_master_defines_is_caught() {
        let mut corpus = corpus3();
        corpus.remove("2-1");
        let f = exercises_index_failures(&index_page(), &corpus, &printed2(), &veins());
        assert!(
            f.iter()
                .any(|m| m.contains("`2-1`") && m.contains("no master")),
            "{f:#?}"
        );
    }

    #[test]
    fn a_chapter_heading_that_did_not_move_with_its_rows_is_caught() {
        let page = index_page().replace("## ch01 — 2 exercises", "## ch01 — 7 exercises");
        let f = exercises_index_failures(&page, &corpus3(), &printed2(), &veins());
        assert!(
            f.iter()
                .any(|m| m.contains("`## ch01` says 7 exercises and has 2 rows")),
            "{f:#?}"
        );
    }

    #[test]
    fn a_header_total_that_drifted_from_the_table_is_caught() {
        let page = index_page().replace("corpus: 3 total", "corpus: 331 total");
        let f = exercises_index_failures(&page, &corpus3(), &printed2(), &veins());
        assert!(
            f.iter()
                .any(|m| m.contains("header says 331 total and the table has 3")),
            "{f:#?}"
        );
    }

    #[test]
    fn one_drifted_tier_figure_is_named_and_the_sum_is_caught_separately() {
        // The wolf-book#40 shape: the totals line stayed internally
        // consistent while one figure and the sum both went stale.
        let page = index_page().replace(
            "Tier totals: 2 run (lupin) · 1 prose. That is 3.",
            "Tier totals: 1 run (lupin) · 1 prose. That is 2.",
        );
        let f = exercises_index_failures(&page, &corpus3(), &printed2(), &veins());
        assert!(
            f.iter()
                .any(|m| m.contains("says 1 `run (lupin)` and the table has 2")),
            "{f:#?}"
        );
        assert!(
            f.iter()
                .any(|m| m.contains("sums to 2 and the table has 3")),
            "{f:#?}"
        );
    }

    #[test]
    fn a_wrapped_tally_is_read_past_its_first_figure() {
        // The taxonomy line wraps to the page width. Reading one line of
        // it found `fingers` and called the other four missing.
        let t = tally_after(
            &index_page(),
            "Taxonomy spread (tags, hybrids counted once per kind): ",
        )
        .expect("the line parses");
        assert_eq!(t.get("comprehension"), Some(&1));
        assert_eq!(t.get("design"), Some(&1));
        assert_eq!(t.len(), 5);
    }

    #[test]
    fn a_section_cell_carrying_its_own_pipe_is_still_one_row() {
        // ``§6.2 — `?`, `else`, `else |err|` `` — read the cells from the
        // right or this row vanishes from the count.
        let cells = index_row_cells(
            "| §6.2 — `?`, `else`, `else |err|` | 6-14 | comprehension · lupin | run (lupin) |",
        )
        .expect("row parses");
        assert_eq!(cells.0, "6-14");
        assert_eq!(cells.2, "run (lupin)");
    }

    #[test]
    fn an_annotated_id_cell_still_names_its_exercise() {
        let cells = index_row_cells(
            "| §13.1 — `par` (held) | 13-2 → printed in §13.2 | fingers · lupin | run (lupin) |",
        )
        .expect("row parses");
        assert_eq!(cells.0, "13-2");
    }

    #[test]
    fn a_hybrid_counts_once_per_kind_and_a_vein_is_not_a_kind() {
        let v = veins();
        let (kinds, unknown) = row_kinds("comprehension + extension · lupin", &v);
        assert_eq!(kinds.len(), 2);
        assert!(unknown.is_empty());
        // EXERCISES.md §3: a kind joined to a vein with `+` reads as the
        // parenthetical form, so only the kind counts.
        let (kinds, unknown) = row_kinds("comprehension + schedule play · lupin", &v);
        assert_eq!(kinds.into_iter().collect::<Vec<_>>(), vec!["comprehension"]);
        assert!(unknown.is_empty());
        let (kinds, unknown) = row_kinds("extension (break-it-on-purpose) · lupin", &v);
        assert_eq!(kinds.into_iter().collect::<Vec<_>>(), vec!["extension"]);
        assert!(unknown.is_empty());
    }

    #[test]
    fn a_taxonomy_token_that_is_neither_kind_nor_vein_is_caught() {
        let (_, unknown) = row_kinds("fingerz · lupin", &veins());
        assert_eq!(unknown, vec!["fingerz".to_string()]);
    }

    #[test]
    fn a_stale_printed_count_is_caught() {
        let page = index_page().replace("**2 of the 3 are printed**", "**1 of the 3 are printed**");
        let f = exercises_index_failures(&page, &corpus3(), &printed2(), &veins());
        assert!(
            f.iter()
                .any(|m| m.contains("says 1 are printed") && m.contains("print 2")),
            "{f:#?}"
        );
    }

    #[test]
    fn the_real_index_agrees_with_the_real_corpus() {
        // The check as CI runs it, against the repository's own files.
        let root = crate::repo_root().expect("repo root");
        let mut failures = Vec::new();
        verify_exercises_index(&root, &mut failures).expect("check runs");
        assert!(failures.is_empty(), "{failures:#?}");
    }

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

    // ------------------------------------------------ Appendix C's counts

    /// A miniature Appendix C: a two-sentence opening and a three-row
    /// table, shaped exactly like the real page.
    const APPENDIX_C: &str = "# Appendix C — Diagnostics

Every diagnostic the book shows. The catalog holds 4 codes; these 3 are
the ones a page in this edition names.

| Code | What it says | Shown by | Sections |
|------|--------------|----------|----------|
| `E0001` | a thing | wolf | 1.1 |
| `E0005` | another thing | lupin | 2.2 |
| `W0603` | a lint | wolf | 6.1 |
";

    fn catalog4() -> Vec<&'static str> {
        vec!["E0001", "E0005", "E0409", "W0603"]
    }

    #[test]
    fn appendix_c_control_passes() {
        assert!(appendix_c_count_failures(&catalog4(), APPENDIX_C).is_empty());
    }

    /// The planted defect this gate exists for, in the direction that
    /// actually happened: a code retires upstream, the catalog is one
    /// shorter, and the sentence still says the old number. Green for
    /// the whole 0.2 line before wolf-book#7.
    #[test]
    fn a_catalog_that_shrank_under_the_sentence_is_caught() {
        let catalog = vec!["E0001", "E0005", "W0603"];
        let failures = appendix_c_count_failures(&catalog, APPENDIX_C);
        assert_eq!(failures.len(), 1, "{failures:#?}");
        assert!(
            failures[0].contains("The catalog holds 3 codes"),
            "{failures:#?}"
        );
    }

    /// The other half of the same sentence: a row joins the table and
    /// nobody re-counts it.
    #[test]
    fn a_table_row_added_without_recounting_is_caught() {
        let page = APPENDIX_C.replace(
            "| `E0001` | a thing | wolf | 1.1 |",
            "| `E0001` | a thing | wolf | 1.1 |\n| `E0409` | a fourth | wolf | 5.1 |",
        );
        let failures = appendix_c_count_failures(&catalog4(), &page);
        assert_eq!(failures.len(), 1, "{failures:#?}");
        assert!(failures[0].contains("these 4 are"), "{failures:#?}");
    }

    #[test]
    fn the_count_sentence_survives_the_page_wrap() {
        // The real page wraps the sentence across two lines; the check
        // has to collapse whitespace or it passes a wrong number.
        let page = APPENDIX_C.replace(
            "The catalog holds 4 codes; these 3 are
the ones",
            "The catalog
holds 4 codes; these 3 are the
ones",
        );
        assert!(appendix_c_count_failures(&catalog4(), &page).is_empty());
    }

    #[test]
    fn the_table_reader_takes_the_first_cell_only() {
        // A section cell naming a code must not be counted as a row.
        let page = APPENDIX_C.replace(
            "| `W0603` | a lint | wolf | 6.1 |",
            "| `W0603` | a lint, see `E0409` | wolf | 6.1 |",
        );
        assert_eq!(appendix_c_table_codes(&page).len(), 3);
    }

    #[test]
    fn the_real_appendix_c_agrees_with_the_vendored_catalog() {
        // The check as CI runs it, against the repository's own files.
        let root = crate::repo_root().expect("repo root");
        let mut failures = Vec::new();
        verify_diagnostic_codes(&root, &mut failures).expect("check runs");
        assert!(failures.is_empty(), "{failures:#?}");
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

    // --- wolf-book#7 part 2: the version-literal audit ---

    fn pins() -> (String, String) {
        ("0.2.9+dev.e0ce018".to_string(), "0.1.30".to_string())
    }

    fn seen(pairs: &[(&str, &str, usize)]) -> BTreeMap<(String, String), usize> {
        pairs
            .iter()
            .map(|(p, l, n)| ((p.to_string(), l.to_string()), *n))
            .collect()
    }

    const ALLOW: &str = "\
# a comment and a blank line are skipped

book/ch01.md 0.2.9 3 audited-at-wolf=0.2.9+dev.e0ce018
book/ch01.md 0.1.30 1 audited-at-lupin=0.1.30
book/ch23.md v1.4.0 2 not-a-toolchain-version
";

    #[test]
    fn the_version_literal_control_passes() {
        let f = version_literal_failures(
            &seen(&[
                ("book/ch01.md", "0.2.9", 3),
                ("book/ch01.md", "0.1.30", 1),
                ("book/ch23.md", "v1.4.0", 2),
            ]),
            ALLOW,
            &pins(),
        );
        assert!(f.is_empty(), "{f:#?}");
    }

    #[test]
    fn a_pin_that_moved_past_an_audit_is_caught() {
        // THE DEFECT THIS GATE EXISTS FOR, planted: the compiler pin
        // bumps and §1.2's archive names are not re-read. Five bumps
        // running healed these by hand.
        let moved = ("0.2.10".to_string(), "0.1.30".to_string());
        let f = version_literal_failures(
            &seen(&[
                ("book/ch01.md", "0.2.9", 3),
                ("book/ch01.md", "0.1.30", 1),
                ("book/ch23.md", "v1.4.0", 2),
            ]),
            ALLOW,
            &moved,
        );
        assert_eq!(f.len(), 1, "{f:#?}");
        assert!(f[0].contains("the wolf pin is now 0.2.10"), "{f:#?}");
        assert!(f[0].contains("re-stamp its audited-at-wolf"), "{f:#?}");
    }

    #[test]
    fn an_interpreter_only_bump_re_reads_the_interpreter_sentences() {
        // The one clock wolf-web#8 found missing: keying everything to
        // the compiler means a lupin bump re-reads nothing.
        let moved = ("0.2.9+dev.e0ce018".to_string(), "0.1.31".to_string());
        let f = version_literal_failures(
            &seen(&[
                ("book/ch01.md", "0.2.9", 3),
                ("book/ch01.md", "0.1.30", 1),
                ("book/ch23.md", "v1.4.0", 2),
            ]),
            ALLOW,
            &moved,
        );
        assert_eq!(f.len(), 1, "{f:#?}");
        assert!(f[0].contains("audited at lupin 0.1.30"), "{f:#?}");
    }

    /// bs38: one (page, literal) pair can be a claim about BOTH
    /// projects. §1.2 says `0.1.31` once about the interpreter the
    /// COMPILER names and once about the interpreter this book PINS.
    /// With one clock per entry the pair rides one of them and the
    /// other project's bump leaves the sentence unread.
    const ALLOW_TWO_CLOCKS: &str = "\
book/ch01.md 0.1.31 2 audited-at-wolf=0.2.10 audited-at-lupin=0.1.31
";

    #[test]
    fn a_two_clock_entry_holds_when_neither_pin_moved() {
        let f = version_literal_failures(
            &seen(&[("book/ch01.md", "0.1.31", 2)]),
            ALLOW_TWO_CLOCKS,
            &("0.2.10".to_string(), "0.1.31".to_string()),
        );
        assert!(f.is_empty(), "{f:#?}");
    }

    #[test]
    fn a_two_clock_entry_is_re_read_when_either_pin_moves() {
        let found = seen(&[("book/ch01.md", "0.1.31", 2)]);
        let compiler_moved = version_literal_failures(
            &found,
            ALLOW_TWO_CLOCKS,
            &("0.2.11".to_string(), "0.1.31".to_string()),
        );
        assert_eq!(compiler_moved.len(), 1, "{compiler_moved:#?}");
        assert!(
            compiler_moved[0].contains("re-stamp its audited-at-wolf"),
            "{compiler_moved:#?}"
        );
        let interpreter_moved = version_literal_failures(
            &found,
            ALLOW_TWO_CLOCKS,
            &("0.2.10".to_string(), "0.1.32".to_string()),
        );
        assert_eq!(interpreter_moved.len(), 1, "{interpreter_moved:#?}");
        assert!(
            interpreter_moved[0].contains("re-stamp its audited-at-lupin"),
            "{interpreter_moved:#?}"
        );
        let both_moved = version_literal_failures(
            &found,
            ALLOW_TWO_CLOCKS,
            &("0.2.11".to_string(), "0.1.32".to_string()),
        );
        assert_eq!(both_moved.len(), 2, "{both_moved:#?}");
    }

    #[test]
    fn one_clock_named_twice_is_a_parse_failure() {
        let f = version_literal_failures(
            &seen(&[("book/ch01.md", "0.1.31", 2)]),
            "book/ch01.md 0.1.31 2 audited-at-wolf=0.2.10 audited-at-wolf=0.2.10\n",
            &("0.2.10".to_string(), "0.1.31".to_string()),
        );
        assert!(
            f.iter()
                .any(|m| m.contains("the same clock is named twice")),
            "{f:#?}"
        );
    }

    #[test]
    fn not_a_toolchain_version_stands_alone() {
        let f = version_literal_failures(
            &seen(&[("book/ch23.md", "v1.4.0", 2)]),
            "book/ch23.md v1.4.0 2 not-a-toolchain-version audited-at-wolf=0.2.10\n",
            &pins(),
        );
        assert!(
            f.iter()
                .any(|m| m.contains("`not-a-toolchain-version` stands alone")),
            "{f:#?}"
        );
    }

    #[test]
    fn a_new_literal_no_one_classified_is_caught() {
        let f = version_literal_failures(
            &seen(&[
                ("book/ch01.md", "0.2.9", 3),
                ("book/ch01.md", "0.1.30", 1),
                ("book/ch23.md", "v1.4.0", 2),
                ("book/ch07.md", "0.3.0", 1),
            ]),
            ALLOW,
            &pins(),
        );
        assert_eq!(f.len(), 1, "{f:#?}");
        assert!(f[0].contains("`0.3.0` x1 is not in"), "{f:#?}");
    }

    #[test]
    fn a_count_that_drifted_and_a_literal_that_left_are_both_caught() {
        let f = version_literal_failures(
            &seen(&[("book/ch01.md", "0.2.9", 2), ("book/ch01.md", "0.1.30", 1)]),
            ALLOW,
            &pins(),
        );
        assert_eq!(f.len(), 2, "{f:#?}");
        assert!(f
            .iter()
            .any(|m| m.contains("appears x2, audit/version-literals.txt says x3")));
        assert!(f.iter().any(|m| m.contains("which no longer carries it")));
    }

    #[test]
    fn a_not_a_toolchain_entry_rides_no_clock() {
        let moved = ("9.9.9".to_string(), "9.9.9".to_string());
        let f = version_literal_failures(
            &seen(&[("book/ch23.md", "v1.4.0", 2)]),
            "book/ch23.md v1.4.0 2 not-a-toolchain-version\n",
            &moved,
        );
        assert!(f.is_empty(), "{f:#?}");
    }

    #[test]
    fn an_unparsable_entry_says_the_shape() {
        let f = version_literal_failures(
            &seen(&[]),
            "book/ch01.md 0.2.9 3 audited-at-rust=1.0\n",
            &pins(),
        );
        assert_eq!(f.len(), 1, "{f:#?}");
        assert!(f[0].contains("names no clock"), "{f:#?}");
    }

    #[test]
    fn the_scanner_reads_the_shapes_the_book_carries_and_not_the_ones_it_does_not() {
        let text = "\
`0.2.9+dev.e0ce018` is a build made after the 0.2.9 release, and lupin 0.1.30 sits beside it.
tar -xzf wolf-0.2.9-x86_64-pc-windows-msvc.tar.gz
a tag `v1.4.0`, a slice `318..322`, a range `0..100`, an address 127.0.0.1:0, and 0.5 seconds
";
        let mut got = version_literals(text);
        got.sort();
        assert_eq!(
            got,
            vec!["0.1.30", "0.2.9", "0.2.9", "0.2.9+dev.e0ce018", "v1.4.0"],
            "a slice range, an IPv4 address and a two-part number are not version claims"
        );
    }

    #[test]
    fn the_audited_text_keeps_a_text_fence_and_drops_the_replayed_ones() {
        let page = "\
prose 1.2.3
<!-- ledger 4.5.6 -->
```console
$ wolf --version
wolf 7.8.9
```
```text
tar -xzf wolf-1.2.3-x86_64-pc-windows-msvc.tar.gz
```
```wolf,run(exit=0)
// 9.9.9
```
";
        let mut got = version_literals(&auditable_text(page));
        got.sort();
        assert_eq!(got, vec!["1.2.3", "1.2.3"]);
    }

    #[test]
    fn number_words_cover_the_counts_the_page_uses() {
        assert_eq!(number_word(11), Some("eleven"));
        assert_eq!(number_word(12), Some("twelve"));
        assert_eq!(number_word(21), None);
    }

    // ---- wolf-book#5: interpolation the vendored grammar can paint ----

    #[test]
    fn interpolation_control_passes() {
        // Every shape the book actually prints, in one listing.
        let code = concat!(
            "fn main() -> !int {\n",
            "    print(\"{\"total\":<10}{total:>6}\")\n",
            "    print(\"{audit(\"340\\n275\")}\")\n",
            "    print(\"[{out.replace(\"\\t\", \"<tab>\")}]\")\n",
            "    (mut cases).push(r\"select { (a from q => { ) }\")\n",
            "    0\n",
            "}\n",
        );
        assert!(
            unpaintable_nested_literals(code).is_empty(),
            "{:#?}",
            unpaintable_nested_literals(code)
        );
    }

    /// THE DEFECT THIS GATE EXISTS FOR, planted in the shape it had:
    /// exercise 2-9 printed `"e\u{301}"` from inside a hole for six
    /// pins, and the `{` of the escape closed the hole. bs44 bound the
    /// string to a `let` and added this check in the same commit.
    #[test]
    fn a_brace_in_a_nested_literal_is_caught() {
        let code = "    print(\"{\"e\\u{301}\".len} {\"e\\u{301}\".chars().len}\")\n";
        let hits = unpaintable_nested_literals(code);
        assert_eq!(hits.len(), 1, "{hits:#?}");
        assert_eq!(hits[0].0, 1);
        assert!(hits[0].1.contains("carries `{`"), "{hits:#?}");
    }

    /// The other half, and the one wolf-book#5 wrote down: an escaped
    /// quote. The closed form's class excludes `"`, so the match ends
    /// at the escape and everything after it leaves the string scope.
    #[test]
    fn an_escaped_quote_in_a_nested_literal_is_caught() {
        let code = "    print(\"{f(\"a\\\"b\")}\")\n";
        let hits = unpaintable_nested_literals(code);
        assert_eq!(hits.len(), 1, "{hits:#?}");
        assert!(hits[0].1.contains("escaped quote"), "{hits:#?}");
    }

    /// Deliberately NOT a hit: a plain escape is inside the character
    /// class, the literal matches whole and paints as a string. Only
    /// the escape sequence goes unpainted, which is a shade and not a
    /// scope — and ch04 has printed one for a year.
    #[test]
    fn a_plain_escape_in_a_nested_literal_is_not_caught() {
        let code = "    print(\"{audit(\"340\\n275\")}\")\n";
        assert!(unpaintable_nested_literals(code).is_empty());
    }

    /// A raw string is not interpolation and its braces are not holes.
    #[test]
    fn a_raw_string_is_not_scanned() {
        let code = "    (mut cases).push(r\"select { (a from q => { ) }\")\n";
        assert!(unpaintable_nested_literals(code).is_empty());
    }

    #[test]
    fn the_real_book_paints_every_nested_literal() {
        let root = crate::repo_root().unwrap();
        let mut failures = Vec::new();
        verify_paintable_interpolation(&root, &mut failures).unwrap();
        assert!(failures.is_empty(), "{failures:#?}");
    }
}
