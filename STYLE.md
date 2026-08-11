# STYLE.md — mechanics

The mechanics deliverable of bs00 (contract target 4). Voice and
register live in `principles/TONE.md`; the visual system lives in
`principles/DESIGN.md`. Where this file overlaps either, they win and
the overlap here is a pointer, not a second rule.

## Samples

- **≤ 25 lines on the main path.** A longer program is split into
  `part(name)` blocks threaded through prose, or it belongs in the
  capstone.
- **Every sample is executed by CI.** No code appears in the book that
  CI did not execute (the Crafting Interpreters invariant, inverted:
  we extract from prose). Directives on the fence say how:
  `wolf,run(exit=0, stdout="…")`, `wolf,fail(E1001)`,
  `wolf,part(name)` / `part(name, cont)`, `wolf-repl`.
- **Tool output is verbatim from CI runs, never hand-typed.** Console
  blocks keep their `$`; transcripts keep their `wolf>` prompts;
  diagnostics keep the compiler's exact text and layout, shown in
  full, never elided with `...` (TONE.md §1).
- **Console blocks are replayed, not trusted.** A ```` ```console ````
  block whose commands are all pinned tools (`lupin …`, `wolf …`,
  `./binary`, `echo $?`, joined by `&&`) is re-run by
  `cargo xtask samples` against the program printed above it, and its
  output is byte-compared. A block that needs a shell, or a command
  that is not one of ours, is reported as skipped by name on every run
  — never silently unchecked. Prefer a replayable block: if the command
  cannot be replayed, the output on the page cannot be defended.
- **Blocks name their source when the page cannot**:
  ```` ```diagnostic,from(id) ```` ties shown text to a captured run,
  and ```` ```console,from(id) ```` points a console block at a sample
  other than the one immediately above it. The doc-truth and samples
  jobs diff both.

## The running example

Every chapter advances one running example; the example is chosen by
the part's first sprint and recorded in that sprint's file. A chapter
that cannot advance the example says so in its audit ledger rather
than inventing a second one.

## Callouts

"Coming from Python/Rust/C/Go" boxes use the standard admonition
form — a `blockquote` whose first line is `**Coming from X:**` — and
obey the comparison rule (TONE.md §1: defensible to that language's
designers). One per section at most.

## Typography and spelling

- American spelling; second person; present tense (TONE.md §1 owns
  the register rules).
- Em dash `—` closed up for asides; en dash `–` for ranges. Straight
  quotes in source files; smart punctuation is applied at render time
  (never inside code blocks).
- Headings: chapter titles carry their number (`# 8. Regions: …`);
  section headings carry `N.M`. The numbers come from
  `principles/TOC.md` and become the page anchors; do not invent or
  renumber locally.
- *Italics* for a term at its definition, once, never again
  (TONE.md §1). Bold is for UI/labels, not emphasis.
- Code voice in prose: backticks for identifiers, keywords, paths,
  and commands; never for concept names.

## Exercises

Numbering, density, taxonomy, and solutions policy are
`principles/EXERCISES.md`'s. Mechanically: stems end a section;
chapter batches end the chapter; every solution program lives under
the sample pipeline and its claimed output is pasted from a run.
