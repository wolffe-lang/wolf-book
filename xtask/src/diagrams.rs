//! `cargo xtask diagrams` — memory diagrams generated from the
//! interpreter's place trace (bs62).
//!
//! Chapter 7 drew its ownership trees by hand, and a hand-drawn tree is a
//! claim nothing checks. A ```` ```memory,from(book/ch07/s3),line(5) ````
//! fence names a program on the page (by the samples runner's id) and a
//! line of it. This module runs that program under the trace-capable
//! `lupin` (`LUPIN_TRACE`), which reports the state of every place the
//! program has touched after each statement, from the same move-state
//! machinery that produces `trap(use-after-move)`. From the record before
//! the line and the record after it, it writes:
//!
//! - the fence's body: a text rendering of both trees, which is what the
//!   markdown source and the one-file edition show, and the web edition's
//!   alt text;
//! - `book/diagrams/<page>/<id>-L<line>.svg`: the same trees as a static
//!   SVG, which the web edition shows as an `<img>` and the PDF sets with
//!   typst's `image`. No script draws anything: lupp.us serves the book
//!   under `script-src 'self'`.
//!
//! The trace itself is kept under `snapshots/traces/`, so the text and
//! the SVG can be regenerated, and checked, on a host with no trace
//! lupin. `--check` regenerates all three and fails on any difference;
//! without `LUPIN_TRACE` it says so loudly and checks the two it can.

use crate::directives::parse_fence_info;
use crate::fence::{segments, Segment};
use anyhow::{bail, Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The fence language this module owns.
pub const LANG: &str = "memory";

/// One `memory` fence on a page.
#[derive(Debug, Clone)]
pub struct MemFence {
    pub md: PathBuf,
    /// Zero-based line of the opening fence.
    pub open_line: usize,
    /// The program's sample id, `book/ch07/s3` or `book/ch07/part-packcopy`.
    pub from: String,
    /// One-based line of the program the diagram is about.
    pub line: u32,
    /// The body as written.
    pub body: String,
}

impl MemFence {
    /// `ch07`, the page directory the diagram lives under.
    pub fn page(&self) -> Result<&str> {
        id_parts(&self.from).map(|(p, _)| p)
    }
    /// `s3-L5` — the diagram's file stem.
    pub fn stem(&self) -> Result<String> {
        let (_, name) = id_parts(&self.from)?;
        Ok(format!("{name}-L{}", self.line))
    }
    /// Where the SVG lives, relative to `book/`.
    pub fn svg_rel(&self) -> Result<String> {
        Ok(format!("diagrams/{}/{}.svg", self.page()?, self.stem()?))
    }
}

/// `book/ch07/s3` → (`ch07`, `s3`). Only book pages carry diagrams.
pub fn id_parts(id: &str) -> Result<(&str, &str)> {
    let rest = id
        .strip_prefix("book/")
        .with_context(|| format!("memory fence: `from({id})` must name a book sample"))?;
    let (page, name) = rest
        .rsplit_once('/')
        .with_context(|| format!("memory fence: `from({id})` has no sample name"))?;
    if page.is_empty() || name.is_empty() || page.contains('/') {
        bail!("memory fence: `from({id})` is not `book/<page>/<sample>`");
    }
    Ok((page, name))
}

/// The SVG path a `memory` fence's info names, relative to `book/`, for
/// the two renders. `None` when the info is not a memory fence.
pub fn svg_for_info(info: &str) -> Option<String> {
    let fi = parse_fence_info(info).ok()?;
    if fi.lang != LANG {
        return None;
    }
    let f = MemFence {
        md: PathBuf::new(),
        open_line: 0,
        from: fi.from?,
        line: fi.line?,
        body: String::new(),
    };
    f.svg_rel().ok()
}

// ------------------------------------------------------------ the pages

/// The programs one page defines, by sample name, under exactly the
/// samples runner's rules (`samples::collect_book`): an un-named `wolf`
/// fence is `sN` in page order, a `part(name)` fence accumulates and is
/// a program once its slice carries a check.
pub fn page_programs(source: &str) -> Result<BTreeMap<String, String>> {
    let mut out = BTreeMap::new();
    let mut parts: BTreeMap<String, String> = BTreeMap::new();
    let mut counter = 0usize;
    for seg in segments(source) {
        let Segment::Fence(f) = seg else { continue };
        if f.info.trim().is_empty() {
            continue;
        }
        let fi = parse_fence_info(&f.info)?;
        if fi.file.is_some() || fi.lang != "wolf" {
            continue;
        }
        match &fi.part {
            Some((name, _cont)) => {
                let acc = parts.entry(name.clone()).or_default();
                acc.push_str(&f.content);
                if fi.check.is_some() {
                    out.insert(format!("part-{name}"), acc.clone());
                }
            }
            None => {
                counter += 1;
                out.insert(format!("s{counter}"), f.content.clone());
            }
        }
    }
    Ok(out)
}

/// Every book page (the samples runner's walk: `book/**/*.md` but
/// `SUMMARY.md`, `solutions.md` and `_`-prefixed scaffolding), its
/// memory fences, and the programs they name.
pub fn collect(root: &Path) -> Result<(Vec<MemFence>, BTreeMap<String, String>)> {
    let base = root.join("book");
    let mut pages = Vec::new();
    walk_md(&base, &mut pages)?;
    pages.sort();
    let mut fences = Vec::new();
    let mut programs = BTreeMap::new();
    for md in pages {
        let source = std::fs::read_to_string(&md)?;
        let rel = md.strip_prefix(&base).unwrap().with_extension("");
        let stem = rel.to_string_lossy().replace(['\\', '/'], "-");
        let mut here = Vec::new();
        for seg in segments(&source) {
            let Segment::Fence(f) = seg else { continue };
            let fi = match parse_fence_info(&f.info) {
                Ok(fi) if fi.lang == LANG => fi,
                _ => continue,
            };
            let (Some(from), Some(line)) = (fi.from, fi.line) else {
                bail!(
                    "{}:{}: a memory fence needs from(<sample id>) and line(N)",
                    md.display(),
                    f.open_line + 1
                );
            };
            here.push(MemFence {
                md: md.clone(),
                open_line: f.open_line,
                from,
                line,
                body: f.content.clone(),
            });
        }
        if here.is_empty() {
            continue;
        }
        for (name, program) in page_programs(&source)? {
            programs.insert(format!("book/{stem}/{name}"), program);
        }
        fences.extend(here);
    }
    Ok((fences, programs))
}

fn walk_md(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let p = entry?.path();
        if p.is_dir() {
            walk_md(&p, out)?;
            continue;
        }
        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if p.extension().and_then(|e| e.to_str()) == Some("md")
            && name != "SUMMARY.md"
            && name != "solutions.md"
            && !name.starts_with('_')
        {
            out.push(p);
        }
    }
    Ok(())
}

// ------------------------------------------------------------ the trace

/// What a place is, at one instant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    /// Holds a value: the printable form for a scalar or `str`, the type
    /// name for an aggregate.
    Live,
    /// Moved out, by `take`, `move` or a plain move, at `line:col`.
    Moved,
    /// Declared and never given a value.
    Uninit,
    /// Moved out earlier and assigned again, at `line:col`.
    Reinit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Place {
    pub path: String,
    pub state: State,
    /// The printable value (a scalar, a `str` in quotes).
    pub value: Option<String>,
    /// The type name, for an aggregate.
    pub ty: Option<String>,
    /// For a moved place: `take`, `move` or `plain`.
    pub by: Option<String>,
    /// For a moved or re-initialized place: `line:col` of the site.
    pub at: Option<String>,
    /// For a `Copy` read or a `copy`: the place this value was copied
    /// from (the record's `copy` event, `to` this place).
    pub copied_from: Option<String>,
    /// For an uninitialized part: the enclosing place that moved.
    pub of: Option<String>,
}

/// One record: the places of one function's frame after one statement.
#[derive(Debug, Clone)]
pub struct Record {
    pub line: u32,
    pub func: String,
    /// The activation's depth (1 for `main`).
    pub depth: u32,
    pub places: Vec<Place>,
}

fn str_field<'a>(v: &'a serde_json::Value, keys: &[&str]) -> Option<&'a str> {
    keys.iter().find_map(|k| v.get(*k).and_then(|x| x.as_str()))
}

/// A value as the page prints it: a `str` in double quotes and a `char`
/// in single ones, as the source spells them; anything else as given.
fn shown_value(raw: &serde_json::Value, ty: Option<&str>) -> Option<String> {
    let text = match raw {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Null => return None,
        other => other.to_string(),
    };
    Some(match ty {
        Some("str") => format!("{:?}", text),
        Some("char") => format!("'{text}'"),
        _ => text,
    })
}

/// One place of is72's schema, version 1:
/// `{"path":"p.lead","state":"live","type":"str","value":"ada"}`, with
/// `"reinit":"11:2"` on a live place that was moved out and assigned
/// again; `"state":"moved"` with `"by"` and `"at"`; `"state":"uninit"`
/// with `"of"` for a part whose enclosing place moved.
fn parse_place(v: &serde_json::Value) -> Result<Place> {
    let path = str_field(v, &["path"])
        .context("a place without a path")?
        .to_string();
    let raw = str_field(v, &["state"]).context("a place without a state")?;
    let ty = str_field(v, &["type"]).map(str::to_string);
    let reinit = str_field(v, &["reinit"]).map(str::to_string);
    let (state, at) = match raw {
        "live" if reinit.is_some() => (State::Reinit, reinit),
        "live" => (State::Live, None),
        "moved" => (State::Moved, str_field(v, &["at"]).map(str::to_string)),
        "uninit" => (State::Uninit, None),
        other => bail!("place `{path}`: unknown state `{other}`"),
    };
    Ok(Place {
        value: v.get("value").and_then(|x| shown_value(x, ty.as_deref())),
        ty,
        by: str_field(v, &["by"]).map(str::to_string),
        at,
        copied_from: None,
        of: str_field(v, &["of"]).map(str::to_string),
        path,
        state,
    })
}

/// `"10:2"` → 10.
fn line_of(at: &str) -> Option<u32> {
    at.split(':').next()?.parse().ok()
}

/// Parse a trace (is72's schema, version 1): one JSON object per line,
/// `{"trace":1,"task":0,"fn":"main","depth":1,"at":"10:2","places":[…],
/// "events":[…]}`, written after every statement of every activation.
/// A `copy` event with a `to` marks that binding as a copy of its
/// source, which is the one thing the places alone do not say.
pub fn parse_trace(text: &str) -> Result<Vec<Record>> {
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let v: serde_json::Value =
            serde_json::from_str(line).with_context(|| format!("trace line {}", i + 1))?;
        let Some(places) = v.get("places").and_then(|p| p.as_array()) else {
            continue;
        };
        match v.get("trace").and_then(|t| t.as_u64()) {
            Some(1) => {}
            other => bail!("trace line {}: schema version {other:?}, not 1", i + 1),
        }
        let at_line = str_field(&v, &["at"])
            .and_then(line_of)
            .with_context(|| format!("trace line {}: a record without `at`", i + 1))?;
        let func = str_field(&v, &["fn"]).unwrap_or("main").to_string();
        let depth = v.get("depth").and_then(|d| d.as_u64()).unwrap_or(1) as u32;
        let mut places = places
            .iter()
            .map(parse_place)
            .collect::<Result<Vec<_>>>()
            .with_context(|| format!("trace line {}", i + 1))?;
        for ev in v
            .get("events")
            .and_then(|e| e.as_array())
            .map(Vec::as_slice)
            .unwrap_or_default()
        {
            if str_field(ev, &["ev"]) != Some("copy") {
                continue;
            }
            let (Some(src), Some(to)) = (str_field(ev, &["path"]), str_field(ev, &["to"])) else {
                continue;
            };
            if let Some(p) = places.iter_mut().find(|p| p.path == to) {
                p.copied_from = Some(src.to_string());
            }
        }
        out.push(Record {
            line: at_line,
            func,
            depth,
            places,
        });
    }
    Ok(out)
}

/// The places before and after `line` in `func`'s outermost activation:
/// the record the line produced, and the record of the same activation
/// before it (none, for the function's first statement).
pub fn before_after<'a>(
    records: &'a [Record],
    func: &str,
    line: u32,
) -> Result<(&'a [Place], &'a [Place])> {
    let top = records
        .iter()
        .filter(|r| r.func == func)
        .map(|r| r.depth)
        .min()
        .unwrap_or(1);
    let mine: Vec<&Record> = records
        .iter()
        .filter(|r| r.func == func && r.depth == top)
        .collect();
    let Some(at) = mine.iter().position(|r| r.line == line) else {
        let lines: BTreeSet<u32> = mine.iter().map(|r| r.line).collect();
        bail!("the trace has no record for line {line} of `{func}` (it has {lines:?})");
    };
    let before: &[Place] = if at == 0 { &[] } else { &mine[at - 1].places };
    Ok((before, &mine[at].places))
}

// ------------------------------------------------------------- the tree

/// How a row differs from the same path before the line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Change {
    Same,
    Changed,
    New,
}

/// One row of a drawn tree.
#[derive(Debug, Clone)]
pub struct Row {
    pub depth: usize,
    /// The last path segment (`lead`), or the binding (`p`).
    pub name: String,
    /// What the row says about the place.
    pub label: String,
    pub state: State,
    /// An aggregate's row: it has rows under it.
    pub aggregate: bool,
    pub change: Change,
    /// Is this the last child of its parent (for the text glyphs)?
    pub last: bool,
    /// For each ancestor depth, does a later sibling still follow there?
    pub rails: Vec<bool>,
    /// The parent row changed the same way (a new binding's fields are
    /// new because the binding is), so the text marks only the parent.
    pub inherited: bool,
}

/// Split a place path into segments: `d.meta.author` → [d, meta,
/// author]; `xs[0].name` → [xs, [0], name].
pub fn split_path(path: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for c in path.chars() {
        match c {
            '.' => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            '[' => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
                cur.push(c);
            }
            ']' => {
                cur.push(c);
                out.push(std::mem::take(&mut cur));
            }
            _ => cur.push(c),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn by_word(by: Option<&str>) -> String {
    match by {
        None | Some("plain") => "plain =".to_string(),
        Some(word) => word.to_string(),
    }
}

/// What a row says: the value, the type, or the move.
pub fn label(p: &Place) -> String {
    let held = p.value.clone().or_else(|| p.ty.clone()).unwrap_or_default();
    match p.state {
        State::Live => match &p.copied_from {
            Some(src) => format!("{held} · copy of {src}"),
            None => held,
        },
        State::Moved => match &p.at {
            Some(at) => format!("moved · {} {at}", by_word(p.by.as_deref())),
            None => format!("moved · {}", by_word(p.by.as_deref())),
        },
        State::Uninit => match &p.of {
            Some(of) => format!("gone with {of}"),
            None => "uninitialized".to_string(),
        },
        State::Reinit => match &p.at {
            Some(at) => format!("{held} · re-initialized {at}"),
            None => format!("{held} · re-initialized"),
        },
    }
}

/// The comparable content of a place: what would make a reader see a
/// difference.
type Shown<'a> = (
    State,
    Option<&'a str>,
    Option<&'a str>,
    Option<&'a str>,
    Option<&'a str>,
);

fn shown(p: &Place) -> Shown<'_> {
    (
        p.state.clone(),
        p.value.as_deref(),
        p.by.as_deref(),
        p.at.as_deref(),
        p.copied_from.as_deref(),
    )
}

/// Lay a snapshot out as rows, in `order` (each path's first appearance
/// across the whole trace, so a binding keeps its position in both
/// trees), marking each row against `prior` when given.
pub fn rows(
    places: &[Place],
    order: &BTreeMap<String, usize>,
    prior: Option<&[Place]>,
) -> Vec<Row> {
    // A tree of segments. A path the trace names is a node with a place;
    // a prefix it does not name (rare: an aggregate it summarized) is a
    // node without one.
    #[derive(Default)]
    struct Node {
        place: Option<Place>,
        kids: Vec<(String, Node)>,
        key: usize,
    }
    fn insert(node: &mut Node, segs: &[String], place: &Place, key: usize) {
        if segs.is_empty() {
            node.place = Some(place.clone());
            node.key = key;
            return;
        }
        let pos = match node.kids.iter().position(|(n, _)| *n == segs[0]) {
            Some(i) => i,
            None => {
                node.kids.push((
                    segs[0].clone(),
                    Node {
                        key,
                        ..Node::default()
                    },
                ));
                node.kids.len() - 1
            }
        };
        insert(&mut node.kids[pos].1, &segs[1..], place, key);
    }
    fn sort(node: &mut Node) {
        node.kids.sort_by_key(|(_, n)| n.key);
        for (_, k) in &mut node.kids {
            sort(k);
        }
    }
    let mut root = Node::default();
    for p in places {
        let key = order.get(&p.path).copied().unwrap_or(usize::MAX);
        insert(&mut root, &split_path(&p.path), p, key);
    }
    sort(&mut root);
    let prior_map: BTreeMap<&str, &Place> = prior
        .unwrap_or(&[])
        .iter()
        .map(|p| (p.path.as_str(), p))
        .collect();
    let mut out = Vec::new();
    fn emit(
        node: &Node,
        depth: usize,
        rails: &mut Vec<bool>,
        prior: Option<&BTreeMap<&str, &Place>>,
        parent: Change,
        out: &mut Vec<Row>,
    ) {
        let n = node.kids.len();
        for (i, (name, kid)) in node.kids.iter().enumerate() {
            let last = i + 1 == n;
            let (label_text, state, change) = match &kid.place {
                Some(p) => {
                    let change = match prior {
                        None => Change::Same,
                        Some(m) => match m.get(p.path.as_str()) {
                            None => Change::New,
                            Some(q) if shown(q) != shown(p) => Change::Changed,
                            Some(_) => Change::Same,
                        },
                    };
                    (label(p), p.state.clone(), change)
                }
                None => (String::new(), State::Live, Change::Same),
            };
            out.push(Row {
                depth,
                name: name.clone(),
                label: label_text,
                state,
                aggregate: !kid.kids.is_empty(),
                change,
                last,
                rails: rails.clone(),
                inherited: change != Change::Same && change == parent,
            });
            rails.push(!last);
            emit(kid, depth + 1, rails, prior, change, out);
            rails.pop();
        }
    }
    let mut rails = Vec::new();
    emit(
        &root,
        0,
        &mut rails,
        prior.map(|_| &prior_map),
        Change::Same,
        &mut out,
    );
    out
}

/// Each path's first appearance across the whole trace.
pub fn first_seen(records: &[Record]) -> BTreeMap<String, usize> {
    let mut order = BTreeMap::new();
    let mut n = 0usize;
    for r in records {
        for p in &r.places {
            // Prefixes too, so a parent sorts by its earliest descendant.
            let segs = split_path(&p.path);
            let mut acc = String::new();
            for (i, s) in segs.iter().enumerate() {
                if i > 0 && !s.starts_with('[') {
                    acc.push('.');
                }
                acc.push_str(s);
                order.entry(acc.clone()).or_insert_with(|| {
                    n += 1;
                    n
                });
            }
        }
    }
    order
}

/// A finished diagram: the statement, and the two trees.
#[derive(Debug, Clone)]
pub struct Diagram {
    pub line: u32,
    pub statement: String,
    pub before: Vec<Row>,
    pub after: Vec<Row>,
}

pub fn diagram(program: &str, trace: &[Record], line: u32) -> Result<Diagram> {
    let statement = program
        .lines()
        .nth(line as usize - 1)
        .with_context(|| format!("the program has no line {line}"))?
        .trim()
        .to_string();
    let (before, after) = before_after(trace, "main", line)?;
    let order = first_seen(trace);
    Ok(Diagram {
        line,
        statement,
        before: rows(before, &order, None),
        after: rows(after, &order, Some(before)),
    })
}

// ------------------------------------------------------- text rendering

fn tree_prefix(r: &Row) -> String {
    let mut s = String::new();
    if r.depth == 0 {
        return s;
    }
    for &more in &r.rails[1..] {
        s.push_str(if more { "│  " } else { "   " });
    }
    s.push_str(if r.last { "└─ " } else { "├─ " });
    s
}

fn text_panel(out: &mut String, rows: &[Row]) {
    if rows.is_empty() {
        out.push_str("(nothing yet)\n");
        return;
    }
    let width = rows
        .iter()
        .map(|r| tree_prefix(r).chars().count() + r.name.chars().count())
        .max()
        .unwrap_or(0);
    for r in rows {
        let head = format!("{}{}", tree_prefix(r), r.name);
        let pad = width - head.chars().count() + 2;
        // A subtree that changed with its root says so once, at the root.
        let shown = if r.inherited { Change::Same } else { r.change };
        let mark = match shown {
            Change::Same => "",
            Change::Changed => "   (changed)",
            Change::New => "   (new)",
        };
        let _ = writeln!(out, "{head}{}{}{mark}", " ".repeat(pad), r.label);
    }
}

/// The text rendering: the fence body, the one-file edition's figure, and
/// the web edition's alt text.
pub fn render_text(d: &Diagram) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "before line {}: {}", d.line, d.statement);
    text_panel(&mut out, &d.before);
    out.push('\n');
    let _ = writeln!(out, "after line {}", d.line);
    text_panel(&mut out, &d.after);
    out
}

// -------------------------------------------------------- SVG rendering

// Geometry, in pixels. Everything is set in one monospace face, so a
// width is a character count: 13px Source Code Pro advances 0.6em, which
// is 7.8px; kept in tenths so the output is integer arithmetic and
// byte-identical on every host.
const FONT: u32 = 13;
const CHAR_W10: u32 = 78;
const ROW_H: u32 = 26;
const PAD: u32 = 16;
const INDENT: u32 = 22;
const GAP: u32 = 24;
const HEAD_H: u32 = 30;
const CAPTION_H: u32 = 34;
/// Wider than this and the two trees stack instead of sitting side by
/// side (the book's text column is about 750px).
const MAX_W: u32 = 720;

// The palette is the book's (dialects.rs): the program accent, the code
// ground, and a neutral gray for what is gone.
const INK: &str = "#1d1d1b";
const MUTED: &str = "#6b6b66";
const ACCENT: &str = "#1a4f8a";
const ACCENT_TINT: &str = "#e7eef7";
const GROUND: &str = "#faf8f4";
const RULE: &str = "#ddd6c8";
const GONE_FILL: &str = "#f0efec";
const GONE_RULE: &str = "#9a9a94";
const LIVE_FILL: &str = "#ffffff";

fn chars_w(n: usize) -> u32 {
    (n as u32 * CHAR_W10).div_ceil(10)
}

fn xml(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => o.push_str("&amp;"),
            '<' => o.push_str("&lt;"),
            '>' => o.push_str("&gt;"),
            '"' => o.push_str("&quot;"),
            '\'' => o.push_str("&#39;"),
            _ => o.push(c),
        }
    }
    o
}

struct PanelGeom {
    w: u32,
    h: u32,
    val_x: u32,
}

fn panel_geom(rows: &[Row]) -> PanelGeom {
    let name_w = rows
        .iter()
        .map(|r| r.depth as u32 * INDENT + chars_w(r.name.chars().count()))
        .max()
        .unwrap_or(chars_w(12));
    let val_x = PAD + name_w + 18;
    let label_w = rows
        .iter()
        .filter(|r| !r.label.is_empty())
        .map(|r| chars_w(r.label.chars().count()) + 14)
        .max()
        .unwrap_or(0);
    let w = (val_x + label_w + PAD).max(PAD * 2 + chars_w(14));
    let h = HEAD_H + ROW_H * rows.len().max(1) as u32 + PAD / 2;
    PanelGeom { w, h, val_x }
}

fn svg_panel(out: &mut String, title: &str, rows: &[Row], x0: u32, y0: u32, g: &PanelGeom) {
    let _ = writeln!(
        out,
        "<text x=\"{}\" y=\"{}\" font-weight=\"700\" fill=\"{INK}\">{}</text>",
        x0 + PAD,
        y0 + 18,
        xml(title)
    );
    if rows.is_empty() {
        let _ = writeln!(
            out,
            "<text x=\"{}\" y=\"{}\" fill=\"{MUTED}\">(nothing yet)</text>",
            x0 + PAD,
            y0 + HEAD_H + 17
        );
        return;
    }
    // Connectors first, so the boxes sit on top of them.
    for (i, r) in rows.iter().enumerate() {
        if r.depth == 0 {
            continue;
        }
        let cy = y0 + HEAD_H + ROW_H * i as u32 + ROW_H / 2;
        // The parent is the nearest row above at depth - 1.
        let parent = rows[..i]
            .iter()
            .rposition(|p| p.depth + 1 == r.depth)
            .unwrap_or(0);
        let py = y0 + HEAD_H + ROW_H * parent as u32 + ROW_H / 2 + 7;
        let rail_x = x0 + PAD + (r.depth as u32 - 1) * INDENT + 5;
        let nx = x0 + PAD + r.depth as u32 * INDENT - 4;
        let _ = writeln!(
            out,
            "<path d=\"M{rail_x} {py}V{cy}H{nx}\" fill=\"none\" stroke=\"{RULE}\" stroke-width=\"1.5\"/>"
        );
    }
    for (i, r) in rows.iter().enumerate() {
        let cy = y0 + HEAD_H + ROW_H * i as u32 + ROW_H / 2;
        let nx = x0 + PAD + r.depth as u32 * INDENT;
        let gone = matches!(r.state, State::Moved | State::Uninit);
        let name_fill = if gone { MUTED } else { INK };
        let _ = writeln!(
            out,
            "<text x=\"{nx}\" y=\"{}\" fill=\"{name_fill}\">{}</text>",
            cy + 4,
            xml(&r.name)
        );
        if r.label.is_empty() {
            continue;
        }
        let lx = x0 + g.val_x;
        let lw = chars_w(r.label.chars().count()) + 14;
        if r.aggregate && !gone {
            // An aggregate's row names its type; the boxes are its leaves.
            let fill = if r.change == Change::Same {
                MUTED
            } else {
                ACCENT
            };
            let _ = writeln!(
                out,
                "<text x=\"{}\" y=\"{}\" fill=\"{fill}\" font-style=\"italic\">{}</text>",
                lx + 7,
                cy + 4,
                xml(&r.label)
            );
            continue;
        }
        let (fill, stroke, dash, ink, sw) = match (gone, r.change) {
            (true, Change::Same) => (
                GONE_FILL,
                GONE_RULE,
                " stroke-dasharray=\"4 3\"",
                MUTED,
                "1",
            ),
            (true, _) => (
                GONE_FILL,
                ACCENT,
                " stroke-dasharray=\"4 3\"",
                MUTED,
                "1.75",
            ),
            (false, Change::Same) => (LIVE_FILL, RULE, "", INK, "1"),
            (false, _) => (ACCENT_TINT, ACCENT, "", INK, "1.75"),
        };
        let _ = writeln!(
            out,
            "<rect x=\"{lx}\" y=\"{}\" width=\"{lw}\" height=\"20\" rx=\"4\" fill=\"{fill}\" stroke=\"{stroke}\" stroke-width=\"{sw}\"{dash}/>",
            cy - 10
        );
        let _ = writeln!(
            out,
            "<text x=\"{}\" y=\"{}\" fill=\"{ink}\">{}</text>",
            lx + 7,
            cy + 4,
            xml(&r.label)
        );
    }
}

/// The SVG: the statement as a caption, then the tree before the line and
/// the tree after it, side by side when they fit and stacked when not.
/// Static markup only: no script, no external reference, no font file.
pub fn render_svg(d: &Diagram, alt: &str) -> String {
    let gb = panel_geom(&d.before);
    let ga = panel_geom(&d.after);
    let caption = format!("line {} · {}", d.line, d.statement);
    let cap_w = PAD * 2 + chars_w(caption.chars().count());
    let side = gb.w + GAP + ga.w <= MAX_W;
    let (w, h, ax, ay) = if side {
        (
            (gb.w + GAP + ga.w).max(cap_w),
            CAPTION_H + gb.h.max(ga.h),
            gb.w + GAP,
            CAPTION_H,
        )
    } else {
        (
            gb.w.max(ga.w).max(cap_w),
            CAPTION_H + gb.h + GAP / 2 + ga.h,
            0,
            CAPTION_H + gb.h + GAP / 2,
        )
    };
    let mut out = String::new();
    let _ = writeln!(
        out,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\" role=\"img\" font-family=\"'Source Code Pro', ui-monospace, monospace\" font-size=\"{FONT}\">"
    );
    let _ = writeln!(out, "<title>{}</title>", xml(&caption));
    let _ = writeln!(out, "<desc>{}</desc>", xml(alt.trim_end()));
    let _ = writeln!(
        out,
        "<!-- GENERATED by `cargo xtask diagrams` from the interpreter's place trace — do not edit. -->"
    );
    let _ = writeln!(
        out,
        "<rect x=\"0.5\" y=\"0.5\" width=\"{}\" height=\"{}\" rx=\"6\" fill=\"{GROUND}\" stroke=\"{RULE}\"/>",
        w - 1,
        h - 1
    );
    let _ = writeln!(
        out,
        "<text x=\"{PAD}\" y=\"22\" fill=\"{ACCENT}\">{}</text>",
        xml(&caption)
    );
    let _ = writeln!(
        out,
        "<line x1=\"{PAD}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{RULE}\"/>",
        CAPTION_H - 2,
        w - PAD,
        CAPTION_H - 2
    );
    svg_panel(&mut out, "before", &d.before, 0, CAPTION_H, &gb);
    if side {
        let _ = writeln!(
            out,
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{RULE}\" stroke-dasharray=\"2 3\"/>",
            gb.w + GAP / 2,
            CAPTION_H + 8,
            gb.w + GAP / 2,
            h - 8
        );
    }
    svg_panel(&mut out, "after", &d.after, ax, ay, &ga);
    out.push_str("</svg>\n");
    out
}

// ------------------------------------------------------------ the command

/// The trace-capable lupin, when the host has one. Unset is a loud
/// skip of the trace half; set to something that is not a file is an
/// error, because a CI lane that built one and pointed at the wrong path
/// must not pass as a host without one.
fn tracer() -> Result<Option<PathBuf>> {
    match std::env::var_os("LUPIN_TRACE") {
        None => Ok(None),
        Some(p) if p.is_empty() => Ok(None),
        Some(p) => {
            let p = PathBuf::from(p);
            if !p.is_file() {
                bail!("LUPIN_TRACE={} is not a file", p.display());
            }
            Ok(Some(p))
        }
    }
}

/// Run `program` under the trace lupin and return the trace. The program
/// is written alone in its own directory (files in one directory are one
/// module, D32) under its sample name, so its lines are the fence's lines.
pub fn trace_program(lupin: &Path, work: &Path, name: &str, program: &str) -> Result<String> {
    let dir = work.join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir)?;
    let file = format!("{name}.lu");
    std::fs::write(dir.join(&file), program)?;
    let trace = dir.join("trace.jsonl");
    let out = Command::new(lupin)
        .current_dir(&dir)
        .arg(format!("--trace-places={}", trace.display()))
        .arg(&file)
        .output()
        .with_context(|| format!("running {}", lupin.display()))?;
    if !out.status.success() {
        bail!(
            "{name}: the trace run exited {:?}: {}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    std::fs::read_to_string(&trace).with_context(|| format!("{name}: no trace written"))
}

fn trace_path(root: &Path, id: &str) -> Result<PathBuf> {
    let (page, name) = id_parts(id)?;
    Ok(root
        .join("snapshots/traces")
        .join(page)
        .join(format!("{name}.jsonl")))
}

pub fn run(root: &Path, args: &[String]) -> Result<()> {
    let check = args.iter().any(|a| a == "--check");
    let (fences, programs) = collect(root)?;
    let lupin = tracer()?;
    match &lupin {
        Some(l) => println!("diagrams: trace lupin = {}", l.display()),
        None => println!(
            "diagrams: SKIP the trace half — LUPIN_TRACE is unset, so the \
             traces under snapshots/traces/ are not re-taken; the text and the SVG are \
             still regenerated from them and {}",
            if check { "checked" } else { "written" }
        ),
    }
    let work = root.join("target/diagrams");
    let used: BTreeSet<&str> = fences.iter().map(|f| f.from.as_str()).collect();
    let mut failures = Vec::new();
    let mut traces: BTreeMap<&str, Vec<Record>> = BTreeMap::new();
    for id in &used {
        let program = programs
            .get(*id)
            .with_context(|| format!("memory fence: no sample `{id}` on its page"))?;
        let at = trace_path(root, id)?;
        let text = match &lupin {
            Some(l) => {
                let (_, name) = id_parts(id)?;
                let fresh = trace_program(l, &work, name, program)?;
                let old = std::fs::read_to_string(&at).unwrap_or_default();
                if fresh != old {
                    if check {
                        failures.push(format!(
                            "{}: the trace of `{id}` differs from a fresh run — run \
                             `cargo xtask diagrams` with LUPIN_TRACE and commit",
                            at.strip_prefix(root).unwrap_or(&at).display()
                        ));
                    } else {
                        std::fs::create_dir_all(at.parent().unwrap())?;
                        std::fs::write(&at, &fresh)?;
                    }
                }
                fresh
            }
            None => std::fs::read_to_string(&at).with_context(|| {
                format!(
                    "{}: no trace for `{id}`, and no LUPIN_TRACE to take one",
                    at.display()
                )
            })?,
        };
        traces.insert(
            id,
            parse_trace(&text).with_context(|| format!("trace of `{id}`"))?,
        );
    }

    let mut bodies: BTreeMap<PathBuf, BTreeMap<usize, String>> = BTreeMap::new();
    let mut wanted_svgs: BTreeSet<PathBuf> = BTreeSet::new();
    for f in &fences {
        let program = &programs[&f.from];
        let d = diagram(program, &traces[f.from.as_str()], f.line).with_context(|| {
            format!(
                "{}:{}: from({}), line({})",
                f.md.display(),
                f.open_line + 1,
                f.from,
                f.line
            )
        })?;
        let text = render_text(&d);
        let svg = render_svg(&d, &text);
        let svg_at = root.join("book").join(f.svg_rel()?);
        wanted_svgs.insert(svg_at.clone());
        let where_ = format!(
            "{}:{}",
            f.md.strip_prefix(root).unwrap_or(&f.md).display(),
            f.open_line + 1
        );
        if f.body != text {
            if check {
                failures.push(format!(
                    "{where_}: the memory fence's text is not what the trace draws — run \
                     `cargo xtask diagrams`"
                ));
            } else {
                bodies
                    .entry(f.md.clone())
                    .or_default()
                    .insert(f.open_line, text.clone());
            }
        }
        let old = std::fs::read_to_string(&svg_at).unwrap_or_default();
        if old != svg {
            if check {
                failures.push(format!(
                    "{}: stale or missing — run `cargo xtask diagrams`",
                    svg_at.strip_prefix(root).unwrap_or(&svg_at).display()
                ));
            } else {
                std::fs::create_dir_all(svg_at.parent().unwrap())?;
                std::fs::write(&svg_at, &svg)?;
            }
        }
    }
    // A diagram no fence asks for is a drawing nothing checks.
    let dir = root.join("book/diagrams");
    if dir.is_dir() {
        let mut have = Vec::new();
        walk_svg(&dir, &mut have)?;
        for p in have {
            if !wanted_svgs.contains(&p) {
                if check {
                    failures.push(format!(
                        "{}: no memory fence draws it — delete it",
                        p.strip_prefix(root).unwrap_or(&p).display()
                    ));
                } else {
                    std::fs::remove_file(&p)?;
                }
            }
        }
    }
    for (md, by_line) in &bodies {
        let source = std::fs::read_to_string(md)?;
        let new = crate::fence::rewrite(&source, |f| {
            let marker = "`".repeat(f.ticks);
            let body = by_line.get(&f.open_line).unwrap_or(&f.content);
            format!("{marker}{}\n{body}{marker}\n", f.info)
        });
        std::fs::write(md, new)?;
    }
    println!(
        "diagrams: {} diagram(s) from {} trace(s){}",
        fences.len(),
        used.len(),
        if check { ", checked" } else { ", written" }
    );
    if !failures.is_empty() {
        for f in &failures {
            eprintln!("  {f}");
        }
        bail!("diagrams --check: {} finding(s)", failures.len());
    }
    Ok(())
}

fn walk_svg(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let p = entry?.path();
        if p.is_dir() {
            walk_svg(&p, out)?;
        } else if p.extension().and_then(|e| e.to_str()) == Some("svg") {
            out.push(p);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // A trace in the shape is72's contract names: one record per
    // statement of `main`, every place it has touched. A test fixture for
    // the renderer, not a measurement.
    const PACK: &str = r#"{"trace":1,"task":0,"fn":"main","depth":1,"at":"4:5","places":[{"path":"p","state":"live","type":"Pack"},{"path":"p.lead","state":"live","type":"str","value":"ada"},{"path":"p.tail","state":"live","type":"str","value":"grace"}],"events":[]}
{"trace":1,"task":0,"fn":"adopt","depth":2,"at":"2:32","tail":true,"places":[{"path":"w","state":"live","type":"str","value":"ada"}],"events":[]}
{"trace":1,"task":0,"fn":"main","depth":1,"at":"5:5","places":[{"path":"p","state":"live","type":"Pack"},{"path":"p.lead","state":"moved","type":"str","by":"take","at":"5:24"},{"path":"p.tail","state":"live","type":"str","value":"grace"},{"path":"a","state":"live","type":"str","value":"ada"}],"events":[{"ev":"move","path":"p.lead","by":"take","at":"5:24"}]}
{"trace":1,"task":0,"fn":"main","depth":1,"at":"6:5","places":[{"path":"p","state":"live","type":"Pack"},{"path":"p.lead","state":"live","type":"str","value":"lin","reinit":"6:5"},{"path":"p.tail","state":"live","type":"str","value":"grace"},{"path":"a","state":"live","type":"str","value":"ada"}],"events":[{"ev":"reinit","path":"p.lead","at":"6:5"}]}
{"trace":1,"task":0,"fn":"main","depth":1,"at":"7:5","places":[{"path":"p","state":"live","type":"Pack"},{"path":"p.lead","state":"live","type":"str","value":"lin","reinit":"6:5"},{"path":"p.tail","state":"live","type":"str","value":"grace"},{"path":"a","state":"live","type":"str","value":"ada"},{"path":"c","state":"live","type":"str","value":"lin"}],"events":[{"ev":"copy","path":"p.lead","by":"plain","at":"7:13","to":"c"}]}
"#;

    const PROGRAM: &str = "struct Pack { lead: str, tail: str }\nfn adopt(take w: str) -> str { w }\nfn main() -> !int {\n    var p = Pack { lead: \"ada\", tail: \"grace\" }\n    let a = adopt(take p.lead)\n    p.lead = \"lin\"\n    let c = p.lead\n    print(\"{a} {c} {p.tail}\")\n    0\n}\n";

    #[test]
    fn paths_split_on_fields_and_elements() {
        assert_eq!(split_path("d.meta.author"), ["d", "meta", "author"]);
        assert_eq!(split_path("xs[0].name"), ["xs", "[0]", "name"]);
        assert_eq!(split_path("t"), ["t"]);
    }

    #[test]
    fn the_record_before_is_the_same_functions() {
        let t = parse_trace(PACK).unwrap();
        // adopt's record sits between lines 5 and 6 and is not main's.
        let (before, after) = before_after(&t, "main", 6).unwrap();
        assert!(before
            .iter()
            .any(|p| p.path == "p.lead" && p.state == State::Moved));
        assert!(after
            .iter()
            .any(|p| p.path == "p.lead" && p.state == State::Reinit));
        let (first, _) = before_after(&t, "main", 4).unwrap();
        assert!(first.is_empty());
        assert!(before_after(&t, "main", 99).is_err());
    }

    #[test]
    fn a_copy_read_draws_the_source_still_live() {
        let t = parse_trace(PACK).unwrap();
        let d = diagram(PROGRAM, &t, 7).unwrap();
        assert_eq!(d.statement, "let c = p.lead");
        let text = render_text(&d);
        assert!(
            text.starts_with("before line 7: let c = p.lead\n"),
            "{text}"
        );
        assert!(
            text.contains("c        \"lin\" · copy of p.lead   (new)"),
            "{text}"
        );
        // The source is unchanged: no mark on its row after.
        let after = text.split("after line 7\n").nth(1).unwrap();
        assert!(
            after.contains("├─ lead  \"lin\" · re-initialized 6:5\n"),
            "{after}"
        );
    }

    #[test]
    fn a_take_marks_the_field_moved_and_the_new_binding() {
        let t = parse_trace(PACK).unwrap();
        let d = diagram(PROGRAM, &t, 5).unwrap();
        let text = render_text(&d);
        assert!(
            text.contains("├─ lead  moved · take 5:24   (changed)"),
            "{text}"
        );
        assert!(text.contains("a        \"ada\"   (new)"), "{text}");
    }

    #[test]
    fn the_svg_is_static_and_carries_its_text() {
        let t = parse_trace(PACK).unwrap();
        let d = diagram(PROGRAM, &t, 5).unwrap();
        let text = render_text(&d);
        let svg = render_svg(&d, &text);
        assert!(svg.starts_with("<svg xmlns=\"http://www.w3.org/2000/svg\""));
        assert!(svg.ends_with("</svg>\n"));
        assert!(!svg.contains("<script"));
        assert!(!svg.contains("href"));
        assert!(svg.contains("<desc>before line 5: let a = adopt(take p.lead)"));
        assert!(
            svg.contains("stroke-dasharray=\"4 3\""),
            "a moved leaf is dashed"
        );
        assert!(svg.contains("&quot;ada&quot;"));
        // Deterministic: the same input draws the same bytes.
        assert_eq!(svg, render_svg(&d, &text));
    }

    #[test]
    fn page_programs_number_like_the_samples_runner() {
        let page = "```wolf,fail(E1)\nA\n```\n\n```wolf\nB\n```\n\n\
                    ```wolf,part(x),run(exit=0)\nX\n```\n\n```wolf,run(exit=0)\nC\n```\n\n\
                    ```console\n$ lupin c.lu\n```\n";
        let p = page_programs(page).unwrap();
        assert_eq!(p["s1"], "A\n");
        assert_eq!(p["s2"], "B\n");
        assert_eq!(p["s3"], "C\n");
        assert_eq!(p["part-x"], "X\n");
    }

    #[test]
    fn ids_name_a_page_and_a_sample() {
        assert_eq!(id_parts("book/ch07/s3").unwrap(), ("ch07", "s3"));
        assert_eq!(
            id_parts("book/ch07/part-packcopy").unwrap(),
            ("ch07", "part-packcopy")
        );
        assert!(id_parts("appx/exB-9").is_err());
        assert_eq!(
            svg_for_info("memory,from(book/ch07/s3),line(5)").as_deref(),
            Some("diagrams/ch07/s3-L5.svg")
        );
        assert_eq!(svg_for_info("wolf,run(exit=0)"), None);
    }
}
