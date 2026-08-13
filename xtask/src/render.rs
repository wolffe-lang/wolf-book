//! `cargo xtask render` — one source tree, three artifacts, one set of
//! decisions (DESIGN.md §4): the mdBook web edition (canonical), the
//! single-file `wolf-book.md`, and the typst-set `wolf-book.pdf`.

use crate::directives::parse_fence_info;
use crate::fence::{segments, Segment};
use crate::highlight::{class_for_scope, style_for};
use crate::preprocess::load_grammars;
use crate::tm::{tokenize, Grammar};
use anyhow::{bail, Context, Result};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

pub fn run(root: &Path, args: &[String]) -> Result<()> {
    let target = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map(String::as_str)
        .unwrap_or("all");
    let require_pdf = args.iter().any(|a| a == "--require-pdf");
    match target {
        "web" => render_web(root),
        "md" => render_md(root).map(|_| ()),
        "pdf" => render_pdf(root, require_pdf),
        "all" => {
            render_web(root)?;
            render_md(root)?;
            render_pdf(root, require_pdf)
        }
        other => bail!("unknown render target `{other}` (web|md|pdf|all)"),
    }
}

// ------------------------------------------------------------------- web

fn render_web(root: &Path) -> Result<()> {
    // mdBook as a library: book.toml declares the wolf preprocessor
    // (this same xtask, via the Cmd protocol), the theme replacement,
    // and the build dir. This is `mdbook build`, cargo-only.
    let md =
        mdbook::MDBook::load(root).map_err(|e| anyhow::anyhow!("loading mdBook config: {e}"))?;
    md.build()
        .map_err(|e| anyhow::anyhow!("mdbook build failed: {e}"))?;
    println!(
        "render: web edition at {}",
        root.join("target/render/web").display()
    );
    Ok(())
}

// -------------------------------------------------------------- summary

pub struct TocEntry {
    pub part: Option<String>,
    #[allow(dead_code)] // structural; the PDF outline will want it (bs11)
    pub title: String,
    pub path: PathBuf,
}

/// Parse SUMMARY.md into ordered entries with their part titles.
pub fn parse_summary(root: &Path) -> Result<Vec<TocEntry>> {
    let text = std::fs::read_to_string(root.join("book/SUMMARY.md"))?;
    let mut out = Vec::new();
    let mut part: Option<String> = None;
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(title) = trimmed.strip_prefix("# ") {
            if title != "Summary" {
                part = Some(title.to_string());
            }
            continue;
        }
        let item = trimmed.strip_prefix("- ").unwrap_or(trimmed);
        if let Some((title, path)) = parse_link(item) {
            out.push(TocEntry {
                part: part.clone(),
                title,
                path: root.join("book").join(path),
            });
        }
    }
    Ok(out)
}

fn parse_link(s: &str) -> Option<(String, String)> {
    let s = s.strip_prefix('[')?;
    let (title, rest) = s.split_once("](")?;
    let (path, _) = rest.split_once(')')?;
    Some((title.to_string(), path.to_string()))
}

// -------------------------------------------------------- wolf-book.md

/// The one-file markdown concatenation: block directives stripped, for
/// grep-shaped readers and downstream tools.
fn render_md(root: &Path) -> Result<PathBuf> {
    let entries = parse_summary(root)?;
    let mut out = String::from(
        "# The Wolf Book\n\n\
         <!-- GENERATED single-file edition (`cargo xtask render md`).\n\
         \x20    The web edition is canonical; this file strips build\n\
         \x20    directives and concatenates every page in reading order. -->\n\n",
    );
    let mut last_part: Option<String> = None;
    for e in &entries {
        if e.part != last_part {
            if let Some(p) = &e.part {
                out.push_str(&format!("\n---\n\n# {p}\n\n"));
            }
            last_part = e.part.clone();
        }
        let content = std::fs::read_to_string(&e.path)
            .with_context(|| format!("reading {}", e.path.display()))?;
        out.push_str(&strip_directives(&content));
        out.push('\n');
    }
    let dir = root.join("target/render");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join("wolf-book.md");
    std::fs::write(&path, out)?;
    println!("render: single-file markdown at {}", path.display());
    Ok(path)
}

/// `wolf,run(…)` → `wolf`; `diagnostic,from(…)` → `diagnostic`.
fn strip_directives(content: &str) -> String {
    crate::fence::rewrite(content, |f| {
        let marker = "`".repeat(f.ticks);
        let lang = f.info.split(',').next().unwrap_or("").trim();
        format!("{marker}{lang}\n{}{marker}\n", f.content)
    })
}

// -------------------------------------------------------- wolf-book.pdf

fn render_pdf(root: &Path, require: bool) -> Result<()> {
    let entries = parse_summary(root)?;
    let grammars = load_grammars(root)?;
    let mut typ = typst_preamble();
    let mut last_part: Option<String> = None;
    for e in &entries {
        if e.part != last_part {
            if let Some(p) = &e.part {
                let _ = write!(
                    typ,
                    "\n#pagebreak(weak: true)\n#partpage[{}]\n",
                    typst_escape(p)
                );
            }
            last_part = e.part.clone();
        }
        let content = std::fs::read_to_string(&e.path)?;
        let _ = write!(typ, "\n#pagebreak(weak: true)\n");
        typ.push_str(&markdown_to_typst(&grammars.wolf, &content)?);
    }

    let dir = root.join("target/render");
    std::fs::create_dir_all(&dir)?;
    let typ_path = dir.join("wolf-book.typ");
    std::fs::write(&typ_path, &typ)?;
    println!("render: typst source at {}", typ_path.display());

    let Some(typst) = find_typst() else {
        let msg = "SKIP: typst not found on PATH (and $TYPST unset) — \
                   wolf-book.typ was generated but wolf-book.pdf was not compiled. \
                   Install typst (cargo install typst-cli) or set TYPST.";
        if require {
            bail!("{msg}");
        }
        println!("render: {msg}");
        return Ok(());
    };
    let pdf_path = dir.join("wolf-book.pdf");
    let status = std::process::Command::new(&typst)
        .arg("compile")
        .arg("--font-path")
        .arg(root.join("print/fonts"))
        .arg(&typ_path)
        .arg(&pdf_path)
        .status()
        .with_context(|| format!("running {}", typst.display()))?;
    if !status.success() {
        bail!("typst compile failed ({status})");
    }
    println!("render: PDF at {}", pdf_path.display());
    Ok(())
}

fn find_typst() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os("TYPST") {
        let p = PathBuf::from(p);
        if p.is_file() {
            return Some(p);
        }
    }
    let exe = if cfg!(windows) { "typst.exe" } else { "typst" };
    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        let candidate = dir.join(exe);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    // cargo install's default home, which may not be on PATH in CI shells.
    let home = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE"))?;
    let candidate = PathBuf::from(home).join(".cargo/bin").join(exe);
    candidate.is_file().then_some(candidate)
}

fn typst_preamble() -> String {
    // The faces and the code palette are the same decisions as the web
    // edition: XCharter (Charter's free extension) for body, the
    // self-hosted Source Code Pro for code, and the classed-span colors
    // mapped from the very same table in highlight.rs.
    String::from(
        r##"// GENERATED by `cargo xtask render pdf` — do not edit.
#set page(
  paper: "us-letter",
  margin: (x: 1.4in, y: 1.2in),
  numbering: "1",
  header: context {
    if counter(page).get().first() > 1 [
      #set text(font: "XCharter", size: 9pt, style: "italic")
      The Wolf Book
      #h(1fr)
    ]
  },
)
#set text(font: "XCharter", size: 10.5pt)
#set par(justify: true, leading: 0.72em)
#show heading: set text(font: "XCharter")
#show heading.where(level: 1): it => { v(1.2em); set text(size: 17pt); it; v(0.4em) }
#show heading.where(level: 2): it => { v(0.8em); set text(size: 13pt); it; v(0.2em) }
#show raw: set text(font: "Source Code Pro", size: 8.8pt)
#let codeblock(body) = block(
  fill: rgb("#faf8f4"),
  inset: 8pt,
  width: 100%,
  breakable: true,
)[#set text(font: "Source Code Pro", size: 8.8pt); #set par(justify: false, leading: 0.55em); #body]
#let epigraph(body) = block(above: 1.2em, below: 1.6em, width: 100%)[#body]
#let partpage(title) = {
  v(2.5in)
  align(center)[#text(size: 20pt, weight: "bold")[#title]]
  pagebreak(weak: true)
}
#align(center)[#v(2in)#text(size: 28pt, weight: "bold")[The Wolf Book]#v(0.5em)]
#align(center)[#text(style: "italic")[every sample executed, every diagnostic real]]
#pagebreak()
"##,
    )
}

fn typst_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' | '#' | '$' | '[' | ']' | '*' | '_' | '`' | '<' | '>' | '@' | '~' | '\'' | '"' => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    out
}

fn typst_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Markdown → typst, sized to the book's own subset (STYLE.md): ATX
/// headings, paragraphs, fenced code, block quotes, lists, HTML
/// comments. Inline: `code`, *emph*, **strong**, [links](…).
fn markdown_to_typst(wolf: &Grammar, content: &str) -> Result<String> {
    let mut out = String::new();
    for seg in segments(content) {
        match seg {
            Segment::Fence(f) => {
                let info = f.info.trim();
                let lang = info.split(',').next().unwrap_or("").trim();
                if matches!(lang, "wolf") {
                    // Directives are build instructions; strip them here too.
                    let _ = parse_fence_info(info); // validated in samples
                    out.push_str(&typst_wolf_block(wolf, &f.content)?);
                } else {
                    let _ = writeln!(
                        out,
                        "#codeblock[#raw(block: true, {})]",
                        typst_str(f.content.trim_end_matches('\n'))
                    );
                }
            }
            Segment::Text(text) => {
                out.push_str(&typst_prose(&strip_html_comments(&text)));
            }
        }
    }
    Ok(out)
}

pub fn strip_html_comments(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find("<!--") {
        out.push_str(&rest[..start]);
        match rest[start..].find("-->") {
            Some(end) => rest = &rest[start + end + 3..],
            None => return out,
        }
    }
    out.push_str(rest);
    out
}

fn typst_prose(text: &str) -> String {
    let mut out = String::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0usize;
    while i < lines.len() {
        let line = lines[i];
        i += 1;
        // Epigraphs are the one raw-HTML construct the chapters use
        // (DESIGN.md §1, TONE.md §4): the web edition styles
        // `<p class="epigraph">` from the theme, and the print path has
        // to build the same shape instead of escaping the tags onto the
        // page. Collect the paragraph, then set it italic with the
        // attribution flush right.
        if line.trim_start().starts_with("<p class=\"epigraph\">") {
            let mut block = line.trim_start().to_string();
            while !block.contains("</p>") && i < lines.len() {
                block.push('\n');
                block.push_str(lines[i]);
                i += 1;
            }
            out.push_str(&typst_epigraph(&block));
            continue;
        }
        let trimmed = line.trim_start();
        // `<details>` wrappers are the Solutions page's collapse (DESIGN.md
        // §3). Print has no collapse: the wrapper disappears and the
        // summary becomes the block's heading.
        if trimmed.starts_with("<details") || trimmed.starts_with("</details>") {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix("<summary>") {
            let head = rest.trim_end_matches("</summary>");
            out.push_str(&format!("==== {}\n", typst_inline(head)));
            continue;
        }
        // A markdown table: the header row, the alignment row, then body
        // rows until the block ends. Typst sets it as a real table so the
        // PDF does not print pipes.
        if trimmed.starts_with('|')
            && lines
                .get(i)
                .map(|l| is_table_rule(l.trim_start()))
                .unwrap_or(false)
        {
            let header = split_row(trimmed);
            let cols = header.len();
            i += 1; // the alignment row
            let mut rows: Vec<Vec<String>> = Vec::new();
            while let Some(next) = lines.get(i) {
                let next = next.trim_start();
                if !next.starts_with('|') {
                    break;
                }
                rows.push(split_row(next));
                i += 1;
            }
            out.push_str(&typst_table(cols, &header, &rows));
            continue;
        }
        let hashes = trimmed.chars().take_while(|&c| c == '#').count();
        if (1..=6).contains(&hashes) && trimmed[hashes..].starts_with(' ') {
            let mut title = trimmed[hashes..].trim().to_string();
            let mut label = String::new();
            if let Some(idx) = title.find("{#") {
                if let Some(close) = title[idx..].find('}') {
                    let id = title[idx + 2..idx + close].to_string();
                    label = format!(" <sec-{}>", id.replace('.', "-"));
                    title.replace_range(idx..idx + close + 1, "");
                    title = title.trim_end().to_string();
                }
            }
            let level = "=".repeat(hashes.min(4));
            out.push_str(&format!("{level} {}{label}\n", typst_inline(&title)));
        } else if let Some(item) = trimmed.strip_prefix("- ") {
            out.push_str(&format!("- {}\n", typst_inline(item)));
        } else if let Some(quote) = trimmed.strip_prefix("> ") {
            out.push_str(&format!("#pad(left: 1em)[_{}_]\n", typst_inline(quote)));
        } else if trimmed == ">" || trimmed == "---" {
            out.push('\n');
        } else {
            out.push_str(&typst_inline(line));
            out.push('\n');
        }
    }
    out
}

/// `|---|:--:|` — the row that makes the row above it a header.
fn is_table_rule(line: &str) -> bool {
    line.starts_with('|')
        && line
            .chars()
            .all(|c| matches!(c, '|' | '-' | ':' | ' ' | '\t'))
        && line.contains('-')
}

fn split_row(line: &str) -> Vec<String> {
    line.trim()
        .trim_start_matches('|')
        .trim_end_matches('|')
        .split('|')
        .map(|c| c.trim().to_string())
        .collect()
}

fn typst_table(cols: usize, header: &[String], rows: &[Vec<String>]) -> String {
    let mut out = format!(
        "#table(\n  columns: {cols},\n  stroke: 0.4pt + rgb(\"#c8c2b6\"),\n  inset: 5pt,\n"
    );
    let cells = |row: &[String], strong: bool| {
        let mut s = String::from("  ");
        for i in 0..cols {
            let cell = row.get(i).map(String::as_str).unwrap_or("");
            let body = typst_inline(cell);
            if strong {
                let _ = write!(s, "[*{body}*], ");
            } else {
                let _ = write!(s, "[{body}], ");
            }
        }
        s.push('\n');
        s
    };
    out.push_str(&cells(header, true));
    for row in rows {
        out.push_str(&cells(row, false));
    }
    out.push_str(")\n");
    out
}

/// One `<p class="epigraph">…</p>` block → typst. An inner
/// `<span class="attribution">…</span>` becomes the flush-right line the
/// web edition's CSS produces; everything else is the italic epigraph
/// body. Whitespace collapses, because the source wraps for the 68ch
/// measure and typst reflows on its own.
fn typst_epigraph(block: &str) -> String {
    let inner = block
        .trim()
        .trim_start_matches("<p class=\"epigraph\">")
        .trim_end_matches("</p>");
    let (body, source) = match inner.split_once("<span class=\"attribution\">") {
        Some((b, s)) => (b, Some(s.trim_end_matches("</span>"))),
        None => (inner, None),
    };
    let collapse = |s: &str| s.split_whitespace().collect::<Vec<_>>().join(" ");
    let body = typst_inline(&collapse(body));
    match source {
        Some(s) => format!(
            "#epigraph[#emph[{body}]#v(0.2em)#align(right)[{}]]\n",
            typst_inline(&collapse(s))
        ),
        None => format!("#epigraph[#emph[{body}]]\n"),
    }
}

/// Inline markdown → typst: code spans pass through as typst raw
/// (same backtick syntax), emphasis converts, everything else escapes.
fn typst_inline(line: &str) -> String {
    let mut out = String::new();
    let mut rest = line;
    while let Some(start) = rest.find('`') {
        let (before, after) = rest.split_at(start);
        out.push_str(&typst_escape_inline(before));
        match after[1..].find('`') {
            Some(end) => {
                out.push_str(&format!("#raw({})", typst_str(&after[1..1 + end])));
                rest = &after[end + 2..];
            }
            None => {
                out.push_str(&typst_escape_inline(after));
                return out;
            }
        }
    }
    out.push_str(&typst_escape_inline(rest));
    out
}

fn typst_escape_inline(s: &str) -> String {
    // Links become their text; emphasis markers convert.
    let mut s = s.to_string();
    // [text](url) → text
    fn link_span(s: &str) -> Option<(usize, usize, usize)> {
        let open = s.find('[')?;
        let mid = s[open..].find("](")?;
        let close = s[open + mid..].find(')')?;
        Some((open, mid, close))
    }
    while let Some((open, mid, close)) = link_span(&s) {
        let text = s[open + 1..open + mid].to_string();
        s.replace_range(open..open + mid + close + 1, &text);
    }
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    let mut strong = false;
    let mut emph = false;
    while let Some(c) = chars.next() {
        match c {
            '*' => {
                if chars.peek() == Some(&'*') {
                    chars.next();
                    out.push_str(if strong { "]" } else { "#strong[" });
                    strong = !strong;
                } else {
                    out.push_str(if emph { "]" } else { "#emph[" });
                    emph = !emph;
                }
            }
            '\\' | '#' | '$' | '[' | ']' | '_' | '`' | '<' | '>' | '@' | '~' => {
                out.push('\\');
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    if emph {
        out.push(']');
    }
    if strong {
        out.push(']');
    }
    out
}

/// A wolf code block, highlighted through the same grammar and the same
/// palette as the web edition, set in typst.
fn typst_wolf_block(wolf: &Grammar, code: &str) -> Result<String> {
    let mut out = String::from("#codeblock[\n");
    let lines: Vec<&str> = code.lines().collect();
    let tokenized = tokenize(wolf, code)?;
    for (line, tokens) in lines.iter().zip(&tokenized) {
        if line.is_empty() {
            out.push_str("#linebreak()\n");
            continue;
        }
        for t in tokens {
            let chunk = &line[t.start..t.end];
            let class = t.scope.as_deref().and_then(class_for_scope);
            let style = class.and_then(style_for);
            match style {
                Some(st) => {
                    let mut args = Vec::new();
                    if let Some(c) = st.color {
                        args.push(format!("fill: rgb({})", typst_str(c)));
                    }
                    if st.bold {
                        args.push("weight: \"bold\"".into());
                    }
                    if st.italic {
                        args.push("style: \"italic\"".into());
                    }
                    let _ = write!(out, "#text({})[#{}]", args.join(", "), typst_str(chunk));
                }
                None => {
                    let _ = write!(out, "#{}", typst_str(chunk));
                }
            }
        }
        out.push_str("#linebreak()\n");
    }
    out.push_str("]\n");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_parses_in_order() {
        let root = crate::repo_root().unwrap();
        let entries = parse_summary(&root).unwrap();
        assert!(
            entries.len() >= 36,
            "front + 26 chapters + back, got {}",
            entries.len()
        );
        assert!(entries[0].path.ends_with("front/how-to-read.md"));
        let ch1 = entries
            .iter()
            .position(|e| e.path.ends_with("ch01.md"))
            .unwrap();
        let ch26 = entries
            .iter()
            .position(|e| e.path.ends_with("ch26.md"))
            .unwrap();
        assert!(ch1 < ch26);
        assert_eq!(entries[ch1].part.as_deref(), Some("Part 1 — Foundations"));
    }

    #[test]
    fn strip_directives_leaves_lang() {
        let md = "```wolf,run(exit=0, stdout=\"hi\")\nfn main() -> !int { 0 }\n```\n";
        let out = strip_directives(md);
        assert!(out.starts_with("```wolf\n"));
        assert!(!out.contains("run(exit"));
    }

    #[test]
    fn typst_escapes_specials() {
        assert_eq!(typst_escape("a#b$c"), "a\\#b\\$c");
        let inline = typst_inline("use `wolf run` on *day one*");
        assert!(inline.contains("#raw(\"wolf run\")"));
        assert!(inline.contains("#emph[day one]"));
    }

    #[test]
    fn typst_epigraph_sets_italic_and_flush_right_attribution() {
        // Chapter 2's epigraph, wrapped for the measure as the source is.
        let md = "<p class=\"epigraph\">\"Skin and Bones.\"<span class=\"attribution\">— Cage the\nElephant</span></p>\n\nprose follows\n";
        let out = typst_prose(md);
        assert!(out.contains("#epigraph[#emph["), "got: {out}");
        assert!(out.contains("#align(right)["), "got: {out}");
        // No raw HTML reaches the page, escaped or otherwise.
        assert!(!out.contains("<p"), "got: {out}");
        assert!(!out.contains("class=\\\"epigraph"), "got: {out}");
        assert!(!out.contains("span"), "got: {out}");
        assert!(out.contains("prose follows"));
    }

    #[test]
    fn typst_epigraph_without_attribution() {
        let md = "<p class=\"epigraph\">Bruckner's Fourth opens with one horn call.</p>\n";
        let out = typst_prose(md);
        assert!(out.starts_with("#epigraph[#emph["), "got: {out}");
        assert!(!out.contains("#align(right)"), "got: {out}");
    }

    #[test]
    fn a_markdown_table_becomes_a_typst_table() {
        let md = "| Kind | Fault |\n|------|-------|\n| `bounds` | outside |\n| `ub` | oracle |\n";
        let out = typst_prose(md);
        assert!(out.contains("#table("), "got: {out}");
        assert!(out.contains("columns: 2"), "got: {out}");
        // The header row is set bold, the body rows plain, and no pipe
        // character survives into the PDF.
        assert!(out.contains("[*Kind*]"), "got: {out}");
        assert!(out.contains("[outside]"), "got: {out}");
        assert!(!out.contains('|'), "pipes reached the print path: {out}");
    }

    #[test]
    fn details_wrappers_expand_for_print() {
        let md = "<details>\n<summary>Exercise 3-2 — [§3.1](../ch03.md#3.1)</summary>\n\nbody text\n</details>\n";
        let out = typst_prose(md);
        assert!(!out.contains("details"), "got: {out}");
        assert!(out.contains("==== Exercise 3-2"), "got: {out}");
        assert!(out.contains("body text"), "got: {out}");
    }

    #[test]
    fn typst_wolf_block_colors_keywords() {
        let root = crate::repo_root().unwrap();
        let grammars = crate::preprocess::load_grammars(&root).unwrap();
        let t = typst_wolf_block(&grammars.wolf, "fn main() -> !int { 0 }\n").unwrap();
        assert!(t.contains("weight: \"bold\""));
        assert!(t.contains("#\"fn\"") || t.contains("[#\"fn\"]"));
    }
}
