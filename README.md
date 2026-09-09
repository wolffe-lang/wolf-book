# The Wolf Book

The wolf language text. The web edition at [lupp.us](https://lupp.us)
is canonical and is set to look like a printed page. The single-file
markdown and the PDF are rendered from the same source.
`principles/DESIGN.md` is the binding design document.

29 of the 33 chapters are written. Chapter 29 is reserved whole,
because it needs `Pool` and `handle` to execute on one lane, and says
so on its own page. Chapters 13, 21, 23 and 25 each reserve sections
for the same kind of reason. Chapter 33 is the newest and sits at the
end of part 4 with a number out of order: section numbers are permanent
anchors, so a late chapter takes the next free number instead of
renumbering thirty others (`principles/TOC.md` §Deltas).

Code samples are [GPL-3.0-or-later](LICENSE) with the [wolf Runtime
Library Exception](LICENSE-EXCEPTION), matching the runtime, so code
you take from this book into your programs is yours. The prose license
is proposed as CC BY 4.0 and is not settled.

## The toolchain

Everything runs through cargo.

- `cargo xtask samples` extracts and executes every code sample: the
  exercise programs under `principles/exercises/` and every fenced wolf
  block in `book/`, against the pinned tools in `wolf-toolchain.toml`.
  Every ```` ```console ```` block whose commands are all pinned tools
  is replayed against the program printed above it, and the pasted
  output is byte-compared. A block that needs a shell is reported as
  skipped, with its name. Diagnostics from `fail(…)` samples are
  snapshot-checked under `snapshots/`. `samples-pending.toml` lists the
  samples whose directives are waiting on a named feature; a pass there
  is reported as a flip, so a feature that lands gets noticed.
  `--self-test` checks that the rig catches deliberately broken samples.
  `--bless` updates snapshots for review.
- `cargo xtask contrast` compiles and runs the other-language code the
  book quotes (`samples/contrast/`), with warnings denied, and checks
  that every ```` ```rust ```` block in `book/` appears there verbatim.
- `cargo xtask backmatter [--check]` regenerates the two generated
  pages: Appendix A from the pinned spec's `grammar.ebnf`, and the
  Solutions page from the exercise corpus. `--check` fails on drift and
  also compares `vendor/spec/` against a sibling wolf-lang checkout.
- `cargo xtask grammar-sync` re-vendors the wolf tmLanguage grammars
  from wolf-lsp at the pin into `highlight/` and regenerates the
  highlight stylesheet (`theme/highlight.css`). `--check` fails on
  drift. Highlighting happens at build time in the preprocessor, and
  highlight.js ships as an empty file on purpose.
- `cargo xtask render [web|md|pdf|all]` produces the three artifacts:
  the mdBook web edition (`target/render/web/`, with a full custom theme
  and section-number anchors like `#8.4`), `target/render/wolf-book.md`,
  and `target/render/wolf-book.pdf` via typst (XCharter and Source Code
  Pro, same palette as the web). A missing typst is reported as a skip;
  `--require-pdf` makes it an error, which is how CI runs it.
- `cargo xtask verify-docs` checks the book's factual claims: corpus
  counts, pin well-formedness, TOC-to-chapter numbering, line-count
  claims, clause tags and diagnostic codes against `vendor/spec/`, a
  published solution for every printed exercise, and the tense
  conventions in `principles/TONE.md`.

Every code block in the book is executed by CI.

## Layout

`book/` (SUMMARY, front matter, the chapters, back matter) · `theme/`
(the mdBook theme replacement) · `highlight/` (vendored grammars, edited
only upstream) · `vendor/spec/` (the pinned spec's grammar, clause
anchors and diagnostic-code list) · `print/` (PDF fonts) · `xtask/` (the
pipeline) · `principles/` (TONE.md, TOC.md, EXERCISES.md and the
exercise corpus, DESIGN.md, INTERACTIVITY.md, TWO-MACHINES.md) ·
`docs/` (the reviewer
checklist, the errata policy, the standing prose audit) · `STYLE.md`
(mechanics) · `PRINT.md` (the typst decision) · `PERMISSIONS.md` (the
lyric-quotation ledger).
