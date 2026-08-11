# PERMISSIONS.md — the rights ledger

Two registers are tracked here, both for the same reason: they touch
material with a rights holder. §1 is the lyric-quotation ledger
mandated by TONE.md §4.2. §2 is the K&R attribution ledger mandated by
bs10, which sets three of the book's projects beside their C ancestors.

Everything in this file is reviewed in full before any print edition.
The web edition carries the same discipline; "it's only online" is not
a legal theory we test.

---

## 1. Lyric quotations (TONE.md §4.2)

Song lyrics are copyrighted and lyric quotation is litigious even at
short length. Every quotation in the Cage the Elephant register is
logged here the same commit it enters a chapter.

Policy, restated from TONE.md so this file stands alone:

- Prefer **title fragments** and **near-title allusion**, which sit
  safer than verse lines. Direct quotes stay brief and attributed.
- At most one placement per part; five or six in the whole book.
- Always typographically distinct (epigraph position or a set-off
  attributed line), never load-bearing: delete the quotation and the
  page must still teach.

Status values: `planned` (worked placement exists in TONE.md, chapter
not yet written) · `placed` (live in a chapter) · `cleared` (reviewed
for the print edition) · `cut`.

| # | quotation | artist / work | form | placement | status | notes |
|---|-----------|---------------|------|-----------|--------|-------|
| 1 | "Ain't no rest for the wicked." | Cage the Elephant, "Ain't No Rest for the Wicked" | title fragment, epigraph | Part 3, scheduler chapter (ch17) | planned | TONE.md §4.2 worked placement 1; title-only quotation |
| 2 | "Trouble" | Cage the Elephant, "Trouble" | near-title allusion, set-off line | Appendix C intro (diagnostics) | planned | TONE.md §4.2 worked placement 2; allusion, no verse quoted |
| 3 | "Come a little closer." | Cage the Elephant, "Come a Little Closer" | title fragment, epigraph | Part 2, chapter 9 head (`book/ch09.md`) | placed | bs05. Title only, no verse; epigraph position, attribution flush right, and the first prose sentence pays the double-take off ("this chapter is about exactly how close we let it come"). Part 2's single placement in this register; delete it and the chapter still teaches |
| 4 | "Skin and Bones." | Cage the Elephant, "Skin and Bones" (*Social Cues*, 2019) | title fragment, epigraph | Part 1, chapter 2 head (`book/ch02.md`) | placed | bs01. Title only, no verse; epigraph position, attribution flush right, and the first prose sentence pays the double-take off ("a wolf string is skin and bones: bytes, and a length"). Part 1's single placement in this register; delete it and the chapter still teaches |

Chapter 1's epigraph is Bruckner (dark-Romantic register, untracked
below by design). Part 1 therefore spends one Cage placement and one
public-domain allusion, and neither chapter carries both.

bs02 (chapters 3–6) adds no row: Part 1's single Cage placement is
spent on chapter 2, and TONE.md §4.2 allows one per part. Chapters 3, 4,
and 5 carry no epigraph and no in-prose simile at all; chapter 6 carries
one dark-Romantic epigraph (Mahler's Ninth, public domain, untracked
here by design), which is Part 1's second and last placement in that
register. Chapters 7 and 8 inherit TONE.md §4.1's two worked placements
(the Mahler Sixth hammer at chapter 7's head, the Tristan simile in the
freeze section), so bs03 should place one of them, not both.

bs03 (chapter 7) adds no row. It places the Mahler Sixth hammer at the
chapter head — dark-Romantic, public domain, untracked here — and no
in-prose simile, so chapter 7 spends one reference placement in total.
Part 2's single Cage placement stays reserved for chapter 9 (row 3
above), which is where TONE.md §4.2's worked placement puts it; the
Tristan simile stays available to bs04's freeze section.

bs04 (chapter 8) adds no row. It places the second of TONE.md §4.1's
two worked placements — the Tristan simile in the freeze section
(`book/ch08.md` §8.5) — and nothing else: no chapter epigraph, no
second simile. Chapter 8 therefore spends one reference placement in
total, as chapter 7 did, and the allocation this ledger anticipated
above (the hammer to chapter 7, Tristan to chapter 8) is now spent in
full. Part 2's single Cage placement stays reserved for chapter 9
(row 3), the only unspent placement left in this part.

bs05 (chapter 9) spends row 3, the placement this ledger has held for
Part 2 since bs01: the title fragment "Come a little closer." at the
chapter head, attributed, with the payoff in the first sentence of
prose ("this chapter is about exactly how close we let it come" — the
chapter is about the C membrane). Chapter 9 carries no dark-Romantic
epigraph and no in-prose simile, so it spends one reference placement
in total, as chapters 7 and 8 did. Part 2 is now closed at three
placements — the Mahler Sixth hammer, the Tristan simile, and this one
— and no placement in this register is left unspent before Part 3,
whose reservation is row 1.

bs06 (chapters 10–12) adds no row, and leaves row 1 unspent. It places
one reference in total, and it is the third of TONE.md §4.1's worked
placements: the Bruckner pause at chapter 10's head (`book/ch10.md`),
public domain, untracked here by design, with the two sentences of prose
§4.1 specifies tying the image to the scoped join. Chapters 11 and 12
carry no epigraph and no in-prose simile at all. Part 3's single Cage
placement stays reserved for the scheduler chapter (row 1, chapter 17),
which is where TONE.md §4.2's worked placement 1 puts it and which bs07
owns; bs06 deliberately does not spend it, because a part has one and
chapter 17's material is what the quotation was chosen for.

bs10 (Part 5, the projects) adds no row and spends nothing. No chapter
prose landed — the three side-by-side chapters wait on their gates
(`principles/TOC.md` §Deltas) — so Part 5's whole reference allocation
is still unspent and reserved as the sprint contract sets it: **at most
two dark-Romantic placements** across the part's seven chapters (the
intended homes are chapter 28 or the solo, plus the coda's, which is
the natural site of the book's final such epigraph) and **at most one
Cage placement** for the entire part. Part openers are prose at the head
of a part's first chapter (chapter 7 is the model), so Part 5's opener —
and any epigraph on it — lands with chapter 26.

The dark-Romantic register (TONE.md §4.1 — Mahler, Bruckner, Wagner
allusions) is public domain and is deliberately *not* tracked here;
this ledger is for material with a rights holder.

---

## 2. K&R attributions (bs10)

Three of the projects part's builds are set beside the C programs they
descend from. The programs are folklore; **the listings in K&R are
copyrighted**, so the rule is absolute and has no exceptions:

- **No verbatim K&R code, ever.** Each C twin is an ORIGINAL
  implementation written *in the manner of* the named section — its own
  identifiers, its own comments, its own structure decisions.
- **Attributed twice:** in the chapter prose ("after K&R §x.y") and in
  the header comment of the vendored `.c` file, which also points back
  at this ledger.
- **Executed, not quoted:** every twin is compiled `-std=c99 -Wall
  -Werror` and run against declared cases by `cargo xtask contrast`
  (`samples/contrast/cases.toml`). A twin nobody runs is a paraphrase
  of a listing, which is the thing this section forbids.
- The book's side-by-side honesty rule applies to the comparison
  itself: line counts are measured with `wc`, and where wolf is not
  shorter the text says so.

Status values as in §1: `planned` (the twin is vendored and asserted,
its chapter not yet written) · `placed` (live in a chapter) ·
`cleared` (reviewed for the print edition) · `cut`.

| # | source | our twin | project | status | notes |
|---|--------|----------|---------|--------|-------|
| K1 | Kernighan & Ritchie, *The C Programming Language*, 2nd ed., §1.5–1.6 (character counting, the word-state machine) | `samples/contrast/count.c` | P1 `count` | planned | Original. The IN/OUT state machine and the `getc` loop are the teaching shape; identifiers, the per-file/total row structure, the `struct tally`, and the sentinel-int failure convention are ours. Compiled `-Werror`, four asserted cases (two files with a total, stdin with no total, one file, a file that will not open) |
| K2 | Kernighan & Ritchie, *The C Programming Language*, 2nd ed., §4.3 (the reverse-Polish calculator) | `samples/contrast/rpn.c` | P2 `rpn` | planned | Original. The getop/push/pop division of labor and the getch/ungetch pushback pair are the teaching shape; the `broken` error flag, the operator set, and the end-of-expression handling are ours. Compiled `-Werror`, four asserted cases (nested expression, division by zero, a pushed zero indistinguishable from an underflow, negative operands) |
| K3 | Kernighan & Ritchie, *The C Programming Language*, 2nd ed., §6.5 (the word-frequency binary tree) | `samples/contrast/wordtree.c` | P3 `wordtree` | planned | Original. The addtree/treeprint/talloc division of labor and the recursive in-order walk are the teaching shape; `dupstr` (not POSIX `strdup`), the `nomem` flag, the half-built-node cleanup, and `treefree` are ours. Compiled `-Werror`, three asserted cases (alphabetized walk with repeats, case folding, the empty tree) |

The three chapters that will print these twins are not written: their
wolf halves do not build at the pins (see the bs10 gate note in
`principles/TOC.md`). The twins are vendored and asserted now because
the C half of each side-by-side is buildable today, and because a
vendored twin is what keeps this ledger's attribution checkable rather
than promissory. No `.md` file quotes them yet, so no row is `placed`.
