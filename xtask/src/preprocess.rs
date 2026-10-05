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

/// The full chapter transform, anchors then fences, for a page
/// `to_root` below the book's root (`""` for
/// `ch07.md`, `"../"` for `front/notation.md`): the prefix a page needs to
/// reach `diagrams/`.
pub fn transform_chapter_at(grammars: &Grammars, content: &str, to_root: &str) -> Result<String> {
    let anchored = rewrite_heading_anchors(content);
    render_fences_at(grammars, &anchored, to_root)
}

/// The relative path from a chapter's page back to the book's root.
pub fn to_root(chapter_path: &str) -> String {
    "../".repeat(chapter_path.matches(['/', '\\']).count())
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

pub fn anchor_heading_line(line: &str) -> String {
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
#[cfg(test)]
pub fn render_fences(grammars: &Grammars, content: &str) -> Result<String> {
    render_fences_at(grammars, content, "")
}

fn render_fences_at(grammars: &Grammars, content: &str, to_root: &str) -> Result<String> {
    let mut err: Option<anyhow::Error> = None;
    let out = fence::rewrite(content, |f: &Fence| {
        match render_one_fence(grammars, f, to_root) {
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
        }
    });
    match err {
        Some(e) => Err(e),
        None => Ok(out),
    }
}

fn render_one_fence(grammars: &Grammars, f: &Fence, to_root: &str) -> Result<Option<String>> {
    let info = f.info.trim();
    if info.is_empty() {
        return Ok(None);
    }
    let fi = parse_fence_info(info).with_context(|| format!("bad fence info `{}`", info))?;
    // A memory diagram (bs62) is a figure: the generated SVG, with the
    // fence's text rendering as its alt. A static image, never a script
    // (lupp.us: `script-src 'self'`).
    if fi.lang == crate::diagrams::LANG {
        let svg = crate::diagrams::svg_for_info(info)
            .with_context(|| format!("a memory fence needs from(…) and line(N): `{info}`"))?;
        return Ok(Some(memory_figure(&format!("{to_root}{svg}"), &f.content)));
    }
    // The taxonomy is dialects.rs's (rp03): one table classes the fence,
    // names it for the reader, and styles it on both renders. A fence
    // outside the table (text, ebnf, …) is a figure, not a dialect.
    let Some((dialect, label)) = crate::dialects::classify(&fi) else {
        return Ok(None);
    };
    let body = match fi.lang.as_str() {
        "wolf" => render_code_html(&grammars.wolf, &f.content)?,
        "wolfi" => render_code_html(&grammars.wolfi, &f.content)?,
        "wolf-pkg" => render_code_html(&grammars.wolf_pkg, &f.content)?,
        "wolf-repl" => render_output_html("repl", &f.content),
        // A C twin's run (bs10) reads as a console transcript and is a
        // checked claim: `cargo xtask contrast` derives every line of it
        // from the named case in `samples/contrast/cases.toml`. Its
        // `from(…)` binding never reaches the page — the label says
        // what it is; the case name is CI's business.
        "console" | "c-run" => render_output_html("console", &f.content),
        "diagnostic" => render_output_html("diagnostic", &f.content),
        // Contrast code (rust, c): escaped verbatim — no wolf grammar
        // pretends to know another language's tokens.
        "rust" | "c" => crate::highlight::render_plain_html(&f.content),
        other => unreachable!("dialects::classify admitted unhandled lang `{other}`"),
    };
    let lang_class = fi.lang.replace('.', "-");
    Ok(Some(format!(
        "<pre class=\"dialect-{} language-{lang_class}\" data-dialect=\"{}\"><code>{body}</code></pre>\n\n",
        dialect.key,
        crate::highlight::escape_html(&label)
    )))
}

/// The web edition's memory diagram: the SVG as an image, its text
/// rendering as the alt, so a reader who cannot see the drawing reads the
/// same two trees.
pub fn memory_figure(src: &str, text: &str) -> String {
    let alt = text.trim_end().replace('\n', " / ");
    format!(
        "<figure class=\"memory-diagram\"><img src=\"{}\" alt=\"{}\"></figure>\n\n",
        attr(src),
        attr(&alt)
    )
}

/// An HTML attribute value: the text escapes plus the quote.
fn attr(s: &str) -> String {
    crate::highlight::escape_html(s).replace('"', "&quot;")
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
    // The sidebar's label is mdBook's `number` field, which it assigns
    // by POSITION in SUMMARY. Chapter 33 sits between 25 and 26
    // (front/how-to-read.md: numbers are permanent links), so the
    // positional count labelled it "26." and every later chapter one off
    // (bs55). The chapter's own `# N.` heading is the number; a numbered
    // chapter takes it, an unnumbered one stays unnumbered.
    let own = chapter
        .get("content")
        .and_then(|c| c.as_str())
        .and_then(own_number);
    if let Some(n) = own {
        if chapter.get("number").is_some_and(|v| !v.is_null()) {
            chapter["number"] = serde_json::json!([n]);
        }
    }
    if let Some(content) = chapter.get("content").and_then(|c| c.as_str()) {
        let name = chapter
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("<unnamed>")
            .to_string();
        let path = chapter
            .get("path")
            .and_then(|p| p.as_str())
            .unwrap_or_default()
            .to_string();
        let new = transform_chapter_at(grammars, content, &to_root(&path))
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

/// The number a chapter gives itself: its first line, `# 33. The
/// serving loop`, says 33. Nothing else is a chapter's number.
pub fn own_number(content: &str) -> Option<u32> {
    let first = content.lines().find(|l| !l.trim().is_empty())?;
    let text = first.strip_prefix("# ")?.trim_start();
    let (num, rest) = text.split_once(". ")?;
    if num.is_empty() || !num.chars().all(|c| c.is_ascii_digit()) || rest.trim().is_empty() {
        return None;
    }
    num.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn own_number_is_the_first_heading() {
        assert_eq!(own_number("# 33. The serving loop\n\nBody."), Some(33));
        assert_eq!(own_number("\n# 1. Hello, Wolf\n"), Some(1));
        assert_eq!(own_number("# How to read this book\n"), None);
        assert_eq!(own_number("# Appendix A — Grammar summary\n"), None);
        // A section heading is not a chapter number, nor is a later line.
        assert_eq!(own_number("## 33.1 Accept\n"), None);
        assert_eq!(own_number("Intro.\n# 7. Late\n"), None);
        assert_eq!(own_number("# 33.\n"), None);
    }

    #[test]
    fn chapter_takes_its_own_number_not_its_position() {
        // mdBook numbered ch33 "26" by its place in SUMMARY.
        let mut item = serde_json::json!({"Chapter": {
            "name": "The serving loop",
            "content": "# 33. The serving loop\n\nText.\n",
            "number": [26],
            "sub_items": []
        }});
        let grammars = load_grammars(&crate::repo_root().unwrap()).unwrap();
        transform_item(&grammars, &mut item).unwrap();
        assert_eq!(item["Chapter"]["number"], serde_json::json!([33]));
        // A prefix chapter (no number) stays unnumbered.
        let mut front = serde_json::json!({"Chapter": {
            "name": "How to read this book",
            "content": "# How to read this book\n",
            "number": null,
            "sub_items": []
        }});
        transform_item(&grammars, &mut front).unwrap();
        assert!(front["Chapter"]["number"].is_null());
    }

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
        assert!(out.contains(
            "<pre class=\"dialect-program language-wolf\" data-dialect=\"wolf · runs, exit 0\">"
        ));
        assert!(out.contains("<span class=\"hl-kw\">fn</span>"));
        // The directive never reaches the reader raw — the label is its
        // reader-facing spelling.
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
    fn a_twin_run_renders_as_its_own_dialect_and_keeps_its_case_name_off_the_page() {
        let root = crate::repo_root().unwrap();
        let grammars = load_grammars(&root).unwrap();
        let md = "```c-run,from(the alphabetized walk)\n$ ./wordtree\n   3 wolf\n```\n";
        let out = render_fences(&grammars, md).unwrap();
        assert!(out.contains("dialect-twin"));
        assert!(out.contains("data-dialect=\"c · run\""));
        assert!(out.contains("hl-prompt"));
        // The binding to `cases.toml` is CI's business, not the reader's.
        assert!(!out.contains("the alphabetized walk"));
    }

    #[test]
    fn contrast_code_renders_labeled_and_escaped() {
        let root = crate::repo_root().unwrap();
        let grammars = load_grammars(&root).unwrap();
        let md = "```rust\npub fn tokens(input: &str) -> Vec<Token<'_>> {}\n```\n";
        let out = render_fences(&grammars, md).unwrap();
        assert!(out.contains("dialect-contrast"));
        assert!(out.contains("data-dialect=\"rust · contrast\""));
        // Escaped verbatim, no wolf-grammar spans.
        assert!(out.contains("Vec&lt;Token&lt;'_&gt;&gt;"));
        assert!(!out.contains("hl-kw"));
    }

    #[test]
    fn a_memory_fence_becomes_a_static_figure() {
        let root = crate::repo_root().unwrap();
        let grammars = load_grammars(&root).unwrap();
        let md = "```memory,from(book/ch07/s3),line(5)\nbefore line 5: let who = move d.meta.author\nd  Doc\n   └─ title  \"regions\"\n```\n";
        let out = render_fences_at(&grammars, md, "../").unwrap();
        assert!(out
            .contains("<figure class=\"memory-diagram\"><img src=\"../diagrams/ch07/s3-L5.svg\""));
        assert!(out.contains(
            "alt=\"before line 5: let who = move d.meta.author / d  Doc /    └─ title  &quot;regions&quot;\""
        ));
        assert!(!out.contains("<script"));
        assert!(!out.contains("```"));
        assert_eq!(to_root("ch07.md"), "");
        assert_eq!(to_root("front/notation.md"), "../");
    }

    #[test]
    fn part_fences_carry_their_name_in_the_label() {
        let root = crate::repo_root().unwrap();
        let grammars = load_grammars(&root).unwrap();
        let md = "```wolf,part(greet)\nfn greet() -> str { \"hi\" }\n```\n";
        let out = render_fences(&grammars, md).unwrap();
        assert!(out.contains("dialect-part"));
        assert!(out.contains("data-dialect=\"wolf · part(greet)\""));
    }
}
