//! The dialect taxonomy — the ONE source (rp03). The Notation page
//! teaches these seven kinds of code block; this table is what the
//! teaching binds to. The web preprocessor classes and labels fences
//! from it, `generate_css()` styles those classes from it, and the
//! typst preamble sets the PDF's equivalent blocks from it — so a new
//! dialect added here appears everywhere at once, and a dialect
//! forgotten in a hand-kept stylesheet is a bug this module makes
//! impossible.
//!
//! The visual treatment (DESIGN.md §2): a small label naming the
//! dialect at the block's top edge (the accessible channel — text, not
//! hue), a left accent rule whose *style* also varies (solid vs dashed
//! — the shape channel that survives monochrome), and a faint ground
//! tint as the at-a-glance signal. Fence directives that carry reader
//! signal (`run(exit=…)`, `part(…)`) surface in the label; `from(…)`
//! bindings stay CI's business and never reach the page.

use crate::directives::{Check, FenceInfo};

/// One dialect: its class key, base label, and frame colors.
pub struct Dialect {
    /// CSS class suffix (`dialect-<key>`) and typst function suffix
    /// (`#dialect-<key>`).
    pub key: &'static str,
    /// Left accent rule and label ink.
    pub accent: &'static str,
    /// Block ground tint. Every accent/tint pair holds a WCAG contrast
    /// ratio of at least 6.8:1; body ink on every tint exceeds 15:1.
    pub tint: &'static str,
    /// Accent rule style — `solid` or `dashed`; the same word is valid
    /// CSS `border-style` and typst stroke `dash`. Dashed marks the
    /// derived kinds (a part is a slice, a twin run is a measurement).
    pub dash: &'static str,
}

/// The seven dialects the Notation page declares, in its order.
pub const DIALECTS: &[Dialect] = &[
    // A complete `.lu` program (also `.wolfi` and `wolf.pkg` sources).
    Dialect {
        key: "program",
        accent: "#1a4f8a",
        tint: "#faf8f4",
        dash: "solid",
    },
    // A named slice of a larger program, stitched by the extractor.
    Dialect {
        key: "part",
        accent: "#1a4f8a",
        tint: "#faf8f4",
        dash: "dashed",
    },
    // A session with the interpreter.
    Dialect {
        key: "repl",
        accent: "#355e3b",
        tint: "#f4f7f4",
        dash: "solid",
    },
    // A command and its output, prompt kept. Cool gray ground — the
    // terminal's, not the book's warm page.
    Dialect {
        key: "console",
        accent: "#3c3c3c",
        tint: "#f2f2f0",
        dash: "solid",
    },
    // A C twin's measured run (`c-run`): console affordance, dashed —
    // every line derived from the declared case.
    Dialect {
        key: "twin",
        accent: "#3c3c3c",
        tint: "#f2f2f0",
        dash: "dashed",
    },
    // The compiler's exact text and layout.
    Dialect {
        key: "diagnostic",
        accent: "#a12622",
        tint: "#fdf7f5",
        dash: "solid",
    },
    // Another language's code, vendored and CI-run (`rust`, `c`).
    Dialect {
        key: "contrast",
        accent: "#7a4a12",
        tint: "#faf5ec",
        dash: "solid",
    },
];

pub fn dialect(key: &str) -> Option<&'static Dialect> {
    DIALECTS.iter().find(|d| d.key == key)
}

/// Classify a parsed fence: the dialect and the full reader-facing
/// label. `None` means the fence is not a dialect (figures: `text`,
/// `ebnf`, …) and keeps its plain ground.
pub fn classify(fi: &FenceInfo) -> Option<(&'static Dialect, String)> {
    let (key, base): (&str, &str) = match fi.lang.as_str() {
        "wolf" if fi.part.is_some() => ("part", "wolf"),
        "wolf" => ("program", "wolf"),
        "wolfi" => ("program", "wolfi"),
        "wolf-pkg" => ("program", "wolf.pkg"),
        "wolf-repl" => ("repl", "repl"),
        "console" => ("console", "console"),
        "c-run" => ("twin", "c · run"),
        "diagnostic" => ("diagnostic", "diagnostic"),
        "rust" => ("contrast", "rust · contrast"),
        "c" => ("contrast", "c · contrast"),
        _ => return None,
    };
    let mut label = base.to_string();
    if let Some((name, _cont)) = &fi.part {
        label.push_str(&format!(" · part({name})"));
    }
    if let Some(check) = &fi.check {
        if let Some(suffix) = check_suffix(check) {
            label.push_str(" · ");
            label.push_str(&suffix);
        }
    }
    Some((
        dialect(key).expect("classify keys come from DIALECTS"),
        label,
    ))
}

/// The reader-facing spelling of a fence's check — the existing
/// metadata, surfaced ("wolf · runs, exit 0" teaches that the tier
/// lives). `stdout=` stays off the label: the block below shows it.
fn check_suffix(check: &Check) -> Option<String> {
    match check {
        Check::Run { exit, .. } => Some(format!("runs, exit {exit}")),
        Check::WolfRun { exit, .. } => Some(format!("compiled run, exit {exit}")),
        Check::Trap { kind } => Some(format!("runs, trap({kind})")),
        Check::Fail { code } => Some(format!("rejected, {code}")),
        Check::Ub { row } => Some(format!("ub, {row}")),
        Check::Audit { code } => Some(format!("audit, {code}")),
        Check::Compile => None,
    }
}

/// The dialect-frame CSS, appended to the generated highlight
/// stylesheet — same table, same commit, no hand-kept list to forget.
pub fn dialect_css() -> String {
    use std::fmt::Write as _;
    let mut css = String::from(
        "\n/* The dialect frame (rp03): label + left accent + ground tint,\n\
         \x20  generated from the taxonomy in xtask/src/dialects.rs — the\n\
         \x20  same table the typst PDF blocks come from. */\n\
         pre[data-dialect]::before {\n\
         \x20   content: attr(data-dialect);\n\
         \x20   display: block;\n\
         \x20   font-family: var(--mono, monospace);\n\
         \x20   font-size: 0.75em;\n\
         \x20   letter-spacing: 0.06em;\n\
         \x20   margin: 0 0 0.65em 0;\n\
         }\n",
    );
    for d in DIALECTS {
        let _ = writeln!(
            css,
            "pre.dialect-{} {{ background-color: {}; border-inline-start: 3px {} {}; }}",
            d.key, d.tint, d.dash, d.accent
        );
        let _ = writeln!(
            css,
            "pre.dialect-{}::before {{ color: {}; }}",
            d.key, d.accent
        );
    }
    css
}

/// The typst preamble section: one frame function, then one
/// `#dialect-<key>` per dialect — the PDF's mirror of the CSS above.
pub fn typst_defs() -> String {
    use std::fmt::Write as _;
    let mut out = String::from(
        "// The dialect frame (rp03), generated from xtask/src/dialects.rs —\n\
         // the same taxonomy the web classes and highlight.css come from.\n\
         #let dialect(dtag, accent, tint, dash, body) = block(\n\
         \x20 fill: tint,\n\
         \x20 stroke: (left: (paint: accent, thickness: 2.25pt, dash: dash)),\n\
         \x20 inset: (left: 10pt, top: 8pt, bottom: 8pt, right: 8pt),\n\
         \x20 width: 100%,\n\
         \x20 breakable: true,\n\
         )[#text(font: \"Source Code Pro\", size: 7.3pt, fill: accent, tracking: 0.06em)[#dtag]\n\
         #v(0.4em, weak: true)\n\
         #set text(font: \"Source Code Pro\", size: 8.8pt); #set par(justify: false, leading: 0.55em); #body]\n",
    );
    for d in DIALECTS {
        let _ = writeln!(
            out,
            "#let dialect-{}(dtag, body) = dialect(dtag, rgb(\"{}\"), rgb(\"{}\"), \"{}\", body)",
            d.key, d.accent, d.tint, d.dash
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::directives::parse_fence_info;

    fn classify_info(info: &str) -> Option<(&'static str, String)> {
        let fi = parse_fence_info(info).unwrap();
        classify(&fi).map(|(d, label)| (d.key, label))
    }

    #[test]
    fn the_seven_dialects_have_distinct_frames() {
        assert_eq!(DIALECTS.len(), 7);
        // Distinct at a glance means no two dialects share the whole
        // (accent, tint, dash) triple — the label alone never carries it.
        for (i, a) in DIALECTS.iter().enumerate() {
            for b in &DIALECTS[i + 1..] {
                assert!(
                    (a.accent, a.tint, a.dash) != (b.accent, b.tint, b.dash),
                    "{} and {} wear the same frame",
                    a.key,
                    b.key
                );
            }
        }
    }

    #[test]
    fn fences_classify_and_label() {
        assert_eq!(
            classify_info("wolf,run(exit=0, stdout=\"hi\")"),
            Some(("program", "wolf · runs, exit 0".into()))
        );
        assert_eq!(
            classify_info("wolf,part(greet)"),
            Some(("part", "wolf · part(greet)".into()))
        );
        assert_eq!(
            classify_info("wolf,part(greet, cont),run(exit=0)"),
            Some(("part", "wolf · part(greet) · runs, exit 0".into()))
        );
        assert_eq!(classify_info("wolf-repl"), Some(("repl", "repl".into())));
        assert_eq!(
            classify_info("console,from(book/ch01/s2)"),
            Some(("console", "console".into()))
        );
        assert_eq!(
            classify_info("c-run,from(the alphabetized walk)"),
            Some(("twin", "c · run".into()))
        );
        assert_eq!(
            classify_info("diagnostic,from(ch03/ex3-2)"),
            Some(("diagnostic", "diagnostic".into()))
        );
        assert_eq!(
            classify_info("rust"),
            Some(("contrast", "rust · contrast".into()))
        );
        assert_eq!(
            classify_info("c"),
            Some(("contrast", "c · contrast".into()))
        );
        assert_eq!(
            classify_info("wolf,fail(E1001)"),
            Some(("program", "wolf · rejected, E1001".into()))
        );
        assert_eq!(
            classify_info("wolf,run(exit=trap(bounds))"),
            Some(("program", "wolf · runs, trap(bounds)".into()))
        );
        // Figures are not dialects.
        assert_eq!(classify_info("text"), None);
        assert_eq!(classify_info("ebnf"), None);
    }

    #[test]
    fn css_and_typst_carry_every_dialect() {
        let css = dialect_css();
        let typ = typst_defs();
        for d in DIALECTS {
            assert!(css.contains(&format!("pre.dialect-{}", d.key)), "{}", d.key);
            assert!(css.contains(d.accent) && css.contains(d.tint));
            assert!(
                typ.contains(&format!("#let dialect-{}(", d.key)),
                "{}",
                d.key
            );
        }
        // The label channel exists on both renders.
        assert!(css.contains("content: attr(data-dialect)"));
        assert!(typ.contains("#dtag"));
    }
}
