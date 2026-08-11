# The Wolf Book

The official wolf language text. The web edition is canonical and looks
like a well-set printed page; the single-file markdown and the PDF are
generated from the same source and inherit the same decisions
(`principles/DESIGN.md` is the binding contract; bs00 built the
toolchain below).

Prose license: TBD at bc00 (CC BY 4.0 proposed). Code samples:
MIT or Apache-2.0, matching the language.

## The toolchain

Everything is cargo (D34's spirit: one tool):

- `cargo xtask samples` — extracts and executes **every** code sample:
  the 170 directive-headed exercise programs under
  `principles/exercises/` and every fenced wolf block in `book/`, run
  against the pinned tools (`wolf-toolchain.toml`). Diagnostics from
  `fail(…)` samples are snapshot-checked under `snapshots/`;
  `samples-pending.toml` lists the samples whose directives await a
  named feature — report-only, and a pass there is a FLIP error so a
  landing feature is noticed. `--self-test` proves the rig catches
  deliberately-broken samples; `--bless` updates snapshots for review.
- `cargo xtask contrast` — compiles and runs the vendored other-language
  code the book quotes (`samples/contrast/`), with warnings denied, and
  checks that every ```` ```rust ```` block in `book/` appears there
  verbatim. Contrast code rots too.
- `cargo xtask grammar-sync` — re-vendors the wolf tmLanguage grammars
  from wolf-lsp at the pin into `highlight/` and regenerates the one
  highlight stylesheet (`theme/highlight.css`); `--check` fails on any
  drift. Highlighting happens at build time in the preprocessor —
  highlight.js is shipped as an empty file, deliberately.
- `cargo xtask render [web|md|pdf|all]` — the three artifacts:
  the mdBook web edition (`target/render/web/`, custom full theme,
  section-number anchors `#8.4`), `target/render/wolf-book.md`, and
  `target/render/wolf-book.pdf` via typst (XCharter + Source Code Pro,
  same palette as the web; loud SKIP if typst is absent,
  `--require-pdf` makes that fatal as CI does).
- `cargo xtask verify-docs` — doc truth: corpus counts, pin
  well-formedness, TOC ↔ chapter-stub numbering.

No code appears in the book that CI did not execute.

## Layout

`book/` (SUMMARY + chapter stubs; prose lands with bs01+) ·
`theme/` (the full mdBook theme replacement) · `highlight/` (vendored
grammars — never edited here) · `print/` (PDF fonts) · `xtask/` (the
pipeline) · `principles/` (the book's constitution: TONE.md, TOC.md,
EXERCISES.md + the exercise corpus, DESIGN.md, INTERACTIVITY.md) ·
`STYLE.md` (mechanics) · `PRINT.md` (the typst decision) ·
`PERMISSIONS.md` (the lyric-quotation ledger).
