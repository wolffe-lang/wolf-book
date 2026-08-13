# Review checklist

Run this over a chapter before it is called done, and over every chapter
before a release. It is five checks. Two of them CI does on every commit;
three need a person.

The register rules are `principles/TONE.md`. The mechanics are
`STYLE.md`. This file is the order in which to apply them.

## 1. Truth (person)

Every normative claim traces to a spec clause, a locked decision (D, X or
I number), or a program on the page. A claim with no trace gets cut or the
spec gets fixed. Appendix D is the list of clauses the book cites;
`vendor/spec/anchors.json` is the set that exists.

## 2. Currency (CI)

`cargo xtask samples` executes every sample against the pinned toolchain,
replays every console block, and snapshot-checks every diagnostic.
`cargo xtask verify-docs` recomputes the line-count claims, checks the
clause tags and the diagnostic codes against the vendored spec artifacts,
and holds the generated back matter to its sources. A stale count fails
the build. Nothing on a page is pasted by hand.

## 3. Fairness (person)

Every comparison to another language is one its own users would accept.
The C and Rust programs the book prints are vendored, compiled with
warnings denied, and executed by `cargo xtask contrast`, so the code is
current; whether the comparison is fair is a reading, not a test.

## 4. Scope honesty (person)

No feature is described as if it exists when it does not. Held chapters
and held sections say so on their own pages, in the present tense, with
no schedule. Deferral language, sprint identifiers, and scaffold output
are absent from reader-facing text; `principles/TONE.md` §Tense
discipline is the rule and `docs/audit/promissory-prose-audit.md` is the
standing ledger.

## 5. Determinism vocabulary (person)

The flags the book prints are the flags the tools accept: `--seed`,
`--explore`, `--schedule`, `--native`. Where a chapter argues about
reproducibility, it says which of the two claims it is making — that a
seeded run reproduces the runtime's own choices, or that it reproduces an
interleaving. Only the first is true.

## Sign-off

Sign-off is per chapter and per release. The mechanical columns come from
the gate logs and are not a matter of opinion; the reading columns are
signed by a person who did not write the chapter.

| Part | Chapters | Mechanical checks | Reading review |
|------|----------|-------------------|----------------|
| 1 | 1–6 | green at the pin | unsigned |
| 2 | 7–9 | green at the pin | unsigned |
| 3 | 10–17 | green at the pin | unsigned |
| 4 | 18, 22–24 | green at the pin | unsigned |
| 4 (reserved) | 19–21, 25 | no samples to run | not applicable |
| 5 | 26–28, 30–32 | green at the pin | unsigned |
| 5 (reserved) | 29 | no samples to run | not applicable |
| Back matter | appendices, glossary, index, solutions, errata | green at the pin | unsigned |

"Unsigned" means what it says. The book has had one author and no second
reader, and the mechanical half of this checklist is what stands in for
the missing one. A release that wants the reading columns filled needs
readers, and naming people who have not read a chapter would be the
first false claim in a book built to avoid them.
