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

/// The budget for a GRADUATION probe — asking the other machine whether
/// it has started serving a program declared one-machine. That answer is
/// a watch, not a gate: the sample's verdict is already settled by the
/// machine that owns it. A program the other machine parks on (a
/// deadlock exercise under the native runtime, which has no detector)
/// must not cost the full sample budget on every run, forever.
const GRADUATION_TIMEOUT: Duration = Duration::from_secs(5);

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
    /// That process's stdout, verbatim. A refusal that reaches the
    /// program as a D30 ROW rather than as a driver message arrives
    /// here — `main` propagating one prints `error: <tag>` on stdout
    /// and exits 1 — so a `samples-os.toml` row can pin the words of a
    /// host refusal that never touches stderr (bs30, chapter 33's
    /// inherit set on windows).
    stdout: String,
    /// Rendered diagnostic text (fail samples), for snapshots and
    /// `diagnostic,from(…)` cross-checks.
    diagnostic: Option<String>,
    /// phase_reached reported by wolf, for corpus-export headers.
    phase_reached: Option<String>,
    /// Set when a one-machine fence's *other* machine now serves the
    /// program: the sample passes, and the fence has outlived its
    /// spelling. The runner reports it as a FLIP, which is how a
    /// per-machine note gets retired in the pin-bump commit instead of
    /// remembered (`principles/TWO-MACHINES.md`).
    graduates: Option<String>,
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
            } else if outcome.exit == Some(row.exit)
                && outcome.stderr.trim_end() == row.stderr
                && row
                    .stdout
                    .as_ref()
                    .is_none_or(|want| outcome.stdout.trim_end() == want)
            {
                refused_seen += 1;
                println!(
                    "samples: REFUSED({}) {} [{}] — declared: exit {}{}, retires at {} ({})",
                    row.os,
                    s.id,
                    s.check,
                    row.exit,
                    match row.stdout.as_deref() {
                        Some(w) => format!(", stdout {w:?}"),
                        None => String::new(),
                    },
                    row.retires,
                    row.note
                );
            } else {
                failures.push(format!(
                    "{}: the {} refusal drifted from samples-os.toml — the row is the \
                     book's claim about this host and it is now wrong\n     \
                     declared: exit {}, stdout {:?}\n       {}\n     actual: exit {:?}, \
                     stdout {:?}\n       {}",
                    s.id,
                    row.os,
                    row.exit,
                    row.stdout.as_deref().unwrap_or("<not declared>"),
                    row.stderr,
                    outcome.exit,
                    outcome.stdout.trim_end(),
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
        // A one-machine fence whose other machine now serves the
        // program: the sample passes and the SPELLING is stale. Same
        // handling as a pending row that starts passing, and for the
        // same reason — a feature landing is noticed, never absorbed.
        if let (Some(g), false) = (&outcome.graduates, is_pending) {
            flips.push(format!(
                "{}: directive `{}` holds, but {g} — both machines serve this program \
                 now; graduate the fence to `run(…)`, retire the page's per-machine \
                 note, and close the ledger row in the pin-bump commit",
                s.id, s.check
            ));
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

/// Kill a timed-out sample and everything it started.
///
/// `wolf run` compiles the program and then EXECUTES it, so the process
/// this function spawns has a grandchild: the sample's own binary. Killing
/// the child alone leaves that binary running and holding the write end of
/// the pipes, and the reader threads below then block on `read_to_end`
/// forever — a hang, not a timeout, and the one this rig hit on the very
/// first two-machine run (`appx/exB-9`, a deadlock exercise: lupin traps
/// it, the native binary parks). So the child gets its own process group
/// and the whole group is signalled.
#[cfg(unix)]
fn kill_group(child: &mut std::process::Child) {
    let pid = child.id();
    let _ = Command::new("kill")
        .arg("-9")
        .arg(format!("-{pid}"))
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
    let _ = child.kill();
}

#[cfg(not(unix))]
fn kill_group(child: &mut std::process::Child) {
    // No process groups here; the job-object equivalent is a bigger
    // change than this rig needs. A surviving grandchild on windows is
    // handled by not depending on the pipes closing — see the collector
    // below, which is what actually makes the timeout survivable.
    let _ = child.kill();
}

fn run_with_timeout(cmd: Command) -> Result<(Option<i32>, String, String)> {
    run_within(cmd, SAMPLE_TIMEOUT)
}

fn run_within(mut cmd: Command, budget: Duration) -> Result<(Option<i32>, String, String)> {
    use std::io::Read;
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    cmd.stdin(std::process::Stdio::null());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt as _;
        cmd.process_group(0);
    }
    let mut child = cmd.spawn().with_context(|| format!("spawning {cmd:?}"))?;
    let mut stdout_pipe = child.stdout.take().unwrap();
    let mut stderr_pipe = child.stderr.take().unwrap();
    // The readers hand their buffers back over a channel and are never
    // joined. `read_to_end` returns when the WRITE END closes, and a
    // killed child's grandchild still holds it — so joining these
    // threads makes a hung program hang the runner instead of timing
    // out. That is what cancelled the ubuntu and windows samples lanes
    // at their 60-minute ceiling while macOS passed: five deadlock
    // exercises whose compiled binaries never return.
    let (tx_out, rx_out) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stdout_pipe.read_to_end(&mut buf);
        let _ = tx_out.send(buf);
    });
    let (tx_err, rx_err) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stderr_pipe.read_to_end(&mut buf);
        let _ = tx_err.send(buf);
    });
    let deadline = Instant::now() + budget;
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break Some(status);
        }
        if Instant::now() > deadline {
            kill_group(&mut child);
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    // Whatever arrived before the pipes went quiet. A process that left
    // a grandchild behind contributes what it had written; it does not
    // contribute a hang.
    const DRAIN: Duration = Duration::from_secs(2);
    let drain = |rx: std::sync::mpsc::Receiver<Vec<u8>>| {
        String::from_utf8_lossy(&rx.recv_timeout(DRAIN).unwrap_or_default()).into_owned()
    };
    let stdout = drain(rx_out);
    let stderr = drain(rx_err);
    match status {
        Some(s) => Ok((s.code(), stdout, stderr)),
        None => bail!("timed out after {budget:?}"),
    }
}

/// The two machines the book is true for. Every executable claim in
/// this repository names one of them or both; nothing runs unattributed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Machine {
    Lupin,
    Wolf,
}

impl Machine {
    fn label(self) -> &'static str {
        match self {
            Machine::Lupin => "lupin",
            Machine::Wolf => "wolf run",
        }
    }

    /// The status a trapping program carries out of this machine. D60
    /// rules the exit half of a trap per-machine and documents both:
    /// lupin exits 3, a compiled binary dies at 134. Appendix B prints
    /// the table. The trap KIND is the contract; this number is not.
    fn trap_exit(self) -> i32 {
        match self {
            Machine::Lupin => 3,
            Machine::Wolf => 134,
        }
    }
}

/// What one machine did with one program.
struct MachineRun {
    code: Option<i32>,
    stdout: String,
    stderr: String,
}

impl MachineRun {
    /// Refusals and traps reach different streams on the two machines,
    /// and a refusal that arrives as a D30 row reaches stdout. Both are
    /// searched, always, so a moved stream cannot pass for silence.
    fn says(&self, needle: &str) -> bool {
        self.stderr.contains(needle) || self.stdout.contains(needle)
    }
}

/// The sentence appended to every refusal the runner reports, because
/// the fix is never "wait" and a lane that reads only the first clause
/// will otherwise reach for `samples-pending.toml`.
const RUN_IS_BOTH: &str = " — a `run(…)` fence is a claim about both machines, and a machine that refuses has not met it. If one machine alone serves this program, spell the fence for that machine (`lupin-run(…)`, `wolf-run(…)`), give the prose the per-machine note, and file the chapter ledger row with an owner (principles/TWO-MACHINES.md)";

fn machine_run(m: Machine, tools: &Tools, s: &Sample) -> Result<MachineRun> {
    machine_run_within(m, tools, s, SAMPLE_TIMEOUT)
}

/// A machine the sample's verdict depends on. A program that never
/// returns has not met its claim, so the timeout is reported as the
/// failure it is rather than aborting the run: before bs31 only a
/// declared `wolf-run(…)` sample could hang the compiler lane, and now
/// every `run(…)` sample reaches it.
fn machine_gate(m: Machine, tools: &Tools, s: &Sample) -> MachineRun {
    machine_run(m, tools, s).unwrap_or_else(|e| MachineRun {
        code: None,
        stdout: String::new(),
        stderr: format!("<{m:?} never returned: {e}>"),
    })
}

/// The other machine, asked on a short budget and allowed to say
/// nothing. A probe that times out has not met the claim, which is the
/// only thing the caller wants to know.
fn machine_probe(m: Machine, tools: &Tools, s: &Sample) -> Option<MachineRun> {
    machine_run_within(m, tools, s, GRADUATION_TIMEOUT).ok()
}

/// Has the COMPILER started serving a program declared interpreter-only?
///
/// This asks `wolf conform-run`, which reaches a verdict without code
/// generation and without ever executing the program. Two reasons, and
/// both were learned from CI rather than guessed. It is ~3ms against a
/// compile and a link, and a one-machine sample must not cost a native
/// build on every run of every lane forever. And it cannot hang: some of
/// these programs are deadlock exercises whose compiled binaries never
/// return by design, so a probe that RUNS them is a probe that parks.
///
/// The signal is deliberately weaker than the gate's. A refusal by name
/// or a static rejection means nothing has changed; anything else means
/// the compiler now accepts the program and a human should look at
/// graduating the fence. Reporting that as a FLIP is the point — the
/// runner remembers so nobody has to.
fn compiler_still_declines(tools: &Tools, s: &Sample) -> bool {
    match conform_run_with(tools, s, false) {
        // No verdict at all is not evidence that anything changed.
        Err(_) => true,
        // Two ways to decline and they are not interchangeable: a
        // static rejection says the program is illegal, and an
        // `x-unsupported-construct` says the compiler will not lower it.
        // The bare verdict is useless here — `unsupported` is also what
        // a perfectly good program reports, because `conform-run` stops
        // before it would run one.
        Ok(r) => r.verdict.starts_with("fail(") || r.unsupported_construct.is_some(),
    }
}

fn machine_run_within(
    m: Machine,
    tools: &Tools,
    s: &Sample,
    budget: Duration,
) -> Result<MachineRun> {
    let mut cmd = match m {
        Machine::Lupin => {
            let mut c = Command::new(&tools.lupin);
            c.arg(&s.file_name);
            c
        }
        Machine::Wolf => {
            let mut c = Command::new(&tools.wolf);
            c.arg("run").arg(&s.file_name);
            c
        }
    };
    cmd.current_dir(&s.dir);
    let (code, stdout, stderr) = run_within(cmd, budget)?;
    Ok(MachineRun {
        code,
        stdout,
        stderr,
    })
}

/// A machine's refusal to serve the program AT ALL, in its own words.
/// This is the absence of product, not a diagnostic about the reader's
/// program (TONE's tense discipline draws that line), and it is the
/// thing this rig used never to look at on the compiler's side.
fn refusal(r: &MachineRun) -> Option<String> {
    let line = |needle: &str| {
        r.stderr
            .lines()
            .chain(r.stdout.lines())
            .find(|l| l.contains(needle))
            .map(|l| l.trim().to_string())
    };
    // The refusal is quoted in the machine's own words, because WHICH
    // construct was refused is the whole content of the finding: it is
    // what routes a ledger row to the campaign that owns it. Reporting
    // only "the compiler refused" would repeat, in the rig, the mistake
    // this rule exists to fix.
    line("cannot compile this yet").or_else(|| line("unsupported:"))
}

/// Did this machine meet the fence's exit-and-stdout claim?
fn meets(r: &MachineRun, exit: i32, stdout: Option<&str>) -> std::result::Result<(), String> {
    if r.code != Some(exit) {
        let mut why = format!("exited {:?}, expected {exit}", r.code);
        if !r.stderr.trim().is_empty() {
            why.push_str(&format!(" (stderr: {})", r.stderr.trim()));
        }
        return Err(why);
    }
    if let Some(want) = stdout {
        let got = r.stdout.trim_end_matches('\n');
        if got != want {
            return Err(format!("stdout mismatch: expected {want:?}, got {got:?}"));
        }
    }
    Ok(())
}

/// Did this machine fault, naming that kind? lupin spells a defined
/// fault `trap(kind)` and the UB checker's faults `file: ub(clause)`;
/// the compiled binary spells it `wolf-trap: kind`.
fn traps(m: Machine, r: &MachineRun, kind: &str) -> std::result::Result<(), String> {
    let named = match m {
        Machine::Lupin => r.says(&format!("trap({kind})")) || r.says(&format!(": {kind}(")),
        Machine::Wolf => r.says(&format!("wolf-trap: {kind}")) || r.says(&format!("trap({kind})")),
    };
    if !named {
        return Err(format!(
            "expected the {kind} trap named; exit {:?}, stderr: {}",
            r.code,
            r.stderr.trim()
        ));
    }
    let want = m.trap_exit();
    if r.code != Some(want) {
        return Err(format!(
            "trap named but exit {:?}, expected {want} ({} traps at {want}; D60 rules the \
             status per-machine)",
            r.code,
            m.label()
        ));
    }
    Ok(())
}

fn execute(tools: &Tools, s: &Sample) -> Result<Outcome> {
    match &s.check {
        // A `run(…)` fence is a claim about BOTH machines, and a
        // machine that refuses has not met it (TWO-MACHINES.md). Before
        // bs31 this arm asked lupin alone, so a program the compiler
        // declined by conservatism was scored green and printed at a
        // reader as if `wolf run` served it.
        Check::Run { exit, stdout } => {
            let lupin = machine_gate(Machine::Lupin, tools, s);
            let wolf = machine_gate(Machine::Wolf, tools, s);
            let mut why: Vec<String> = Vec::new();
            if let Err(e) = meets(&lupin, *exit, stdout.as_deref()) {
                why.push(format!("lupin: {e}"));
            }
            match refusal(&wolf) {
                Some(word) => why.push(format!("wolf run: {word}{}", RUN_IS_BOTH)),
                None => {
                    if let Err(e) = meets(&wolf, *exit, stdout.as_deref()) {
                        why.push(format!("wolf run: {e}"));
                    }
                }
            }
            let blamed = if meets(&lupin, *exit, stdout.as_deref()).is_err() {
                &lupin
            } else {
                &wolf
            };
            Ok(Outcome {
                passed: why.is_empty(),
                detail: why.join("\n     "),
                // Two processes ran; the fields a `samples-os.toml`
                // refusal row compares are the failing machine's, and
                // lupin's when both agree.
                exit: blamed.code,
                stderr: blamed.stderr.clone(),
                stdout: blamed.stdout.clone(),
                diagnostic: None,
                phase_reached: None,
                graduates: None,
            })
        }
        // The interpreter alone, by declaration. The compiler is still
        // asked — not to score the sample, but to notice the day it
        // starts serving the program, so the fence and the page's
        // per-machine note retire in the pin-bump commit.
        Check::LupinRun { exit, stdout } => {
            let lupin = machine_gate(Machine::Lupin, tools, s);
            let passed = meets(&lupin, *exit, stdout.as_deref());
            let graduates = (!compiler_still_declines(tools, s))
                .then(|| "the compiler no longer declines this program".to_string());
            Ok(Outcome {
                passed: passed.is_ok(),
                detail: match passed {
                    Ok(()) => String::new(),
                    Err(e) => format!("lupin: {e}"),
                },
                exit: lupin.code,
                stderr: lupin.stderr.clone(),
                stdout: lupin.stdout.clone(),
                diagnostic: None,
                phase_reached: None,
                graduates,
            })
        }
        // `run(exit=trap(k))` is a `run(…)` fence and carries the same
        // claim: both machines fault, and both name the kind. The exit
        // status is the one half that is per-machine — D60 rules it so,
        // and Appendix B prints the table — hence `trap_exit()` rather
        // than one number.
        Check::Trap { kind } => {
            let lupin = machine_gate(Machine::Lupin, tools, s);
            let wolf = machine_gate(Machine::Wolf, tools, s);
            let mut why: Vec<String> = Vec::new();
            if let Err(e) = traps(Machine::Lupin, &lupin, kind) {
                why.push(format!("lupin: {e}"));
            }
            match refusal(&wolf) {
                Some(word) => why.push(format!("wolf run: {word}{}", RUN_IS_BOTH)),
                None => {
                    if let Err(e) = traps(Machine::Wolf, &wolf, kind) {
                        why.push(format!("wolf run: {e}"));
                    }
                }
            }
            Ok(Outcome {
                passed: why.is_empty(),
                detail: why.join("\n     "),
                exit: lupin.code,
                stderr: lupin.stderr.clone(),
                stdout: lupin.stdout.clone(),
                // The snapshotted trap text is the interpreter's: it is
                // the one the chapters print.
                diagnostic: Some(lupin.stderr.trim_end().to_string()),
                phase_reached: None,
                graduates: None,
            })
        }
        // NO GRADUATION PROBE, deliberately. A trap claim retires on a
        // RUNTIME fact — the compiled binary faulting the same way — and
        // the cheap probe cannot see that: it never executes anything,
        // and the compiler accepts every one of these programs happily.
        // (Measured: `appx/exB-4` compiles and exits 0 without trapping;
        // the two deadlock exercises compile and then park forever, which
        // is what a probe that ran them would do too.) Guessing from a
        // compile-time verdict would report a graduation that has not
        // happened, so these rows retire by hand, against the ledger row
        // and the spec clause that put them here.
        Check::LupinTrap { kind } => {
            let lupin = machine_gate(Machine::Lupin, tools, s);
            let passed = traps(Machine::Lupin, &lupin, kind);
            let graduates: Option<String> = None;
            Ok(Outcome {
                passed: passed.is_ok(),
                detail: match passed {
                    Ok(()) => String::new(),
                    Err(e) => format!("lupin: {e}"),
                },
                exit: lupin.code,
                stderr: lupin.stderr.clone(),
                stdout: lupin.stdout.clone(),
                diagnostic: Some(lupin.stderr.trim_end().to_string()),
                phase_reached: None,
                graduates,
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
                stdout: String::new(),
                diagnostic: Some(diag),
                phase_reached: phase,
                graduates: None,
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
                stdout: String::new(),
                diagnostic: Some(checked.diagnostic),
                phase_reached: checked.phase,
                graduates: None,
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
                stdout: String::new(),
                diagnostic: Some(normalize(&diag)),
                phase_reached: None,
                graduates: None,
            })
        }
        // The compiler builds and runs it: `wolf run` compiles the
        // module and executes the binary in one step, which is the
        // reader-facing spelling and the honest one.
        Check::WolfRun { exit, stdout } => {
            let wolf = machine_gate(Machine::Wolf, tools, s);
            let passed = meets(&wolf, *exit, stdout.as_deref());
            // The other half of the one-machine rule, read the other
            // way: the interpreter is asked too, so the day it serves a
            // program declared compiler-only is the day the runner says
            // so rather than the day someone re-probes by hand.
            let graduates = machine_probe(Machine::Lupin, tools, s)
                .filter(|l| refusal(l).is_none() && meets(l, *exit, stdout.as_deref()).is_ok())
                .map(|_| format!("`lupin` now meets `run(exit={exit}…)` too"));
            Ok(Outcome {
                passed: passed.is_ok(),
                detail: match passed {
                    Ok(()) => String::new(),
                    Err(e) => format!("wolf run: {e}"),
                },
                exit: wolf.code,
                stderr: wolf.stderr.clone(),
                stdout: wolf.stdout.clone(),
                diagnostic: None,
                phase_reached: None,
                graduates,
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
                stdout: String::new(),
                diagnostic: Some(diag),
                phase_reached: phase,
                graduates: None,
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
    /// `x-unsupported-construct` — the construct the compiler declines,
    /// in its own words. Present exactly when the conservatism ledger
    /// answered, which makes it the signal a graduation probe wants:
    /// the refusal itself goes to stderr, and the verdict is a bare
    /// `unsupported` that a clean program also reports.
    unsupported_construct: Option<String>,
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
    let unsupported_construct = parsed
        .get("x-unsupported-construct")
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
        unsupported_construct,
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
    // The one-machine corpus files stay home. wolf-lang's runner is the
    // compiler's, and these are the programs the compiler declines;
    // shipping them over as `run(…)` would hand that corpus a claim its
    // own machine cannot meet, which is the defect
    // `principles/TWO-MACHINES.md` exists to stop making. A package's
    // members go with its root, so the unit skipped is the directory.
    let mut home: std::collections::BTreeSet<PathBuf> = Default::default();
    for f in &lu_files {
        let source = std::fs::read_to_string(f)?;
        let stays = parse_lu_header(&source)
            .ok()
            .and_then(|h| h.check)
            .is_some_and(|c| c.corpus_directive().is_none());
        if stays {
            home.insert(f.parent().unwrap().to_path_buf());
        }
    }
    let mut kept_home = 0usize;
    for f in &lu_files {
        if home.contains(f.parent().unwrap()) {
            kept_home += 1;
            continue;
        }
        let rel = f.strip_prefix(&base).unwrap();
        let dst = export.join("exercises").join(rel);
        std::fs::create_dir_all(dst.parent().unwrap())?;
        std::fs::copy(f, &dst)?;
        count += 1;
    }
    if kept_home > 0 {
        println!(
            "samples: {kept_home} corpus file(s) held back from the export — one-machine \
             directives the other runner cannot spell (principles/TWO-MACHINES.md)"
        );
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
/// fail. Four breakages, one per checker lane, and the fourth is the
/// bs31 rule — a `run(…)` fence on a program only one machine serves
/// is a broken sample, not a green one.
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
        (
            // A comptime fold: the compiler evaluates it and prints
            // 285; lupin declines `comptime fn` by design, and has
            // since the namespace was reserved. Declared `run(…)` — a
            // claim about both machines — it must fail, because one of
            // them did not meet it. This is the wolf-book#9 shape with
            // the machines swapped, and it is the case the rig had no
            // way to catch before principles/TWO-MACHINES.md.
            "one-machine.lu",
            "comptime fn sum_squares(n: int) -> int {\n    var acc = 0\n    var i = 1\n    while i <= n {\n        acc += i * i\n        i += 1\n    }\n    acc\n}\nfn main() -> !int {\n    const T = sum_squares(9)\n    print(\"{T}\")\n    0\n}\n",
            Check::Run {
                exit: 0,
                stdout: Some("285".into()),
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
    println!("samples: self-test ok — {broken_caught}/4 deliberate breakages caught");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The bug that cancelled two CI lanes at their 60-minute ceiling.
    /// A child that leaves a grandchild holding the pipes must still
    /// TIME OUT: `read_to_end` returns when the write end closes, and a
    /// killed child's grandchild has not closed it. Before the fix the
    /// runner joined those readers and hung forever.
    #[cfg(unix)]
    #[test]
    fn a_surviving_grandchild_does_not_hang_the_runner() {
        let mut cmd = Command::new("sh");
        // The backgrounded `sleep` inherits stdout and outlives `sh`.
        cmd.arg("-c").arg("sleep 45 & echo started; wait");
        let started = Instant::now();
        let r = run_within(cmd, Duration::from_secs(2));
        assert!(r.is_err(), "expected a timeout, got {r:?}");
        assert!(
            started.elapsed() < Duration::from_secs(30),
            "timed out but only after {:?} — the readers are blocking again",
            started.elapsed()
        );
    }

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
