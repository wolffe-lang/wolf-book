//! wolf-book xtask — the book's build system (bs00).
//!
//! Commands, per DESIGN.md §4:
//!   samples            extract + execute every code sample (corpus + book)
//!   contrast           compile + run the vendored other-language contrast code
//!   grammar-sync       vendor the tmLanguage grammars, regenerate highlight.css
//!   render [target]    web | md | pdf | all (default all)
//!   mdbook-preprocess  the mdBook preprocessor protocol (highlight + anchors)
//!   backmatter         regenerate the generated back matter; --check for CI
//!   verify-docs        doc-truth checks (counts, pins, TOC↔stub numbering)
//!   ledger             audit-ledger state; --check gates unfiled open rows

mod backmatter;
mod console;
mod contrast;
mod dialects;
mod directives;
mod fence;
mod highlight;
mod ledger;
mod oslane;
mod preprocess;
mod render;
mod samples;
mod tm;
mod verify;

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = match dispatch(&args) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("error: {e:#}");
            1
        }
    };
    std::process::exit(code);
}

fn dispatch(args: &[String]) -> Result<()> {
    let cmd = args.first().map(String::as_str);
    let rest = if args.is_empty() { &[][..] } else { &args[1..] };
    match cmd {
        Some("samples") => samples::run(&repo_root()?, rest),
        Some("backmatter") => backmatter::run(&repo_root()?, rest),
        Some("contrast") => contrast::run(&repo_root()?),
        Some("grammar-sync") => highlight::grammar_sync(&repo_root()?, rest),
        Some("render") => render::run(&repo_root()?, rest),
        Some("mdbook-preprocess") => preprocess::cmd_preprocess(&repo_root()?, rest),
        Some("verify-docs") => verify::run(&repo_root()?),
        Some("ledger") => ledger::run(&repo_root()?, rest),
        Some("help") | None => {
            print_help();
            Ok(())
        }
        Some(other) => bail!("unknown command `{other}` — try `cargo xtask help`"),
    }
}

fn print_help() {
    eprintln!(
        "cargo xtask <command>\n\n\
         commands:\n\
         \x20 samples [--bless]     extract and execute every code sample; \n\
         \x20                       --bless updates diagnostic snapshots\n\
         \x20 contrast              compile and assert the vendored contrast code\n\
         \x20 grammar-sync [--check] re-vendor grammars from wolf-lsp at the pin;\n\
         \x20                       --check fails on drift without writing\n\
         \x20 backmatter [--check]  regenerate Appendix A and the Solutions page\n\
         \x20                       from the spec and the corpus; --check fails on drift\n\
         \x20 render [web|md|pdf|all]\n\
         \x20 mdbook-preprocess     (internal) mdBook preprocessor protocol\n\
         \x20 verify-docs           doc-truth checks\n\
         \x20 ledger [--check]      audit-ledger state per chapter; --check fails\n\
         \x20                       on open ba:* rows neither filed nor waived"
    );
}

/// The wolf-book repository root: the nearest ancestor of the current
/// directory containing `wolf-toolchain.toml`.
pub fn repo_root() -> Result<PathBuf> {
    let mut dir = std::env::current_dir().context("cannot read current dir")?;
    loop {
        if dir.join("wolf-toolchain.toml").is_file() {
            return Ok(dir);
        }
        if !dir.pop() {
            bail!("not inside wolf-book (no wolf-toolchain.toml above the current directory)");
        }
    }
}

/// Pinned-toolchain values from wolf-toolchain.toml.
#[derive(Debug, Clone)]
pub struct Pins {
    pub wolf_lsp_rev: String,
}

pub fn load_pins(root: &Path) -> Result<Pins> {
    let text = std::fs::read_to_string(root.join("wolf-toolchain.toml"))
        .context("reading wolf-toolchain.toml")?;
    let value: toml::Value = text.parse().context("parsing wolf-toolchain.toml")?;
    let rev = |table: &str| -> Result<String> {
        Ok(value
            .get(table)
            .and_then(|t| t.get("rev"))
            .and_then(|v| v.as_str())
            .with_context(|| format!("wolf-toolchain.toml: [{table}].rev missing"))?
            .to_string())
    };
    Ok(Pins {
        wolf_lsp_rev: rev("wolf-lsp")?,
    })
}
