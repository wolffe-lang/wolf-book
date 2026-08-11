# DESIGN.md — the visual and system contract

The web edition is the canonical book. It looks like a well-set printed
page that happens to live in a browser: white ground, black serif text,
one measure, no chrome. Everything else — the single-file markdown, the
PDF — is generated from the same source and inherits the same decisions.
bs00 implements what this document contracts; a page that departs from it
is a build defect, not a style choice.

Cross-references: the voice on the page is TONE.md's; the section numbers
this document makes into anchors are TOC.md's; the collapsed solution
pages are EXERCISES.md's; the widgets that may appear on a page are
INTERACTIVITY.md's, and they obey §1's restraint rules like everything
else.

---

## 1. The page

- **Ground:** white (`#ffffff`). No hero banners, no cards, no
  accent-colored section headers, no dark mode at v1. Print-like is the
  identity; a reader should wonder for a moment whether the page was
  typeset.
- **Body face:** Charter — a text serif with real italics, designed for
  exactly this duty. Stack:

  ```css
  font-family: "Charter", "Bitstream Charter", "Sitka Text",
               "Iowan Old Style", "Palatino Linotype", Palatino,
               Georgia, serif;
  ```

  Charter ships on macOS; Bitstream Charter on most Linux distributions;
  the Sitka/Palatino/Georgia tail covers Windows without embarrassment.
  If rendering audits show the tail diverging too far, bs00 may self-host
  Charter as woff2 (its license permits it) — that is an implementation
  option, not a design change.
- **Measure:** 65–70ch, centered, one column. Leading 1.45–1.55. Body
  size defaults to the browser's (16px-class); the book does not shrink
  text to look dense.
- **Headings:** the same serif, differentiated by weight and size, never
  by color. Chapter titles carry their number ("8. Regions: memory in
  the shape you meant"); section headings carry `N.M`.
- **Color is for meaning only:** link underlines, diagnostic severity in
  code blocks, nothing decorative. Body text is near-black on white and
  is never gray.
- **Epigraphs** (TONE.md §4) are set in italics at the chapter head,
  attribution flush right — typographically distinct from body prose, as
  the register rules require.

## 2. Code blocks and the single-source highlight contract

- **Presentation:** a very light warm gray ground (`#faf8f4` class),
  no border, slight inset, same measure as prose. Monospace stack:

  ```css
  font-family: "Source Code Pro", ui-monospace, "Cascadia Mono",
               Consolas, Menlo, "DejaVu Sans Mono", monospace;
  ```

  Source Code Pro is self-hosted (OFL) so web and PDF set code
  identically; the tail exists for readers who block fonts.
- **Dialects:** the five block kinds from TOC.md's notation section
  (program, part, REPL transcript, console run, diagnostic) are visually
  distinguishable — transcripts keep their prompts, console runs keep
  their `$`, diagnostics keep the compiler's exact text and layout.
- **Syntax highlighting is defined once.** One highlight definition for
  wolf and one small CSS file, shared by every page and every output
  format. The seed is wolf-lsp's generated grammar: `cargo xtask
  grammar-generate` (wolf-lsp, `xtask/src/vscode.rs`) derives
  `syntaxes/wolf.tmLanguage.json` — plus the `.wolfi` and
  `wolf.pkg`/`wolf.sum` grammars — from the pinned spec EBNF at
  `vendor/upstream/spec/grammar.ebnf`, which is itself extracted from
  the spec by `wolf xtask spec-extract`. The book consumes that
  generated tmLanguage rather than writing a second wolf grammar:
  spec → EBNF → tmLanguage → book, one lineage, drift-checked at each
  arrow (wolf-lsp already checks its arrow with `grammar-drift`; the
  book's vendored copy is pinned and checked the same way).
- **Highlighting happens at build time, not in the reader's browser.**
  The pipeline renders wolf code to classed spans during the build
  (syntect reads TextMate grammars natively, so the vendored tmLanguage
  is consumed directly); the one CSS file styles the classes. No
  client-side highlighter, no JS required to read code.
- **The palette is restrained and survives grayscale.** Keywords,
  literals, comments, and diagnostic severity — four roles, five colors
  at most, each distinguishable when the PDF is printed on a monochrome
  laser. A highlight scheme that only works in color fails the print
  requirement by definition.

## 3. Navigation

- **Every page:** previous/next links at top and bottom, a home link,
  and the ToC rail (Part → chapter → section, two levels) with the
  reader's position marked. The reader always knows where they are and
  what is adjacent.
- **Anchors are section numbers and they are stable.** `#8.4` is section
  8.4 forever; TOC.md's numbering doctrine (numbers survive retirement,
  tombstones over renumbering) is what makes external links and the
  spec cross-reference durable. Heading ids are rewritten to the section
  number at build time — auto-slugged text ids drift when a title is
  edited, so they are not the anchor.
- **Index, appendices, and solutions are first-class pages** with the
  same navigation, the same typography, and entries in the ToC rail.
  Solutions render collapsed by default via `<details>` — no JS, and
  the print pipeline expands them into the solutions back matter.
- **The exercise↔solution link is bidirectional:** each exercise links
  to its solution page; each solution links back to the exercise and to
  the section that taught it.

## 4. The multi-format pipeline

One source tree, three artifacts, one set of decisions. The pipeline —
bs00 implements it — is `wolf-book`'s cargo xtask, in the wolf-lang
xtask convention:

- **`cargo xtask samples`** — extracts every fenced block, executes it
  against the pinned toolchain (wolf) and pinned interpreter (lupin),
  snapshot-checks diagnostics, and exports the corpus tree. Defined in
  bs00's sprint file; unchanged here.
- **`cargo xtask contrast`** — compiles, with warnings denied, and runs
  the other-language programs the book quotes for honest contrast
  (`samples/contrast/`), and diffs each against the block printed on the
  page. The executed-truth invariant does not stop at wolf's border. Two
  dialects, because the two languages carry assertions differently:
  - **Rust** (`.rs`) — built by `rustc --test --deny warnings` and run,
    so the file's own `#[test]` assertions are the proof.
  - **C** (`.c`, bs10) — compiled `cc -std=c99 -Wall -Werror` and
    executed against the declared cases in `samples/contrast/cases.toml`,
    each naming argv, input files, stdin, and the exact stdout, stderr,
    and exit status. Both streams default to empty, so an unexpected
    diagnostic fails the case; a `.c` file with no case fails the lane.
    Where no C compiler exists the lane skips *loudly*, by name and
    count, per the house rule.
- **`cargo xtask grammar-sync`** — vendors the generated tmLanguage
  files from the pinned wolf-lsp revision and regenerates the one
  highlight CSS; drift between the vendored grammar and the pin fails
  the build.
- **`cargo xtask render`** — produces all three artifacts:
  - **web** (canonical): mdBook with the book's own theme, build-time
    highlighting, and section-number anchors (§5);
  - **`wolf-book.md`**: the one-file markdown concatenation, block
    directives stripped, for grep-shaped readers and downstream tools;
  - **`wolf-book.pdf`**: the paginated PDF via the typst converter bs00
    chose — same Charter/Source Code Pro faces, same highlight scheme
    (the classed-span colors map to typst styles from the same
    definition), running heads, real page numbers, and the widget
    fallbacks from INTERACTIVITY.md in place of widgets.

CI builds all three on every commit; the PDF is not a nightly chore, it
is an artifact whose absence fails the build.

## 5. The mdBook verdict, revisited honestly

bs00 decided mdBook and asked this sprint to confirm or overturn it once
the typeset/print requirements were concrete. The requirements are now
concrete, and the verdict is: **mdBook stands** — with three obligations
recorded so nobody mistakes "stands" for "works out of the box."

What this document requires that mdBook does not provide by default:

1. **The look.** mdBook's default theme is a sans-serif app shell with
   five color themes. Every visual rule in §1 requires replacing it.
   mdBook supports full theme override; the work is a stylesheet and a
   handlebars template, done once in bs00. Obligation, not obstacle.
2. **The highlight contract.** mdBook ships highlight.js, which runs in
   the reader's browser from its own grammar format — two violations of
   §2 (second grammar, client-side JS). The build disables highlight.js
   and a preprocessor renders code through syntect from the vendored
   tmLanguage instead. The preprocessor API is the very hook bs00 chose
   mdBook for; this is its second customer, after the sample extractor.
3. **Anchors.** mdBook auto-generates heading ids from text; §3 requires
   section-number ids. The same preprocessor pass rewrites them.

What mdBook provides that the contract needs: the sidebar is §3's ToC
rail; prev/next navigation is built in; search comes free; the
preprocessor chain is where the extractor, highlighter, and anchor pass
all live; and the toolchain stays cargo-only, which the no-drift story
depends on. The print requirement does not touch mdBook at all — the PDF
comes from the typst converter, not from rendering the web edition.

Re-evaluation trigger: if bs00's implementation finds the preprocessor
seam cannot deliver build-time highlighting or stable anchors without
forking mdBook, that is new evidence; the fallback is a small bespoke
renderer over the same source tree, and the source format does not
change either way. Nothing found in this sprint makes that likely.
