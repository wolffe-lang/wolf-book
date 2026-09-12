# STYLE.md — mechanics

The mechanics deliverable of bs00 (contract target 4). Voice and
register live in `principles/TONE.md`; the visual system lives in
`principles/DESIGN.md`. Where this file overlaps either, they win and
the overlap here is a pointer, not a second rule.

## Samples

- **≤ 25 lines on the main path.** A longer program is split into
  `part(name)` blocks threaded through prose, or it belongs in a
  projects chapter, where the walkthrough form grows one program across
  numbered stages.
- **Every sample is executed by CI.** No code appears in the book that
  CI did not execute (the Crafting Interpreters invariant, inverted:
  we extract from prose). Directives on the fence say how:
  `wolf,run(exit=0, stdout="…")`, `wolf,fail(E1001)`,
  `wolf,part(name)` / `part(name, cont)`, `wolf-repl`.
- **A warning the page teaches is asserted, not hoped for.** A fence
  that adds `warns(W0601)` is held to that exact set of warning codes
  by `wolf conform-run`'s record, in both directions; the exercise
  corpus spells the same claim as `//! warns: W0601`, wolf-lang's own
  header key. A sample that declares nothing and warns anyway is not
  red — it is counted and named in the runner's log on every run,
  report-only, which is how wolf-book#21's inventory stays visible
  while the chapters that carry it are edited one at a time.
- **A `run(…)` fence is a claim about both machines.** `lupin` and
  `wolf run` both execute the program and both must meet the declared
  exit and stdout; a machine that refuses has not met it. A program only
  one machine serves is spelled for that machine — `lupin-run(…)` or
  `wolf-run(…)` — and then owes the reader a per-machine note in the
  prose and the chapter's ledger a row with an owner.
  `principles/TWO-MACHINES.md` owns the rule, including the trap half
  (both machines name the kind; D60 rules the exit status per-machine)
  and the FLIP that retires a one-machine fence.
- **Where the spec leaves the number open, the fence does too.**
  `wolf,run(exit=nonzero)` claims both machines run the program and
  both die badly, and claims nothing about which number — the spelling
  for the places `[conf.trap.exit]` asks a conforming tool to compare
  the outcome class and never the status, such as the root supervisor
  domain dying (`[conc.proc.root]`), where `wolf` says 121 and `lupin`
  says 1. Picking one of those numbers on the fence is the page telling
  half its readers their tool is broken. An exit code is only credited
  to a program that was actually built: a package that does not compile
  exits nonzero too, and the runner reds instead of scoring it.
- **Undefined behavior needs both machines.** `wolf,ub(P1)` runs the
  sample under the interpreter's oracle *and* the compiler's checked
  build, and passes only if lupin faults and the checked build names
  that `[mem.ub]` row — a UB claim one implementation makes alone is
  not evidence. `wolf,audit(E1303)` runs `wolf audit-surface` and
  holds the manifest rule to its diagnostic. Both snapshot the
  compiler's text like any `fail(…)` sample.
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
