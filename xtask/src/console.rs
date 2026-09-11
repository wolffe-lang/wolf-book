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
    /// Set for a block in the EXERCISE CORPUS (wolf-book#24): the
    /// exercise directory this page lives in, staged into the replay
    /// directory so the solutions the prompts name are on disk. A
    /// corpus block is bound to FILES rather than to the program
    /// printed above it — `$ lupin ex11-1.lu` means the checked-in
    /// `ch11/ex11-1.lu`, which the samples runner already executes —
    /// so the two lanes grade the same bytes from opposite ends.
    pub corpus_dir: Option<PathBuf>,
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

/// Why a block was not replayed, and whether that is a fact about the
/// BLOCK or about this HOST.
///
/// The distinction is load-bearing for the declined ledger (bs41): a
/// block the unix lane replays and windows cannot is not a hole in the
/// book, it is one third of the matrix standing down, and it must not
/// want a declared row. A block nothing can replay anywhere is the
/// hole, and every one of those carries a row.
pub struct Decline {
    pub why: String,
    /// True when the only thing in the way is the lane this is running
    /// on — the native half (`wolf build`, `./prog`) needs a linker and
    /// an executable bit, so it replays on unix and reports itself
    /// elsewhere.
    pub host_only: bool,
}

fn here(why: String) -> Decline {
    Decline {
        why,
        host_only: false,
    }
}

fn off_lane(why: String) -> Decline {
    Decline {
        why,
        host_only: true,
    }
}

/// A block is replayable only if every command in it is recognized.
fn replayable(steps: &[Step]) -> std::result::Result<(), Decline> {
    for step in steps {
        for piece in step.command.split(" && ") {
            let Some(argv) = words(piece) else {
                return Err(here(format!("`{piece}` needs a shell")));
            };
            match classify(&argv) {
                None => return Err(here(format!("`{piece}` is not a pinned tool"))),
                Some(Verb::Local(..)) if !cfg!(unix) => {
                    return Err(off_lane(format!(
                        "`{piece}` runs a built binary (unix lane only)"
                    )))
                }
                Some(Verb::Wolf(args))
                    if !cfg!(unix) && args.first().is_some_and(|a| a == "build" || a == "run") =>
                {
                    return Err(off_lane(format!(
                        "`{piece}` links a binary (unix lane only)"
                    )))
                }
                Some(_) => {}
            }
        }
    }
    Ok(())
}

/// `wolf --explain E1001` and nothing else: the catalog read.
///
/// The corpus rule below draws its line at the FILE, and this is the
/// one verb admitted with no file behind it. It earns the exception on
/// three counts: it takes no input, touches no disk and reaches no
/// phase, so there is nothing for it to be about except the catalog;
/// the catalog it prints from is already compared in both directions by
/// `cargo xtask verify-docs`, so admitting it costs nothing and closes
/// the one surface of that catalog nothing compared — the explain text
/// itself; and its four blocks (wolf-book#29 §3) were declined only
/// because the line was drawn at the file, never because anything was
/// missing. `lupin eval '…'` also has no file behind it and is NOT
/// admitted here: an expression typed at a tool is the REPL lane's
/// shape, and it waits for wolf-interp is08 with the four sessions that
/// open on a bare `$ lupin`.
fn is_catalog_read(argv: &[String]) -> bool {
    matches!(classify(argv), Some(Verb::Wolf(args))
        if args.len() == 2 && args[0] == "--explain")
}

/// A CORPUS block is replayed only when every command in it is a plain
/// run of a solution: a `lupin` or `wolf` invocation naming a `.lu`
/// that exists in the exercise directory, the `wolf --explain E…` that
/// reads the catalog, or the `echo $?` that reads the exit the previous
/// command produced. Everything else is reported by name.
///
/// The line is drawn at the FILE rather than at the verb, and that is
/// the whole design. A prompt that names a checked-in solution has
/// something on disk to replay against; a prompt that does not —
/// `lupin eval '…'`, `wolf interface`, the REPL sessions that open
/// with a bare `$ lupin` — needs something this module cannot conjure:
/// an expression to type, a package on disk, or an interactive session.
/// Those are counted and named, never silently unchecked, and since
/// bs41 each also carries a DECLARED row in `samples-declined.toml`, so
/// "not replayed" is a sentence someone wrote rather than a silence.
///
/// `conform-run` is admitted here and was not before, because the thing
/// that made it look unreplayable turned out to be a stream and not a
/// verb — see `conform_run_stdout`.
fn plain_run_of_a_solution(dir: &Path, steps: &[Step]) -> std::result::Result<(), Decline> {
    for step in steps {
        for piece in step.command.split(" && ") {
            let Some(argv) = words(piece) else {
                return Err(here(format!("`{piece}` needs a shell")));
            };
            if is_catalog_read(&argv) {
                continue;
            }
            match classify(&argv) {
                Some(Verb::LastExit) => {}
                Some(Verb::Lupin(args)) | Some(Verb::Wolf(args)) => {
                    if !args
                        .iter()
                        .any(|a| a.ends_with(".lu") && dir.join(a).exists())
                    {
                        return Err(here(format!(
                            "`{piece}` names no solution in this exercise directory"
                        )));
                    }
                }
                Some(Verb::Local(name, _)) => {
                    return Err(here(format!(
                        "`{piece}` runs the binary `{name}`, not a solution"
                    )))
                }
                None => return Err(here(format!("`{piece}` is not a pinned tool"))),
            }
        }
    }
    Ok(())
}

/// The two streams, kept apart. Every caller but one joins them back
/// together in the order a terminal would show them; the one that does
/// not is `wolf conform-run`, whose stdout is addressed to a machine.
fn run_capture(mut cmd: Command) -> Result<(i32, String, String)> {
    let out = cmd.output().with_context(|| format!("spawning {cmd:?}"))?;
    Ok((
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    ))
}

/// What a reader sees on stdout from a `wolf` invocation.
///
/// `wolf conform-run` answers two audiences at once, on two streams: a
/// machine, in ONE line of protocol JSON on stdout, and a reader, in
/// the rendered diagnostic on stderr. `principles/TWO-MACHINES.md` §6
/// puts it in the tool's own terms — "it never executes anything" — and
/// twenty-four corpus pages print the reader's half and nothing else,
/// which is the right thing for a reader and was, until bs41, read by
/// this module as a block it could not replay. It is not: it is a block
/// whose stdout has a line in it that no page should ever carry. So the
/// verdict line is dropped and the rest is compared byte-for-byte,
/// exactly the way the `fail(…)` lane's `conform_run_with` splits the
/// same output for `diagnostic,from(id)`.
///
/// THIS IS THE WHOLE DIFFERENCE between a corpus `conform-run` block
/// and a book one, and it is a difference of TOOL rather than of
/// corpus. The book's `conform-run` console blocks are the
/// interpreter's (`lupin conform-run … --explore=N`, ch13 and ch17),
/// and the explorer's report IS its stdout — there is no protocol half
/// to drop, which is why those have replayed byte-for-byte since the
/// console lane existed while the compiler's twenty-four could not. Six
/// of the twenty-four are lupin's too, and they were caught by a guard
/// written for the compiler's shape.
///
/// A block that asks for the machine's half by name (`--json`) keeps
/// it: it is then a transcript of the protocol, which is a thing a page
/// may legitimately print.
fn conform_run_stdout(args: &[String], out: &str) -> String {
    if args.first().map(String::as_str) != Some("conform-run") || args.iter().any(|a| a == "--json")
    {
        return out.to_string();
    }
    let Some(verdict) = out.lines().rev().find(|l| l.trim_start().starts_with('{')) else {
        return out.to_string();
    };
    out.lines()
        .filter(|l| *l != verdict)
        .map(|l| format!("{l}\n"))
        .collect()
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
                    let (code, out, err) = run_capture(cmd)?;
                    (code, format!("{out}{err}"))
                }
                Verb::Wolf(args) => {
                    let mut cmd = Command::new(&tools.wolf);
                    cmd.args(&args).current_dir(dir);
                    let (code, out, err) = run_capture(cmd)?;
                    (code, format!("{}{err}", conform_run_stdout(&args, &out)))
                }
                Verb::Local(name, args) => {
                    let mut cmd = Command::new(dir.join(name));
                    cmd.args(&args).current_dir(dir);
                    let (code, out, err) = run_capture(cmd)?;
                    (code, format!("{out}{err}"))
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
    /// Of `checked`, how many came from the exercise corpus.
    pub corpus_checked: usize,
    /// Corpus blocks NOTHING can replay, anywhere, as (block key,
    /// reason). This is the declined ledger's subject: every one of
    /// these carries a row in `samples-declined.toml`, and a row
    /// naming anything else is stale.
    pub corpus_skipped: Vec<(String, String)>,
    /// Corpus blocks THIS lane cannot replay although another can —
    /// the native half off unix. Reported, never balanced against the
    /// ledger: a third of the matrix standing down is not a hole in
    /// the book.
    pub corpus_off_lane: Vec<(String, String)>,
    /// Every corpus block key that REPLAYED here — what a declined row
    /// is measured stale against.
    pub corpus_replayed: Vec<String>,
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
        corpus_checked: 0,
        corpus_skipped: Vec::new(),
        corpus_off_lane: Vec::new(),
        corpus_replayed: Vec::new(),
        skipped: Vec::new(),
        declared: Vec::new(),
        failures: Vec::new(),
    };
    for block in blocks {
        let where_ = format!("{}:{}", block.md.display(), block.line);
        let key = block_key(root, block);
        let steps = match parse_steps(&block.text) {
            Ok(s) => s,
            Err(e) => {
                report.failures.push(format!("{where_}: {e:#}"));
                continue;
            }
        };
        let decline = |report: &mut Report, d: Decline| {
            if block.corpus_dir.is_none() {
                report.skipped.push(format!("{where_}: {}", d.why));
            } else if d.host_only {
                report.corpus_off_lane.push((key.clone(), d.why));
            } else {
                report.corpus_skipped.push((key.clone(), d.why));
            }
        };
        if let Err(d) = replayable(&steps) {
            decline(&mut report, d);
            continue;
        }
        if let Some(src) = &block.corpus_dir {
            if let Err(d) = plain_run_of_a_solution(src, &steps) {
                decline(&mut report, d);
                continue;
            }
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
        // A corpus block's solutions are checked into the exercise
        // directory. Stage a private copy: `wolf build` drops a binary
        // beside its source and the checked-in tree stays pristine.
        if let Some(src) = &block.corpus_dir {
            if let Err(e) = stage(src, &dir) {
                report
                    .failures
                    .push(format!("{where_}: staging {}: {e:#}", src.display()));
                continue;
            }
        }
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
        let expected: Vec<String> = trim_trailing_blanks(
            block
                .text
                .lines()
                .map(|l| l.trim_end().to_string())
                .collect(),
        );
        let actual = trim_trailing_blanks(actual);
        // A host whose tool text differs declares the difference in
        // full (samples-os.toml). The block is still replayed and still
        // byte-compared — against the declaration instead of the page.
        if let Some(row) = os.transcript(&key) {
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
                    if block.corpus_dir.is_some() {
                        report.corpus_checked += 1;
                        report.corpus_replayed.push(key.clone());
                    }
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
        if block.corpus_dir.is_some() {
            report.corpus_checked += 1;
            report.corpus_replayed.push(key.clone());
        }
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

/// Trailing blank lines are not compared, on either side. The
/// compiler's diagnostic renderer ends with one (`…  |\n\n`) and a
/// markdown fence cannot carry a trailing blank reliably — editors,
/// formatters and `git` all eat it, so a byte-compare that insisted on
/// it would fail for a reason no author could see or fix. Every other
/// blank line, including one BETWEEN two diagnostics, is compared
/// exactly as before.
fn trim_trailing_blanks(mut lines: Vec<String>) -> Vec<String> {
    while lines.last().is_some_and(|l| l.is_empty()) {
        lines.pop();
    }
    lines
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
    fn trailing_blanks_are_not_compared_but_inner_ones_are() {
        let l = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(
            trim_trailing_blanks(l(&["$ wolf build a.lu", "warning[E0802]", "", ""])),
            l(&["$ wolf build a.lu", "warning[E0802]"])
        );
        assert_eq!(
            trim_trailing_blanks(l(&["one", "", "two"])),
            l(&["one", "", "two"])
        );
    }

    #[test]
    fn the_corpus_rule_declines_what_it_cannot_conjure() {
        let dir = Path::new("/does/not/exist");
        let evaluated = parse_steps("$ lupin eval '1 + 1'\n2\n").unwrap();
        let d = plain_run_of_a_solution(dir, &evaluated).unwrap_err();
        assert!(d.why.contains("names no solution"), "{}", d.why);
        assert!(!d.host_only);
        let repl = parse_steps("$ lupin\nwolf> 1\n").unwrap();
        let d = plain_run_of_a_solution(dir, &repl).unwrap_err();
        assert!(d.why.contains("names no solution"), "{}", d.why);
        let binary = parse_steps("$ ./ex19-1\n42\n").unwrap();
        let d = plain_run_of_a_solution(dir, &binary).unwrap_err();
        assert!(d.why.contains("not a solution"), "{}", d.why);
    }

    /// bs41, wolf-book#29 §3: the catalog read is the one command with
    /// no file behind it that the corpus rule admits, and `lupin eval`
    /// — the other command with no file behind it — stays declined.
    #[test]
    fn the_catalog_read_is_admitted_and_eval_is_not() {
        let dir = Path::new("/does/not/exist");
        let explain = parse_steps("$ wolf --explain E1001\nE1001: …\n").unwrap();
        assert!(plain_run_of_a_solution(dir, &explain).is_ok());
        assert!(is_catalog_read(&words("wolf --explain E0701").unwrap()));
        // Not a catalog read: it has a file behind it after all, and
        // must be judged as one.
        assert!(!is_catalog_read(
            &words("wolf --explain E1001 ex7-3.lu").unwrap()
        ));
        assert!(!is_catalog_read(&words("lupin eval '1 + 1'").unwrap()));
    }

    /// bs41, wolf-book#29 §1: a `conform-run` naming a solution in the
    /// directory is a run of that solution as far as this module is
    /// concerned. What made it look otherwise was a stream, not a verb.
    #[test]
    fn a_conform_run_naming_a_solution_is_admitted() {
        let dir = std::env::temp_dir().join("bs41-console-rule");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("ex7-3.lu"), "fn main() -> !int { 0 }\n").unwrap();
        let probe = parse_steps("$ wolf conform-run ./ex7-3.lu\nerror[E1001]: …\n").unwrap();
        assert!(plain_run_of_a_solution(&dir, &probe).is_ok());
        let explorer =
            parse_steps("$ lupin conform-run ex7-3.lu --explore=8\nex7-3.lu: …\n").unwrap();
        assert!(plain_run_of_a_solution(&dir, &explorer).is_ok());
        // Still declined: the file is not in THIS directory.
        let elsewhere = parse_steps("$ wolf conform-run ./ex99-1.lu\n…\n").unwrap();
        assert!(plain_run_of_a_solution(&dir, &elsewhere).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The protocol line is the machine's half and never a page's.
    #[test]
    fn the_verdict_json_is_dropped_and_only_it() {
        let args = |s: &str| words(s).unwrap()[1..].to_vec();
        let out = "{\"protocol\":1,\"verdict\":\"fail(E1001)\"}\n";
        assert_eq!(
            conform_run_stdout(&args("wolf conform-run ./a.lu"), out),
            ""
        );
        // A page that asks for the machine's half by name keeps it.
        assert_eq!(
            conform_run_stdout(&args("wolf conform-run --json ./a.lu"), out),
            out
        );
        // Any other verb is untouched, JSON-looking output and all.
        assert_eq!(conform_run_stdout(&args("wolf interface ./a.lu"), out), out);
        // The reader's half, wherever the driver put it, survives.
        let mixed = format!("error[E1001]: moved\n{out}");
        assert_eq!(
            conform_run_stdout(&args("wolf conform-run ./a.lu"), &mixed),
            "error[E1001]: moved\n"
        );
    }

    /// A decline off the unix lane is a fact about the lane; a decline
    /// the corpus rule makes is a fact about the block. bs41 keeps them
    /// apart so the declined ledger balances on every host.
    #[test]
    fn an_off_lane_decline_is_marked_as_one() {
        let built = parse_steps("$ ./ex19-1\n42\n").unwrap();
        if cfg!(unix) {
            assert!(replayable(&built).is_ok());
        } else {
            let d = replayable(&built).unwrap_err();
            assert!(d.host_only, "{}", d.why);
        }
        let shell = parse_steps("$ grep -rn x . | wc -l\n3\n").unwrap();
        let d = replayable(&shell).unwrap_err();
        assert!(!d.host_only, "{}", d.why);
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
            corpus_dir: None,
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
