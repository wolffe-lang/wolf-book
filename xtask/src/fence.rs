//! Fenced-block walking over book markdown. Line-based on purpose: the
//! book's own format is under our control (STYLE.md fixes it), and a
//! line walker preserves byte-exact block content for extraction and
//! for diagnostic cross-checks.

/// A fenced code block found in a markdown file.
#[derive(Debug, Clone)]
pub struct Fence {
    /// Zero-based line index of the opening fence.
    pub open_line: usize,
    /// Number of backticks in the opening fence.
    pub ticks: usize,
    /// The info string, verbatim (may be empty).
    pub info: String,
    /// Block content, exactly as written (no trailing fence line).
    pub content: String,
}

/// Split a markdown document into fences and the text around them.
#[derive(Debug)]
pub enum Segment {
    Text(String),
    Fence(Fence),
}

/// Walk `source`, yielding text segments and fences. Only backtick
/// fences are recognized (STYLE.md: the book uses no tilde fences).
/// Longer opening fences require equally long closers, so nested
/// examples survive.
pub fn segments(source: &str) -> Vec<Segment> {
    let mut out = Vec::new();
    let mut text = String::new();
    let mut lines = source.lines().enumerate().peekable();
    while let Some((idx, line)) = lines.next() {
        let trimmed = line.trim_start();
        let indent = line.len() - trimmed.len();
        let ticks = trimmed.chars().take_while(|&c| c == '`').count();
        if ticks >= 3 && indent < 4 {
            let fence_marker: String = "`".repeat(ticks);
            let info = trimmed[ticks..].trim().to_string();
            let mut content = String::new();
            let mut closed = false;
            for (_, l) in lines.by_ref() {
                let lt = l.trim_start();
                if lt.starts_with(&fence_marker)
                    && lt.chars().take_while(|&c| c == '`').count() >= ticks
                    && lt.trim_end().chars().all(|c| c == '`')
                {
                    closed = true;
                    break;
                }
                content.push_str(l);
                content.push('\n');
            }
            let _ = closed; // an unterminated fence swallows to EOF, as in CommonMark
            if !text.is_empty() {
                out.push(Segment::Text(std::mem::take(&mut text)));
            }
            out.push(Segment::Fence(Fence {
                open_line: idx,
                ticks,
                info,
                content,
            }));
        } else {
            text.push_str(line);
            text.push('\n');
        }
    }
    if !text.is_empty() {
        out.push(Segment::Text(text));
    }
    out
}

/// Reassemble segments into markdown, mapping each fence through `f`,
/// which returns the full replacement text for the block (including
/// any fence markers it wants to keep).
pub fn rewrite(source: &str, mut f: impl FnMut(&Fence) -> String) -> String {
    let mut out = String::new();
    for seg in segments(source) {
        match seg {
            Segment::Text(t) => out.push_str(&t),
            Segment::Fence(fence) => out.push_str(&f(&fence)),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walks_fences_and_text() {
        let md = "para\n\n```wolf,run(exit=0)\nfn main() -> !int { 0 }\n```\n\ntail\n";
        let segs = segments(md);
        assert_eq!(segs.len(), 3);
        match &segs[1] {
            Segment::Fence(fence) => {
                assert_eq!(fence.info, "wolf,run(exit=0)");
                assert_eq!(fence.content, "fn main() -> !int { 0 }\n");
            }
            other => panic!("expected fence, got {other:?}"),
        }
    }

    #[test]
    fn preserves_content_bytes() {
        let md = "```diagnostic\nerror[E1001]: `p.lead` moved\n   |\n```\n";
        let segs = segments(md);
        match &segs[0] {
            Segment::Fence(fence) => {
                assert_eq!(fence.content, "error[E1001]: `p.lead` moved\n   |\n");
            }
            other => panic!("expected fence, got {other:?}"),
        }
    }

    #[test]
    fn rewrite_roundtrips_text() {
        let md = "a\n\n```wolf\nx\n```\nb\n";
        let out = rewrite(md, |fence| {
            format!("```{}\n{}```\n", fence.info, fence.content)
        });
        assert_eq!(out, md);
    }
}
