# The fix register — bs15

The named fixes the sanitation rewrite applies, one entry per systemic
pattern from the three bs14 lane scoreboards (LANE-A.md, LANE-B.md,
LANE-C.md). Each entry states the pattern, its bs14 evidence, THE RULE,
and one worked before/after. The rules are written to be applied by a
different agent on a different chapter without taste: where a rule
offers alternatives it orders them, and the first that reads as a
plain sentence wins. TONE.md is law and is cited, not re-litigated;
the two 2026-08-28 rulings (decision-numbers-are-apparatus; the
rule-sentence register) are applied here as R10.

**The metric, standardized.** The lanes measured dashes three ways:
lane A's per-1000 figures include template dashes, lane B counts them
inside totals but reports them separately, lane C excludes them. The
register adopts lane C's method as the standard: **dash density is
running-prose em-dashes per 1000 prose words** — prose only, fences
and ledger comments stripped, template dashes (R1) excluded. Restate
any lane number in this metric before comparing it to the budget.

**Scope guard.** Code blocks, console blocks, diagnostics, and REPL
transcripts are CI-owned verbatim and untouched by every rule below.
TONE §3 calibration seeds lifted into chapters (identified in the
per-chapter catalogs) are not "fixed". A row the chapter's catalog
defends (severity: noted, keep) is kept; the register never overrules
a catalog defense, only names which rule would have applied.

---

## R1 — the exercise-header template dash

**Pattern.** Every exercise is spelled «**Exercise N-M** *(kind ·
tool)* — stem…», and the back matter repeats the shape: the glossary's
`**term** (§) — definition` (31/31 of its dashes), the solutions
page's three-per-exercise headers (546 of 991), appendix D's `NN —
Title` table.

**Evidence.** 73 em-dashes in lane B alone — 27% of the lane's total —
before a single sentence is written; ~600 book-wide retirable by
punctuation decisions (lane C, SYSTEMIC).

**THE RULE.** One punctuation decision per template, made once, at the
template: the separator becomes a period. «**Exercise N-M** *(kind ·
tool)*. Stem…». The same decision (period, or colon where the right
side is a definition rather than a sentence, as in the glossary)
retires each back-matter template. Template dashes never count against
the R2 budget — and after R1 there are none to count.

**Before.**
> **Exercise 15-2** *(comprehension · lupin)* — `monitor` delivers a
> message; `link` shares fate.

**After.**
> **Exercise 15-2** *(comprehension · lupin)*. `monitor` delivers a
> message; `link` shares fate.

---

## R2 — the running-dash budget

**Pattern.** The em-dash as default connective: hinge-dashes where a
comma, colon, or period serves, and dash-pair insertions where parens
serve.

**Evidence.** 13.3/1000 lane A, 12.7/1000 lane B, 14.0–14.9/1000 in
five lane-C files — the constant is the author voice, not any
chapter's habit. A K&R page runs roughly a third of that.

**THE RULE.** Budget: **at most 5 running-prose em-dashes per 1000
prose words per chapter, and at most one dash-pair per page.** The
budget is a ceiling, not a quota. For every dash, try the substitutes
in order and take the first that reads as a plain sentence:

1. **comma** — the aside is an apposition or a trailing clause;
2. **period** — the clause after the dash stands as a sentence
   (split it; this is the default for the hinge-dash);
3. **parentheses** — a true digression the sentence survives
   without (this is the default for the dash-pair);
4. **colon** — the dash introduces an expansion, a list, or a
   consequence of what precedes it.

A dash survives only when all four substitutions misread — when the
sentence stages an interruption its meaning depends on. Expect one or
two survivors per chapter, not one per paragraph.

**Before.**
> `escalate` is the `Exhausted` return, and it is a policy rather
> than an accident — the level above knows something this level does
> not.

**After.**
> `escalate` is the `Exhausted` return, and it is a policy rather
> than an accident: the level above knows something this level does
> not.

---

## R3 — the negation family

**Pattern.** The reveal-closer mold («It is not X. It is Y», «did not
X. It Y-ed», «not X — it *is* Y») as the default way to end a section;
the negation-antithesis paragraph rhythm; the fronted-negation
flourish («What none of these is, is…»).

**Evidence.** ≈116 instances lane A, 111 lane B, ~80 lane C; the mold
alone ~18–30 per lane; the reveal closes sections by default in every
chapter.

**THE RULE.** State the positive claim first; the negation, if the
reader actually holds the misconception it corrects, follows
subordinated — or goes. Mechanically:

- «It is not X. It is Y.» → «It is Y» (add «, not X» only if X is a
  misreading a reader plausibly arrives with).
- «What none of these is, is X.» → «None of these is X.»
- **Keep the best instance per chapter** — at most one full reveal,
  chosen because the contrast is the section's actual teaching.
- **Semantic contrast that carries content stays.** The test: do X
  and Y name two real designs, behaviors, or measurements (the
  sc-register «per region, not per object» class)? Then it is
  content, not the mold — keep it, in comma form («X, not Y»), not
  reveal form.

**Before.**
> A link does not deliver an error to the linked proc — it *ends* it.

**After.**
> A link ends the linked proc; no error is delivered.

---

## R4 — cross-chapter template sentences

**Pattern.** Whole formula-sentences recurring across chapters as if
pasted from one authoring prompt.

**Evidence** (lane A SYSTEMIC 2, lane B pattern echoes, lane C
cross-file echoes):

| template | sites |
|---|---|
| the measurement credo («this book does not make measured claims without the measurement…») | ch02 ≈ ch03, near-verbatim |
| «Credit where it is due/owed» | ch06, ch08, ch11 |
| «The cost, stated plainly» | ch08, ch11 |
| «N absences, stated plainly» | ch17, ch18 (verbatim frame echo) |
| «The shape to keep/build» | ch06, ch08, ch10 |
| «Neither X is free» | ch01, ch02, ch06 |
| the negative-space opener («notice/count what is not in it») | ch03, ch08, ch10 |
| the 3 a.m. failure trope | ch06, ch08 |
| ch23's package epigram | ch23 ≈ glossary L54–55 |
| ch24's covenant sentence | ch24 ≈ solutions L6561 |

**THE RULE.** One survivor per template, book-wide. The survivor is
the earliest instance whose section the claim is actually about
(proposed survivor list in EXEMPLAR-REVIEW.md; the human may swap
any). Every other instance is rewritten to state its local claim in
local words — not a synonym of the formula. Cross-file echo pairs
(lane C) are fixed in the same sprint or not at all.

**Before** (ch03, the second measurement credo).
> This book does not make measured claims without the measurement:
> chapter 21 runs the comparison, prints the numbers CI produced, and
> names the cases where C still wins.

**After.**
> Chapter 21 runs this comparison and prints the numbers CI produced,
> including the cases where C still wins.

---

## R5 — the «worth V-ing» significance marker

**Pattern.** «worth stating / worth knowing / worth pausing on / worth
memorizing / worth predicting» as the connective wherever a section
pivots from evidence to takeaway; siblings: the self-ranking family
(«the chapter's largest claim», «the sentence to keep»).

**Evidence.** 19 instances lane B (ch14 ×5, ch18 ×5, ch17 ×4, ch15 ×3,
ch16 ×2), ≈15 lane A (five in ch10), plus «worth reading twice» lane
C. It enters the book at ch14 and runs through Part 4.

**THE RULE.** Delete the marker; let the sentence make the claim. No
survivors — the marker is a connective, not content. If deleting it
leaves nothing, the sentence was only the marker: delete the sentence.
An imperative may replace it when the reader is being told to do
something («predict it on paper before running»), never to rank the
prose.

**Before.**
> exercise 15-5 is that run, and the off-by-one in the budget check
> is worth predicting on paper.

**After.**
> Exercise 15-5 is that run; predict the off-by-one in the budget
> check on paper before you run it.

---

## R6 — the cross-reference appendage

**Pattern.** «— §N.M does X» / «which is §N.M's business» bolted onto
sentence ends by dash or relative clause.

**Evidence.** ch01 ×5 rowed, plus ch02, ch03, ch04, ch06, ch10
(lane A SYSTEMIC 3).

**THE RULE.** A cross-reference is parenthetical or it is a sentence.
Either «(§N.M)» / «(the §N.M table)» in place, or its own plain
sentence («§N.M's table names the verb whose defers run.»). Never a
dash appendage. A reference woven into the clause's own grammar
(«§14.2's table says that is the verb…») is prose, not apparatus, and
stays.

**Before.**
> "who is supposed to notice if this dies" is a question with a code
> answer — the supervisor that spawned it, in the loop you can read.

**After.**
> "who is supposed to notice if this dies" is a question with a code
> answer (the supervisor that spawned it, in the loop you can read).

---

## R7 — the dash-pair-enclosing-a-list

**Pattern.** «X — a, b, c — Y»: a list interrupting a sentence between
two dashes.

**Evidence.** notation.md ×3 (the template's origin), ch02, ch03 ×2,
ch10 ×2 (lane A SYSTEMIC 4); the four-item survivor list in ch15.

**THE RULE.** Lists go in parentheses, after a colon, or in their own
sentence — never between dashes. Order of preference: parentheses if
the sentence must not stop; colon if the list is the sentence's
payload; a new sentence if the list has four or more items and the
frame sentence survives without them.

**Before.**
> If the survivor has something useful to do — retry, reroute,
> degrade, log and carry on — the failure is information.

**After.**
> If the survivor has something useful to do (retry, reroute,
> degrade, log and carry on), the failure is information.

---

## R8 — the section-close maxim cadence

**Pattern.** Sections (and many paragraphs) landing on an aphorism,
epigram, or balanced chiasmus by default («one less X, one more Y»;
«a supervisor that cannot fail cannot be supervised»).

**Evidence.** 30+ rowed instances lane A; 5 chiasmus closers rowed in
lane B (ch15, ch17, ch18); TONE §3 models the device once per passage,
the book deploys it as the default landing gear.

**THE RULE.** Per chapter: **at most half the sections end on a maxim,
and at most one balanced aphorism/chiasmus closer survives** — the one
that is the section's checkable thesis, not its ornament. The test for
the survivor: is the sentence a claim the chapter argued and a reader
could falsify («the budget makes the supervisor able to fail»), or a
symmetry manufactured for the cadence («one less pattern to get right,
one more channel to declare»)? Every other section ends on the
concrete fact, instruction, or program — the plain sentence that was
previously second-to-last usually serves.

**Before.**
> wolf's `monitor` hands back a channel, so the exit arrives on its
> own route and a `select` arm can wait on it without the service's
> other traffic interleaving. One less pattern to get right, one more
> channel to declare.

**After.**
> wolf's `monitor` hands back a channel, so the exit arrives on its
> own route and a `select` arm can wait on it without the service's
> other traffic interleaving; the cost is one more channel to declare.

---

## R9 — schedule narration (charter class)

**Pattern.** Prose that teaches the gap between book and toolchain:
«yet», «today», «at this printing», «has (not) landed», «the pinned
interpreter», «until X lands».

**Evidence.** Ten charter rows, lane A: ch05 ×9, ch02 ×1. The lane's
only systemic charter defect.

**THE RULE.** TONE's tense discipline, applied as editorial policy:
state scope, never schedule. Each instance is rewritten as a
present-tense fact about the product («v1 does not do X»; the
per-machine note names which machine runs the sample, not when the
other will) or deleted, and the gap it taught goes to the audit
ledger, where "not yet" lives. A section that cannot be written
without the gap is a section whose gate has not opened; it reverts to
draft rather than keeping the narration. No new prose may name a
sprint, a pin, or a landing.

**Before** (the class's shape; ch05 carries nine).
> The compiler lane does not run this program yet; until the
> interpreter's char work lands, use lupin.

**After.**
> lupin runs this program. *(The gap moves to the ledger row, at full
> volume.)*

---

## R10 — the two rulings, applied mechanically

**Pattern & evidence.** (a) D/X decision-number citations in
reader-facing prose (rowed by ch03, ch07, ch10 catalogs as the
boundary question). (b) Bolded sentences in running prose beyond the
ratified set, often introduced by «the rule is one sentence:» (lane A
SYSTEMIC 5; 13 bold-emphasis instances lane C).

**THE RULE.** TONE §1 and the tense discipline, verbatim — cited, not
re-litigated:

- **Decision numbers are apparatus.** State the decision's content in
  the sentence and drop the citation: «wolf has no lifetimes by
  design», never «per D28». «CI» is engineering vocabulary and stays.
- **Bold in running prose marks a ratified rule-sentence, nothing
  else.** The five blessed sentences (ch04, ch07, ch08 ×2, ch09) keep
  bold. Every other running-prose bold is demoted: a chapter's
  organizing *question* goes to italics (TONE §3.6's precedent); an
  emphasized claim goes to plain text and must carry itself. A
  candidate sixth rule-sentence is filed for amendment, never bolded
  locally. Structural bold — exercise labels, run-in paragraph
  headers, «Coming from X:» — is layout and stays.
- Every «the rule is one sentence:» signpost is cut. The bold is the
  signpost.

**Before.**
> **Is this failure information, or is it fate?** If the survivor has
> something useful to do…

**After.**
> *Is this failure information, or is it fate?* If the survivor has
> something useful to do…

---

## R11 — the count-led parallel section (supplementary)

**Pattern.** The count-then-bold-led-parallel-paragraphs shape: «Walk
the four decisions…», «Three things the rule buys… **Builds.**
**Comprehension.** **Interfaces.**», the «N absences, stated plainly»
frame (whose sentence-form is R4's row).

**Evidence.** Lane B SYSTEMIC 2: ch14, ch15, ch18, ch22, plus the
ch17→ch18 verbatim frame echo; lane C's measurement-section shape
(ch26, ch28, ch32).

**THE RULE.** The shape stays; the stage direction goes. Run-in bold
leads over parallel paragraphs are layout (R10 keeps them). The
opener states the count at most once, as a plain sentence, without
the walk/notice/count-with-me imperative and without restating the
count's significance. The count must be real (it is, usually — the
paragraphs are countable).

**Before.**
> Walk the four decisions in it, because they are the four decisions
> every supervisor makes and no library removes them:

**After.**
> Four decisions are in it, and no library removes them:

---

## Reconciliations (where the scoreboards disagree)

- **Dash metric:** lanes measured template dashes inconsistently;
  standardized above on lane C's running-prose-only method. Lane A's
  per-1000 column overstates against the R2 budget by the template
  share; recompute before fanning out.
- **«worth V-ing» totals:** lane A says ≈15, lane B says 19, the bs15
  contract says 19+; the counts are per-lane, not conflicting — the
  book-wide total is the sum (~35+ with lane C's siblings). The fix
  (R5, delete all) does not depend on the count.
- **Negation keep-policy:** lane B says «keep the best instance per
  chapter»; lane A rows some instances as content-defensible
  corrections. R3 merges both: one reveal survivor per chapter by
  choice, unlimited semantic contrasts by test.
- **Bold rule-sentences:** lane A filed the question (ratify or
  unbold); TONE's 2026-08-28 amendment answered it. R10 applies the
  answer; the register does not reopen it.
- **ch21:** lane B notes it is nearest the register in prose and
  furthest in punctuation — evidence that R2 alone carries most of a
  chapter's convergence; the fan-out should not over-rewrite
  low-tell chapters.
