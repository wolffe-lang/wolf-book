//! The mdBook preprocessor (DESIGN.md §5, obligations 2 and 3): rewrites
//! heading ids to TOC section numbers and renders code fences to classed
//! spans at build time from the vendored grammars. highlight.js never
//! runs; the theme ships it as an empty file.

use crate::directives::parse_fence_info;
use crate::fence::{self, Fence};
use crate::highlight::{render_code_html, render_output_html};
use crate::tm::Grammar;
use anyhow::{Context, Result};
use std::io::Read;
use std::path::Path;

pub struct Grammars {
    pub wolf: Grammar,
    pub wolfi: Grammar,
    pub wolf_pkg: Grammar,
}

pub fn load_grammars(root: &Path) -> Result<Grammars> {
    let read = |name: &str| -> Result<String> {
        std::fs::read_to_string(root.join("highlight").join(name))
            .with_context(|| format!("reading vendored grammar {name}"))
    };
    let wolf_json = read("wolf.tmLanguage.json")?;
    let wolfi_json = read("wolfi.tmLanguage.json")?;
    let pkg_json = read("wolf-pkg.tmLanguage.json")?;
    Ok(Grammars {
        wolf: Grammar::load(&wolf_json).context("loading wolf.tmLanguage.json")?,
        wolfi: Grammar::load_with(&wolfi_json, &[&wolf_json])
            .context("loading wolfi.tmLanguage.json")?,
        wolf_pkg: Grammar::load_with(&pkg_json, &[&wolf_json])
            .context("loading wolf-pkg.tmLanguage.json")?,
    })
}

/// The full chapter transform: anchors, then fences.
pub fn transform_chapter(grammars: &Grammars, content: &str) -> Result<String> {
    let anchored = rewrite_heading_anchors(content);
    render_fences(grammars, &anchored)
}

/// Section-number anchors (DESIGN.md §3): a heading whose text begins
/// with `N.M ` gets id `N.M`; a chapter heading `N. Title` gets id `N`.
/// Auto-slugged text ids drift when a title is edited, so they are not
/// the anchor. Uses mdBook's `{#id}` heading-attribute syntax.
pub fn rewrite_heading_anchors(content: &str) -> String {
    let mut out = String::new();
    for seg in fence::segments(content) {
        match seg {
            fence::Segment::Fence(f) => {
                let marker = "`".repeat(f.ticks);
                out.push_str(&format!("{marker}{}\n{}{marker}\n", f.info, f.content));
            }
            fence::Segment::Text(text) => {
                for line in text.lines() {
                    out.push_str(&anchor_heading_line(line));
                    out.push('\n');
                }
            }
        }
    }
    out
}

fn anchor_heading_line(line: &str) -> String {
    let hashes = line.chars().take_while(|&c| c == '#').count();
    if hashes == 0 || hashes > 6 {
        return line.to_string();
    }
    let rest = &line[hashes..];
    if !rest.starts_with(' ') {
        return line.to_string();
    }
    let text = rest.trim_start();
    if text.contains("{#") {
        return line.to_string(); // an explicit id wins
    }
    if let Some(id) = leading_section_number(text) {
        return format!("{} {} {{#{}}}", "#".repeat(hashes), text, id);
    }
    line.to_string()
}

/// `8.4 Title` → `8.4`; `8. Title` → `8`. Nothing else anchors.
fn leading_section_number(text: &str) -> Option<String> {
    let first = text.split_whitespace().next()?;
    let trimmed = first.trim_end_matches('.');
    if trimmed.is_empty()
        || !trimmed
            .split('.')
            .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
    {
        return None;
    }
    // Chapter headings are written `8.`; sections `8.4`.
    if first.ends_with('.') && !trimmed.contains('.') {
        return Some(trimmed.to_string());
    }
    if trimmed.contains('.') && first == trimmed {
        return Some(trimmed.to_string());
    }
    None
}

/// Render every recognized fence to pre-highlighted HTML. Directives
/// are build instructions, not reader content: they never render.
pub fn render_fences(grammars: &Grammars, content: &str) -> Result<String> {
    let mut err: Option<anyhow::Error> = None;
    let out = fence::rewrite(content, |f: &Fence| match render_one_fence(grammars, f) {
        Ok(Some(html)) => html,
        Ok(None) => {
            let marker = "`".repeat(f.ticks);
            format!("{marker}{}\n{}{marker}\n", f.info, f.content)
        }
        Err(e) => {
            if err.is_none() {
                err = Some(e);
            }
            String::new()
        }
    });
    match err {
        Some(e) => Err(e),
        None => Ok(out),
    }
}

fn render_one_fence(grammars: &Grammars, f: &Fence) -> Result<Option<String>> {
    let info = f.info.trim();
    if info.is_empty() {
        return Ok(None);
    }
    let fi = parse_fence_info(info).with_context(|| format!("bad fence info `{}`", info))?;
    let (dialect, body) = match fi.lang.as_str() {
        "wolf" => {
            let dialect = if fi.part.is_some() { "part" } else { "program" };
            (dialect, render_code_html(&grammars.wolf, &f.content)?)
        }
        "wolfi" => ("program", render_code_html(&grammars.wolfi, &f.content)?),
        "wolf-pkg" => ("program", render_code_html(&grammars.wolf_pkg, &f.content)?),
        "wolf-repl" => ("repl", render_output_html("repl", &f.content)),
        "console" => ("console", render_output_html("console", &f.content)),
        // A C twin's run (bs10). It reads as a console transcript and it is
        // a checked claim: `cargo xtask contrast` derives every line of it
        // from the named case in `samples/contrast/cases.toml`. It renders
        // as the console dialect, and its `from(…)` never reaches the page.
        "c-run" => ("console", render_output_html("console", &f.content)),
        "diagnostic" => ("diagnostic", render_output_html("diagnostic", &f.content)),
        _ => return Ok(None), // text, mermaid, … — not ours
    };
    let lang_class = fi.lang.replace('.', "-");
    Ok(Some(format!(
        "<pre class=\"dialect-{dialect} language-{lang_class}\"><code>{body}</code></pre>\n\n"
    )))
}

/// The mdBook CmdPreprocessor protocol: `supports <renderer>` exits 0;
/// otherwise read `[context, book]` JSON on stdin, transform every
/// chapter, print the book JSON on stdout.
pub fn cmd_preprocess(root: &Path, args: &[String]) -> Result<()> {
    if args.first().map(String::as_str) == Some("supports") {
        // html only — the typst path renders from source, not through mdBook.
        let renderer = args.get(1).map(String::as_str).unwrap_or("");
        if renderer == "html" {
            return Ok(());
        }
        std::process::exit(1);
    }

    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .context("reading preprocessor input")?;
    let parsed: serde_json::Value =
        serde_json::from_str(&input).context("parsing preprocessor input")?;
    let mut book = parsed
        .as_array()
        .and_then(|a| a.get(1))
        .cloned()
        .context("preprocessor input is not [context, book]")?;

    let grammars = load_grammars(root)?;
    if let Some(sections) = book.get_mut("sections").and_then(|s| s.as_array_mut()) {
        for item in sections {
            transform_item(&grammars, item)?;
        }
    }
    println!("{}", serde_json::to_string(&book)?);
    Ok(())
}

fn transform_item(grammars: &Grammars, item: &mut serde_json::Value) -> Result<()> {
    let Some(chapter) = item.get_mut("Chapter") else {
        return Ok(());
    };
    if let Some(content) = chapter.get("content").and_then(|c| c.as_str()) {
        let name = chapter
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("<unnamed>")
            .to_string();
        let new = transform_chapter(grammars, content)
            .with_context(|| format!("preprocessing chapter `{name}`"))?;
        chapter["content"] = serde_json::Value::String(new);
    }
    if let Some(subs) = chapter.get_mut("sub_items").and_then(|s| s.as_array_mut()) {
        for sub in subs {
            transform_item(grammars, sub)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chapter_heading_anchor() {
        assert_eq!(
            anchor_heading_line("# 8. Regions: memory in the shape you meant"),
            "# 8. Regions: memory in the shape you meant {#8}"
        );
    }

    #[test]
    fn section_heading_anchor() {
        assert_eq!(
            anchor_heading_line("## 8.4 Cycles are fine here"),
            "## 8.4 Cycles are fine here {#8.4}"
        );
    }

    #[test]
    fn unnumbered_heading_untouched() {
        assert_eq!(anchor_heading_line("## Exercises"), "## Exercises");
        assert_eq!(anchor_heading_line("# Notation"), "# Notation");
    }

    #[test]
    fn explicit_id_wins() {
        assert_eq!(
            anchor_heading_line("## 8.4 Cycles {#custom}"),
            "## 8.4 Cycles {#custom}"
        );
    }

    #[test]
    fn headings_inside_fences_untouched() {
        let md = "```text\n# 8. not a heading\n```\n";
        assert_eq!(rewrite_heading_anchors(md), md);
    }

    #[test]
    fn fences_render_to_pre_spans() {
        let root = crate::repo_root().unwrap();
        let grammars = load_grammars(&root).unwrap();
        let md = "intro\n\n```wolf,run(exit=0)\nfn main() -> !int { 0 }\n```\n";
        let out = render_fences(&grammars, md).unwrap();
        assert!(out.contains("<pre class=\"dialect-program language-wolf\">"));
        assert!(out.contains("<span class=\"hl-kw\">fn</span>"));
        // The directive never reaches the reader.
        assert!(!out.contains("run(exit=0)"));
    }

    #[test]
    fn console_fence_renders() {
        let root = crate::repo_root().unwrap();
        let grammars = load_grammars(&root).unwrap();
        let md = "```console\n$ lupin hello.lu\nhello, wolf\n```\n";
        let out = render_fences(&grammars, md).unwrap();
        assert!(out.contains("dialect-console"));
        assert!(out.contains("hl-prompt"));
    }

    #[test]
    fn a_twin_run_renders_as_a_console_and_keeps_its_case_name_off_the_page() {
        let root = crate::repo_root().unwrap();
        let grammars = load_grammars(&root).unwrap();
        let md = "```c-run,from(the alphabetized walk)\n$ ./wordtree\n   3 wolf\n```\n";
        let out = render_fences(&grammars, md).unwrap();
        assert!(out.contains("dialect-console"));
        assert!(out.contains("hl-prompt"));
        // The binding to `cases.toml` is CI's business, not the reader's.
        assert!(!out.contains("the alphabetized walk"));
    }
}
