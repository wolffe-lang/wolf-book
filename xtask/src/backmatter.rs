//! `cargo xtask backmatter [--check]` — the two back-matter pages that
//! are generated rather than written: Appendix A (the surface grammar,
//! from the pinned spec's `grammar.ebnf`) and the Solutions page (every
//! exercise's solution, from the corpus under `principles/`).
//!
//! The generated files live in the tree like any other page, so the web,
//! markdown, and PDF targets need no special case. `--check`
//! regenerates in memory and fails on drift, which is what CI runs; the
//! same call also compares the vendored spec artifacts against a sibling
//! wolf-lang checkout when one is present, and says so loudly when it is
//! not.

use anyhow::{bail, Context, Result};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// One printed exercise: the chapter file that prints it and the section
/// heading it sits under.
pub struct Printed {
    pub chapter: u32,
    pub section: String,
}

/// Chapters whose exercises the Solutions page publishes. Chapter 31 is
/// the one exception in the book (EXERCISES.md §4: the solo publishes
/// milestone checkpoints instead of answers) and chapter 32 sets none.
const SOLUTION_CHAPTERS: std::ops::RangeInclusive<u32> = 1..=30;

pub fn run(root: &Path, args: &[String]) -> Result<()> {
    let check = args.iter().any(|a| a == "--check");
    let pages = [
        ("book/back/appendix-a.md", appendix_a(root)?),
        ("book/back/solutions.md", solutions(root)?),
    ];
    let mut drift = Vec::new();
    for (rel, generated) in pages {
        let path = root.join(rel);
        let current = std::fs::read_to_string(&path).unwrap_or_default();
        if current == generated {
            println!("backmatter: {rel} up to date");
            continue;
        }
        if check {
            drift.push(rel.to_string());
        } else {
            std::fs::write(&path, &generated)
                .with_context(|| format!("writing {}", path.display()))?;
            println!("backmatter: wrote {rel}");
        }
    }
    spec_vendor_check(root)?;
    if !drift.is_empty() {
        bail!(
            "backmatter: {} is generated and has drifted — run `cargo xtask backmatter`",
            drift.join(", ")
        );
    }
    Ok(())
}

/// Appendix A: the surface grammar, verbatim from the vendored EBNF.
fn appendix_a(root: &Path) -> Result<String> {
    let ebnf = read_vendored(root, "grammar.ebnf")?;
    let body: String = ebnf
        .lines()
        .skip_while(|l| l.starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");
    let mut out = String::from(
        "# Appendix A — Grammar summary\n\
         \n\
         The whole surface grammar of wolf, in one place. It is not written\n\
         here: it is extracted from the specification's grammar document and\n\
         copied into this book by CI, so a production on this page and a\n\
         production the parser implements cannot disagree. Where this appendix\n\
         and a chapter differ about what parses, this appendix is right and the\n\
         chapter has a bug.\n\
         \n\
         Terminals are quoted. `IDENT`, `INT`, `FLOAT`, `STRING` and the\n\
         character classes they are built from come first; the item, type,\n\
         statement and expression grammars follow. `TERM` is a statement\n\
         terminator: a newline, or a semicolon where one line holds two\n\
         statements.\n\
         \n\
         ```ebnf\n",
    );
    out.push_str(body.trim_end());
    out.push_str("\n```\n");
    Ok(out)
}

/// Every exercise the chapters actually print, with the section it ends.
/// An exercise held out of a chapter has no reader-facing stem, so the
/// Solutions page does not publish an answer to a question the reader
/// was never asked.
pub fn printed_exercises(root: &Path) -> Result<BTreeMap<(u32, u32), Printed>> {
    let mut found = BTreeMap::new();
    for ch in 1..=32u32 {
        let path = root.join(format!("book/ch{ch:02}.md"));
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let text = crate::render::strip_html_comments(&text);
        let mut section = format!("{ch}.0");
        for line in text.lines() {
            if let Some(rest) = line.strip_prefix("## ") {
                if let Some(num) = rest.split_whitespace().next() {
                    if num.starts_with(char::is_numeric) {
                        section = num.to_string();
                    }
                }
            }
            if let Some(key) = exercise_marker(line) {
                found.insert(
                    key,
                    Printed {
                        chapter: ch,
                        section: section.clone(),
                    },
                );
            }
        }
    }
    Ok(found)
}

/// The Solutions page: one collapsed block per printed exercise, in
/// chapter and then numeric order, assembled from the two places
/// solutions live.
fn solutions(root: &Path) -> Result<String> {
    let printed = printed_exercises(root)?;
    let mut blocks: BTreeMap<(u32, u32), String> = BTreeMap::new();
    let exemplar = root.join("principles/EXERCISES.md");
    harvest(&exemplar, &mut blocks)?;
    for ch in SOLUTION_CHAPTERS {
        let path = root.join(format!("principles/exercises/ch{ch:02}/EXERCISES.md"));
        if path.is_file() {
            harvest(&path, &mut blocks)?;
        }
    }
    let mut out = String::from(
        "# Solutions\n\
         \n\
         Every exercise in chapters 1 through 30 has a solution here, and\n\
         every solution program is a sample like any other: extracted,\n\
         executed, and snapshot-checked in the same CI run as the chapters. A\n\
         solution that stops compiling fails the book's build. Outputs are\n\
         pasted from those runs.\n\
         \n\
         Solutions are collapsed. Open one when you have written yours, or\n\
         when you are stuck in the specific way that a hint cannot reach.\n\
         Reading a solution before attempting the exercise costs you the\n\
         exercise; the book has no way to stop you and no interest in trying.\n\
         \n\
         The solo project publishes six checkpoints inside its own chapter\n\
         instead of answers, and it is the only page in the book that\n\
         withholds one. The coda sets no exercises.\n",
    );
    let mut current_chapter = 0;
    let mut published = 0usize;
    for ((ch, num), body) in &blocks {
        let Some(where_printed) = printed.get(&(*ch, *num)) else {
            continue;
        };
        if *ch != current_chapter {
            current_chapter = *ch;
            out.push_str(&format!("\n## Chapter {ch}\n"));
        }
        // The backlink to the section that set the exercise, written as
        // raw HTML rather than markdown (wolf-book#3). The `<summary>`
        // line is inside a raw-HTML block, so a markdown parser leaves
        // its contents alone: `[§3.4](../ch03.md#3.4)` reached the web
        // edition as punctuation, on all 280 of them. An `<a>` is what
        // the block's own grammar can carry — mdBook rewrites the `.md`
        // to `.html` in an href it did not itself build, so the target
        // is still written the way every other link in this repository
        // is — and the print edition turns the same element into a
        // typst cross-reference to the section's label
        // (`render::typst_inline`), which is the half the site lane
        // declined to guess at.
        let link = format!(
            "<a href=\"../ch{:02}.md#{}\">§{}</a>",
            where_printed.chapter, where_printed.section, where_printed.section
        );
        out.push_str(&format!(
            "\n<details>\n<summary>Exercise {ch}-{num}. {link}</summary>\n\n{}\n</details>\n",
            body.trim_end()
        ));
        published += 1;
    }
    println!(
        "backmatter: {published} solution(s) published, {} on file",
        blocks.len()
    );
    if blocks.is_empty() {
        bail!("backmatter: no solutions found under principles/ — the corpus moved");
    }
    Ok(out)
}

/// Pull `**Exercise N-M** …` blocks out of one solutions document. A
/// block runs to the next exercise marker or the next `##` heading,
/// whichever comes first; the section headings themselves are repo
/// structure and stay out of the reader's page.
fn harvest(path: &Path, into: &mut BTreeMap<(u32, u32), String>) -> Result<()> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut key: Option<(u32, u32)> = None;
    let mut buf = String::new();
    let mut in_fence = false;
    let flush = |key: &mut Option<(u32, u32)>,
                 buf: &mut String,
                 into: &mut BTreeMap<(u32, u32), String>| {
        if let Some(k) = key.take() {
            let body = buf.trim().to_string();
            if !body.is_empty() {
                into.insert(k, body);
            }
        }
        buf.clear();
    };
    for line in text.lines() {
        if line.starts_with("```") {
            in_fence = !in_fence;
        }
        if !in_fence {
            if let Some(k) = exercise_marker(line) {
                flush(&mut key, &mut buf, into);
                key = Some(k);
            } else if line.starts_with("## ") {
                flush(&mut key, &mut buf, into);
                continue;
            } else if line.starts_with("### ") {
                // Deeper headings are the same repo structure `## `
                // already keeps off the reader's page; skip them
                // without ending the block.
                continue;
            }
        }
        if key.is_some() {
            buf.push_str(line);
            buf.push('\n');
        }
    }
    flush(&mut key, &mut buf, into);
    Ok(())
}

/// `**Exercise 12-3**` → `(12, 3)`. Appendix exercises (`B-1`, `C-2`)
/// are not chapter exercises and are not published here.
fn exercise_marker(line: &str) -> Option<(u32, u32)> {
    let rest = line.trim_start().strip_prefix("**Exercise ")?;
    let id = rest.split("**").next()?;
    let (ch, num) = id.split_once('-')?;
    Some((ch.trim().parse().ok()?, num.trim().parse().ok()?))
}

fn read_vendored(root: &Path, name: &str) -> Result<String> {
    let path = root.join("vendor/spec").join(name);
    std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))
}

/// The vendored spec artifacts against a sibling wolf-lang checkout.
/// Absent sibling is a loud SKIP, never a silent pass (house rule).
fn spec_vendor_check(root: &Path) -> Result<()> {
    let sibling = match sibling_spec(root) {
        Some(dir) => dir,
        None => {
            println!(
                "backmatter: SKIP spec drift check — no wolf-lang checkout beside this repo \
                 (set WOLF_LANG_PATH); the vendored copies under vendor/spec/ are what \
                 Appendix A and the clause checks read"
            );
            return Ok(());
        }
    };
    let mut drift = Vec::new();
    for name in ["grammar.ebnf", "anchors.json"] {
        let theirs = std::fs::read_to_string(sibling.join(name))
            .with_context(|| format!("reading {}", sibling.join(name).display()))?;
        if read_vendored(root, name)? != theirs {
            drift.push(name);
        }
    }
    if drift.is_empty() {
        println!("backmatter: vendored spec artifacts match the sibling wolf-lang checkout");
        Ok(())
    } else {
        bail!(
            "backmatter: vendor/spec/{} differs from the sibling wolf-lang checkout — \
             re-vendor deliberately, at a pin bump",
            drift.join(", vendor/spec/")
        )
    }
}

fn sibling_spec(root: &Path) -> Option<PathBuf> {
    let candidates = [
        std::env::var_os("WOLF_LANG_PATH").map(PathBuf::from),
        root.parent().map(|p| p.join("wolf-lang")),
    ];
    candidates
        .into_iter()
        .flatten()
        .map(|p| p.join("spec"))
        .find(|p| p.join("grammar.ebnf").is_file())
}
