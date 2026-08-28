# Lane C scoreboard — bs14 tells audit (ch23–ch32 + book/back/)

Fifteen files cataloged: eight substantive chapters (ch25 and ch29 are
held chapters with ~15 lines of prose each), four appendices, glossary,
colophon, errata, book index, and solutions. All prose-only; code,
console, diagnostic, and ledger-comment blocks excluded throughout.

## Files ranked by tell density

Flagged density = (charter + tell) rows per 1000 prose words. Dash
figures are running-prose dashes (template dashes excluded).

| File | Prose words | Dashes/1000 | Negation family | charter/tell/noted | Flagged/1000 |
|------|------------|-------------|-----------------|--------------------|--------------|
| ch31 | 1073 | 7.5 | 5 | 1/4/3 | 4.7 |
| ch30 | 2296 | ~8 | 13 | 2/8/7 | 4.4 |
| ch23 | 1361 | ~10 | 7 | 0/6/5 | 4.4 |
| glossary | 790 | 0 (31 template) | 5 | 1/2/4 | 3.8 |
| ch26 | 2239 | ~13 | 4 | 2/5/9 | 3.1 |
| ch32 | 1400 | 7.1 | 11 | 0/4/8 | 2.9 |
| ch24 | 2748 | ~12 | 11 | 0/7/8 | 2.5 |
| errata | 425 | 4.7 | 4 | 0/1/3 | 2.4 |
| ch28 | 1875 | ~9 | 9 | 0/3/9 | 1.6 |
| ch27 | 2174 | ~11 | 6 | 0/3/7 | 1.4 |
| solutions | ~31,700 | 14.0 | 12 strict + sampled | 0/5/3 | 0.2* |
| appendix-d | 695 | 4.3 | 0 | 0/0/2 | 0 |
| colophon | 374 | 10.7 | 2 | 0/0/3 | 0 |
| appendix-c | 1015 | 2.0 | 1 | 0/0/2 | 0 |
| appendix-b | 463 | 0 | 2 | 0/0/2 | 0 |
| ch25, ch29, appendix-a, book-index | ~1,500 | ~0 | ~2 | 0/0/5 | 0 |

\* solutions' per-word figure is low only because its tell rows are
whole-file pattern rows (the mold ×11, the closer formula, the dash
constant); read it as "same density as the chapters", which is what its
14.0/1000 dash figure says.

Reading of the ranking: the reference back matter is essentially clean —
TONE §3.8's deadpan convergence holds where the register demands it. The
distance from K&R lives in the chapters and in the solutions' discussion
prose, and it is rhetorical (dash asides, negation antithesis, emphasis
devices), not lexical: **zero banned-vocabulary hits in the entire
lane**, and only three charter rows (two British spellings in ch30, one
British spelling pair in ch26, sass-in-reference in the glossary).

## Top five recurring patterns

1. **The 14/1000 em-dash constant.** Running-prose dash density lands at
   14.0–14.9/1000 in ch23, ch24, ch26, ch27, and the solutions file —
   five files, same number — with ch28/ch30 near 9–13 and only
   ch31/ch32 near half that. ~190 running dashes in the chapters plus
   445 in solutions. This is the author voice, not any chapter's habit.
2. **The negation family**, ~80 counted instances lane-wide (per-file
   counts in the table). Sub-forms: "not X; it is Y" section openers and
   closers; "X rather than Y" as the default contrast frame (~50+
   instances; 34 in solutions alone, 5 in ch23, 6 in ch24); "X, not Y"
   appositive corrections.
3. **The "did not X. It Y-ed." mold** — the family's most templated
   member: ch24 L169–170 («Wolf did not remove the phase. It removed the
   authority.»), ch28 L7–8, ch30 L3–4, ch32 L233–234, and eleven-plus
   «X is not A; it is B» siblings in solutions. ~18 instances of one
   sentence shape.
4. **Bold-for-emphasis thesis/measurement sentences** — 13 instances:
   ch23 L61–63, ch24 L10–11, ch26 L13–15 + four §26.5 leads, ch28 L412,
   ch30 L453 + L490, ch32 L43/L49/L221.
5. **Emphasis-by-negation fragments and stage-managed reader address** —
   «Not any — every.» ×3 in ch31, «Not "usually" and not "for the seeds
   we tried"» ch30 L445, «Not a thread, not a coroutine, not a promise.»
   glossary L96; plus «Sit with that line» ch24 L253, «Hold that
   number» ch27 L196, «worth reading twice» ch30 L453, «read the layout
   comment twice» ch32 L56 — one author habit, two spellings,
   concentrated in the part's closing chapters.

## SYSTEMIC (template-baked — one bs15 fix, not thirty)

- **Template dashes dominate raw density in the back matter.** The
  glossary's 31/31 dashes are its `**term** (§) — definition` template;
  solutions carries 546 template dashes (three per exercise: summary
  line, stem lead, `Solution — path` header); appendix-d's table has
  seven `NN — Title` dashes; every chapter has `**Exercise N-M**
  *(class · lane)* —` stems and `## N.M title — subtitle` / `### M<n> —`
  headings. One punctuation decision per template (colon or period)
  retires ~600 dashes without touching a sentence of prose. Any bs15
  dash budget must count running prose only, or these templates will eat
  the whole allowance.
- **The measurement-section shape in the K&R-twin chapters** (ch26
  §26.5, ch28 §28.5, ch32 §32.3): bold measurement number, bold
  antithesis leads, "where the lines went" accounting. The accounting is
  the part's best material; the bolding is one shape to unbold across
  three chapters.
- **Cross-file echoes must be fixed together or not at all:** ch23's
  package epigram («…is not technical — it is that you did not read
  it») recurs in the glossary's package entry (L54–55); ch24's covenant
  sentence («The covenant is not a policy document…») recurs at
  solutions L6561. Rewrite one and the other becomes a stale quote.
- **Solutions mirrors chapters 1–30, not just this lane.** Its stems
  quote chapter text verbatim and its discussion prose paraphrases
  chapter arguments; every rewrite lanes A and B make has a
  mirror-image edit in this file. bs15 should sequence solutions after
  the chapter lanes, or the echoes above multiply.
- **The held-chapter boilerplate is good** — ch25 and ch29's "This
  edition does not carry this chapter" pages are the charter's scope
  register working; nothing to fix, and worth holding up as the template
  it is.
