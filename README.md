# The Wolf Book

The official wolf language text. The web edition is canonical and is set
to look like a printed page. The single-file markdown and the PDF come
from the same source and inherit the same decisions.
`principles/DESIGN.md` is the binding contract.

27 of the 32 chapters are written. Five are reserved: chapters 19, 20 and
21 need a benchmark instrument the toolchain does not ship, chapter 25
needs an edition mechanism and a publish client, and chapter 29 needs
`Pool` and `handle` to execute on one lane. Each of the five says so on
its own page. Chapter 13 and chapter 23 each reserve sections for the
same kind of reason.

Code samples are [GPL-3.0-or-later](LICENSE) with the [wolf Runtime
Library Exception](LICENSE-EXCEPTION), matching the runtime, so code you
take from this book into your programs is yours. The prose license is
proposed as CC BY 4.0 and is not settled.

## The toolchain

Everything is cargo (D34's spirit: one tool).

- `cargo xtask samples` extracts and executes every code sample: the
  directive-headed exercise programs under `principles/exercises/` and
  every fenced wolf block in `book/`, run against the pinned tools
  (`wolf-toolchain.toml`). Every ```` ```console ```` block whose commands
  are all pinned tools is replayed too, against the program printed above
  it, and the pasted output is byte-compared. A block that needs a shell
  is reported as skipped by name and never silently trusted. Diagnostics
  from `fail(…)` samples are snapshot-checked under `snapshots/`.
  `samples-pending.toml` lists the samples whose directives await a named
  feature. That list is report-only, and a pass there is a FLIP error, so
  a feature that lands gets noticed. `--self-test` proves the rig catches
  deliberately broken samples. `--bless` updates snapshots for review.
- `cargo xtask contrast` compiles and runs the vendored other-language
  code the book quotes (`samples/contrast/`), with warnings denied, and
  checks that every ```` ```rust ```` block in `book/` appears there
  verbatim. Contrast code rots too.
- `cargo xtask backmatter [--check]` regenerates the two generated pages:
  Appendix A from the pinned spec's `grammar.ebnf`, and the Solutions
  page from the exercise corpus. `--check` fails on drift and also
  compares `vendor/spec/` against a sibling wolf-lang checkout.
- `cargo xtask grammar-sync` re-vendors the wolf tmLanguage grammars from
  wolf-lsp at the pin into `highlight/` and regenerates the one highlight
  stylesheet (`theme/highlight.css`). `--check` fails on any drift.
  Highlighting happens at build time in the preprocessor, and
  highlight.js ships as an empty file on purpose.
- `cargo xtask render [web|md|pdf|all]` produces the three artifacts: the
  mdBook web edition (`target/render/web/`, custom full theme,
  section-number anchors `#8.4`), `target/render/wolf-book.md`, and
  `target/render/wolf-book.pdf` via typst (XCharter and Source Code Pro,
  same palette as the web). A missing typst is a loud SKIP, and
  `--require-pdf` makes it fatal as CI does.
- `cargo xtask verify-docs` checks doc truth: corpus counts, pin
  well-formedness, TOC to chapter-stub numbering, the line-count claims,
  clause tags and diagnostic codes against `vendor/spec/`, a published
  solution for every printed exercise, and the tense-discipline greps
  `principles/TONE.md` mandates.

No code appears in the book that CI did not execute.

## Layout

`book/` (SUMMARY, front matter, the 32 chapters, back matter) ·
`theme/` (the full mdBook theme replacement) · `highlight/` (vendored
grammars, never edited here) · `vendor/spec/` (the pinned spec's grammar,
clause anchors, and diagnostic-code list) · `print/` (PDF fonts) ·
`xtask/` (the pipeline) · `principles/` (the book's constitution:
TONE.md, TOC.md, EXERCISES.md and the exercise corpus, DESIGN.md,
INTERACTIVITY.md) · `docs/` (the reviewer checklist, the errata policy,
the standing prose audit) · `STYLE.md` (mechanics) · `PRINT.md` (the
typst decision) · `PERMISSIONS.md` (the lyric-quotation ledger).
