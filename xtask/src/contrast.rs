//! `cargo xtask contrast` — the other languages the book quotes.
//!
//! Chapter 7 §7.6 prints a Rust program, because the honest-contrast
//! rule (TONE.md §1) means showing what the other design buys rather
//! than describing it. Contrast code rots exactly like wolf code does,
//! so it is vendored under `samples/contrast/` and compiled here: every
//! `.rs` file is built as a test binary with warnings denied and run,
//! which compiles the code, runs its assertions, and proves the printed
//! claims about it.
//!
//! The C dialect (bs10) works the same way with a different shape,
//! because a C program has no `#[test]` to carry its own assertions. The
//! projects part sets three tools beside their K&R ancestors, so those
//! ancestors are vendored as `.c` files, compiled with `-std=c99 -Wall
//! -Werror`, and executed against the cases in
//! `samples/contrast/cases.toml`: each case names argv, the input files
//! to lay down, stdin, and the exact stdout, stderr, and exit status
//! expected. A `.c` file with no case is a failure — an unasserted C
//! twin is a claim, and the side-by-side pages are made of claims about
//! what the C costs. Nothing about these programs is taken on the
//! book's word.

use anyhow::{bail, Context, Result};
use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn run(root: &Path) -> Result<()> {
    let rust = rust_lane(root)?;
    let c = c_lane(root)?;
    let failures: Vec<String> = rust.into_iter().chain(c).collect();
    if failures.is_empty() {
        Ok(())
    } else {
        for f in &failures {
            eprintln!("contrast: FAIL {f}");
        }
        bail!("{} contrast failure(s)", failures.len())
    }
}

/// The Rust lane: `.rs` files built as test binaries, warnings denied.
fn rust_lane(root: &Path) -> Result<Vec<String>> {
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
    for (md, block) in blocks(root, "rust")? {
        printed += 1;
        if !sources.iter().any(|s| s.contains(block.trim_end())) {
            failures.push(format!(
                "{}: a ```rust block is not vendored under samples/contrast/ \
                 — the book prints Rust that CI does not compile",
                md.display()
            ));
        }
    }

    println!(
        "contrast: rust — {} file(s) compiled and asserted, {printed} book block(s) matched",
        files.len()
    );
    Ok(failures)
}

/// Every fence of one language in `book/**/*.md`, with its source file.
fn blocks(root: &Path, lang: &str) -> Result<Vec<(std::path::PathBuf, String)>> {
    let base = root.join("book");
    let mut out = Vec::new();
    let mut mds: Vec<std::path::PathBuf> = Vec::new();
    collect_md(&base, &mut mds)?;
    mds.sort();
    for md in mds {
        let text = std::fs::read_to_string(&md)?;
        for seg in crate::fence::segments(&text) {
            if let crate::fence::Segment::Fence(f) = seg {
                if f.info.trim() == lang {
                    out.push((md.clone(), f.content.clone()));
                }
            }
        }
    }
    Ok(out)
}

/// One ```` ```c-run,from(case name) ```` block: the transcript the book
/// prints for a declared C case, with where it was printed.
struct CRunBlock {
    md: PathBuf,
    line: usize,
    case: String,
    text: String,
}

/// The `c-run` dialect (bs10): a C twin's run, printed in the book's
/// console shape and bound by name to a case in `cases.toml`. The
/// transcript is entirely derivable from the case — the prompt from the
/// file stem and argv, the body from the asserted streams — so the book's
/// copy is checked rather than trusted, exactly as a ```` ```console ````
/// block is replayed. The C half of a side-by-side page is then as
/// rot-proof as the wolf half.
fn c_run_blocks(root: &Path) -> Result<Vec<CRunBlock>> {
    let base = root.join("book");
    let mut out = Vec::new();
    let mut mds: Vec<PathBuf> = Vec::new();
    collect_md(&base, &mut mds)?;
    mds.sort();
    for md in mds {
        let text = std::fs::read_to_string(&md)?;
        for seg in crate::fence::segments(&text) {
            let crate::fence::Segment::Fence(f) = seg else {
                continue;
            };
            let info = f.info.trim();
            let Some(rest) = info.strip_prefix("c-run") else {
                continue;
            };
            let case = rest
                .trim_start()
                .strip_prefix(',')
                .map(str::trim)
                .and_then(|r| r.strip_prefix("from("))
                .and_then(|r| r.strip_suffix(')'))
                .map(str::to_string);
            let Some(case) = case else {
                bail!(
                    "{}:{}: a `c-run` fence needs `from(<case name>)` — got `{info}`",
                    md.display(),
                    f.open_line + 1
                );
            };
            out.push(CRunBlock {
                md: md.clone(),
                line: f.open_line + 1,
                case,
                text: f.content.clone(),
            });
        }
    }
    Ok(out)
}

/// The transcript a case must print, derived from the case alone.
fn expected_transcript(case: &CCase) -> Vec<String> {
    let stem = case.file.trim_end_matches(".c");
    let mut lines = vec![format!(
        "$ ./{stem}{}",
        case.argv
            .iter()
            .map(|a| format!(" {a}"))
            .collect::<String>()
    )];
    for stream in [&case.stdout, &case.stderr] {
        lines.extend(
            trim_end(stream)
                .lines()
                .filter(|l| !l.is_empty())
                .map(str::to_string),
        );
    }
    if case.exit != 0 {
        lines.push("$ echo $?".to_string());
        lines.push(case.exit.to_string());
    }
    lines
}

// ------------------------------------------------------------------- C
//
// The projects part's side-by-sides (bs10). A C twin is an ORIGINAL
// program written in K&R's idiom and attributed in prose and in
// PERMISSIONS.md; the machinery here is what makes the attribution
// honest, because it proves the twin is a program that runs and not a
// paraphrase of one.

/// One declared run of one C twin: what goes in, what must come out.
#[derive(Debug, serde::Deserialize)]
struct CCase {
    /// The `.c` file this case exercises, e.g. `count.c`.
    file: String,
    /// What the case is for, quoted in the failure message.
    name: String,
    #[serde(default)]
    argv: Vec<String>,
    /// Input files laid down in the run directory before the program
    /// starts — `count` takes file names, so it needs files to take.
    #[serde(default)]
    files: BTreeMap<String, String>,
    #[serde(default)]
    stdin: String,
    /// Asserted byte-for-byte. Both streams are asserted, and both
    /// default to empty: an unexpected diagnostic fails the case.
    #[serde(default)]
    stdout: String,
    #[serde(default)]
    stderr: String,
    #[serde(default)]
    exit: i32,
}

#[derive(Debug, serde::Deserialize)]
struct CaseManifest {
    #[serde(default)]
    case: Vec<CCase>,
}

/// The C compiler to use: `$CC`, else the first of cc/gcc/clang that
/// answers. `None` means this machine has no C toolchain, which is a
/// loud skip and never a silent pass.
fn c_compiler() -> Option<String> {
    let mut candidates: Vec<String> = Vec::new();
    if let Some(cc) = std::env::var_os("CC") {
        candidates.push(cc.to_string_lossy().into_owned());
    }
    candidates.extend(["cc", "gcc", "clang"].iter().map(|s| s.to_string()));
    candidates.into_iter().find(|cc| {
        Command::new(cc)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|s| s.success())
    })
}

/// The C lane: compile every `.c` twin with warnings denied, then run it
/// against every case declared for it.
fn c_lane(root: &Path) -> Result<Vec<String>> {
    let dir = root.join("samples/contrast");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("c"))
        .collect();
    files.sort();

    // Doc truth first, because it needs no compiler: every ```c block
    // the book prints must be vendored here verbatim.
    let mut failures: Vec<String> = Vec::new();
    let sources: Vec<String> = files
        .iter()
        .map(std::fs::read_to_string)
        .collect::<std::io::Result<_>>()?;
    let mut printed = 0usize;
    for (md, block) in blocks(root, "c")? {
        printed += 1;
        if !sources.iter().any(|s| s.contains(block.trim_end())) {
            failures.push(format!(
                "{}: a ```c block is not vendored under samples/contrast/ \
                 — the book prints C that CI does not compile",
                md.display()
            ));
        }
    }

    if files.is_empty() {
        println!("contrast: c — no .c twins vendored, {printed} book block(s) matched");
        return Ok(failures);
    }

    // Every twin must be asserted by at least one case.
    let manifest_path = dir.join("cases.toml");
    let manifest: CaseManifest = match std::fs::read_to_string(&manifest_path) {
        Ok(text) => {
            toml::from_str(&text).with_context(|| format!("parsing {}", manifest_path.display()))?
        }
        Err(_) => {
            failures.push(format!(
                "{} is missing — every vendored .c twin needs declared cases",
                manifest_path.display()
            ));
            return Ok(failures);
        }
    };
    for path in &files {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        if !manifest.case.iter().any(|c| c.file == name) {
            failures.push(format!(
                "samples/contrast/{name}: no case in cases.toml — an \
                 unasserted C twin is a claim, not a program"
            ));
        }
    }
    for c in &manifest.case {
        if !files
            .iter()
            .any(|p| p.file_name().unwrap() == c.file.as_str())
        {
            failures.push(format!(
                "cases.toml names `{}` but samples/contrast/{} does not exist",
                c.file, c.file
            ));
        }
    }

    // The printed transcripts: every `c-run` block against its case. This
    // needs no compiler — the case file is the source of truth — so it runs
    // before the toolchain check and holds on every machine.
    let transcripts = c_run_blocks(root)?;
    for block in &transcripts {
        let Some(case) = manifest.case.iter().find(|c| c.name == block.case) else {
            failures.push(format!(
                "{}:{}: c-run,from({}) names no case in cases.toml",
                block.md.display(),
                block.line,
                block.case
            ));
            continue;
        };
        let want = expected_transcript(case);
        let got: Vec<String> = trim_end(&block.text).lines().map(str::to_string).collect();
        if got != want {
            failures.push(format!(
                "{}:{}: the printed transcript for case \"{}\" is not what the case \
                 asserts\n     expected:\n{}\n     actual:\n{}",
                block.md.display(),
                block.line,
                block.case,
                indent(&want.join("\n")),
                indent(&got.join("\n")),
            ));
        }
    }
    println!(
        "contrast: c — {} printed transcript(s) checked against their cases",
        transcripts.len()
    );

    let Some(cc) = c_compiler() else {
        println!(
            "contrast: SKIP c — no C compiler ($CC, cc, gcc, clang all absent); \
             {} twin(s) and {} case(s) not run on this machine",
            files.len(),
            manifest.case.len()
        );
        return Ok(failures);
    };

    let out_dir = root.join("target/contrast");
    std::fs::create_dir_all(&out_dir)?;
    let mut built: BTreeMap<String, PathBuf> = BTreeMap::new();
    for path in &files {
        let stem = path.file_stem().unwrap().to_string_lossy().into_owned();
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let bin = out_dir.join(if cfg!(windows) {
            format!("{stem}.exe")
        } else {
            stem.clone()
        });
        let build = Command::new(&cc)
            .args(["-std=c99", "-Wall", "-Werror"])
            .arg(path)
            .arg("-o")
            .arg(&bin)
            .output()
            .with_context(|| format!("running {cc} on {}", path.display()))?;
        if !build.status.success() {
            failures.push(format!(
                "{}: {cc} -std=c99 -Wall -Werror rejected it\n{}",
                path.display(),
                String::from_utf8_lossy(&build.stderr).trim()
            ));
            continue;
        }
        println!(
            "contrast: ok {} ({cc} -std=c99 -Wall -Werror)",
            path.strip_prefix(root).unwrap().display()
        );
        built.insert(name, bin);
    }

    let mut ran = 0usize;
    for (i, case) in manifest.case.iter().enumerate() {
        let Some(bin) = built.get(&case.file) else {
            continue; // its compile already failed, and said so
        };
        let run_dir = out_dir.join("cases").join(format!("c{i}"));
        let _ = std::fs::remove_dir_all(&run_dir);
        std::fs::create_dir_all(&run_dir)?;
        for (name, text) in &case.files {
            std::fs::write(run_dir.join(name), text)?;
        }
        let mut child = Command::new(bin)
            .args(&case.argv)
            .current_dir(&run_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .with_context(|| format!("spawning {}", bin.display()))?;
        child
            .stdin
            .as_mut()
            .expect("piped")
            .write_all(case.stdin.as_bytes())?;
        drop(child.stdin.take());
        let out = child.wait_with_output()?;
        let got_out = String::from_utf8_lossy(&out.stdout).into_owned();
        let got_err = String::from_utf8_lossy(&out.stderr).into_owned();
        let code = out.status.code().unwrap_or(-1);
        let mut trouble: Vec<String> = Vec::new();
        if trim_end(&got_out) != trim_end(&case.stdout) {
            trouble.push(format!(
                "stdout:\n     expected:\n{}\n     actual:\n{}",
                indent(&case.stdout),
                indent(&got_out)
            ));
        }
        if trim_end(&got_err) != trim_end(&case.stderr) {
            trouble.push(format!(
                "stderr:\n     expected:\n{}\n     actual:\n{}",
                indent(&case.stderr),
                indent(&got_err)
            ));
        }
        if code != case.exit {
            trouble.push(format!("exit: expected {}, got {code}", case.exit));
        }
        if trouble.is_empty() {
            ran += 1;
        } else {
            failures.push(format!(
                "samples/contrast/{} case \"{}\" drifted from the real run\n   {}",
                case.file,
                case.name,
                trouble.join("\n   ")
            ));
        }
    }

    println!(
        "contrast: c — {} twin(s) compiled with {cc} -std=c99 -Wall -Werror, \
         {ran}/{} case(s) asserted, {printed} book block(s) matched",
        built.len(),
        manifest.case.len()
    );
    Ok(failures)
}

/// Trailing whitespace is not the subject of any assertion here.
fn trim_end(s: &str) -> String {
    s.lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .to_string()
}

fn indent(text: &str) -> String {
    text.lines()
        .map(|l| format!("       | {l}"))
        .collect::<Vec<_>>()
        .join("\n")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_case_declares_argv_files_and_both_streams() {
        let manifest: CaseManifest = toml::from_str(
            r#"
[[case]]
file = "count.c"
name = "per-file rows and a total"
argv = ["one.txt"]
files = { "one.txt" = "the wolf runs\n" }
exit = 0
stdout = """
       1       3      14 one.txt
"""
"#,
        )
        .expect("the manifest shape parses");
        assert_eq!(manifest.case.len(), 1);
        let c = &manifest.case[0];
        assert_eq!(c.file, "count.c");
        assert_eq!(c.argv, vec!["one.txt"]);
        assert_eq!(c.files["one.txt"], "the wolf runs\n");
        // Omitted streams assert emptiness rather than going unchecked.
        assert_eq!(c.stderr, "");
        assert_eq!(c.stdin, "");
        assert_eq!(c.exit, 0);
        assert!(c.stdout.contains("14 one.txt"));
    }

    #[test]
    fn the_default_case_asserts_silence_and_success() {
        let manifest: CaseManifest =
            toml::from_str("[[case]]\nfile = \"quiet.c\"\nname = \"says nothing\"\n")
                .expect("parses");
        let c = &manifest.case[0];
        assert_eq!(c.stdout, "");
        assert_eq!(c.stderr, "");
        assert_eq!(c.exit, 0);
        assert!(c.argv.is_empty());
        assert!(c.files.is_empty());
    }

    #[test]
    fn comparison_ignores_trailing_whitespace_only() {
        // A block's trailing newline is not the subject of an assertion;
        // an interior difference is.
        assert_eq!(trim_end("a\nb\n"), trim_end("a\nb"));
        assert_eq!(trim_end("a  \nb\t\n"), trim_end("a\nb"));
        assert_ne!(trim_end("a\nb"), trim_end("a\nc"));
        assert_ne!(trim_end("a\nb"), trim_end("a\n\nb"));
    }

    #[test]
    fn a_transcript_is_derived_from_its_case_and_nothing_else() {
        let manifest: CaseManifest = toml::from_str(
            r#"
[[case]]
file = "count.c"
name = "rows"
argv = ["one.txt", "two.txt"]
exit = 0
stdout = """
       2       6      31 one.txt
"""
"#,
        )
        .expect("parses");
        assert_eq!(
            expected_transcript(&manifest.case[0]),
            vec![
                "$ ./count one.txt two.txt",
                "       2       6      31 one.txt"
            ]
        );
    }

    #[test]
    fn a_failing_case_carries_its_exit_status_into_the_transcript() {
        let manifest: CaseManifest = toml::from_str(
            r#"
[[case]]
file = "count.c"
name = "missing"
argv = ["nope.txt"]
exit = 1
stderr = """
count: cannot open nope.txt
"""
"#,
        )
        .expect("parses");
        // Both streams in declared order, then the status, because a
        // nonzero exit is part of what the page is claiming.
        assert_eq!(
            expected_transcript(&manifest.case[0]),
            vec![
                "$ ./count nope.txt",
                "count: cannot open nope.txt",
                "$ echo $?",
                "1"
            ]
        );
    }

    #[test]
    fn a_stdin_case_prints_a_bare_prompt() {
        let manifest: CaseManifest = toml::from_str(
            "[[case]]\nfile = \"rpn.c\"\nname = \"n\"\nstdin = \"2 3 +\\n\"\nexit = 0\nstdout = \"5\\n\"\n",
        )
        .expect("parses");
        assert_eq!(expected_transcript(&manifest.case[0]), vec!["$ ./rpn", "5"]);
    }

    #[test]
    fn a_c_fence_is_read_as_the_c_lane_and_rust_is_not() {
        let text = "```c\nint main(void) { return 0; }\n```\n\
                    ```rust\nfn main() {}\n```\n";
        let mut c = 0;
        let mut rust = 0;
        for seg in crate::fence::segments(text) {
            if let crate::fence::Segment::Fence(f) = seg {
                match f.info.trim() {
                    "c" => c += 1,
                    "rust" => rust += 1,
                    _ => {}
                }
            }
        }
        assert_eq!((c, rust), (1, 1));
    }
}
