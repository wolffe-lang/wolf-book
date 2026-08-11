//! `cargo xtask contrast` — the other languages the book quotes.
//!
//! Chapter 7 §7.6 prints a Rust program, because the honest-contrast
//! rule (TONE.md §1) means showing what the other design buys rather
//! than describing it. Contrast code rots exactly like wolf code does,
//! so it is vendored under `samples/contrast/` and compiled here: every
//! `.rs` file is built as a test binary with warnings denied and run,
//! which compiles the code, runs its assertions, and proves the printed
//! claims about it.

use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

pub fn run(root: &Path) -> Result<()> {
    let dir = root.join("samples/contrast");
    if !dir.is_dir() {
        bail!("samples/contrast is missing — the book quotes contrast code from there");
    }
    let out_dir = root.join("target/contrast");
    std::fs::create_dir_all(&out_dir)?;

    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("rs"))
        .collect();
    files.sort();
    if files.is_empty() {
        bail!("samples/contrast holds no .rs files");
    }

    let mut failures = Vec::new();
    for path in &files {
        let stem = path.file_stem().unwrap().to_string_lossy().into_owned();
        let bin = out_dir.join(if cfg!(windows) {
            format!("{stem}.exe")
        } else {
            stem.clone()
        });
        let build = Command::new("rustc")
            .arg("--edition")
            .arg("2021")
            .arg("--test")
            .arg("--deny")
            .arg("warnings")
            .arg(path)
            .arg("-o")
            .arg(&bin)
            .output()
            .with_context(|| format!("running rustc on {}", path.display()))?;
        if !build.status.success() {
            failures.push(format!(
                "{}: rustc rejected it\n{}",
                path.display(),
                String::from_utf8_lossy(&build.stderr).trim()
            ));
            continue;
        }
        let test = Command::new(&bin)
            .output()
            .with_context(|| format!("running {}", bin.display()))?;
        if !test.status.success() {
            failures.push(format!(
                "{}: its assertions failed\n{}",
                path.display(),
                String::from_utf8_lossy(&test.stdout).trim()
            ));
            continue;
        }
        println!(
            "contrast: ok {}",
            path.strip_prefix(root).unwrap().display()
        );
    }

    // Doc truth: every ```rust block in the book must appear verbatim in
    // one of these files, so the printed program is the compiled program.
    let sources: Vec<String> = files
        .iter()
        .map(std::fs::read_to_string)
        .collect::<std::io::Result<_>>()?;
    let mut printed = 0usize;
    for (md, block) in crate::contrast::rust_blocks(root)? {
        printed += 1;
        if !sources.iter().any(|s| s.contains(block.trim_end())) {
            failures.push(format!(
                "{}: a ```rust block is not vendored under samples/contrast/ \
                 — the book prints Rust that CI does not compile",
                md.display()
            ));
        }
    }

    if failures.is_empty() {
        println!(
            "contrast: {} file(s) compiled and asserted, {printed} book block(s) matched",
            files.len()
        );
        Ok(())
    } else {
        for f in &failures {
            eprintln!("contrast: FAIL {f}");
        }
        bail!("{} contrast failure(s)", failures.len())
    }
}

/// Every ```rust fence in `book/**/*.md`, with the file it came from.
fn rust_blocks(root: &Path) -> Result<Vec<(std::path::PathBuf, String)>> {
    let base = root.join("book");
    let mut out = Vec::new();
    let mut mds: Vec<std::path::PathBuf> = Vec::new();
    collect_md(&base, &mut mds)?;
    mds.sort();
    for md in mds {
        let text = std::fs::read_to_string(&md)?;
        for seg in crate::fence::segments(&text) {
            if let crate::fence::Segment::Fence(f) = seg {
                if f.info.trim() == "rust" {
                    out.push((md.clone(), f.content.clone()));
                }
            }
        }
    }
    Ok(out)
}

fn collect_md(dir: &Path, out: &mut Vec<std::path::PathBuf>) -> Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_md(&path, out)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("md") {
            out.push(path);
        }
    }
    Ok(())
}
