//! Console-block checking — the second half of "the book cannot rot".
//!
//! `wolf` and `lupin` blocks were executed from the day the rig existed;
//! ```` ```console ```` blocks were not, so a prompt line and its pasted
//! output could drift apart at a pin bump and nothing would notice. This
//! module closes that hole: a console block whose commands are all
//! recognized is *replayed* — the program printed above it is written
//! under the file name the prompt uses, the commands run in order, and
//! the block's remaining lines are compared byte-for-byte against what
//! the tools actually said.
//!
//! Recognized commands: `lupin …`, `wolf …`, `./name` (a binary `wolf
//! build` just produced), and `echo $?`. `&&` chains them. A block
//! containing anything else is reported as skipped, loudly and by name —
//! never silently unchecked.

use crate::samples::Tools;
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

/// One ```` ```console ```` block, with the program that precedes it.
#[derive(Debug, Clone)]
pub struct ConsoleBlock {
    /// `ch01`, `front-notation`, … — the markdown file's extraction stem.
    pub stem: String,
    /// Display path of the markdown file, for messages.
    pub md: PathBuf,
    /// One-based line of the opening fence.
    pub line: usize,
    /// The block's content, verbatim.
    pub text: String,
    /// The nearest preceding wolf program in the same file, if any.
    pub program: Option<String>,
    /// `console,from(id)` — bind the block to a named sample instead of
    /// the program above it, for the pages where the two differ.
    pub from: Option<String>,
    /// `console,in(pkg/name)` — stage `samples/<name>` into the replay
    /// directory first. A multi-package walkthrough needs a project on
    /// disk, and the project is a fixture in this repository.
    pub fixture: Option<String>,
}

/// One `$ …` command and the output the book says it produces.
struct Step {
    command: String,
    expected: Vec<String>,
}

fn parse_steps(text: &str) -> Result<Vec<Step>> {
    let mut steps: Vec<Step> = Vec::new();
    for line in text.lines() {
        match line.strip_prefix("$ ") {
            Some(cmd) => steps.push(Step {
                command: cmd.to_string(),
                expected: Vec::new(),
            }),
            None => match steps.last_mut() {
                Some(step) => step.expected.push(line.to_string()),
                None => bail!("console block does not start with a `$ ` prompt line"),
            },
        }
    }
    if steps.is_empty() {
        bail!("console block has no `$ ` prompt line");
    }
    Ok(steps)
}

/// Split a command into words, honoring single and double quotes. Not a
/// shell: no globbing, no substitution, no redirection — a block that
/// needs any of those is one this module declines to replay.
fn words(cmd: &str) -> Option<Vec<String>> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quote: Option<char> = None;
    let mut started = false;
    for c in cmd.chars() {
        match (quote, c) {
            (Some(q), _) if c == q => quote = None,
            (Some(_), _) => cur.push(c),
            (None, '\'') | (None, '"') => {
                quote = Some(c);
                started = true;
            }
            (None, ' ') | (None, '\t') => {
                if started || !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                    started = false;
                }
            }
            // Anything a shell would interpret: decline the whole block.
            (None, '|') | (None, '>') | (None, '<') | (None, ';') | (None, '(') | (None, ')') => {
                return None
            }
            (None, _) => cur.push(c),
        }
    }
    if quote.is_some() {
        return None;
    }
    if started || !cur.is_empty() {
        out.push(cur);
    }
    Some(out)
}

/// What one recognized command does when it runs.
enum Verb<'a> {
    Lupin(Vec<String>),
    Wolf(Vec<String>),
    Local(&'a str, Vec<String>),
    LastExit,
}

fn classify(argv: &[String]) -> Option<Verb<'_>> {
    let (head, rest) = argv.split_first()?;
    let rest: Vec<String> = rest.to_vec();
    match head.as_str() {
        "lupin" => Some(Verb::Lupin(rest)),
        "wolf" => Some(Verb::Wolf(rest)),
        "echo" if rest == ["$?"] => Some(Verb::LastExit),
        _ if head.starts_with("./") => Some(Verb::Local(&head[2..], rest)),
        _ => None,
    }
}

/// A block is replayable only if every command in it is recognized. The
/// native half (`wolf build`, `./prog`) needs a linker and an
/// executable bit, so it replays on unix and reports itself elsewhere.
fn replayable(steps: &[Step]) -> std::result::Result<(), String> {
    for step in steps {
        for piece in step.command.split(" && ") {
            let Some(argv) = words(piece) else {
                return Err(format!("`{piece}` needs a shell"));
            };
            match classify(&argv) {
                None => return Err(format!("`{piece}` is not a pinned tool")),
                Some(Verb::Local(..)) if !cfg!(unix) => {
                    return Err(format!("`{piece}` runs a built binary (unix lane only)"))
                }
                Some(Verb::Wolf(args))
                    if !cfg!(unix) && args.first().is_some_and(|a| a == "build" || a == "run") =>
                {
                    return Err(format!("`{piece}` links a binary (unix lane only)"))
                }
                Some(_) => {}
            }
        }
    }
    Ok(())
}

fn run_capture(mut cmd: Command) -> Result<(i32, String)> {
    let out = cmd.output().with_context(|| format!("spawning {cmd:?}"))?;
    let mut text = String::from_utf8_lossy(&out.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    Ok((out.status.code().unwrap_or(-1), text))
}

/// Replay one block. Returns the transcript the tools produced, in the
/// book's own shape: the `$ ` lines kept, the output lines replaced.
fn replay(
    tools: &Tools,
    dir: &Path,
    program: Option<&String>,
    steps: &[Step],
) -> Result<Vec<String>> {
    std::fs::create_dir_all(dir)?;
    let mut transcript: Vec<String> = Vec::new();
    let mut last_exit = 0i32;
    for step in steps {
        transcript.push(format!("$ {}", step.command));
        for piece in step.command.split(" && ") {
            let argv = words(piece).expect("replayable() vetted this");
            let verb = classify(&argv).expect("replayable() vetted this");
            // Any `.lu` an argument names is the program printed above.
            for arg in &argv {
                if arg.ends_with(".lu") && !dir.join(arg).exists() {
                    let Some(program) = program else {
                        bail!("`{piece}` names {arg} but no wolf program precedes the block");
                    };
                    std::fs::write(dir.join(arg), program)?;
                }
            }
            let (code, text) = match verb {
                Verb::LastExit => (last_exit, format!("{last_exit}\n")),
                Verb::Lupin(args) => {
                    let mut cmd = Command::new(&tools.lupin);
                    cmd.args(&args).current_dir(dir);
                    run_capture(cmd)?
                }
                Verb::Wolf(args) => {
                    let mut cmd = Command::new(&tools.wolf);
                    cmd.args(&args).current_dir(dir);
                    run_capture(cmd)?
                }
                Verb::Local(name, args) => {
                    let mut cmd = Command::new(dir.join(name));
                    cmd.args(&args).current_dir(dir);
                    run_capture(cmd)?
                }
            };
            for line in text.lines() {
                transcript.push(line.trim_end().to_string());
            }
            last_exit = code;
            // `&&` stops at the first failure, as the shell would.
            if code != 0 {
                break;
            }
        }
    }
    Ok(transcript)
}

/// Copy a fixture tree into a fresh replay directory.
fn stage(from: &Path, to: &Path) -> Result<()> {
    let _ = std::fs::remove_dir_all(to);
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)
        .with_context(|| format!("reading {}", from.display()))?
        .flatten()
    {
        let src = entry.path();
        let dst = to.join(entry.file_name());
        if src.is_dir() {
            stage(&src, &dst)?;
        } else {
            std::fs::copy(&src, &dst).with_context(|| format!("copying {}", src.display()))?;
        }
    }
    Ok(())
}

/// The ledger key for a block: repo-relative, forward slashes, and the
/// opening fence's line — `book/ch23.md:117`. The absolute path the
/// messages carry is the runner's; the key has to be the same three
/// characters on all three hosts.
pub fn block_key(root: &Path, block: &ConsoleBlock) -> String {
    let rel = block
        .md
        .strip_prefix(root)
        .unwrap_or(&block.md)
        .to_string_lossy()
        .replace('\\', "/");
    format!("{rel}:{}", block.line)
}

/// What a console-check pass found.
pub struct Report {
    pub checked: usize,
    pub skipped: Vec<String>,
    /// Blocks that matched a per-host declared transcript rather than
    /// the book's — reported by name, never silent.
    pub declared: Vec<String>,
    pub failures: Vec<String>,
}

pub fn check(
    root: &Path,
    tools: &Tools,
    blocks: &[ConsoleBlock],
    programs: &std::collections::BTreeMap<String, String>,
    os: &crate::oslane::Ledger,
) -> Result<Report> {
    let base = root.join("samples/extracted/console");
    let _ = std::fs::remove_dir_all(&base);
    let mut report = Report {
        checked: 0,
        skipped: Vec::new(),
        declared: Vec::new(),
        failures: Vec::new(),
    };
    for block in blocks {
        let where_ = format!("{}:{}", block.md.display(), block.line);
        let steps = match parse_steps(&block.text) {
            Ok(s) => s,
            Err(e) => {
                report.failures.push(format!("{where_}: {e:#}"));
                continue;
            }
        };
        if let Err(why) = replayable(&steps) {
            report.skipped.push(format!("{where_}: {why}"));
            continue;
        }
        let program = match &block.from {
            Some(id) => match programs.get(id) {
                Some(p) => Some(p),
                None => {
                    report
                        .failures
                        .push(format!("{where_}: console,from({id}) — no such sample"));
                    continue;
                }
            },
            None => block.program.as_ref(),
        };
        let dir = base.join(&block.stem).join(format!("l{}", block.line));
        // A fixture block gets the project staged into a private copy —
        // `wolf add`/`update` rewrite manifests and ledgers, and the
        // checked-in fixture stays pristine.
        if let Some(name) = &block.fixture {
            let src = root.join("samples").join(name);
            if !src.is_dir() {
                report.failures.push(format!(
                    "{where_}: console,in({name}) — no fixture at samples/{name}"
                ));
                continue;
            }
            if let Err(e) = stage(&src, &dir) {
                report
                    .failures
                    .push(format!("{where_}: staging samples/{name}: {e:#}"));
                continue;
            }
        }
        let actual = match replay(tools, &dir, program, &steps) {
            Ok(t) => t,
            Err(e) => {
                report.failures.push(format!("{where_}: {e:#}"));
                continue;
            }
        };
        let expected: Vec<String> = block
            .text
            .lines()
            .map(|l| l.trim_end().to_string())
            .collect();
        // A host whose tool text differs declares the difference in
        // full (samples-os.toml). The block is still replayed and still
        // byte-compared — against the declaration instead of the page.
        if let Some(row) = os.transcript(&block_key(root, block)) {
            let declared: Vec<String> = row
                .expect
                .lines()
                .map(|l| l.trim_end().to_string())
                .collect();
            match declared_verdict(&actual, &expected, &declared) {
                Declared::Stale => report.failures.push(format!(
                    "{where_}: this host now prints the book's own transcript, so \
                     samples-os.toml's {} row for this block is stale — {} \
                     landed; remove the row",
                    row.os, row.filed
                )),
                Declared::Held => {
                    report.checked += 1;
                    report.declared.push(format!(
                        "{where_} ({}): {} — {}, retires at {}",
                        row.os, row.note, row.filed, row.retires
                    ));
                }
                Declared::Drifted => report.failures.push(format!(
                    "{where_}: the declared {} transcript drifted from the real run\n     declared:\n{}\n     actual:\n{}",
                    row.os,
                    indent(&declared),
                    indent(&actual),
                )),
            }
            continue;
        }
        if actual != expected {
            report.failures.push(format!(
                "{where_}: console block drifted from the real run\n     expected:\n{}\n     actual:\n{}",
                indent(&expected),
                indent(&actual),
            ));
            continue;
        }
        report.checked += 1;
    }
    Ok(report)
}

/// What a declared per-host transcript row is worth on this run.
#[derive(Debug, PartialEq, Eq)]
enum Declared {
    /// The host said what the row says. The row still earns its keep.
    Held,
    /// The host said what the BOOK says: the difference the row exists
    /// to record is gone, so the row is a lie now. Hard error — the
    /// same discipline `samples-pending.toml` applies to a flip.
    Stale,
    /// The host said a third thing. The row was the book's claim about
    /// this host, and the claim is wrong.
    Drifted,
}

/// The order matters: `Stale` is decided FIRST, so a host that starts
/// agreeing with the page is never reported as mere drift, and a row
/// whose `expect` was written to equal the book's own transcript reads
/// as stale rather than passing quietly. That is the safe direction: it
/// makes a useless row loud.
fn declared_verdict(actual: &[String], book: &[String], declared: &[String]) -> Declared {
    if actual == book {
        Declared::Stale
    } else if actual == declared {
        Declared::Held
    } else {
        Declared::Drifted
    }
}

fn indent(lines: &[String]) -> String {
    lines
        .iter()
        .map(|l| format!("       | {l}"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_split_on_prompts() {
        let steps = parse_steps("$ lupin hello.lu\nhello, wolf\n$ echo $?\n0\n").unwrap();
        assert_eq!(steps.len(), 2);
        assert_eq!(steps[0].command, "lupin hello.lu");
        assert_eq!(steps[0].expected, vec!["hello, wolf"]);
        assert_eq!(steps[1].command, "echo $?");
    }

    #[test]
    fn a_block_without_a_prompt_is_an_error() {
        assert!(parse_steps("hello, wolf\n").is_err());
    }

    #[test]
    fn words_honors_quotes() {
        assert_eq!(
            words("lupin eval '\"{3.14:>8.2}\"'").unwrap(),
            vec!["lupin", "eval", "\"{3.14:>8.2}\""]
        );
    }

    #[test]
    fn words_declines_shell_syntax() {
        assert!(words("lupin hello.lu | head -1").is_none());
        assert!(words("(cd wolf-lang && cargo build)").is_none());
    }

    #[test]
    fn unknown_commands_are_skipped_not_failed() {
        let steps = parse_steps("$ grep -n 'mut ' shelf.lu\n3:fn add(mut s: Shelf)\n").unwrap();
        assert!(replayable(&steps).is_err());
    }

    #[test]
    fn the_pinned_tools_are_replayable() {
        let steps = parse_steps("$ lupin hello.lu\nhello, wolf\n$ echo $?\n0\n").unwrap();
        assert!(replayable(&steps).is_ok());
    }

    #[test]
    fn a_declared_transcript_holds_when_the_host_says_it() {
        let book = vec!["$ wolf add".to_string(), "created app/wolf.pkg".to_string()];
        let win = vec![
            "$ wolf add".to_string(),
            r"created app\wolf.pkg".to_string(),
        ];
        assert_eq!(declared_verdict(&win, &book, &win), Declared::Held);
    }

    #[test]
    fn a_declared_transcript_is_stale_once_the_host_agrees_with_the_page() {
        // wolf-lang#222 lands: the row is a lie now, and it must not
        // pass quietly on its way out.
        let book = vec!["$ wolf add".to_string(), "created app/wolf.pkg".to_string()];
        let win = vec![
            "$ wolf add".to_string(),
            r"created app\wolf.pkg".to_string(),
        ];
        assert_eq!(declared_verdict(&book, &book, &win), Declared::Stale);
    }

    #[test]
    fn a_third_answer_is_drift_not_a_shrug() {
        let book = vec!["$ wolf add".to_string(), "created app/wolf.pkg".to_string()];
        let win = vec![
            "$ wolf add".to_string(),
            r"created app\wolf.pkg".to_string(),
        ];
        let other = vec!["$ wolf add".to_string(), "created APP/WOLF.PKG".to_string()];
        assert_eq!(declared_verdict(&other, &book, &win), Declared::Drifted);
    }

    #[test]
    fn a_block_key_is_repo_relative_with_forward_slashes() {
        let root = Path::new("/w/wolf-book");
        let block = ConsoleBlock {
            stem: "ch23".into(),
            md: root.join("book").join("ch23.md"),
            line: 117,
            text: String::new(),
            program: None,
            from: None,
            fixture: None,
        };
        assert_eq!(block_key(root, &block), "book/ch23.md:117");
    }

    #[test]
    fn classify_reads_each_verb() {
        assert!(matches!(
            classify(&["lupin".into(), "a.lu".into()]),
            Some(Verb::Lupin(_))
        ));
        assert!(matches!(
            classify(&["wolf".into(), "build".into()]),
            Some(Verb::Wolf(_))
        ));
        assert!(matches!(
            classify(&["echo".into(), "$?".into()]),
            Some(Verb::LastExit)
        ));
        assert!(matches!(
            classify(&["./hello".into()]),
            Some(Verb::Local("hello", _))
        ));
        assert!(classify(&["cargo".into(), "build".into()]).is_none());
    }
}
