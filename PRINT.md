# PRINT.md — the print pipeline decision

**Decision: typst** (bs00 target 6; re-affirmed by DESIGN.md §4, which
names the render targets). Recorded so bs11 inherits a decision, not a
debate.

## Rationale

- **Compile speed suits CI.** The whole-book PDF builds on every commit
  (DESIGN.md: "an artifact whose absence fails the build"); LaTeX makes
  that a nightly chore.
- **Programmable enough** for the listing/callout styles the book needs;
  the highlight scheme is shared — the classed-span colors from the one
  vendored-grammar definition map to typst text styles from the same
  mapping table (`xtask/src/highlight.rs`), so web and PDF cannot drift
  apart.
- **Modern font stack.** The PDF sets XCharter (the free Charter
  extension — vendored under `print/fonts/`) and the same self-hosted
  Source Code Pro as the web edition.

## Risk accepted, and the fallback path

Typst's ecosystem is younger than LaTeX's. Mitigation, unchanged from
the bs00 contract: the print artifact is generated from the same
markdown via a converter in `xtask` (`cargo xtask render pdf` emits
`wolf-book.typ`, then runs `typst compile`). The markdown stays
canonical, so falling back to LaTeX in bs11 is a converter swap, not a
rewrite.

## Toolchain availability

`typst` is the one non-cargo tool in the pipeline. Where it is absent,
`cargo xtask render pdf` still emits `wolf-book.typ` (the converter is
always exercised) and then fails loudly with `SKIP: typst not found` —
never a silent pass. CI installs typst explicitly; the web and
single-file-markdown targets never depend on it.
