//! `cargo xtask tiers [--check]` — the EXERCISES-INDEX tier column,
//! generated from the corpus (ruling #40, wolf-book#43).
//!
//! The column means **which machines executed the program**, and the
//! runner already decides that from each file's `//! check:` head:
//! `samples.rs` gates a `run(…)` file on both machines (`RUN_IS_BOTH`),
//! a `lupin-run(…)` file on the interpreter alone, a `wolf-run(…)` file
//! on the compiler alone, and a `fail(…)` or `audit(…)` file on the
//! compiler's static verdict alone. Until bs61 the column was typed by
//! hand and meant something else on 168 of its 237 rows with a program
//! (bs45 counted the largest class at 136 rows: `run(…)` files tiered
//! `run (lupin)`). Now it is read off the heads, and `--check` fails CI
//! on any row that says otherwise.
//!
//! A row's programs are the corpus files named for it (`chNN/exN-M.lu`,
//! and `exN-Ma.lu`, `exN-Mb.lu` … for a pair), plus any package
//! directory its master section owns (chapter 22's `metrics/`, `leak/`
//! …), less `//! member: true` files, which are modules of a program
//! rather than programs. A row with no program on disk — a prose
//! answer, a REPL transcript, a workflow replayed against a fixture —
//! keeps the tier the page gives it, and the run says how many did.

use crate::directives::{parse_lu_header, Check};
use anyhow::{bail, Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

const INDEX: &str = "principles/EXERCISES-INDEX.md";

/// The tiers in the order the totals line prints them; a hand tier
/// outside this list is printed after them, alphabetically.
const ORDER: &[&str] = &[
    "run (lupin)",
    "run (lupin REPL)",
    "run (wolf)",
    "run (wolf + lupin)",
    "prose",
    "pending",
];

pub fn run(root: &Path, args: &[String]) -> Result<()> {
    let check = match args {
        [] => false,
        [a] if a == "--check" => true,
        _ => bail!("usage: cargo xtask tiers [--check]"),
    };
    let page =
        std::fs::read_to_string(root.join(INDEX)).with_context(|| format!("reading {INDEX}"))?;
    let corpus = Corpus::read(root)?;
    let gen = generate(&page, &corpus)?;
    if check {
        if gen.page == page {
            println!("tiers: {INDEX} is generated — {}", gen.summary);
            return Ok(());
        }
        for d in &gen.drift {
            eprintln!("tiers: FAIL {d}");
        }
        if gen.drift.is_empty() {
            eprintln!("tiers: FAIL {INDEX}: the tier totals line is not the table's");
        }
        bail!(
            "{INDEX} is stale: {} row(s) disagree with their `//! check:` heads — run \
             `cargo xtask tiers` and commit the result",
            gen.drift.len()
        );
    }
    std::fs::write(root.join(INDEX), &gen.page).with_context(|| format!("writing {INDEX}"))?;
    for d in &gen.drift {
        println!("tiers: moved {d}");
    }
    println!("tiers: wrote {INDEX} — {}", gen.summary);
    Ok(())
}

/// Which machines a head asks. The runner's own reading, arm by arm
/// (`samples::execute`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Machine {
    Wolf,
    Lupin,
}

fn machines(check: &Check) -> BTreeSet<Machine> {
    use Machine::*;
    match check {
        // Both machines run it and both must meet the claim; `ub(…)` is
        // lupin's oracle and the compiler's checked build.
        Check::Run { .. } | Check::RunNonzero { .. } | Check::Trap { .. } | Check::Ub { .. } => {
            [Wolf, Lupin].into()
        }
        // The interpreter alone scores it; the compiler is only probed
        // for a graduation, which is not an execution of the claim.
        Check::LupinRun { .. } | Check::LupinTrap { .. } => [Lupin].into(),
        // The compiler alone: `wolf run`, `conform-run`, `audit-surface`.
        Check::WolfRun { .. } | Check::Fail { .. } | Check::Audit { .. } | Check::Compile => {
            [Wolf].into()
        }
    }
}

fn tier_of(set: &BTreeSet<Machine>) -> &'static str {
    match (set.contains(&Machine::Wolf), set.contains(&Machine::Lupin)) {
        (true, true) => "run (wolf + lupin)",
        (true, false) => "run (wolf)",
        _ => "run (lupin)",
    }
}

/// One corpus program: its path under `principles/exercises/`, its
/// directive spelled as the file spells it, and its pending status.
#[derive(Debug, Clone)]
struct Program {
    rel: String,
    check: Check,
}

struct Corpus {
    /// Every non-member `.lu` under `principles/exercises/`, by path.
    programs: BTreeMap<String, Program>,
    /// `samples-pending.toml` ids (`ch05/ex5-8`).
    pending: BTreeSet<String>,
    /// `ch22/metrics` → `22-1`.
    packages: BTreeMap<String, String>,
}

impl Corpus {
    fn read(root: &Path) -> Result<Corpus> {
        let base = root.join("principles/exercises");
        let mut files = Vec::new();
        walk(&base, &mut files)?;
        files.sort();
        let mut programs = BTreeMap::new();
        for path in files {
            let source = std::fs::read_to_string(&path)
                .with_context(|| format!("reading {}", path.display()))?;
            let header = parse_lu_header(&source)
                .with_context(|| format!("parsing directives in {}", path.display()))?;
            if header.member {
                continue;
            }
            let rel = rel_path(&base, &path);
            let check = header
                .check
                .with_context(|| format!("{rel}: corpus file has no `//! check:` directive"))?;
            programs.insert(rel.clone(), Program { rel, check });
        }
        let manifest = std::fs::read_to_string(root.join("samples-pending.toml"))
            .context("reading samples-pending.toml")?;
        let parsed: toml::Value = manifest.parse().context("parsing samples-pending.toml")?;
        let pending = parsed
            .get("pending")
            .and_then(|p| p.as_array())
            .map(|rows| {
                rows.iter()
                    .filter_map(|r| r.get("id").and_then(|v| v.as_str()))
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default();
        let packages = package_owners(&base)?;
        Ok(Corpus {
            programs,
            pending,
            packages,
        })
    }

    /// The programs a row owns.
    fn programs_of(&self, id: &str) -> Vec<&Program> {
        let Some((ch, n)) = id.split_once('-') else {
            return Vec::new();
        };
        let dir = match ch.parse::<u32>() {
            Ok(c) => format!("ch{c:02}"),
            Err(_) => "appx".to_string(),
        };
        let stem = format!("{dir}/ex{ch}-{n}");
        let mut out: Vec<&Program> = self
            .programs
            .values()
            .filter(|p| {
                let Some(rest) = p.rel.strip_prefix(&stem) else {
                    return false;
                };
                // `ex9-7.lu`, `ex9-7a.lu` — never `ex9-70.lu`.
                rest == ".lu"
                    || (rest.len() == 4
                        && rest.ends_with(".lu")
                        && rest.as_bytes()[0].is_ascii_lowercase())
            })
            .collect();
        for (pkg, owner) in &self.packages {
            if owner == id {
                let prefix = format!("{pkg}/");
                out.extend(
                    self.programs
                        .values()
                        .filter(|p| p.rel.starts_with(&prefix)),
                );
            }
        }
        out
    }

    fn is_pending(&self, p: &Program) -> bool {
        self.pending
            .contains(p.rel.strip_suffix(".lu").unwrap_or(&p.rel))
    }
}

fn rel_path(base: &Path, path: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for e in std::fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let p = e?.path();
        let hidden = p
            .file_name()
            .is_some_and(|n| n.to_string_lossy().starts_with('.'));
        if p.is_dir() && !hidden {
            walk(&p, out)?;
        } else if p.extension().and_then(|x| x.to_str()) == Some("lu") {
            out.push(p);
        }
    }
    Ok(())
}

/// A package directory under a chapter (`ch22/metrics/`) belongs to the
/// exercise whose solution label names it (``Solution. `ch22/metrics/`:``),
/// or, when no label does, to the one exercise whose master section
/// mentions it (`leak/main.lu` in 22-2's transcript). A directory no
/// section mentions, or that two sections mention with no label to
/// decide, is an error: a program no row can claim is the dark corner
/// this generator exists to close.
fn package_owners(base: &Path) -> Result<BTreeMap<String, String>> {
    let mut out = BTreeMap::new();
    let mut chapters: Vec<PathBuf> = std::fs::read_dir(base)
        .with_context(|| format!("reading {}", base.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_dir())
        .collect();
    chapters.sort();
    for chapter in chapters {
        let ch = chapter
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        let mut pkgs: Vec<String> = std::fs::read_dir(&chapter)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| is_package(p))
            .map(|p| {
                p.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        if pkgs.is_empty() {
            continue;
        }
        pkgs.sort();
        let master = std::fs::read_to_string(chapter.join("EXERCISES.md"))
            .with_context(|| format!("{ch}: package directories and no EXERCISES.md"))?;
        let sections = sections(&master);
        for pkg in pkgs {
            let label = format!("Solution. `{ch}/{pkg}/`");
            let labeled: Vec<&String> = sections
                .iter()
                .filter(|(_, t)| t.contains(&label))
                .map(|(id, _)| id)
                .collect();
            let mentioned: Vec<&String> = sections
                .iter()
                .filter(|(_, t)| mentions_dir(t, &pkg))
                .map(|(id, _)| id)
                .collect();
            let owner = match (labeled.as_slice(), mentioned.as_slice()) {
                ([one], _) => (*one).clone(),
                ([], [one]) => (*one).clone(),
                _ => bail!(
                    "{ch}/{pkg}/: a package directory with no one owning exercise — labeled \
                     by {labeled:?}, mentioned by {mentioned:?}; give it a `Solution. \
                     `{ch}/{pkg}/`` label in the exercise it answers"
                ),
            };
            out.insert(format!("{ch}/{pkg}"), owner);
        }
    }
    Ok(out)
}

/// A package is a visible directory holding at least one `.lu`. The
/// runner leaves a `.lu-cache/` of built binaries in every chapter it
/// compiles, and that is not an exercise anyone answers (found at the
/// bs61 head gate: `tiers --check` refused `appx/.lu-cache/` after
/// `cargo xtask samples` had run in the same checkout).
fn is_package(dir: &Path) -> bool {
    let hidden = dir
        .file_name()
        .is_some_and(|n| n.to_string_lossy().starts_with('.'));
    if hidden || !dir.is_dir() {
        return false;
    }
    let mut lus = Vec::new();
    walk(dir, &mut lus).is_ok() && !lus.is_empty()
}

/// `**Exercise N-M**` sections of one master page, in order.
fn sections(master: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for line in master.lines() {
        if let Some(rest) = line.trim_start().strip_prefix("**Exercise ") {
            if let Some(id) = rest.split("**").next() {
                out.push((id.trim().to_string(), String::new()));
            }
        }
        if let Some((_, text)) = out.last_mut() {
            text.push_str(line);
            text.push('\n');
        }
    }
    out
}

/// `leak/` as a path segment: `leak/main.lu`, `` `ch22/leak/` ``, never
/// `unleak/`.
fn mentions_dir(text: &str, pkg: &str) -> bool {
    let needle = format!("{pkg}/");
    text.match_indices(&needle).any(|(at, _)| {
        text[..at]
            .chars()
            .next_back()
            .is_none_or(|c| c.is_whitespace() || matches!(c, '`' | '/' | '('))
    })
}

struct Generated {
    page: String,
    drift: Vec<String>,
    summary: String,
}

fn generate(page: &str, corpus: &Corpus) -> Result<Generated> {
    let mut out = String::with_capacity(page.len());
    let mut drift = Vec::new();
    let mut claimed: BTreeSet<&str> = BTreeSet::new();
    let mut tally: BTreeMap<String, usize> = BTreeMap::new();
    let (mut generated, mut hand) = (0usize, 0usize);
    let mut hand_tally: BTreeMap<String, usize> = BTreeMap::new();
    let mut rows = 0usize;
    for line in page.split_inclusive('\n') {
        let body = line.trim_end_matches(['\n', '\r']);
        let Some((id, _ty, tier)) = crate::verify::index_row_cells(body) else {
            out.push_str(line);
            continue;
        };
        rows += 1;
        let progs = corpus.programs_of(&id);
        let want = if progs.is_empty() {
            hand += 1;
            *hand_tally.entry(tier.clone()).or_default() += 1;
            tier.clone()
        } else {
            generated += 1;
            for p in &progs {
                claimed.insert(p.rel.as_str());
            }
            if progs.iter().any(|p| corpus.is_pending(p)) {
                "pending".to_string()
            } else {
                let set: BTreeSet<Machine> =
                    progs.iter().flat_map(|p| machines(&p.check)).collect();
                tier_of(&set).to_string()
            }
        };
        *tally.entry(want.clone()).or_default() += 1;
        if want == tier {
            out.push_str(line);
            continue;
        }
        let heads = progs
            .iter()
            .map(|p| format!("{} `{}`", p.rel, p.check))
            .collect::<Vec<_>>()
            .join(", ");
        drift.push(format!(
            "`{id}`: the index says `{tier}`; its head(s) {heads} make it `{want}`"
        ));
        let trimmed = body.trim_end();
        let end = trimmed.len() - 1; // the closing `|`
        let start = trimmed[..end].rfind('|').context("a row lost its cells")?;
        out.push_str(&trimmed[..=start]);
        out.push(' ');
        out.push_str(&want);
        out.push_str(" |");
        out.push_str(&line[body.len()..]);
    }
    let unclaimed: Vec<&str> = corpus
        .programs
        .keys()
        .map(String::as_str)
        .filter(|p| !claimed.contains(p))
        .collect();
    if !unclaimed.is_empty() {
        bail!(
            "{INDEX}: {} corpus program(s) no row claims: {} — name the file for its \
             exercise (`exN-M.lu`) or label its package directory",
            unclaimed.len(),
            unclaimed.join(", ")
        );
    }
    let totals = totals_line(&tally, rows);
    let page = replace_totals(&out, &totals)?;
    let hand_list = hand_tally
        .iter()
        .map(|(t, n)| format!("{n} {t}"))
        .collect::<Vec<_>>()
        .join(", ");
    let summary = format!(
        "{rows} rows; {generated} read off their heads, {hand} hand-kept with no \
         program on disk ({hand_list}); {}",
        totals.trim_end_matches('.')
    );
    Ok(Generated {
        page,
        drift,
        summary,
    })
}

fn totals_line(tally: &BTreeMap<String, usize>, rows: usize) -> String {
    let mut parts: Vec<String> = ORDER
        .iter()
        .filter_map(|t| tally.get(*t).map(|n| format!("{n} {t}")))
        .collect();
    parts.extend(
        tally
            .iter()
            .filter(|(t, _)| !ORDER.contains(&t.as_str()))
            .map(|(t, n)| format!("{n} {t}")),
    );
    format!("Tier totals: {}. That is {rows}.", parts.join(" · "))
}

/// The totals line is one sentence pair on one line; anything else is a
/// page shape this generator was not written for, said out loud.
fn replace_totals(page: &str, totals: &str) -> Result<String> {
    let found = page
        .lines()
        .filter(|l| l.starts_with("Tier totals: "))
        .count();
    if found != 1 {
        bail!("{INDEX}: expected one `Tier totals: ` line, found {found}");
    }
    let mut s = String::with_capacity(page.len());
    for l in page.split_inclusive('\n') {
        if l.starts_with("Tier totals: ") {
            s.push_str(totals);
            s.push_str(&l[l.trim_end_matches(['\n', '\r']).len()..]);
        } else {
            s.push_str(l);
        }
    }
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::directives::parse_check;

    fn prog(rel: &str, check: &str) -> (String, Program) {
        (
            rel.to_string(),
            Program {
                rel: rel.to_string(),
                check: parse_check(check).expect("a directive"),
            },
        )
    }

    fn corpus(progs: &[(&str, &str)], pending: &[&str], packages: &[(&str, &str)]) -> Corpus {
        Corpus {
            programs: progs.iter().map(|(r, c)| prog(r, c)).collect(),
            pending: pending.iter().map(|s| s.to_string()).collect(),
            packages: packages
                .iter()
                .map(|(d, id)| (d.to_string(), id.to_string()))
                .collect(),
        }
    }

    const PAGE: &str = "\
# index

Tier totals: 9 run (lupin). That is 9.

| section | exercise | type · checker | tier |
|---|---|---|---|
| §1.1 | 1-1 | fingers · lupin | run (lupin) |
| §6.2 — `else |err|` | 6-7 | extension · lupin | run (lupin) |
| §1.4 | 1-4 | fingers · lupin REPL | run (lupin REPL) |
| §1.5 | 1-6 | spelunking · lupin | run (lupin) |
| §5.2 | 5-8 | comprehension · pending | pending |
| §9.4 | 9-7 | comprehension · lupin | run (lupin) |
| §22.1 | 22-1 | fingers · lupin | run (lupin) |
| §B | B-4 | comprehension · lupin | run (lupin) |
| §18 | 18-3 | comprehension · wolf | run (wolf) |
";

    fn the_corpus() -> Corpus {
        corpus(
            &[
                ("ch01/ex1-1.lu", "run(exit=0)"),
                ("ch06/ex6-7.lu", "lupin-run(exit=0)"),
                ("ch01/ex1-6.lu", "fail(E0202)"),
                ("ch05/ex5-8.lu", "run(exit=0)"),
                ("ch09/ex9-7a.lu", "lupin-run(exit=0)"),
                ("ch09/ex9-7b.lu", "lupin-run(exit=trap(ub))"),
                ("ch22/metrics/main.lu", "run(exit=0)"),
                ("appx/exB-4.lu", "lupin-run(exit=trap(deadlock))"),
                ("ch18/ex18-3.lu", "wolf-run(exit=0)"),
            ],
            &["ch05/ex5-8"],
            &[("ch22/metrics", "22-1")],
        )
    }

    #[test]
    fn each_head_names_the_machines_the_runner_asks() {
        let tier = |c: &str| tier_of(&machines(&parse_check(c).unwrap()));
        assert_eq!(tier("run(exit=0)"), "run (wolf + lupin)");
        assert_eq!(tier("run(exit=nonzero)"), "run (wolf + lupin)");
        assert_eq!(tier("run(exit=trap(bounds))"), "run (wolf + lupin)");
        assert_eq!(tier("ub(P1)"), "run (wolf + lupin)");
        assert_eq!(tier("lupin-run(exit=0)"), "run (lupin)");
        assert_eq!(tier("lupin-run(exit=trap(ub))"), "run (lupin)");
        assert_eq!(tier("wolf-run(exit=0)"), "run (wolf)");
        assert_eq!(tier("fail(E0304)"), "run (wolf)");
        assert_eq!(tier("audit(E1303)"), "run (wolf)");
    }

    #[test]
    fn rows_are_read_off_their_heads_and_hand_rows_are_kept() {
        let g = generate(PAGE, &the_corpus()).expect("generates");
        let tiers: Vec<(String, String)> = g
            .page
            .lines()
            .filter_map(crate::verify::index_row_cells)
            .map(|(id, _, t)| (id, t))
            .collect();
        let get = |id: &str| tiers.iter().find(|(i, _)| i == id).unwrap().1.clone();
        assert_eq!(get("1-1"), "run (wolf + lupin)");
        // The section cell carries a `|` of its own; the tier cell is
        // still the last one, and the section survives byte for byte.
        assert_eq!(get("6-7"), "run (lupin)");
        assert!(g.page.contains("| §6.2 — `else |err|` | 6-7 |"));
        assert_eq!(get("1-4"), "run (lupin REPL)", "no program: hand-kept");
        assert_eq!(get("1-6"), "run (wolf)", "fail(…) is the compiler's alone");
        assert_eq!(get("5-8"), "pending", "the manifest wins");
        assert_eq!(get("9-7"), "run (lupin)", "a pair is the union");
        assert_eq!(get("22-1"), "run (wolf + lupin)", "a package owns its row");
        assert_eq!(get("B-4"), "run (lupin)");
        assert_eq!(get("18-3"), "run (wolf)");
        assert!(g.page.contains(
            "Tier totals: 3 run (lupin) · 1 run (lupin REPL) · 2 run (wolf) · \
             2 run (wolf + lupin) · 1 pending. That is 9.\n"
        ));
        assert_eq!(g.drift.len(), 3, "{:#?}", g.drift);
        // Generating again moves nothing: the output is a fixed point.
        let again = generate(&g.page, &the_corpus()).expect("generates");
        assert_eq!(again.page, g.page);
        assert!(again.drift.is_empty());
    }

    #[test]
    fn a_hand_edited_row_is_named_by_its_head() {
        let clean = generate(PAGE, &the_corpus()).unwrap().page;
        let edited = clean.replacen(
            "| 1-1 | fingers · lupin | run (wolf + lupin) |",
            "| 1-1 | fingers · lupin | run (lupin) |",
            1,
        );
        assert_ne!(edited, clean);
        let g = generate(&edited, &the_corpus()).unwrap();
        assert_eq!(g.drift.len(), 1);
        assert!(
            g.drift[0].contains("`1-1`")
                && g.drift[0].contains("ch01/ex1-1.lu `run(exit=0)`")
                && g.drift[0].contains("make it `run (wolf + lupin)`"),
            "{:#?}",
            g.drift
        );
    }

    #[test]
    fn a_program_no_row_claims_is_an_error() {
        let mut c = the_corpus();
        c.programs
            .extend([prog("ch22/stray/main.lu", "run(exit=0)")]);
        let Err(e) = generate(PAGE, &c) else {
            panic!("an unclaimed program must fail the generator");
        };
        assert!(format!("{e:#}").contains("ch22/stray/main.lu"), "{e:#}");
    }

    #[test]
    fn a_numbered_sibling_is_not_a_pair() {
        let c = corpus(
            &[
                ("ch09/ex9-7.lu", "run(exit=0)"),
                ("ch09/ex9-70.lu", "fail(E0101)"),
            ],
            &[],
            &[],
        );
        let p: Vec<&str> = c
            .programs_of("9-7")
            .iter()
            .map(|p| p.rel.as_str())
            .collect();
        assert_eq!(p, vec!["ch09/ex9-7.lu"]);
    }

    #[test]
    fn a_directory_mention_is_a_path_segment() {
        assert!(mentions_dir("$ lupin leak/main.lu", "leak"));
        assert!(mentions_dir("in `ch22/leak/`", "leak"));
        assert!(mentions_dir("`leak/` holds", "leak"));
        assert!(!mentions_dir("see unleak/main.lu", "leak"));
        assert!(!mentions_dir("the leak, plainly", "leak"));
    }

    #[test]
    fn a_package_belongs_to_its_label_then_its_one_mention() {
        let master = "\
**Exercise 22-2** *(x)*. `vault/keys.lu` and the run:\n\
$ lupin leak/main.lu\n\
**Exercise 22-9** *(x)*. Build it in `ch22/wordcount/`.\n\
Solution. `ch22/wordcount/` — the entry:\n\
**Exercise 22-13** *(x)*. Run `wordcount/main.lu` twice.\n";
        assert_eq!(sections(master).len(), 3);
        let dir = std::env::temp_dir().join(format!("bs61-pkg-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        for pkg in ["wordcount", "leak"] {
            std::fs::create_dir_all(dir.join("ch22").join(pkg)).unwrap();
            std::fs::write(
                dir.join("ch22").join(pkg).join("main.lu"),
                "//! check: run(exit=0)\n",
            )
            .unwrap();
        }
        std::fs::write(dir.join("ch22/EXERCISES.md"), master).unwrap();
        let owners = package_owners(&dir).expect("owners");
        assert_eq!(owners.get("ch22/leak").map(String::as_str), Some("22-2"));
        // Two sections mention `wordcount/`; the label decides.
        assert_eq!(
            owners.get("ch22/wordcount").map(String::as_str),
            Some("22-9")
        );
        // A second unlabeled mention of `leak/` leaves it ownerless,
        // and that is an error rather than a guess.
        std::fs::write(
            dir.join("ch22/EXERCISES.md"),
            format!("{master}**Exercise 22-14** *(x)*. Compare `leak/` here.\n"),
        )
        .unwrap();
        let owners = package_owners(&dir);
        let _ = std::fs::remove_dir_all(&dir);
        let e = owners.expect_err("two unlabeled owners is an error");
        assert!(format!("{e:#}").contains("ch22/leak/"), "{e:#}");
    }

    #[test]
    fn a_build_cache_is_not_a_package() {
        let dir = std::env::temp_dir().join(format!("bs61-cache-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("appx/.lu-cache/bin")).unwrap();
        std::fs::write(dir.join("appx/.lu-cache/bin/exB-1"), "ELF").unwrap();
        std::fs::create_dir_all(dir.join("appx/notes")).unwrap();
        std::fs::write(dir.join("appx/notes/readme.txt"), "no program").unwrap();
        std::fs::write(dir.join("appx/EXERCISES.md"), "**Exercise B-1** *(x)*.\n").unwrap();
        let owners = package_owners(&dir);
        let _ = std::fs::remove_dir_all(&dir);
        assert!(owners.expect("no package, no error").is_empty());
    }

    #[test]
    fn the_real_index_is_generated() {
        // The check as CI runs it, against the repository's own files.
        let root = crate::repo_root().expect("repo root");
        let page = std::fs::read_to_string(root.join(INDEX)).expect("the index");
        let g = generate(&page, &Corpus::read(&root).expect("corpus")).expect("generates");
        assert!(g.drift.is_empty(), "{:#?}", g.drift);
        assert_eq!(g.page, page, "the totals line is not the table's");
    }
}
