//! The single-source highlight contract (DESIGN.md §2). The vendored
//! tmLanguage grammars are the only wolf syntax definition; this module
//! owns the one scope→class mapping and the one palette, from which
//! both the web CSS (`theme/highlight.css`) and the typst text styles
//! (render.rs) are generated. Four roles, five colors, all of them
//! distinguishable on a monochrome laser: keywords are bold ink,
//! comments gray italic, strings deep green, numbers dark bronze,
//! types dark blue, diagnostics dark red.

use crate::tm::{tokenize, Grammar};
use anyhow::{bail, Context, Result};
use std::fmt::Write as _;
use std::path::Path;

/// One visual role. `color: None` means body ink.
pub struct Style {
    pub class: &'static str,
    pub color: Option<&'static str>,
    pub bold: bool,
    pub italic: bool,
}

/// The palette — the one definition every output format inherits.
pub const STYLES: &[Style] = &[
    Style {
        class: "hl-kw",
        color: None,
        bold: true,
        italic: false,
    },
    Style {
        class: "hl-comment",
        color: Some("#6e6e6e"),
        bold: false,
        italic: true,
    },
    Style {
        class: "hl-string",
        color: Some("#355e3b"),
        bold: false,
        italic: false,
    },
    Style {
        class: "hl-number",
        color: Some("#7a4a12"),
        bold: false,
        italic: false,
    },
    Style {
        class: "hl-type",
        color: Some("#1a4f8a"),
        bold: false,
        italic: false,
    },
    Style {
        class: "hl-attr",
        color: Some("#6e6e6e"),
        bold: false,
        italic: false,
    },
    Style {
        class: "hl-err",
        color: Some("#a12622"),
        bold: true,
        italic: false,
    },
    Style {
        class: "hl-warn",
        color: Some("#7a4a12"),
        bold: true,
        italic: false,
    },
    Style {
        class: "hl-prompt",
        color: Some("#6e6e6e"),
        bold: false,
        italic: false,
    },
];

pub fn style_for(class: &str) -> Option<&'static Style> {
    STYLES.iter().find(|s| s.class == class)
}

/// TextMate scope → CSS class. Prefix-matched, first hit wins; scopes
/// with no entry render as body ink (operators and punctuation stay
/// quiet on purpose — restraint is the palette's design rule).
const SCOPE_CLASSES: &[(&str, &str)] = &[
    ("comment.", "hl-comment"),
    ("string.", "hl-string"),
    ("constant.numeric", "hl-number"),
    ("constant.character.escape", "hl-number"),
    ("constant.other.format-spec", "hl-number"),
    ("constant.language", "hl-kw"),
    ("keyword.control", "hl-kw"),
    ("keyword.other", "hl-kw"),
    ("storage.", "hl-kw"),
    ("support.type", "hl-type"),
    ("support.function.generalized", "hl-type"),
    ("meta.attribute", "hl-attr"),
    ("punctuation.definition.attribute", "hl-attr"),
    ("punctuation.definition.string", "hl-string"),
    ("punctuation.section.interpolation", "hl-number"),
    // keyword.operator deliberately unmapped: ink.
];

pub fn class_for_scope(scope: &str) -> Option<&'static str> {
    SCOPE_CLASSES
        .iter()
        .find(|(prefix, _)| scope.starts_with(prefix))
        .map(|(_, class)| *class)
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Render wolf-family code to classed spans (no `<pre>` wrapper).
pub fn render_code_html(grammar: &Grammar, code: &str) -> Result<String> {
    let mut out = String::new();
    let lines: Vec<&str> = code.lines().collect();
    let tokenized = tokenize(grammar, code)?;
    for (line, tokens) in lines.iter().zip(&tokenized) {
        for t in tokens {
            let text = escape_html(&line[t.start..t.end]);
            match t.scope.as_deref().and_then(class_for_scope) {
                Some(class) => {
                    let _ = write!(out, r#"<span class="{class}">{text}</span>"#);
                }
                None => out.push_str(&text),
            }
        }
        out.push('\n');
    }
    Ok(out)
}

/// Decorate console runs, REPL transcripts, and diagnostics — the
/// non-grammar dialects. Rule-based and small: prompts gray,
/// `error[E…]`/`trap(…)` heads dark red, `warning[…]` bronze. The text
/// itself is byte-identical to the tool output.
pub fn render_output_html(kind: &str, text: &str) -> String {
    let mut out = String::new();
    for line in text.lines() {
        let escaped = escape_html(line);
        let decorated = if kind == "console" && line.starts_with("$ ") {
            format!(r#"<span class="hl-prompt">$</span>{}"#, &escaped[1..])
        } else if kind == "repl" && line.starts_with("wolf>") {
            format!(
                r#"<span class="hl-prompt">wolf&gt;</span>{}"#,
                &escaped["wolf&gt;".len()..]
            )
        } else if let Some(rest) = line_head(&escaped, "error[") {
            format!(r#"<span class="hl-err">{}</span>{}"#, rest.0, rest.1)
        } else if let Some(rest) = line_head(&escaped, "warning[") {
            format!(r#"<span class="hl-warn">{}</span>{}"#, rest.0, rest.1)
        } else if escaped.contains(": trap(") {
            match escaped.split_once(": trap(") {
                Some((pre, post)) => match post.split_once(')') {
                    Some((kind_txt, tail)) => {
                        format!(r#"{pre}: <span class="hl-err">trap({kind_txt})</span>{tail}"#)
                    }
                    None => escaped.clone(),
                },
                None => escaped.clone(),
            }
        } else {
            escaped.clone()
        };
        out.push_str(&decorated);
        out.push('\n');
    }
    out
}

/// `error[E1001]:` head split: returns (head-with-colon, rest).
fn line_head<'a>(line: &'a str, prefix: &str) -> Option<(&'a str, &'a str)> {
    if !line.starts_with(prefix) {
        return None;
    }
    let colon = line.find(':')?;
    Some((&line[..=colon], &line[colon + 1..]))
}

/// The one highlight stylesheet, generated from `STYLES`.
pub fn generate_css() -> String {
    let mut css = String::from(
        "/* GENERATED by `cargo xtask grammar-sync` from the palette in\n\
         \x20  xtask/src/highlight.rs — the same definition the typst PDF\n\
         \x20  styles come from. Do not edit by hand (DESIGN.md §2). */\n\n",
    );
    for s in STYLES {
        let mut body = String::new();
        if let Some(c) = s.color {
            let _ = write!(body, " color: {c};");
        }
        if s.bold {
            body.push_str(" font-weight: 700;");
        }
        if s.italic {
            body.push_str(" font-style: italic;");
        }
        let _ = writeln!(css, ".{} {{{} }}", s.class, body);
    }
    css
}

/// The grammars the book vendors, and where they come from in wolf-lsp.
pub const GRAMMARS: &[&str] = &[
    "wolf.tmLanguage.json",
    "wolfi.tmLanguage.json",
    "wolf-pkg.tmLanguage.json",
];
const LSP_SYNTAX_DIR: &str = "clients/vscode/syntaxes";

/// `cargo xtask grammar-sync [--check] [--lsp-path <dir>]`
///
/// Re-vendors the generated tmLanguage files from a wolf-lsp checkout
/// and regenerates `theme/highlight.css`. `--check` verifies instead of
/// writing: any byte drift between `highlight/`, the checkout at the
/// pinned rev, and the generated CSS fails the build (the book-side
/// arrow of wolf-lsp's own `grammar-drift` discipline).
pub fn grammar_sync(root: &Path, args: &[String]) -> Result<()> {
    let check = args.iter().any(|a| a == "--check");
    let lsp_path = args
        .iter()
        .position(|a| a == "--lsp-path")
        .and_then(|i| args.get(i + 1))
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| root.parent().unwrap_or(root).join("wolf-lsp"));

    let pins = crate::load_pins(root)?;
    let mut failures = Vec::new();

    // 1. The checkout, if present, must sit at the pin; drift in file
    //    bytes against it fails regardless of rev (belt and braces).
    if lsp_path.is_dir() {
        let rev = git_head(&lsp_path);
        match rev {
            Some(rev) if rev == pins.wolf_lsp_rev => {
                println!("grammar-sync: wolf-lsp checkout at pin {}", &rev[..12]);
            }
            Some(rev) => {
                let msg = format!(
                    "wolf-lsp checkout is at {} but the pin is {} — \
                     bump wolf-toolchain.toml deliberately, in its own commit",
                    &rev[..12.min(rev.len())],
                    &pins.wolf_lsp_rev[..12]
                );
                if check {
                    failures.push(msg);
                } else {
                    println!("grammar-sync: WARNING: {msg}");
                }
            }
            None => println!("grammar-sync: wolf-lsp checkout has no readable git rev"),
        }
        for name in GRAMMARS {
            let src = lsp_path.join(LSP_SYNTAX_DIR).join(name);
            let dst = root.join("highlight").join(name);
            let src_bytes =
                std::fs::read(&src).with_context(|| format!("reading {}", src.display()))?;
            if check {
                let dst_bytes = std::fs::read(&dst).unwrap_or_default();
                if src_bytes != dst_bytes {
                    failures.push(format!(
                        "vendored {} drifted from the wolf-lsp checkout",
                        name
                    ));
                }
            } else {
                std::fs::write(&dst, &src_bytes)
                    .with_context(|| format!("writing {}", dst.display()))?;
                println!("grammar-sync: vendored {name}");
            }
        }
    } else if check {
        // No checkout: report loudly, verify what we can (CSS below).
        println!(
            "grammar-sync: SKIP checkout comparison — no wolf-lsp at {} \
             (CI checks out the pin; locally pass --lsp-path)",
            lsp_path.display()
        );
    } else {
        bail!(
            "no wolf-lsp checkout at {} — pass --lsp-path",
            lsp_path.display()
        );
    }

    // 2. The one CSS file.
    let css = generate_css();
    let css_path = root.join("theme/highlight.css");
    if check {
        let existing = std::fs::read_to_string(&css_path).unwrap_or_default();
        if existing != css {
            failures.push(
                "theme/highlight.css drifted from the palette — run `cargo xtask grammar-sync`"
                    .into(),
            );
        }
    } else {
        std::fs::write(&css_path, css).context("writing theme/highlight.css")?;
        println!("grammar-sync: wrote theme/highlight.css");
    }

    // 3. Whatever happens, the vendored grammars must load in the
    //    interpreter — a grammar bump that outgrows tm.rs fails here.
    crate::preprocess::load_grammars(root).context("loading the vendored grammars after sync")?;

    if !failures.is_empty() {
        for f in &failures {
            eprintln!("grammar-sync: DRIFT: {f}");
        }
        bail!(
            "grammar-sync --check failed ({} finding(s))",
            failures.len()
        );
    }
    println!("grammar-sync: ok");
    Ok(())
}

fn git_head(dir: &Path) -> Option<String> {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_has_every_class() {
        let css = generate_css();
        for s in STYLES {
            assert!(css.contains(s.class));
        }
    }

    #[test]
    fn scope_mapping_is_restrained() {
        assert_eq!(class_for_scope("keyword.operator.wolf"), None);
        assert_eq!(
            class_for_scope("comment.line.double-slash.wolf"),
            Some("hl-comment")
        );
        assert_eq!(class_for_scope("storage.type.wolf"), Some("hl-kw"));
    }

    #[test]
    fn wolf_hello_renders_spans() {
        let root = crate::repo_root().unwrap();
        let json = std::fs::read_to_string(root.join("highlight/wolf.tmLanguage.json")).unwrap();
        let g = Grammar::load(&json).unwrap();
        let html =
            render_code_html(&g, "fn main() -> !int {\n    print(\"hello\")\n    0\n}\n").unwrap();
        assert!(html.contains(r#"<span class="hl-kw">fn</span>"#));
        assert!(html.contains(r#"<span class="hl-string">"#));
        assert!(!html.contains("<pre"));
    }

    #[test]
    fn diagnostic_decoration() {
        let html = render_output_html(
            "diagnostic",
            "error[E1001]: `p.lead` is used here after its value moved away\n",
        );
        assert!(html.starts_with(r#"<span class="hl-err">error[E1001]:</span>"#));
    }

    #[test]
    fn console_prompt_decoration() {
        let html = render_output_html("console", "$ lupin hello.lu\nhello, wolf\n");
        assert!(html.contains(r#"<span class="hl-prompt">$</span>"#));
        assert!(html.contains("hello, wolf"));
    }
}
