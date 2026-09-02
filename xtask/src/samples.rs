//! `cargo xtask samples` — the book cannot rot.
//!
//! Walks two sample sources: the exercise corpus (directive-headed
//! `.lu` files under `principles/exercises/` — 170 files, the first
//! real cargo) and every fenced wolf block in `book/**/*.md`. Each
//! sample is executed against the pinned tools and its directive is
//! enforced. Diagnostics from `fail(…)` samples are snapshot-checked
//! under `snapshots/diagnostics/`. Samples named in
//! `samples-pending.toml` run report-only: their directives are
//! *expected* to fail today, and a sample that starts passing is a
//! flip — a hard error until the manifest row is removed, so a feature
//! landing is noticed, never silently absorbed.

use crate::console::ConsoleBlock;
use crate::directives::{parse_fence_info, parse_lu_header, Check};
use crate::fence::{segments, Segment};
use anyhow::{bail, Context, Result};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

const SAMPLE_TIMEOUT: Duration = Duration::from_secs(20);

#[derive(Debug, Clone, PartialEq)]
enum Origin {
    Corpus,
    Book { md: PathBuf },
}

#[derive(Debug, Clone)]
struct Sample {
    /// e.g. `ch03/ex3-2`, `ch22/tangle/main`, `book/front/notation/s1`
    id: String,
    dir: PathBuf,
    file_name: String,
    check: Check,
    origin: Origin,
}

#[derive(Debug)]
struct Outcome {
    passed: bool,
    detail: String,
    /// The exit status this sample's own process carried, where the
    /// sample IS one process (`run`, `wolf-run`, `run(exit=trap(…))`).
    /// `None` where the verdict comes from `wolf conform-run` instead,
    /// which reports a verdict rather than an exit.
    exit: Option<i32>,
    /// That process's stderr, verbatim — what a declared refusal in
    /// `samples-os.toml` is compared against.
    stderr: String,
    /// Rendered diagnostic text (fail samples), for snapshots and
    /// `diagnostic,from(…)` cross-checks.
    diagnostic: Option<String>,
    /// phase_reached reported by wolf, for corpus-export headers.
    phase_reached: Option<String>,
}

pub struct Tools {
    pub lupin: PathBuf,
    pub wolf: PathBuf,
}

pub fn locate_tools(root: &Path) -> Result<Tools> {
    let exe = |base: &str| {
        if cfg!(windows) {
            format!("{base}.exe")
        } else {
            base.to_string()
        }
    };
    let from_env = |var: &str| std::env::var_os(var).map(PathBuf::from);
    let sibling = |repo: &str, rel: &str, name: &str| {
        root.parent()
            .map(|p| p.join(repo).join(rel).join(exe(name)))
    };
    let lupin = from_env("LUPIN")
        .or_else(|| sibling("wolf-interp", "target/release", "lupin"))
        .filter(|p| p.is_file());
    // EXERCISES.md §5 pins "the wolf-lang debug build": conformance
    // diagnostics live on trunk's debug profile until s31's driver ships.
    let wolf = from_env("WOLF")
        .or_else(|| sibling("wolf-lang", "target/debug", "wolf"))
        .filter(|p| p.is_file());
    match (lupin, wolf) {
        (Some(lupin), Some(wolf)) => Ok(Tools { lupin, wolf }),
        (l, w) => bail!(
            "pinned tools not found (lupin: {}, wolf: {}) — build them at the \
             wolf-toolchain.toml revs or point LUPIN/WOLF at the binaries",
            l.map(|p| p.display().to_string())
                .unwrap_or_else(|| "missing".into()),
            w.map(|p| p.display().to_string())
                .unwrap_or_else(|| "missing".into()),
        ),
    }
}

#[derive(Debug, serde::Deserialize)]
struct PendingManifest {
    #[serde(default)]
    pending: Vec<PendingRow>,
}

#[derive(Debug, serde::Deserialize)]
struct PendingRow {
    id: String,
    blocker: String,
    owner: String,
}

pub fn run(root: &Path, args: &[String]) -> Result<()> {
    let bless = args.iter().any(|a| a == "--bless");
    let self_test = args.iter().any(|a| a == "--self-test");
    let filter = args
        .iter()
        .position(|a| a == "--filter")
        .and_then(|i| args.get(i + 1))
        .cloned();

    let tools = locate_tools(root)?;
    println!("samples: lupin = {}", tools.lupin.display());
    println!("samples: wolf  = {}", tools.wolf.display());
    println!(
        "samples: SKIP extraction book/back/solutions.md — generated from the exercise \
         corpus, which this run executes directly (cargo xtask backmatter --check holds \
         the page to it)"
    );

    if self_test {
        return selftest(root, &tools);
    }

    let pending: BTreeMap<String, PendingRow> = {
        let path = root.join("samples-pending.toml");
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        let manifest: PendingManifest = toml::from_str(&text)?;
        manifest
            .pending
            .into_iter()
            .map(|r| (r.id.clone(), r))
            .collect()
    };

    // The per-host ledger. Rows for other hosts are inert here; every
    // row's subject is still checked for existence on every lane.
    let os_ledger = crate::oslane::Ledger::load(root)?;

    // ---- Collect ----
    let (corpus, member_count) = collect_corpus(root)?;
    let book = collect_book(root)?;
    let (repl_blocks, diag_checks) = (book.repl_blocks, book.diag_checks);
    let file_checks = book.file_checks;
    let console_blocks = book.console_blocks;
    let programs = book.programs;
    let mut samples = corpus;
    samples.extend(book.samples);
    if let Some(f) = &filter {
        samples.retain(|s| s.id.contains(f.as_str()));
    }

    // ---- Execute ----
    let mut failures: Vec<String> = book.lint_failures;
    let mut flips: Vec<String> = Vec::new();
    let mut pass = 0usize;
    let mut pending_seen = 0usize;
    let mut refused_seen = 0usize;
    let mut diagnostics: BTreeMap<String, String> = BTreeMap::new();
    let mut phases: BTreeMap<String, String> = BTreeMap::new();
    let started = Instant::now();

    for s in &samples {
        let outcome = execute(&tools, s).with_context(|| format!("running sample {}", s.id))?;
        if let Some(d) = &outcome.diagnostic {
            diagnostics.insert(s.id.clone(), d.clone());
        }
        if let Some(p) = &outcome.phase_reached {
            phases.insert(s.id.clone(), p.clone());
        }
        let is_pending = pending.contains_key(&s.id);
        // A declared per-host refusal answers first: on THIS host the
        // sample is expected to be refused, by name, in exactly these
        // words. Enforced both ways — wrong words fail, a pass flips.
        if let Some(row) = os_ledger.refusal(&s.id) {
            if is_pending {
                failures.push(format!(
                    "{}: named by both samples-pending.toml and samples-os.toml — \
                     a sample is pending everywhere or refused on one host, not both",
                    s.id
                ));
            } else if outcome.passed {
                flips.push(format!(
                    "{}: samples-os.toml declares `{}` refused on {} at this pin, and it \
                     now PASSES — the host grew the feature; remove the row in the \
                     pin-bump commit (it was retiring at {})",
                    s.id, s.check, row.os, row.retires
                ));
            } else if outcome.exit == Some(row.exit) && outcome.stderr.trim_end() == row.stderr {
                refused_seen += 1;
                println!(
                    "samples: REFUSED({}) {} [{}] — declared: exit {}, retires at {} ({})",
                    row.os, s.id, s.check, row.exit, row.retires, row.note
                );
            } else {
                failures.push(format!(
                    "{}: the {} refusal drifted from samples-os.toml — the row is the \
                     book's claim about this host and it is now wrong\n     \
                     declared: exit {}\n       {}\n     actual: exit {:?}\n       {}",
                    s.id,
                    row.os,
                    row.exit,
                    row.stderr,
                    outcome.exit,
                    if outcome.stderr.trim().is_empty() {
                        "<no stderr captured — this check reports a verdict, not one \
                         process's exit; a refusal row here needs execute() to \
                         say which machine's stderr it means>"
                            .to_string()
                    } else {
                        outcome.stderr.trim_end().to_string()
                    },
                ));
            }
            continue;
        }
        match (outcome.passed, is_pending) {
            (true, false) => {
                // Snapshot check for every sample whose subject is a
                // tool's own text: static rejections, checked-build UB
                // findings, and audit-surface's manifest rule.
                let snapshotted = matches!(
                    s.check,
                    Check::Fail { .. } | Check::Ub { .. } | Check::Audit { .. }
                );
                if let (true, Some(diag)) = (snapshotted, &outcome.diagnostic) {
                    match check_snapshot(root, &s.id, diag, bless)? {
                        SnapshotResult::Ok => {}
                        SnapshotResult::Blessed => {
                            println!("samples: blessed snapshot for {}", s.id)
                        }
                        SnapshotResult::Mismatch(msg) => {
                            failures.push(format!("{}: {msg}", s.id));
                            continue;
                        }
                    }
                }
                pass += 1;
            }
            (false, false) => {
                failures.push(format!("{}: {}", s.id, outcome.detail));
            }
            (false, true) => {
                pending_seen += 1;
                let row = &pending[&s.id];
                println!(
                    "samples: PENDING {} [{}] — blocker: {} (owner: {})",
                    s.id, s.check, row.blocker, row.owner
                );
            }
            (true, true) => {
                flips.push(format!(
                    "{}: directive `{}` now PASSES but the sample is on \
                     samples-pending.toml (blocker was: {}) — the feature \
                     landed; remove the manifest row in the pin-bump commit",
                    s.id, s.check, pending[&s.id].blocker
                ));
            }
        }
    }

    // Pending rows that matched no sample are stale.
    for id in pending.keys() {
        if !samples.iter().any(|s| &s.id == id) && filter.is_none() {
            failures.push(format!(
                "samples-pending.toml names `{id}` but no such sample exists"
            ));
        }
    }
    // So are per-host rows — checked on every lane, not only the lane
    // the row applies to, so a typo cannot hide on two thirds of the
    // matrix.
    for id in os_ledger.all_refusal_ids() {
        if !samples.iter().any(|s| &s.id == id) && filter.is_none() {
            failures.push(format!(
                "samples-os.toml [[refusal]] names `{id}` but no such sample exists"
            ));
        }
    }

    // ---- diagnostic,from(…) cross-checks (doc truth) ----
    for (md, from_id, block_text) in &diag_checks {
        match diagnostics.get(from_id) {
            None => failures.push(format!(
                "{}: diagnostic,from({from_id}) — no fail() sample with that id ran",
                md.display()
            )),
            Some(actual) => {
                if normalize(block_text) != normalize(actual) {
                    failures.push(format!(
                        "{}: diagnostic,from({from_id}) drifted from the captured \
                         diagnostic — update the block from the real run",
                        md.display()
                    ));
                }
            }
        }
    }

    // ---- text,file(…) blocks: the page quotes a file in this repo ----
    for (md, line, path, block) in &file_checks {
        let full = root.join("samples").join(path);
        match std::fs::read_to_string(&full) {
            Err(e) => failures.push(format!(
                "{}:{line}: text,file({path}) — cannot read samples/{path}: {e}",
                md.display()
            )),
            Ok(text) => {
                if text != *block {
                    failures.push(format!(
                        "{}:{line}: text,file({path}) drifted from samples/{path} — \
                         the page and the fixture must be the same bytes",
                        md.display()
                    ));
                }
            }
        }
    }

    // ---- Console blocks (the prompt lines are output too) ----
    let console = if filter.is_none() {
        crate::console::check(root, &tools, &console_blocks, &programs, &os_ledger)?
    } else {
        crate::console::Report {
            checked: 0,
            skipped: Vec::new(),
            declared: Vec::new(),
            failures: Vec::new(),
        }
    };
    // A [[transcript]] row naming no block is stale, on every lane.
    if filter.is_none() {
        for key in os_ledger.all_transcript_blocks() {
            if !console_blocks
                .iter()
                .any(|b| &crate::console::block_key(root, b) == key)
            {
                failures.push(format!(
                    "samples-os.toml [[transcript]] names `{key}` but no console block \
                     opens there"
                ));
            }
        }
    }
    failures.extend(console.failures.iter().cloned());

    // ---- Corpus export (the rot-proofing running both ways) ----
    let exported = export_corpus(root, &samples, &phases)?;

    // ---- Report ----
    let elapsed = started.elapsed();
    println!();
    println!(
        "samples: {} samples ({} corpus roots + {} corpus members on disk, {} book blocks)",
        samples.len(),
        samples
            .iter()
            .filter(|s| s.origin == Origin::Corpus)
            .count(),
        member_count,
        samples
            .iter()
            .filter(|s| matches!(s.origin, Origin::Book { .. }))
            .count(),
    );
    println!(
        "samples: {pass} passed, {pending_seen} pending (report-only), {} failed, {} flips in {:.1}s",
        failures.len(),
        flips.len(),
        elapsed.as_secs_f64()
    );
    println!(
        "samples: {} of {} console block(s) replayed against the pinned tools",
        console.checked,
        console_blocks.len(),
    );
    if os_ledger.here() > 0 {
        println!(
            "samples: samples-os.toml: {} row(s) apply to {} — {} declared \
             refusal(s) and {} declared transcript(s) held",
            os_ledger.here(),
            crate::oslane::host_os(),
            refused_seen,
            console.declared.len(),
        );
    }
    for d in &console.declared {
        println!("samples: DECLARED console {d}");
    }
    for s in &console.skipped {
        println!("samples: SKIP console {s}");
    }
    if repl_blocks > 0 {
        println!(
            "samples: SKIP pending(is08): {repl_blocks} wolf-repl block(s) counted, \
             not replayed — the REPL replay lane lands with wolf-interp is08"
        );
    }
    println!("samples: exported {exported} corpus programs to samples/export/corpus/book/");

    for f in &failures {
        eprintln!("samples: FAIL {f}");
    }
    for f in &flips {
        eprintln!("samples: FLIP {f}");
    }
    if !failures.is_empty() || !flips.is_empty() {
        bail!("{} failure(s), {} flip(s)", failures.len(), flips.len());
    }
    Ok(())
}

fn normalize(s: &str) -> String {
    s.lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .to_string()
}

// ---------------------------------------------------------------- corpus

fn collect_corpus(root: &Path) -> Result<(Vec<Sample>, usize)> {
    let base = root.join("principles/exercises");
    let mut lu_files = Vec::new();
    walk(&base, &mut |p| {
        if p.extension().and_then(|e| e.to_str()) == Some("lu") {
            lu_files.push(p.to_path_buf());
        }
    })?;
    lu_files.sort();
    let mut samples = Vec::new();
    let mut members = 0usize;
    for path in &lu_files {
        let source =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let header = parse_lu_header(&source)
            .with_context(|| format!("parsing directives in {}", path.display()))?;
        if header.member {
            members += 1;
            continue;
        }
        let check = header.check.with_context(|| {
            format!(
                "{}: corpus file has no `//! check:` directive",
                path.display()
            )
        })?;
        let rel = path.strip_prefix(&base).unwrap();
        let id = rel.with_extension("").to_string_lossy().replace('\\', "/");
        samples.push(Sample {
            id,
            dir: path.parent().unwrap().to_path_buf(),
            file_name: path.file_name().unwrap().to_string_lossy().into_owned(),
            check,
            origin: Origin::Corpus,
        });
    }
    Ok((samples, members))
}

// ------------------------------------------------------------------ book

type DiagCheck = (PathBuf, String, String);

/// What one pass over `book/**/*.md` yields: the executable samples, the
/// count of REPL transcripts (counted, replayed at is08), the
/// `diagnostic,from(…)` cross-checks, the console blocks (replayed
/// against the pinned tools), and the Part-1 lint's complaints.
struct BookScan {
    samples: Vec<Sample>,
    repl_blocks: usize,
    diag_checks: Vec<DiagCheck>,
    console_blocks: Vec<ConsoleBlock>,
    /// Sample id → the program as printed, for `console,from(id)`.
    programs: BTreeMap<String, String>,
    /// `text,file(path)` blocks: (markdown, line, path, block content).
    file_checks: Vec<(PathBuf, usize, String, String)>,
    lint_failures: Vec<String>,
}

/// The ownership vocabulary Part 1 does not teach (bs02 acceptance 4).
/// A reader finishes Part 1 writing real single-threaded wolf without
/// having met any of these; the extractor is what keeps that promise
/// honest, because prose can drift and a sample cannot.
const OWNERSHIP_TOKENS: [&str; 4] = ["mut", "take", "region", "shared"];

/// Which book files belong to Part 1, read from SUMMARY.md so the rule
/// follows the structure rather than a hardcoded chapter range.
fn part1_files(root: &Path) -> Result<std::collections::BTreeSet<PathBuf>> {
    Ok(crate::render::parse_summary(root)?
        .into_iter()
        .filter(|e| e.part.as_deref().is_some_and(|p| p.starts_with("Part 1")))
        .map(|e| e.path)
        .collect())
}

/// Ownership annotations in a block. Matched as whole tokens, so `taken`
/// and `mutation` stay ordinary identifiers, and never after a `.`:
/// an ownership annotation is a prefix keyword (`take p.lead`), while
/// `xs.take(3)` is the iterator combinator that happens to share the
/// word. The two are distinguishable only by position — a collision
/// worth knowing about, since it is also what a human grepping for a
/// codebase's move surface has to filter by hand.
///
/// One deliberate exception (bs13): the write-marked receiver,
/// `(mut xs).push(…)`. Since the wolf-lang/wolf-interp pin at a900b8c /
/// lupin 0.1.14, BOTH machines enforce X1's call-site mode on method
/// receivers — the compiler statically (E0804) and the interpreter with
/// `trap(exclusivity)` — so a Part-1 program that builds a `List` cannot
/// be written without it. Part 1 teaches it as "this call writes `xs`"
/// (§3.1) and defers the why to chapter 7; every other position of
/// `mut`, and all of `take`/`region`/`shared`, stay out of Part 1.
fn ownership_annotations(program: &str) -> Vec<&'static str> {
    let bytes = program.as_bytes();
    let is_word = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    // `(mut xs).push(…)` / `(mut cents).pop()`: `mut` opening a
    // parenthesized receiver that is immediately called through. The
    // declaration-mode `fn f(mut x: int)` also follows a `(`, but its
    // paren group is never followed by `.`, which is what tells the
    // two apart here just as it does in the grammar.
    let is_write_marked_receiver = |at: usize| {
        if bytes.get(at.wrapping_sub(1)) != Some(&b'(') {
            return false;
        }
        match program[at..].find(')') {
            Some(rel) => bytes.get(at + rel + 1) == Some(&b'.'),
            None => false,
        }
    };
    let mut hits: Vec<&'static str> = Vec::new();
    for tok in OWNERSHIP_TOKENS {
        let mut from = 0usize;
        while let Some(rel) = program[from..].find(tok) {
            let at = from + rel;
            from = at + tok.len();
            let before = at.checked_sub(1).map(|i| bytes[i]);
            let after = bytes.get(at + tok.len()).copied();
            if before.is_some_and(is_word) || after.is_some_and(is_word) {
                continue; // part of a longer identifier
            }
            if before == Some(b'.') {
                continue; // a method call, not an annotation
            }
            if tok == "mut" && is_write_marked_receiver(at) {
                continue; // the write-marked receiver Part 1 teaches
            }
            hits.push(tok);
            break;
        }
    }
    hits
}

fn collect_book(root: &Path) -> Result<BookScan> {
    let base = root.join("book");
    let part1 = part1_files(root)?;
    let mut lint_failures: Vec<String> = Vec::new();
    let mut md_files = Vec::new();
    walk(&base, &mut |p| {
        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
        // `_template.md` and friends are scaffolding, not book content:
        // their blocks are illustrative placeholders, never executed.
        // `solutions.md` is generated from the exercise corpus, and the
        // corpus is what this runner already executes — extracting it a
        // second time would run every solution twice under a second id.
        // The generator holds the page to its sources
        // (`cargo xtask backmatter --check`).
        if p.extension().and_then(|e| e.to_str()) == Some("md")
            && name != "SUMMARY.md"
            && name != "solutions.md"
            && !name.starts_with('_')
        {
            md_files.push(p.to_path_buf());
        }
    })?;
    md_files.sort();

    let extract_dir = root.join("samples/extracted");
    let _ = std::fs::remove_dir_all(&extract_dir);

    let mut samples = Vec::new();
    let mut repl_blocks = 0usize;
    let mut diag_checks = Vec::new();
    let mut console_blocks = Vec::new();
    let mut file_checks: Vec<(PathBuf, usize, String, String)> = Vec::new();
    let mut programs: BTreeMap<String, String> = BTreeMap::new();

    for md in &md_files {
        let source = std::fs::read_to_string(md)?;
        let rel = md.strip_prefix(&base).unwrap().with_extension("");
        let stem = rel.to_string_lossy().replace(['\\', '/'], "-");
        let out_dir = extract_dir.join(&stem);
        // Parts accumulate per markdown file.
        let mut parts: BTreeMap<String, String> = BTreeMap::new();
        let mut counter = 0usize;
        // The program a console block below is talking about: the last
        // wolf block on the page, which is how the chapters read.
        let mut last_program: Option<String> = None;
        for seg in segments(&source) {
            let Segment::Fence(f) = seg else { continue };
            if f.info.trim().is_empty() {
                continue;
            }
            let fi = parse_fence_info(&f.info)
                .with_context(|| format!("{}: bad fence info `{}`", md.display(), f.info))?;
            // A fence carrying `file(path)` quotes a file in this
            // repository — a member of a multi-file project, which is
            // not a standalone program and must not be compiled as one.
            // CI holds the page and the file to the same bytes.
            if let Some(path) = &fi.file {
                file_checks.push((md.clone(), f.open_line + 1, path.clone(), f.content.clone()));
                continue;
            }
            match fi.lang.as_str() {
                "wolf-repl" => {
                    repl_blocks += 1;
                    continue;
                }
                "diagnostic" => {
                    if let Some(from) = fi.from {
                        diag_checks.push((md.clone(), from, f.content.clone()));
                    }
                    continue;
                }
                "console" => {
                    console_blocks.push(ConsoleBlock {
                        stem: stem.clone(),
                        md: md.clone(),
                        line: f.open_line + 1,
                        text: f.content.clone(),
                        program: last_program.clone(),
                        from: fi.from.clone(),
                        fixture: fi.in_fixture.clone(),
                    });
                    continue;
                }
                "wolf" => {}
                _ => continue,
            }
            // Part-1 blocks carry no ownership annotations. Checked on the
            // block as written, so the failure names the block the author
            // has to fix rather than a stitched whole.
            if part1.contains(md) {
                for tok in ownership_annotations(&f.content) {
                    lint_failures.push(format!(
                        "{}:{}: Part-1 sample carries the ownership annotation `{tok}` — \
                         Part 1 teaches none of `mut`/`take`/`region`/`shared` (bs02 \
                         acceptance 4, narrowed at bs13: the write-marked receiver \
                         `(mut xs).push(…)` is the one permitted form, because both \
                         machines enforce it at the pin); reshape the sample or move \
                         the material",
                        md.display(),
                        f.open_line + 1,
                    ));
                }
            }
            // wolf block: stitch parts, then decide whether it runs.
            let (program, name) = match &fi.part {
                Some((name, cont)) => {
                    let acc = parts.entry(name.clone()).or_default();
                    if !cont && !acc.is_empty() {
                        bail!(
                            "{}:{}: part({name}) restarted without cont",
                            md.display(),
                            f.open_line + 1
                        );
                    }
                    acc.push_str(&f.content);
                    (parts[name].clone(), format!("part-{name}"))
                }
                None => {
                    counter += 1;
                    (f.content.clone(), format!("s{counter}"))
                }
            };
            let check = match fi.check {
                Some(c) => c,
                None => {
                    if fi.part.is_some() {
                        // Not the final slice yet: nothing to run.
                        continue;
                    }
                    Check::Compile
                }
            };
            last_program = Some(program.clone());
            // One directory per sample. Files in one directory are one
            // module (D32), so two extracted blocks side by side collide
            // on `main` the moment either declares a `struct` — which is
            // every chapter from Part 2 on. The selftest has always
            // isolated its cases for this reason; book blocks need the
            // same isolation.
            let sample_dir = out_dir.join(&name);
            std::fs::create_dir_all(&sample_dir)?;
            let file_name = format!("{name}.lu");
            let mut headed = format!(
                "//! check: {}\n",
                check
                    .corpus_directive()
                    .unwrap_or_else(|| check.to_string())
            );
            headed.push_str("//! phase: run\n");
            headed.push_str(&format!(
                "// EXTRACTED by `cargo xtask samples` from {} — do not edit.\n",
                md.strip_prefix(root.parent().unwrap_or(root))
                    .unwrap_or(md)
                    .display()
            ));
            headed.push_str(&program);
            std::fs::write(sample_dir.join(&file_name), &headed)?;
            programs.insert(format!("book/{stem}/{name}"), program.clone());
            samples.push(Sample {
                id: format!("book/{stem}/{name}"),
                dir: sample_dir,
                file_name,
                check,
                origin: Origin::Book { md: md.clone() },
            });
        }
    }
    Ok(BookScan {
        samples,
        repl_blocks,
        diag_checks,
        console_blocks,
        file_checks,
        programs,
        lint_failures,
    })
}

fn walk(dir: &Path, f: &mut impl FnMut(&Path)) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            walk(&path, f)?;
        } else {
            f(&path);
        }
    }
    Ok(())
}

// ------------------------------------------------------------- execution

fn run_with_timeout(mut cmd: Command) -> Result<(Option<i32>, String, String)> {
    use std::io::Read;
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    cmd.stdin(std::process::Stdio::null());
    let mut child = cmd.spawn().with_context(|| format!("spawning {cmd:?}"))?;
    let mut stdout_pipe = child.stdout.take().unwrap();
    let mut stderr_pipe = child.stderr.take().unwrap();
    let out_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stdout_pipe.read_to_end(&mut buf);
        buf
    });
    let err_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stderr_pipe.read_to_end(&mut buf);
        buf
    });
    let deadline = Instant::now() + SAMPLE_TIMEOUT;
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break Some(status);
        }
        if Instant::now() > deadline {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    let stdout = String::from_utf8_lossy(&out_thread.join().unwrap()).into_owned();
    let stderr = String::from_utf8_lossy(&err_thread.join().unwrap()).into_owned();
    match status {
        Some(s) => Ok((s.code(), stdout, stderr)),
        None => bail!("timed out after {SAMPLE_TIMEOUT:?}"),
    }
}

fn execute(tools: &Tools, s: &Sample) -> Result<Outcome> {
    match &s.check {
        Check::Run { exit, stdout } => {
            let mut cmd = Command::new(&tools.lupin);
            cmd.arg(&s.file_name).current_dir(&s.dir);
            let (code, out, err) = run_with_timeout(cmd)?;
            let mut passed = code == Some(*exit);
            let mut detail = format!("lupin exited {code:?}, expected {exit}");
            if passed {
                if let Some(want) = stdout {
                    let got = out.trim_end_matches('\n');
                    if got != want.as_str() {
                        passed = false;
                        detail = format!("stdout mismatch: expected {want:?}, got {got:?}");
                    }
                }
            } else if !err.trim().is_empty() {
                detail.push_str(&format!(" (stderr: {})", err.trim()));
            }
            Ok(Outcome {
                passed,
                detail,
                exit: code,
                stderr: err,
                diagnostic: None,
                phase_reached: None,
            })
        }
        Check::Trap { kind } => {
            let mut cmd = Command::new(&tools.lupin);
            cmd.arg(&s.file_name).current_dir(&s.dir);
            let (code, _out, err) = run_with_timeout(cmd)?;
            // lupin spells defined faults `trap(kind)`; the UB checker's
            // faults come out as `file: ub(clause)` — the corpus directive
            // for those is `run(exit=trap(ub))`.
            let needle = format!("trap({kind})");
            let alt = format!(": {kind}(");
            let trapped = err.contains(&needle) || err.contains(&alt);
            // lupin's trap exit is 3 (EXERCISES.md §5 runs).
            let passed = trapped && code == Some(3);
            let detail = if !trapped {
                format!(
                    "expected {needle} on stderr; exit {code:?}, stderr: {}",
                    err.trim()
                )
            } else {
                format!("trap ok but exit {code:?}, expected 3")
            };
            Ok(Outcome {
                passed,
                detail,
                exit: code,
                stderr: err.clone(),
                diagnostic: Some(err.trim_end().to_string()),
                phase_reached: None,
            })
        }
        Check::Fail { code } => {
            let (verdict, diag, phase) = conform_run(tools, s)?;
            let want = format!("fail({code})");
            Ok(Outcome {
                passed: verdict == want,
                detail: format!("wolf verdict `{verdict}`, expected `{want}`"),
                // A conformance verdict is not one process's exit.
                exit: None,
                stderr: String::new(),
                diagnostic: Some(diag),
                phase_reached: phase,
            })
        }
        Check::Ub { row } => {
            // Undefined behavior is the one verdict the book will not
            // take on a single machine's word: lupin's oracle has to
            // fault the program and the compiler's checked build has to
            // name the same row. The snapshot keeps the E1401.
            let mut cmd = Command::new(&tools.lupin);
            cmd.arg(&s.file_name).current_dir(&s.dir);
            let (code, _out, err) = run_with_timeout(cmd)?;
            let oracle = err.contains(": ub(") && code == Some(3);
            let checked = conform_run_with(tools, s, true)?;
            let named =
                checked.verdict == "ub(mem.ub)" && checked.ub_row.as_deref() == Some(row.as_str());
            let detail = if oracle {
                format!(
                    "wolf --checked said verdict `{}` row `{}`, expected `ub(mem.ub)` row `{row}`",
                    checked.verdict,
                    checked.ub_row.as_deref().unwrap_or("<none>"),
                )
            } else {
                format!("lupin did not fault: exit {code:?}, stderr: {}", err.trim())
            };
            Ok(Outcome {
                passed: oracle && named,
                detail,
                exit: None,
                stderr: String::new(),
                diagnostic: Some(checked.diagnostic),
                phase_reached: checked.phase,
            })
        }
        Check::Audit { code } => {
            let mut cmd = Command::new(&tools.wolf);
            cmd.arg("audit-surface")
                .arg(format!("./{}", s.file_name))
                .current_dir(&s.dir);
            let (_code, _out, err) = run_with_timeout(cmd)?;
            // The ring inventory goes to stdout; stderr carries the
            // rendered diagnostics and one summary line. The snapshot is
            // the diagnostics.
            let diag: String = err
                .lines()
                .filter(|l| !l.starts_with("wolf audit-surface:"))
                .collect::<Vec<_>>()
                .join("\n");
            Ok(Outcome {
                passed: err.contains(&format!("error[{code}]")),
                detail: format!(
                    "`wolf audit-surface` did not report {code}; stderr: {}",
                    err.trim()
                ),
                exit: None,
                stderr: String::new(),
                diagnostic: Some(normalize(&diag)),
                phase_reached: None,
            })
        }
        // The compiler builds and runs it: `wolf run` compiles the
        // module and executes the binary in one step, which is the
        // reader-facing spelling and the honest one.
        Check::WolfRun { exit, stdout } => {
            let mut cmd = Command::new(&tools.wolf);
            cmd.arg("run").arg(&s.file_name).current_dir(&s.dir);
            let (code, out, err) = run_with_timeout(cmd)?;
            let mut passed = code == Some(*exit);
            let mut detail = format!("wolf run exited {code:?}, expected {exit}");
            if passed {
                if let Some(want) = stdout {
                    let got = out.trim_end_matches('\n');
                    if got != want.as_str() {
                        passed = false;
                        detail = format!("stdout mismatch: expected {want:?}, got {got:?}");
                    }
                }
            } else if !err.is_empty() {
                detail = format!("{detail}\n     stderr: {}", err.trim_end());
            }
            Ok(Outcome {
                passed,
                detail,
                exit: code,
                stderr: err,
                diagnostic: None,
                phase_reached: None,
            })
        }
        Check::Compile => {
            let (verdict, diag, phase) = conform_run(tools, s)?;
            let passed = !verdict.starts_with("fail(");
            Ok(Outcome {
                passed,
                detail: format!("wolf verdict `{verdict}` — the block must compile clean"),
                exit: None,
                stderr: String::new(),
                diagnostic: Some(diag),
                phase_reached: phase,
            })
        }
    }
}

/// What one `wolf conform-run` invocation reported.
struct ConformRun {
    verdict: String,
    diagnostic: String,
    phase: Option<String>,
    /// `x-ub-row` — present only on a `ub(…)` verdict from `--checked`.
    ub_row: Option<String>,
}

/// Run `wolf conform-run ./file` and split its output into the verdict
/// JSON (last line, stdout) and the rendered human diagnostic.
fn conform_run(tools: &Tools, s: &Sample) -> Result<(String, String, Option<String>)> {
    let r = conform_run_with(tools, s, false)?;
    Ok((r.verdict, r.diagnostic, r.phase))
}

fn conform_run_with(tools: &Tools, s: &Sample, checked: bool) -> Result<ConformRun> {
    let mut cmd = Command::new(&tools.wolf);
    // The ./ prefix matters: bare relative paths lose their parent
    // directory in package-root resolution (EXERCISES.md audit ledger,
    // finding 2 — ba:papercut, wolf-lang driver).
    cmd.arg("conform-run");
    if checked {
        cmd.arg("--checked");
    }
    cmd.arg(format!("./{}", s.file_name)).current_dir(&s.dir);
    let (_code, out, err) = run_with_timeout(cmd)?;
    let json_line = out
        .lines()
        .rev()
        .find(|l| l.trim_start().starts_with('{'))
        .unwrap_or("");
    let parsed: serde_json::Value = serde_json::from_str(json_line).with_context(|| {
        format!(
            "no verdict JSON from wolf for {} (stdout: {out:?}, stderr: {err:?})",
            s.id
        )
    })?;
    let verdict = parsed
        .get("verdict")
        .and_then(|v| v.as_str())
        .unwrap_or("<none>")
        .to_string();
    let phase = parsed
        .get("phase_reached")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    let ub_row = parsed
        .get("x-ub-row")
        .and_then(|v| v.as_str())
        .map(str::to_string);
    // The human-rendered diagnostic: everything before the JSON line,
    // wherever the driver put it.
    let mut diag = String::new();
    for l in out.lines() {
        if std::ptr::eq(l, json_line) || l == json_line {
            continue;
        }
        diag.push_str(l);
        diag.push('\n');
    }
    if diag.trim().is_empty() {
        diag = err;
    }
    Ok(ConformRun {
        verdict,
        diagnostic: normalize(&diag),
        phase,
        ub_row,
    })
}

// ------------------------------------------------------------- snapshots

enum SnapshotResult {
    Ok,
    Blessed,
    Mismatch(String),
}

fn check_snapshot(root: &Path, id: &str, diag: &str, bless: bool) -> Result<SnapshotResult> {
    let dir = root.join("snapshots/diagnostics");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.txt", id.replace('/', "__")));
    let body = format!("{}\n", normalize(diag));
    match std::fs::read_to_string(&path) {
        Ok(existing) if existing == body => Ok(SnapshotResult::Ok),
        Ok(_) if bless => {
            std::fs::write(&path, body)?;
            Ok(SnapshotResult::Blessed)
        }
        Ok(_) => Ok(SnapshotResult::Mismatch(format!(
            "diagnostic drifted from reviewed snapshot {} — if the compiler's \
             wording changed on purpose, re-run with --bless and review the diff",
            path.display()
        ))),
        Err(_) if bless => {
            std::fs::write(&path, body)?;
            Ok(SnapshotResult::Blessed)
        }
        Err(_) => Ok(SnapshotResult::Mismatch(format!(
            "no reviewed snapshot at {} — run `cargo xtask samples --bless` \
             and commit the reviewed file",
            path.display()
        ))),
    }
}

// ---------------------------------------------------------------- export

/// Build the corpus-export tree wolf-lang's runner can consume
/// (`samples/export/corpus/book/`): every corpus root and member
/// verbatim (their headers are already s01-style), plus the extracted
/// book samples with measured `phase:` headers. The CI job uploads
/// this tree; the sync PR into wolf-lang `corpus/book/` consumes it.
fn export_corpus(
    root: &Path,
    samples: &[Sample],
    phases: &BTreeMap<String, String>,
) -> Result<usize> {
    let export = root.join("samples/export/corpus/book");
    let _ = std::fs::remove_dir_all(&export);
    std::fs::create_dir_all(&export)?;
    let mut count = 0usize;

    // Corpus files: verbatim, structure preserved (members included so
    // multi-file packages stay whole).
    let base = root.join("principles/exercises");
    let mut lu_files = Vec::new();
    walk(&base, &mut |p| {
        if p.extension().and_then(|e| e.to_str()) == Some("lu") {
            lu_files.push(p.to_path_buf());
        }
    })?;
    for f in &lu_files {
        let rel = f.strip_prefix(&base).unwrap();
        let dst = export.join("exercises").join(rel);
        std::fs::create_dir_all(dst.parent().unwrap())?;
        std::fs::copy(f, &dst)?;
        count += 1;
    }

    // Book-extracted samples, phase header refined by measurement.
    for s in samples {
        let Origin::Book { .. } = &s.origin else {
            continue;
        };
        // A check wolf-lang's runner cannot spell does not travel.
        if s.check.corpus_directive().is_none() {
            continue;
        }
        let src = s.dir.join(&s.file_name);
        let text = std::fs::read_to_string(&src)?;
        let text = match phases.get(&s.id) {
            Some(phase) => text.replace("//! phase: run\n", &format!("//! phase: {phase}\n")),
            None => text,
        };
        let dst = export.join(&s.id).with_extension("lu");
        std::fs::create_dir_all(dst.parent().unwrap())?;
        std::fs::write(&dst, text)?;
        count += 1;
    }
    Ok(count)
}

// -------------------------------------------------------------- selftest

/// The acceptance demonstration: a deliberately-broken sample must
/// fail. Three breakages, one per checker lane.
fn selftest(root: &Path, tools: &Tools) -> Result<()> {
    let dir = root.join("samples/selftest");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir)?;

    let cases: Vec<(&str, &str, Check)> = vec![
        (
            "wrong-stdout.lu",
            "fn main() -> !int {\n    print(\"hello, wolf\")\n    0\n}\n",
            Check::Run {
                exit: 0,
                stdout: Some("goodbye, wolf".into()),
            },
        ),
        (
            "wrong-exit.lu",
            "fn main() -> !int {\n    0\n}\n",
            Check::Run {
                exit: 7,
                stdout: None,
            },
        ),
        (
            "wrong-code.lu",
            "struct P { a: str, b: str }\nfn take_it(take s: str) -> str { s }\nfn main() -> !int {\n    var p = P { a: \"x\", b: \"y\" }\n    let q = take_it(take p.a)\n    let r = p.a\n    print(\"{q} {r}\")\n    0\n}\n",
            Check::Fail {
                code: "E9999".into(),
            },
        ),
    ];

    let mut broken_caught = 0;
    for (name, source, check) in cases {
        // One directory per case: files in one directory are one module
        // (D32), so co-located selftest programs would collide on `main`.
        let case_dir = dir.join(name.trim_end_matches(".lu"));
        std::fs::create_dir_all(&case_dir)?;
        std::fs::write(case_dir.join(name), source)?;
        let sample = Sample {
            id: format!("selftest/{name}"),
            dir: case_dir,
            file_name: name.to_string(),
            check,
            origin: Origin::Corpus,
        };
        let outcome = execute(tools, &sample)?;
        if outcome.passed {
            bail!("self-test FAILED: deliberately-broken sample {name} passed its directive");
        }
        println!("samples: self-test caught {name}: {}", outcome.detail);
        broken_caught += 1;
    }
    println!("samples: self-test ok — {broken_caught}/3 deliberate breakages caught");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ownership_lint_catches_each_annotation() {
        assert_eq!(
            ownership_annotations("fn grow(mut xs: List[int]) { xs.push(7) }\n"),
            vec!["mut"]
        );
        assert_eq!(
            ownership_annotations("let a = adopt(take p.lead)\n"),
            vec!["take"]
        );
        assert_eq!(
            ownership_annotations("region scratch { }\n"),
            vec!["region"]
        );
        assert_eq!(
            ownership_annotations("let s = shared[Doc](d)\n"),
            vec!["shared"]
        );
    }

    #[test]
    fn ownership_lint_ignores_lookalike_identifiers() {
        // `taken` is a live identifier in chapter 5's top-2 scan, and a
        // substring match would fail the chapter for spelling.
        let program =
            "var taken = List[str]()\nlet mutation = 1\nlet regions = 2\nlet sharedness = 3\n";
        assert!(ownership_annotations(program).is_empty());
    }

    #[test]
    fn ownership_lint_reports_every_token_present() {
        let hits = ownership_annotations("fn f(mut x: int) { take y }\n");
        assert_eq!(hits, vec!["mut", "take"]);
    }

    #[test]
    fn ownership_lint_allows_the_write_marked_receiver() {
        // bs13: `(mut xs).push(…)` is mandatory on both machines at the
        // pin (wolf E0804 statically, lupin trap(exclusivity) at run
        // time), so Part 1 carries exactly this form and no other.
        assert!(ownership_annotations("(mut names).push(\"ada\")\n").is_empty());
        assert!(ownership_annotations("let last = (mut cents).pop()\n").is_empty());
        // The declaration mode is still an annotation: its paren group
        // is not called through.
        assert_eq!(
            ownership_annotations("fn grow(mut xs: List[int]) { (mut xs).push(7) }\n"),
            vec!["mut"]
        );
        // A bare parenthesized mode with no receiver call stays caught.
        assert_eq!(ownership_annotations("f(mut x)\n"), vec!["mut"]);
    }

    #[test]
    fn ownership_lint_allows_the_take_combinator() {
        // Chapter 5 prints reports/05's combinator chain, where `take` is
        // an iterator method. Position is what tells the two apart.
        let chain = "for (k, n) in totals.pairs().sorted_by(fn(a, b) b.1 <=> a.1).take(1) {}\n";
        assert!(ownership_annotations(chain).is_empty());
        assert_eq!(
            ownership_annotations("let a = adopt(take p.lead)\n"),
            vec!["take"]
        );
    }
}
