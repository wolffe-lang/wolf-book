//! A minimal TextMate-grammar interpreter, sized to the vendored wolf
//! grammars (DESIGN.md §2: the book consumes wolf-lsp's generated
//! tmLanguage directly — one grammar, one lineage). Supported subset:
//! `match` rules, `begin`/`end` spans with captures and end-pattern
//! backreferences, nested `patterns`, and `#repository` includes. That
//! is the entire vocabulary the generated grammars use; anything else
//! is a hard error at load time, so a grammar bump that outgrows the
//! interpreter fails loudly instead of highlighting wrong.

use anyhow::{bail, Context, Result};
use fancy_regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct RawGrammar {
    #[serde(rename = "scopeName", default)]
    scope_name: String,
    #[serde(default)]
    patterns: Vec<RawPattern>,
    #[serde(default)]
    repository: HashMap<String, RawPattern>,
}

#[derive(Debug, Clone, Deserialize)]
struct RawPattern {
    #[serde(default)]
    include: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(rename = "match", default)]
    match_: Option<String>,
    #[serde(default)]
    begin: Option<String>,
    #[serde(default)]
    end: Option<String>,
    #[serde(default)]
    captures: Option<HashMap<String, RawCapture>>,
    #[serde(rename = "beginCaptures", default)]
    begin_captures: Option<HashMap<String, RawCapture>>,
    #[serde(rename = "endCaptures", default)]
    end_captures: Option<HashMap<String, RawCapture>>,
    #[serde(default)]
    patterns: Option<Vec<RawPattern>>,
}

#[derive(Debug, Clone, Deserialize)]
struct RawCapture {
    #[serde(default)]
    name: Option<String>,
}

/// A resolved rule: includes expanded, captures normalized.
#[derive(Debug, Clone)]
pub enum Rule {
    Match {
        re: String,
        name: Option<String>,
        captures: Vec<(usize, String)>,
    },
    Span {
        begin: String,
        end: String,
        name: Option<String>,
        begin_captures: Vec<(usize, String)>,
        end_captures: Vec<(usize, String)>,
        patterns: Vec<Rule>,
    },
}

pub struct Grammar {
    rules: Vec<Rule>,
    regex_cache: std::cell::RefCell<HashMap<String, Regex>>,
}

fn captures_list(caps: &Option<HashMap<String, RawCapture>>) -> Result<Vec<(usize, String)>> {
    let mut out = Vec::new();
    if let Some(map) = caps {
        for (k, v) in map {
            let idx: usize = k
                .parse()
                .with_context(|| format!("capture key `{k}` is not an index"))?;
            if let Some(name) = &v.name {
                out.push((idx, name.clone()));
            }
        }
    }
    out.sort();
    Ok(out)
}

fn resolve(
    p: &RawPattern,
    repo: &HashMap<String, RawPattern>,
    externals: &HashMap<String, RawGrammar>,
    stack: &mut Vec<String>,
    out: &mut Vec<Rule>,
) -> Result<()> {
    if let Some(inc) = &p.include {
        let Some(key) = inc.strip_prefix('#') else {
            // External-scope include (`source.wolf`): splice the whole
            // referenced grammar, resolved against its own repository.
            if stack.iter().any(|s| s == inc) {
                bail!("include cycle through `{inc}`");
            }
            let ext = externals
                .get(inc)
                .with_context(|| format!("include `{inc}` is not a vendored grammar scope"))?;
            stack.push(inc.clone());
            for sub in &ext.patterns {
                resolve(sub, &ext.repository, externals, stack, out)?;
            }
            stack.pop();
            return Ok(());
        };
        if stack.iter().any(|s| s == key) {
            bail!("include cycle through `#{key}` — the interpreter does not support recursion");
        }
        let target = repo
            .get(key)
            .with_context(|| format!("include `#{key}` not in repository"))?;
        stack.push(key.to_string());
        resolve(target, repo, externals, stack, out)?;
        stack.pop();
        return Ok(());
    }
    if let Some(re) = &p.match_ {
        out.push(Rule::Match {
            re: re.clone(),
            name: p.name.clone(),
            captures: captures_list(&p.captures)?,
        });
        return Ok(());
    }
    if let (Some(begin), Some(end)) = (&p.begin, &p.end) {
        let mut inner = Vec::new();
        for sub in p.patterns.as_deref().unwrap_or(&[]) {
            resolve(sub, repo, externals, stack, &mut inner)?;
        }
        out.push(Rule::Span {
            begin: begin.clone(),
            end: end.clone(),
            name: p.name.clone(),
            begin_captures: captures_list(&p.begin_captures)?,
            end_captures: captures_list(&p.end_captures)?,
            patterns: inner,
        });
        return Ok(());
    }
    if let Some(subs) = &p.patterns {
        for sub in subs {
            resolve(sub, repo, externals, stack, out)?;
        }
        return Ok(());
    }
    bail!("pattern with neither include, match, begin/end, nor patterns: {p:?}");
}

impl Grammar {
    pub fn load(json: &str) -> Result<Grammar> {
        Self::load_with(json, &[])
    }

    /// Load a grammar that may `include` other vendored grammars by
    /// scope name (`.wolfi` and `wolf.pkg` are lexically wolf: their
    /// grammars are one include of `source.wolf`).
    pub fn load_with(json: &str, externals_json: &[&str]) -> Result<Grammar> {
        let raw: RawGrammar = serde_json::from_str(json).context("parsing tmLanguage JSON")?;
        let mut externals = HashMap::new();
        for e in externals_json {
            let g: RawGrammar = serde_json::from_str(e).context("parsing external grammar")?;
            externals.insert(g.scope_name.clone(), g);
        }
        let mut rules = Vec::new();
        for p in &raw.patterns {
            resolve(p, &raw.repository, &externals, &mut Vec::new(), &mut rules)?;
        }
        Ok(Grammar {
            rules,
            regex_cache: Default::default(),
        })
    }

    fn find(&self, pattern: &str, hay: &str, from: usize) -> Result<Option<(usize, usize)>> {
        let mut cache = self.regex_cache.borrow_mut();
        let re = match cache.get(pattern) {
            Some(re) => re,
            None => {
                let compiled = Regex::new(pattern)
                    .with_context(|| format!("compiling grammar regex `{pattern}`"))?;
                cache.entry(pattern.to_string()).or_insert(compiled)
            }
        };
        Ok(re
            .find_from_pos(hay, from)
            .with_context(|| format!("running grammar regex `{pattern}`"))?
            .map(|m| (m.start(), m.end())))
    }

    fn find_captures(&self, pattern: &str, hay: &str, from: usize) -> Result<Option<GroupSpans>> {
        let mut cache = self.regex_cache.borrow_mut();
        let re = match cache.get(pattern) {
            Some(re) => re,
            None => {
                let compiled = Regex::new(pattern)
                    .with_context(|| format!("compiling grammar regex `{pattern}`"))?;
                cache.entry(pattern.to_string()).or_insert(compiled)
            }
        };
        let caps = re
            .captures_from_pos(hay, from)
            .with_context(|| format!("running grammar regex `{pattern}`"))?;
        Ok(caps.map(|c| {
            (0..c.len())
                .map(|i| c.get(i).map(|g| (g.start(), g.end())))
                .collect()
        }))
    }
}

/// Capture-group extents for one regex match (index 0 = whole match).
pub type GroupSpans = Vec<Option<(usize, usize)>>;

/// One highlighted piece of a line: byte range plus its innermost scope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub scope: Option<String>,
}

struct ActiveSpan<'g> {
    end_src: String,
    name: Option<String>,
    end_captures: &'g [(usize, String)],
    rules: &'g [Rule],
}

/// Tokenize `text` line by line; spans persist across lines (block
/// strings). Returns one token list per line, covering every byte.
pub fn tokenize(grammar: &Grammar, text: &str) -> Result<Vec<Vec<Token>>> {
    let mut out = Vec::new();
    let mut stack: Vec<ActiveSpan> = Vec::new();
    for line in text.lines() {
        out.push(tokenize_line(grammar, line, &mut stack)?);
    }
    Ok(out)
}

fn substitute_backrefs(end: &str, hay: &str, groups: &[Option<(usize, usize)>]) -> String {
    let mut out = String::new();
    let mut chars = end.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(d) = chars.peek().and_then(|c| c.to_digit(10)) {
                chars.next();
                let text = groups
                    .get(d as usize)
                    .and_then(|g| *g)
                    .map(|(s, e)| &hay[s..e])
                    .unwrap_or("");
                out.push_str(&fancy_regex::escape(text));
                continue;
            }
            out.push(c);
            if let Some(n) = chars.next() {
                out.push(n);
            }
            continue;
        }
        out.push(c);
    }
    out
}

fn emit_with_captures(
    tokens: &mut Vec<Token>,
    groups: &[Option<(usize, usize)>],
    captures: &[(usize, String)],
    default_scope: Option<&str>,
) {
    let (start, end) = groups[0].expect("group 0 always present");
    if start == end {
        return;
    }
    // Whole-match scope: capture 0 overrides the default.
    let whole: Option<String> = captures
        .iter()
        .find(|(i, _)| *i == 0)
        .map(|(_, n)| n.clone())
        .or_else(|| default_scope.map(str::to_string));
    // Sub-captures split the range.
    let mut cuts: Vec<(usize, usize, String)> = captures
        .iter()
        .filter(|(i, _)| *i > 0)
        .filter_map(|(i, n)| {
            groups
                .get(*i)
                .and_then(|g| *g)
                .map(|(s, e)| (s, e, n.clone()))
        })
        .filter(|(s, e, _)| e > s)
        .collect();
    cuts.sort();
    let mut pos = start;
    for (s, e, name) in cuts {
        if s > pos {
            tokens.push(Token {
                start: pos,
                end: s,
                scope: whole.clone(),
            });
        }
        tokens.push(Token {
            start: s,
            end: e,
            scope: Some(name),
        });
        pos = pos.max(e);
    }
    if pos < end {
        tokens.push(Token {
            start: pos,
            end,
            scope: whole,
        });
    }
}

fn tokenize_line<'g>(
    grammar: &'g Grammar,
    line: &str,
    stack: &mut Vec<ActiveSpan<'g>>,
) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut pos = 0usize;
    loop {
        let (rules, span_name): (&[Rule], Option<String>) = match stack.last() {
            Some(top) => (top.rules, top.name.clone()),
            None => (&grammar.rules, None),
        };

        // Earliest end-of-span match, if any.
        let end_match = match stack.last() {
            Some(top) => grammar.find(&top.end_src, line, pos)?,
            None => None,
        };

        // Earliest rule match.
        let mut best: Option<(usize, usize, &'g Rule)> = None;
        for rule in rules {
            let re = match rule {
                Rule::Match { re, .. } => re,
                Rule::Span { begin, .. } => begin,
            };
            if let Some((s, e)) = grammar.find(re, line, pos)? {
                let better = match best {
                    None => true,
                    Some((bs, _, _)) => s < bs,
                };
                if better {
                    best = Some((s, e, rule));
                }
                if s == pos {
                    break; // cannot do better than matching at pos, first in order
                }
            }
        }

        // The end pattern wins ties (TextMate default; no
        // applyEndPatternLast in the vendored grammars).
        let take_end = match (end_match, &best) {
            (Some((es, _)), Some((bs, _, _))) => es <= *bs,
            (Some(_), None) => true,
            (None, _) => false,
        };

        if take_end {
            let top = stack.last().unwrap();
            let groups = grammar
                .find_captures(&top.end_src, line, pos)?
                .expect("end matched above");
            let (gap_end, _) = groups[0].unwrap();
            if gap_end > pos {
                tokens.push(Token {
                    start: pos,
                    end: gap_end,
                    scope: span_name.clone(),
                });
            }
            emit_with_captures(&mut tokens, &groups, top.end_captures, span_name.as_deref());
            let (_, match_end) = groups[0].unwrap();
            stack.pop();
            if match_end == pos && groups[0].unwrap().0 == pos {
                // zero-width end: popping is the progress
            }
            pos = match_end.max(pos);
            if pos >= line.len() {
                break;
            }
            continue;
        }

        match best {
            None => {
                if line.len() > pos {
                    tokens.push(Token {
                        start: pos,
                        end: line.len(),
                        scope: span_name,
                    });
                }
                break;
            }
            Some((s, _e, rule)) => {
                if s > pos {
                    tokens.push(Token {
                        start: pos,
                        end: s,
                        scope: span_name.clone(),
                    });
                }
                match rule {
                    Rule::Match { re, name, captures } => {
                        let groups = grammar
                            .find_captures(re, line, s)?
                            .expect("match found above");
                        let (ms, me) = groups[0].unwrap();
                        emit_with_captures(
                            &mut tokens,
                            &groups,
                            captures,
                            name.as_deref().or(span_name.as_deref()),
                        );
                        pos = if me > ms { me } else { me + 1 }; // never stall
                    }
                    Rule::Span {
                        begin,
                        end,
                        name,
                        begin_captures,
                        end_captures,
                        patterns,
                    } => {
                        let groups = grammar
                            .find_captures(begin, line, s)?
                            .expect("begin found above");
                        let scope = name.as_deref().or(span_name.as_deref());
                        emit_with_captures(&mut tokens, &groups, begin_captures, scope);
                        let end_src = substitute_backrefs(end, line, &groups);
                        stack.push(ActiveSpan {
                            end_src,
                            name: name.clone().or(span_name.clone()),
                            end_captures,
                            rules: patterns,
                        });
                        let (_, me) = groups[0].unwrap();
                        pos = me;
                    }
                }
                if pos >= line.len() {
                    break;
                }
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wolf_grammar() -> Grammar {
        // The vendored grammar, loaded from the repo copy.
        let root = crate::repo_root().unwrap();
        let json = std::fs::read_to_string(root.join("highlight/wolf.tmLanguage.json")).unwrap();
        Grammar::load(&json).unwrap()
    }

    fn scopes(tokens: &[Token], line: &str) -> Vec<(String, Option<String>)> {
        tokens
            .iter()
            .map(|t| (line[t.start..t.end].to_string(), t.scope.clone()))
            .collect()
    }

    #[test]
    fn keywords_and_numbers() {
        let g = wolf_grammar();
        let line = "let x = 42";
        let toks = &tokenize(&g, line).unwrap()[0];
        let got = scopes(toks, line);
        assert!(got.contains(&("let".into(), Some("storage.type.wolf".into()))));
        assert!(got.contains(&("42".into(), Some("constant.numeric.decimal.wolf".into()))));
    }

    #[test]
    fn string_with_interpolation() {
        let g = wolf_grammar();
        let line = r#"print("hello, {name}")"#;
        let toks = &tokenize(&g, line).unwrap()[0];
        let got = scopes(toks, line);
        assert!(got
            .iter()
            .any(|(t, s)| t == "hello, " && s.as_deref() == Some("string.quoted.double.wolf")));
        assert!(got.iter().any(|(t, s)| t == "{"
            && s.as_deref() == Some("punctuation.section.interpolation.begin.wolf")));
    }

    #[test]
    fn comment_swallows_line() {
        let g = wolf_grammar();
        let line = "// let this be prose";
        let toks = &tokenize(&g, line).unwrap()[0];
        assert_eq!(toks.len(), 1);
        assert_eq!(
            toks[0].scope.as_deref(),
            Some("comment.line.double-slash.wolf")
        );
    }

    #[test]
    fn block_string_spans_lines() {
        let g = wolf_grammar();
        let text = "let t = \"\"\"\n  the wolf runs\n  \"\"\"";
        let lines = tokenize(&g, text).unwrap();
        let mid = &lines[1];
        assert!(mid
            .iter()
            .all(|t| t.scope.as_deref() == Some("string.quoted.triple.wolf")));
    }

    #[test]
    fn raw_string_backref_end() {
        let g = wolf_grammar();
        let line = r##"let s = r#"no "escape" here"# + 1"##;
        let toks = &tokenize(&g, line).unwrap()[0];
        let got = scopes(toks, line);
        // The + after the raw string must be back outside the string.
        assert!(got
            .iter()
            .any(|(t, s)| t == "+" && s.as_deref() == Some("keyword.operator.wolf")));
    }

    #[test]
    fn tokens_cover_every_byte() {
        let g = wolf_grammar();
        let line = r#"fn main() -> !int { print("x={1 + 2:>4}") }"#;
        let toks = &tokenize(&g, line).unwrap()[0];
        let mut pos = 0;
        for t in toks {
            assert!(t.start >= pos, "overlap at {}", t.start);
            pos = t.end;
        }
        assert_eq!(pos, line.len());
    }
}
