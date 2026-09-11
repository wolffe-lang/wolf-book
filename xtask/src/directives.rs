//! The directive language — deliberately identical in spirit to
//! wolf-lang s01's corpus directives. One grammar, two carriers: the
//! `//! check:` headers on corpus `.lu` files and the fence info
//! strings in book markdown.

use anyhow::{bail, Result};

/// What a sample must do. The runner enforces it; CI screams otherwise.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Check {
    /// `run(exit=N[, stdout="…"])` — lupin runs it.
    Run { exit: i32, stdout: Option<String> },
    /// `wolf-run(exit=N[, stdout="…"])` — the *compiler* builds it and
    /// runs the binary. Needed for the programs only one lane executes:
    /// a `comptime fn` is the compiler's to evaluate, and the reference
    /// interpreter declines it by design.
    WolfRun { exit: i32, stdout: Option<String> },
    /// `lupin-run(exit=N[, stdout="…"])` — the *interpreter* alone runs
    /// it. The mirror of `wolf-run(…)`, and the spelling a block takes
    /// when the compiler declines the program: the fence names the
    /// machine that served it, the prose beside it carries the
    /// per-machine note, and the chapter's ledger carries the row with
    /// an owner (`principles/TWO-MACHINES.md`). A `lupin-run(…)` sample
    /// the compiler also serves is a FLIP, which is how the note gets
    /// retired in the pin-bump commit rather than remembered.
    LupinRun { exit: i32, stdout: Option<String> },
    /// `run(exit=trap(kind))` — a defined fault, named, on both
    /// machines. The kind is the contract; the exit status is not,
    /// because D60 rules it per-machine: lupin exits 3, the compiler's
    /// binary 134.
    Trap { kind: String },
    /// `lupin-run(exit=trap(kind))` — the trap on the interpreter
    /// alone, under the `lupin-run(…)` rule above.
    LupinTrap { kind: String },
    /// `fail(E1234)` — wolf rejects it statically with that code.
    Fail { code: String },
    /// `ub(P1)` — the program reaches undefined behavior, and both
    /// machines have to say so: lupin's oracle faults it, and the
    /// compiler's checked build names that `[mem.ub]` row. One
    /// directive, two implementations, because a UB row nobody can
    /// reproduce is a claim rather than a fact.
    Ub { row: String },
    /// `audit(E1303)` — `wolf audit-surface` rejects the package's
    /// unsafety surface with that code (the trusted-module manifest
    /// rule; the ring inventory itself goes to stdout).
    Audit { code: String },
    /// bare ```wolf — must get through the static phases clean.
    Compile,
}

impl Check {
    /// The spelling wolf-lang's corpus runner understands, for the
    /// exported tree. `ub(row)` is that runner's `run(exit=trap(ub))`;
    /// `audit(…)` has no counterpart there, so those samples stay home.
    pub fn corpus_directive(&self) -> Option<String> {
        match self {
            Check::Ub { .. } => Some("run(exit=trap(ub))".to_string()),
            Check::Audit { .. } => None,
            // The other runner is the compiler's, and these are the
            // programs the compiler declines. Exporting them as `run(…)`
            // would hand wolf-lang's corpus a claim its own machine
            // cannot meet, which is the defect this directive exists to
            // stop making. They stay home until the fence graduates.
            Check::LupinRun { .. } | Check::LupinTrap { .. } => None,
            // The other runner has one `run`, and it is the same claim
            // about the same program — which lane executed it is this
            // repository's bookkeeping.
            Check::WolfRun { exit, stdout } => Some(
                Check::Run {
                    exit: *exit,
                    stdout: stdout.clone(),
                }
                .to_string(),
            ),
            other => Some(other.to_string()),
        }
    }
}

impl std::fmt::Display for Check {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Check::Run { exit, stdout: None } => write!(f, "run(exit={exit})"),
            Check::Run {
                exit,
                stdout: Some(s),
            } => write!(f, "run(exit={exit}, stdout=\"{s}\")"),
            Check::WolfRun { exit, stdout: None } => write!(f, "wolf-run(exit={exit})"),
            Check::WolfRun {
                exit,
                stdout: Some(s),
            } => write!(f, "wolf-run(exit={exit}, stdout=\"{s}\")"),
            Check::LupinRun { exit, stdout: None } => write!(f, "lupin-run(exit={exit})"),
            Check::LupinRun {
                exit,
                stdout: Some(s),
            } => write!(f, "lupin-run(exit={exit}, stdout=\"{s}\")"),
            Check::Trap { kind } => write!(f, "run(exit=trap({kind}))"),
            Check::LupinTrap { kind } => write!(f, "lupin-run(exit=trap({kind}))"),
            Check::Fail { code } => write!(f, "fail({code})"),
            Check::Ub { row } => write!(f, "ub({row})"),
            Check::Audit { code } => write!(f, "audit({code})"),
            Check::Compile => write!(f, "compile"),
        }
    }
}

/// Parse the value of a `check:` directive, e.g.
/// `run(exit=0, stdout="a b")`, `run(exit=trap(bounds))`, `fail(E1001)`.
pub fn parse_check(s: &str) -> Result<Check> {
    let s = s.trim();
    if let Some(inner) = s.strip_prefix("fail(").and_then(|r| r.strip_suffix(')')) {
        let code = inner.trim();
        if code.is_empty() {
            bail!("fail() needs a diagnostic code");
        }
        return Ok(Check::Fail {
            code: code.to_string(),
        });
    }
    if let Some(inner) = s.strip_prefix("ub(").and_then(|r| r.strip_suffix(')')) {
        let row = inner.trim();
        if row.is_empty() {
            bail!("ub() needs a [mem.ub] row (P1…P6, L1, L2, T1)");
        }
        return Ok(Check::Ub {
            row: row.to_string(),
        });
    }
    if let Some(inner) = s
        .strip_prefix("wolf-run(")
        .and_then(|r| r.strip_suffix(')'))
    {
        return match parse_run_args(inner)? {
            Check::Run { exit, stdout } => Ok(Check::WolfRun { exit, stdout }),
            other => bail!("wolf-run() takes exit=N[, stdout=\"…\"], not `{other}`"),
        };
    }
    if let Some(inner) = s
        .strip_prefix("lupin-run(")
        .and_then(|r| r.strip_suffix(')'))
    {
        return match parse_run_args(inner)? {
            Check::Run { exit, stdout } => Ok(Check::LupinRun { exit, stdout }),
            Check::Trap { kind } => Ok(Check::LupinTrap { kind }),
            other => {
                bail!("lupin-run() takes exit=N[, stdout=\"…\"] or exit=trap(k), not `{other}`")
            }
        };
    }
    if let Some(inner) = s.strip_prefix("audit(").and_then(|r| r.strip_suffix(')')) {
        let code = inner.trim();
        if code.is_empty() {
            bail!("audit() needs a diagnostic code");
        }
        return Ok(Check::Audit {
            code: code.to_string(),
        });
    }
    if let Some(inner) = s.strip_prefix("run(").and_then(|r| r.strip_suffix(')')) {
        return parse_run_args(inner);
    }
    bail!("unrecognized check directive: `{s}`");
}

fn parse_run_args(inner: &str) -> Result<Check> {
    let mut exit: Option<i32> = None;
    let mut trap: Option<String> = None;
    let mut stdout: Option<String> = None;
    for arg in split_args(inner) {
        let arg = arg.trim();
        if let Some(v) = arg.strip_prefix("exit=") {
            if let Some(kind) = v.strip_prefix("trap(").and_then(|r| r.strip_suffix(')')) {
                trap = Some(kind.trim().to_string());
            } else {
                exit = Some(v.trim().parse()?);
            }
        } else if let Some(v) = arg.strip_prefix("stdout=") {
            let v = v.trim();
            let unquoted = v
                .strip_prefix('"')
                .and_then(|r| r.strip_suffix('"'))
                .with_context_str("stdout= wants a double-quoted string")?;
            stdout = Some(unquoted.to_string());
        } else if !arg.is_empty() {
            bail!("unrecognized run() argument: `{arg}`");
        }
    }
    match (trap, exit) {
        (Some(kind), None) => Ok(Check::Trap { kind }),
        (None, Some(exit)) => Ok(Check::Run { exit, stdout }),
        (Some(_), Some(_)) => bail!("run() cannot have both exit=N and exit=trap(…)"),
        (None, None) => bail!("run() needs exit=N or exit=trap(…)"),
    }
}

/// Split on commas that are not inside quotes or parentheses.
fn split_args(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut depth = 0usize;
    let mut in_str = false;
    for c in s.chars() {
        match c {
            '"' => {
                in_str = !in_str;
                cur.push(c);
            }
            '(' if !in_str => {
                depth += 1;
                cur.push(c);
            }
            ')' if !in_str => {
                depth = depth.saturating_sub(1);
                cur.push(c);
            }
            ',' if !in_str && depth == 0 => {
                out.push(std::mem::take(&mut cur));
            }
            _ => cur.push(c),
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur);
    }
    out
}

trait WithContextStr<T> {
    fn with_context_str(self, msg: &'static str) -> Result<T>;
}
impl<T> WithContextStr<T> for Option<T> {
    fn with_context_str(self, msg: &'static str) -> Result<T> {
        self.ok_or_else(|| anyhow::anyhow!(msg))
    }
}

/// A corpus `.lu` header: `//! check: …`, `//! phase: …`, `//! member: true`,
/// `//! warns: W0601`.
#[derive(Debug, Clone)]
pub struct LuHeader {
    pub check: Option<Check>,
    pub phase: Option<String>,
    pub member: bool,
    /// `warns: W0601, W1002` — the exact warning codes the compiler is
    /// expected to print for this file, wolf-lang s67's ledger key
    /// spelled the same way. Sorted and deduplicated.
    pub warns: Vec<String>,
}

/// One warning code, shaped like the catalog spells them: `W0601`,
/// `E0802` (five characters, a severity letter and four digits).
fn parse_warning_codes(list: &str, where_: &str) -> Result<Vec<String>> {
    let mut out = Vec::new();
    for code in list.split(',').map(str::trim).filter(|c| !c.is_empty()) {
        let b = code.as_bytes();
        let shaped =
            b.len() == 5 && matches!(b[0], b'E' | b'W') && b[1..].iter().all(u8::is_ascii_digit);
        if !shaped {
            bail!("{where_}: bad warning code `{code}` (codes look like W0601)");
        }
        out.push(code.to_string());
    }
    if out.is_empty() {
        bail!("{where_}: needs at least one warning code");
    }
    out.sort();
    out.dedup();
    Ok(out)
}

pub fn parse_lu_header(source: &str) -> Result<LuHeader> {
    let mut h = LuHeader {
        check: None,
        phase: None,
        member: false,
        warns: Vec::new(),
    };
    for line in source.lines() {
        let Some(rest) = line.strip_prefix("//!") else {
            // Headers are a leading block; stop at the first non-header line.
            if line.trim().is_empty() {
                continue;
            }
            break;
        };
        let rest = rest.trim();
        if let Some(v) = rest.strip_prefix("check:") {
            h.check = Some(parse_check(v)?);
        } else if let Some(v) = rest.strip_prefix("phase:") {
            h.phase = Some(v.trim().to_string());
        } else if let Some(v) = rest.strip_prefix("member:") {
            h.member = v.trim() == "true";
        } else if let Some(v) = rest.strip_prefix("warns:") {
            h.warns = parse_warning_codes(v, "`//! warns:`")?;
        }
        // Unknown `//!` keys are tolerated: the corpus directive language
        // may grow in wolf-lang first.
    }
    Ok(h)
}

/// A parsed fence info string, e.g. `wolf,part(name, cont),run(exit=0)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FenceInfo {
    /// The base language: `wolf`, `wolf-repl`, `console`, `diagnostic`, …
    pub lang: String,
    /// `part(name)` / `part(name, cont)`.
    pub part: Option<(String, bool)>,
    /// The check, if the fence carries one.
    pub check: Option<Check>,
    /// `diagnostic,from(id)` — cross-check against a captured run.
    pub from: Option<String>,
    /// `console,in(pkg/name)` — replay the block inside a staged copy of
    /// the named fixture tree under `samples/`, instead of writing the
    /// program printed above the block. Multi-package walkthroughs (a
    /// manifest, a dependency, a ledger) need a project, not a file.
    pub in_fixture: Option<String>,
    /// `text,file(pkg/name/wolf.pkg)` — the block must equal that file
    /// byte for byte. A manifest on the page is a manifest CI resolves.
    pub file: Option<String>,
    /// `warns(W0601)` — the exact set of warning codes the compiler
    /// must print for this sample (wolf-book#21). The runner asks
    /// `wolf conform-run` for the set and fails on any difference in
    /// either direction; a sample without the directive that warns is
    /// counted and named in the log, report-only, so a dropped-row
    /// warning the interpreter cannot see is at least seen by a gate.
    pub warns: Vec<String>,
}

pub fn parse_fence_info(info: &str) -> Result<FenceInfo> {
    let info = info.trim();
    let mut items = split_args(info).into_iter();
    let lang = items.next().unwrap_or_default().trim().to_string();
    let mut fi = FenceInfo {
        lang,
        part: None,
        check: None,
        from: None,
        in_fixture: None,
        file: None,
        warns: Vec::new(),
    };
    for item in items {
        let item = item.trim();
        if let Some(inner) = item.strip_prefix("part(").and_then(|r| r.strip_suffix(')')) {
            let mut args = inner.split(',').map(str::trim);
            let name = args.next().unwrap_or_default().to_string();
            if name.is_empty() {
                bail!("part() needs a name");
            }
            let cont = matches!(args.next(), Some("cont"));
            fi.part = Some((name, cont));
        } else if item.starts_with("wolf-run(")
            || item.starts_with("lupin-run(")
            || item.starts_with("run(")
            || item.starts_with("fail(")
            || item.starts_with("ub(")
            || item.starts_with("audit(")
        {
            fi.check = Some(parse_check(item)?);
        } else if let Some(inner) = item.strip_prefix("from(").and_then(|r| r.strip_suffix(')')) {
            fi.from = Some(inner.trim().to_string());
        } else if let Some(inner) = item.strip_prefix("in(").and_then(|r| r.strip_suffix(')')) {
            let name = inner.trim();
            if name.is_empty() {
                bail!("in() needs a fixture path");
            }
            fi.in_fixture = Some(name.to_string());
        } else if let Some(inner) = item.strip_prefix("file(").and_then(|r| r.strip_suffix(')')) {
            let name = inner.trim();
            if name.is_empty() {
                bail!("file() needs a path");
            }
            fi.file = Some(name.to_string());
        } else if let Some(inner) = item
            .strip_prefix("warns(")
            .and_then(|r| r.strip_suffix(')'))
        {
            fi.warns = parse_warning_codes(inner, "warns()")?;
        } else if !item.is_empty() {
            bail!("unrecognized fence directive: `{item}` in `{info}`");
        }
    }
    Ok(fi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_run_plain() {
        assert_eq!(
            parse_check("run(exit=0)").unwrap(),
            Check::Run {
                exit: 0,
                stdout: None
            }
        );
    }

    #[test]
    fn check_run_stdout_with_comma() {
        assert_eq!(
            parse_check(r#"run(exit=0, stdout="3 lines, 11 words")"#).unwrap(),
            Check::Run {
                exit: 0,
                stdout: Some("3 lines, 11 words".into())
            }
        );
    }

    #[test]
    fn check_trap() {
        assert_eq!(
            parse_check("run(exit=trap(use-after-move))").unwrap(),
            Check::Trap {
                kind: "use-after-move".into()
            }
        );
    }

    #[test]
    fn check_fail() {
        assert_eq!(
            parse_check("fail(E1001)").unwrap(),
            Check::Fail {
                code: "E1001".into()
            }
        );
    }

    #[test]
    fn lu_header_roundtrip() {
        let src = "//! check: run(exit=3)\n//! phase: run\nfn main() {}\n";
        let h = parse_lu_header(src).unwrap();
        assert_eq!(
            h.check,
            Some(Check::Run {
                exit: 3,
                stdout: None
            })
        );
        assert_eq!(h.phase.as_deref(), Some("run"));
        assert!(!h.member);
    }

    #[test]
    fn fence_part_cont_with_run() {
        let fi = parse_fence_info(r#"wolf,part(greet, cont),run(exit=0, stdout="hi")"#).unwrap();
        assert_eq!(fi.lang, "wolf");
        assert_eq!(fi.part, Some(("greet".into(), true)));
        assert_eq!(
            fi.check,
            Some(Check::Run {
                exit: 0,
                stdout: Some("hi".into())
            })
        );
    }

    #[test]
    fn check_ub_row() {
        assert_eq!(
            parse_check("ub(P1)").unwrap(),
            Check::Ub { row: "P1".into() }
        );
        assert_eq!(
            Check::Ub { row: "P4".into() }.corpus_directive().unwrap(),
            "run(exit=trap(ub))"
        );
    }

    #[test]
    fn check_audit_code() {
        assert_eq!(
            parse_check("audit(E1303)").unwrap(),
            Check::Audit {
                code: "E1303".into()
            }
        );
        assert!(Check::Audit {
            code: "E1303".into()
        }
        .corpus_directive()
        .is_none());
    }

    #[test]
    fn fence_carries_ub_and_audit() {
        assert_eq!(
            parse_fence_info("wolf,ub(P3)").unwrap().check,
            Some(Check::Ub { row: "P3".into() })
        );
        assert_eq!(
            parse_fence_info("wolf,audit(E1303)").unwrap().check,
            Some(Check::Audit {
                code: "E1303".into()
            })
        );
    }

    #[test]
    fn check_lupin_run_roundtrips() {
        assert_eq!(
            parse_check(r#"lupin-run(exit=8, stdout="counted")"#).unwrap(),
            Check::LupinRun {
                exit: 8,
                stdout: Some("counted".into())
            }
        );
        assert_eq!(
            Check::LupinRun {
                exit: 8,
                stdout: Some("counted".into())
            }
            .to_string(),
            r#"lupin-run(exit=8, stdout="counted")"#
        );
        assert_eq!(
            parse_check("lupin-run(exit=trap(bounds))").unwrap(),
            Check::LupinTrap {
                kind: "bounds".into()
            }
        );
        assert_eq!(
            Check::LupinTrap {
                kind: "bounds".into()
            }
            .to_string(),
            "lupin-run(exit=trap(bounds))"
        );
    }

    #[test]
    fn lupin_only_samples_stay_home() {
        // The other runner is the compiler's; a program the compiler
        // declines is not exported to it as an ordinary `run(…)`.
        assert!(Check::LupinRun {
            exit: 0,
            stdout: None
        }
        .corpus_directive()
        .is_none());
        assert!(Check::LupinTrap {
            kind: "bounds".into()
        }
        .corpus_directive()
        .is_none());
    }

    #[test]
    fn fence_carries_lupin_run() {
        let fi = parse_fence_info(r#"wolf,lupin-run(exit=8, stdout="counted")"#).unwrap();
        assert_eq!(fi.lang, "wolf");
        assert_eq!(
            fi.check,
            Some(Check::LupinRun {
                exit: 8,
                stdout: Some("counted".into())
            })
        );
    }

    #[test]
    fn fence_diagnostic_from() {
        let fi = parse_fence_info("diagnostic,from(ch03/ex3-2)").unwrap();
        assert_eq!(fi.lang, "diagnostic");
        assert_eq!(fi.from.as_deref(), Some("ch03/ex3-2"));
    }

    #[test]
    fn fence_carries_warns() {
        let fi = parse_fence_info(r#"wolf,run(exit=0, stdout="regions"),warns(W0601)"#).unwrap();
        assert_eq!(
            fi.check,
            Some(Check::Run {
                exit: 0,
                stdout: Some("regions".into())
            })
        );
        assert_eq!(fi.warns, vec!["W0601".to_string()]);
        // Sorted, deduplicated, and the order on the fence is not a claim.
        let fi = parse_fence_info("wolf,run(exit=0),warns(W1002, W0601, W0601)").unwrap();
        assert_eq!(fi.warns, vec!["W0601".to_string(), "W1002".to_string()]);
        // A fence without the directive asserts nothing about warnings.
        assert!(parse_fence_info("wolf,run(exit=0)")
            .unwrap()
            .warns
            .is_empty());
    }

    #[test]
    fn warns_wants_catalog_shaped_codes() {
        assert!(parse_fence_info("wolf,run(exit=0),warns(0601)").is_err());
        assert!(parse_fence_info("wolf,run(exit=0),warns(W601)").is_err());
        assert!(parse_fence_info("wolf,run(exit=0),warns()").is_err());
        assert!(
            parse_lu_header("//! check: run(exit=0)\n//! warns: dropped\nfn main() {}\n").is_err()
        );
    }

    #[test]
    fn lu_header_carries_warns() {
        let src = "//! check: run(exit=0)\n//! warns: W0601, E0802\nfn main() {}\n";
        let h = parse_lu_header(src).unwrap();
        assert_eq!(h.warns, vec!["E0802".to_string(), "W0601".to_string()]);
        assert!(parse_lu_header("//! check: run(exit=0)\nfn main() {}\n")
            .unwrap()
            .warns
            .is_empty());
    }
}
