# bs14 lane B scoreboard — ch12–ch22

Eleven chapters cataloged (two of them, ch19 and ch20, are held chapters
carrying a notice paragraph only). 138 rows total: **charter 0 · tell 53
· noted 85**. Zero banned-vocabulary hits anywhere in the lane, and zero
hard-rule charter breaks — no exclamation marks, no "simply/just/easy",
no marketing adjectives. The lane's whole distance from the K&R register
is **rhythm and punctuation, not vocabulary**, which is good news for
bs15: nothing needs a word swap; sentences need reshaping.

## Chapters ranked by tell density

Rank by tell-severity findings per 1000 prose words (em-dash density and
hard negation-family instances alongside; template = the exercise-header
em-dashes counted inside the total):

| rank | ch | prose words | tells /1000w | em-dash (template) | em /1000w | negation hard |
|-----:|----|------------:|-------------:|-------------------:|----------:|--------------:|
| 1 | ch13 | 1665 | 3.6 | 20 (5) | 12.0 | 9 |
| 2 | ch15 | 2070 | 3.4 | 29 (9) | 14.0 | 10 |
| 3 | ch22 | 1831 | 3.3 | 20 (8) | 10.9 | 16 |
| 4 | ch16 | 2293 | 3.1 | 32 (10) | 14.0 | 12 |
| 5 | ch18 | 2993 | 3.0 | 41 (9) | 13.7 | 14 |
| 6 | ch14 | 2810 | 2.5 | 28 (9) | 10.0 | 12 |
| 7 | ch17 | 2737 | 1.8 | 31 (8) | 11.3 | 14 |
| 8 | ch12 | 2857 | 1.4 | 35 (9) | 12.3 | 8 |
| 9 | ch21 | 1732 | 1.2 | 31 (6) | 17.9 | 9 |
| 10 | ch19 | 182 | 0 | 2 (0) | 11.0 | 3 |
| 10 | ch20 | 141 | 0 | 2 (0) | 14.2 | 4 |

Lane totals: 21,311 prose words, 271 em-dashes (12.7/1000w average — a
K&R page runs roughly a third of that), 111 hard negation-family
instances. Notes on the ranking: ch13 keeps the tell-density crown its
catalog predicted, but per word the **densest negation chapter is now
ch22** (16 hard instances in 1831 words ≈ 8.7/1000w vs ch13's 5.4).
ch21 ranks near the bottom on tells while holding the **lane-maximum
em-dash density (17.9/1000w)** — its prose is the closest to register
sentence-by-sentence; its punctuation is the furthest.

## Top five recurring patterns

1. **The negation family** — "not X; it is Y" / "not X — it is Y" /
   "X, not Y" / "not only X but Y". **111 hard instances** lane-wide,
   flagged in every chapter; the reveal form ("Wolf's replacement is not
   a hook. It is a value") is the lane's default way to end a section.
   The single dominant tell.
2. **Em-dash overspend** — **271 em-dashes**, 12.7/1000w lane average;
   every full chapter spends double-to-quadruple a K&R page. 73 of the
   271 come from one template (systemic row 1 below); the rest are
   dominated by the interruptive appositive pair where commas or parens
   serve ("the release binary — the optimized one, the one a benchmark
   would time — refuses…").
3. **The anaphoric negation triple** — "no X, no Y, no Z" / "Not a
   file, not a declaration, not a line…". **21 flagged sites** across 9
   chapters; §22.1 alone has four. Usually the items are real, which is
   why most are severity tell rather than charter — the drumbeat, not
   the content, is the tell.
4. **The "worth V-ing" significance marker** — "worth stating / worth
   knowing / worth pausing on / worth memorizing". **19 flagged
   instances**: ch14 ×5, ch18 ×5, ch17 ×4, ch15 ×3, ch16 ×2 — absent
   from ch12–13, so it enters the book at ch14 and runs through Part 4.
   Editorialized significance in soft form; the fix is deletion plus
   letting the sentence make the claim.
5. **The numeric-pair/triple cadence as section punctuation** — "Two
   tools, one code, one span" / "One semantics, two moments" / "Four
   spellings, one mechanism". **12 flagged sites** (ch13 ×4, ch18 ×3,
   plus ch12, ch14, ch16, ch21). The numbers are usually real; the
   drumbeat is the template.

Runner-up: the balanced-aphorism/chiasmus closer ("a supervisor that
cannot fail cannot be supervised"; "nothing else means green"; "an
unmetered evaluator with an extra step") — 5 flagged sites in ch15,
ch17, ch18 — and the "the difference between X and Y" epigram (ch18,
ch21, ch22).

## SYSTEMIC — template-baked; one bs15 fix, not thirty

1. **The exercise-header em-dash.** Every exercise is spelled
   «**Exercise N-M** *(kind · tool)* — stem…». That template separator
   alone contributes **73 em-dashes (27% of the lane's total)** — 9 or
   so per full chapter before a single sentence is written. One fix:
   change the template's separator (a period after the parenthetical
   serves) and every chapter's density drops by roughly a third at a
   stroke. Fix the template, not the instances.
2. **The count-then-bold-led-parallel-paragraphs section shape.** "Walk
   the four decisions…" (ch15 §15.2), "Each refusal buys something
   specific… **Caching…** **Reproducibility…** **Auditing…**" (ch18
   §18.4), "Three things the rule buys… **Builds.** **Comprehension.**
   **Interfaces.**" (ch22 §22.2), "Three facts, which are contract
   rather than implementation detail" (ch14 costs section), and the
   verbatim frame echo "Two absences, stated plainly" (ch17) → "Three
   absences, stated plainly" (ch18). This is a section template, not an
   author's sentence; bs15 should rule on the shape once (keep the
   parallel paragraphs, drop the meta-count opener and the bold leads,
   or vary the frame) and apply it lane-wide.
3. **The negation-reveal closer.** Sections and chapters end on the
   "not X. It is Y" beat by default (ch12 [328-329], ch13 [334-338],
   ch16 [356-358], ch17 [466-469], ch18 [549-550], ch22 [360-363],
   among the 53 tells). "How does a section end" wants one editorial
   rule in bs15's exemplar chapter — end on the positive claim — rather
   than thirty local rewrites.
4. **The "worth V-ing" marker as the recap connective.** Pattern 4
   above behaves like a template: it appears exactly where a section
   pivots from evidence to takeaway, chapter after chapter from ch14
   on. One rule (delete the marker; state the claim) covers all 19.

## Inputs carried forward

- ch12's headline (em-dash 12.3/1000w; negation-antithesis as default
  rhythm) generalizes: 12.3 turns out to be *below* the lane average.
- ch13's "densest antithesis chapter" title passes to ch22 per word;
  ch13 keeps the tell-density crown and the numeric-triple cadence it
  named recurs in ch18 and ch21.
- ch19/ch20 need nothing from bs15: their notice paragraphs are in
  register and the tense discipline is doing the talking.
- Positive control for bs15's exemplar: ch21's arithmetic-as-argument
  sections and ch22's diagnostic walkthroughs are the register the rest
  of the lane should converge on — fix ch21's punctuation and it reads
  like the charter.
