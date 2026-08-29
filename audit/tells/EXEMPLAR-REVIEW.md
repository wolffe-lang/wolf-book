# bs15 exemplar review — ch15 against the register

For the human's review. The sprint ends here: bs16 (the fan-out) does
not start until this page gets a yes. Full rules in REGISTER.md; full
dispositions in ch15.md's catalog; the diff is `git diff trunk..bs15 --
book/ch15.md`.

## The register in one screen

| # | pattern | the rule |
|---|---------|----------|
| R1 | exercise-header template dash (73 = 27% of lane B; ~600 book-wide) | one punctuation decision per template: the separator becomes a period |
| R2 | running-dash constant (~13–14/1000w, every lane) | budget ≤5/1000w running prose, ≤1 dash-pair/page; substitute in order comma → period → parens → colon; a dash survives only if all four misread |
| R3 | negation family (~300 book-wide; the "not X. It is Y" reveal mold) | positive claim first; ≤1 reveal survivor per chapter; semantic contrast that carries content stays, in comma form |
| R4 | cross-chapter template sentences (measurement credo, "Credit where it is due", "N absences, stated plainly", 3 a.m., …) | one survivor each, book-wide; every echo rewritten in local words |
| R5 | "worth V-ing" evidence→takeaway connective (19+ lane B, ≈15 lane A) | delete the marker, no survivors; an imperative may replace it |
| R6 | "— §N.M does X" cross-ref appendage | parentheses or its own sentence; woven grammar stays |
| R7 | dash-pair-enclosing-a-list | parens, colon, or own sentence — never dashes |
| R8 | section-close maxim cadence; balanced-aphorism closers | ≤ half the sections end on a maxim; ≤1 chiasmus survivor per chapter (the falsifiable one) |
| R9 | schedule narration (ch05 ×9, ch02 ×1 — charter) | scope, never schedule; the gap goes to the ledger; a section that needs the gap reverts to draft |
| R10 | the two 2026-08-28 rulings | D/X numbers out of prose; only the five blessed rule-sentences bold; every "the rule is one sentence:" signpost cut |
| R11 | count-led parallel sections ("Walk the four decisions…") | shape and bold run-in leads stay; the stage-direction opener goes |

## ch15 before / after

| metric (prose only; fences + ledger stripped) | before | after |
|---|---|---|
| prose words | 2070 | 1968 |
| em-dashes, total | 29 (14.0/1000w) | 0 (0.0/1000w) |
| em-dashes, running prose | 20 (9.7/1000w) | 0 |
| em-dashes, exercise-header template | 9 | 0 (R1) |
| negation family, hard | 10 | 6 |
| "not X. It is Y" reveal mold | 4 | 0 |
| tell-severity findings | 7 | 0 |

Kept, with the catalog's defenses: the compressor/uploader mirrored
antithesis (the primitive's real symmetry), «a link is not convertible
into one», «a supervisor that cannot fail cannot be supervised» (the
chapter's one R8 chiasmus), the telephone-game sass line, «A service
that appears in neither list does not exist», «the `stdout=` clause,
not the exit code» (comma form). Code, console, and REPL blocks:
byte-identical (verified by diff and by the sample rig).

## The five most instructive hunks

**1. The reveal mold becomes a plain law (R3), and the bolded question
is demoted, and the dash-pair list goes to parens (R10, R7)** — three
rules in one passage, which is why it leads:

> −A link does not deliver an error to the linked proc — it *ends* it.
> +A link ends the linked proc; no error is delivered.
>
> −So the choice between the two is not a matter of taste, and it is
> −not "link is a monitor with a shorter handler." It is a question you
> −can answer about any pair of procs in your design:
> +The choice between the two answers one question about any pair of
> +procs in your design:
>
> −**Is this failure information, or is it fate?** If the survivor has
> −something useful to do — retry, reroute, degrade, log and carry on —
> +*Is this failure information, or is it fate?* If the survivor has
> +something useful to do (retry, reroute, degrade, log and carry on),

**2. The balanced epigram becomes a cost statement (R8)** — the box
still ends on the trade-off; it stops chanting it:

> −…without the service's other traffic interleaving. One less pattern
> −to get right, one more channel to declare.
> +…without the service's other traffic interleaving; the cost is one
> +more channel to declare.

**3. The "worth V-ing" marker becomes an imperative (R5)** — the
reader is told to do the thing instead of being told it ranks:

> −…at exit 1 — exercise 15-5 is that run, and the off-by-one in the
> −budget check is worth predicting on paper.
> +…at exit 1. Exercise 15-5 is that run; predict the off-by-one in
> +the budget check on paper before you run it.

**4. The stage direction goes, the count stays (R11)** — and the four
bold run-in decision paragraphs below it are untouched (layout, per
TONE):

> −Thirty-two lines including the child and the driver. Walk the four
> −decisions in it, because they are the four decisions every
> −supervisor makes and no library removes them:
> +Thirty-two lines including the child and the driver. Four decisions
> +are in it, and no library removes them:

**5. What restraint looks like** — the negation that carries content
survives in comma form (R3), losing only its dashes (R2):

> −Explain why the `stdout=` clause — not the exit code — is the part
> +Explain why the `stdout=` clause, not the exit code, is the part
> of this header that actually verifies the rule…

And unflagged prose is untouched: «Two shapes of link, because fate
has two shapes», the §15.2 opener's «There is no supervisor in the
language», and every calibration-adjacent sentence stand as written —
the diff is rules applied, not an agent's taste.

## Open calls for the human

1. **The R1 separator, and its home.** ch15 now reads «**Exercise
   15-1** *(comprehension · lupin)*. Stem…» while the other 27
   chapters, EXERCISES.md's corpus, and the generated solutions page
   keep the dash until bs16. Ratify the period (or name different
   punctuation) — and note EXERCISES.md and the solutions/glossary/
   appendix-D templates are outside bs15's write scope, so the
   decision rides into bs16.
2. **The R2 budget number.** ≤5/1000w running prose (K&R runs about a
   third of the lanes' 13–14). Bless or adjust.
3. **ch15 landed at zero dashes.** Every one of its 20 running dashes
   substituted cleanly under the hierarchy; none met the survivor bar.
   If a zero-dash chapter reads overcorrected to you, name the hunk
   where the dash should return — that calibrates the survivor bar for
   bs16 better than any rule text.
4. **The R4 survivor list.** Proposed: measurement credo → ch02;
   «Credit where it is due» → ch06; «The cost, stated plainly» → ch08;
   «absences, stated plainly» → ch17; «The shape to keep» → ch06;
   «Neither X is free» → ch01; negative-space opener → ch03; 3 a.m. →
   ch06; package epigram → ch23 (glossary echo rewritten); covenant
   sentence → ch24 (solutions echo rewritten). ch15 exercises none of
   these, so the exemplar does not test R4 — approve the list on its
   face or swap survivors.
5. **The reveal-mold survivor allowance went unused.** R3 allows one
   reveal per chapter; ch15's four were all rowed as tells with
   suggested rewrites, so none survived. Confirm the allowance reads
   "at most one", not "exactly one".
6. **«rather than» ×3 → ×4.** The [377] fix adopts the catalog's own
   suggested wording («finished rather than misbehaving»). If the
   frame should be rationed too, say so and R3 gains a bullet.
7. **The demotion target for non-blessed bold.** The chapter's
   organizing question went to italics on TONE §3.6's precedent
   (emphasized *claims* go to plain text). Confirm for bs16, which
   hits 13 such bolds in lane C alone.
8. **Exemplar scope.** The contract lets you name a different chapter
   at review; ch15 exercised R1–R3, R5, R7, R8, R10, R11 (R4, R6, R9
   have no ch15 instances — R6's only candidate was already woven
   grammar). If you want R4/R9 proven before fan-out, ch05 (nine
   schedule-narration rows) is the natural second exemplar.

## Sample checks

Prose-only diff (no fenced line touched). The rig was run at the
pinned toolchain (lupin 0.1.14 at a3591de, wolf at a900b8c, built for
this run): failure sets before and after the rewrite are identical,
and every failure is environmental to this host — `wolf build` targets
linux/x86-64 only, so the compile-lane console replays cannot run on
this aarch64-darwin machine; ch15's own samples (all lupin-lane) pass.
CI's linux runners hold the real gate and see a prose-only diff.

---

## Rulings (2026-08-28, human — via the orchestrator's recommendations)

1. R1 period: RATIFIED. 2. The 5/1000w budget: BLESSED. 3. ch15's
zero stands; the budget (not zero) is the fan-out target — a dash
returns where all four substitutes misread. 4. R4 survivor list:
APPROVED as proposed. 5. R3 "at most one": CONFIRMED. 6. "rather
than" RATIONED ≤2/chapter; any connective multiplying under rewrite
gets the same cap (R5 extended). 7. Non-blessed bold demotes to
PLAIN PROSE, not italics (R10 amended). 8. Second exemplar: YES —
ch05 (R4/R6/R9) precedes the bs16 fan-out.

---

# Second exemplar — ch05 against the register (bs15-ch05)

Ordered by ruling 8: ch05 exercises what ch15 structurally could not —
R9 above all (nine of the book's ten schedule-narration charter rows),
R6's dash/relative-clause cross-reference appendages, and the R4
question (on which see the correction below). Full dispositions in
ch05.md's catalog; the diff is `git diff trunk..bs15-ch05 --
book/ch05.md`.

## ch05 before / after

| metric (prose only; fences + ledger stripped) | before | after |
|---|---|---|
| prose words | 2503 | 2214 |
| em-dashes, total | 39 (15.6/1000w) | 1 (0.5/1000w) |
| em-dashes, running prose | 29 (11.6/1000w) | 0 (0.0/1000w) |
| em-dashes, exercise-header template | 9 | 0 (R1) |
| em-dashes, verbatim tool quote in code span | 1 | 1 (untouchable) |
| schedule-narration charter rows | 9 | 0 (R9) |
| negation family, rowed | 6 | 3 (all catalog-defended) |
| reveal mold («was a schedule. It is now a fact») | 1 | 0 |
| «rather than» | 6 | 2 (the ration) |
| «honest» motif | 5 | 1 |
| tell-severity findings | 4 | 0 |

The surviving em-dash is the compiler's own note quoted verbatim in a
code span (`a temporary has no home — bind it first.`): tool voice,
CI-adjacent, not prose punctuation. As in ch15, every running dash
substituted cleanly; none met the survivor bar (the budget amendment
noted — if a hunk reads overcorrected, name it and the dash returns).

## R9 — the verification protocol and its results

The register's rule is scope-not-schedule, but this lane's brief added
a step the ch15 exemplar never needed: VERIFY REALITY FIRST, because
ch05's deferral prose was written when features were absent and the
toolchain has moved. Every gap was probed at the book's pin (lupin
0.1.14 at a3591de, wolf at a900b8c, built for this run) before its
prose was touched:

| deferral taught | probe result at the pin | disposition |
|---|---|---|
| `Set` absent ([183], [186-209]) | still absent (`unsupported`, exit 4) | scope statement: «Part 1's std has no set container» |
| combinators absent ([251]) | still absent (`sorted_by` unsupported) | scope statement: «Part 1's std has one combinator, `count()`» |
| loop-as-concession ([258]) | (same gate) | pedagogy: «we write the loop, because the loop is what the chain means» |
| traits «had not landed» ([419-424]) | LANDED — §5.5's trait fences run | history deleted; teaching choice stated in the present |
| §5.5 «was a schedule. It is now a fact» ([552-554]) | LANDED | opens on the material; heading «Traits: the system lands» → «Traits» |
| `n.text()` sugar on primitives ([598-600]) | still absent, BOTH machines | scope: «does not reach a primitive receiver» |
| container-`dyn` ([675-677]) | absent — `List[dyn Draw]` is E0201 on both machines; no D-number = not stateable as scope | sentence deleted; new ledger row |
| `Eq`/`Ord` operator dispatch ([679-683]) | absent — lupin's struct `==` is structural (an inverting `impl Eq` is never consulted); wolf refuses | cut to the boundary fact; new ledger row |

One healing found where the prose had no deferral left to fix: the
QUALIFIED `Text.text(n)` on a primitive receiver — the reason the
§5.5 fence runs on the wolf-run lane — now resolves under lupin 0.1.14
(prints `int(7)`, exit 0). Recorded in the fence's audit comment;
graduating the fence to `run()` is fence-authority work (bs16).

Nothing needed the flag-instead-of-fake escape: every still-absent
feature was honestly stateable as scope, so no passage reverts to
draft.

## R6 — the worked evidence

- «which is a deliberate economy and §5.3's subject» → its own
  sentence: «The economy is deliberate, and §5.3 returns to it.»
- the §3.1 aside in a dash pair → parentheses: «(`(mut cents).push(340)`
  is §3.1's "this call writes `cents`", in its natural habitat now)».
- «hand the emptiness back as a value, which is chapter 6» →
  «(chapter 6's subject)».
- Woven grammar kept, per the rule: «the linear scan §5.1 will have a
  word about», «the way §4.4's returns do», «the same definition-site
  honesty §5.3 taught for generics».

## R4 — a correction to the ordering premise

ch05 hosts NO instance of the ten ratified R4 templates — the
register's own evidence table places the measurement credo at
ch02 ≈ ch03, and a grep of ch05 for all ten confirms none lands here.
What ch05 does host is template-KIN at motif grain, and those were
rewritten in local words under their own rows: the cost-ledger frame
(«The cost is real and somebody pays it» → the plain cost statement)
and the honest-motif tic (×5 → ×1). The consequence for the sprint:
R4 proper (choosing a survivor, rewriting the echoes) is still
unexercised by any exemplar, and its first true sites are bs16's
(ch02/ch03 and the table's other pairs). The survivor list rides as
ratified; no exemplar evidence contradicts it.

## Open calls for the human

1. **The three refusal fences (§5.1 set.lu, §5.2 chain.lu, §5.4
   bump.lu).** TONE says scaffold output is never a section's
   load-bearing sample, and the [186-209] charter row says drop it —
   but fences are CI-owned and this lane's contract (prose only,
   byte-identical failure sets) rightly refuses me the authority. The
   prose around them now states scope without teaching the refusal,
   which is as far as prose can carry it. bs16 needs a ruling: retire
   the refusal fences (and re-teach those passages product-first), or
   defend them as differential-lane evidence.
2. **The §5.5 heading rename** («Traits: the system lands» →
   «Traits») leaves principles/exercises/ch05/EXERCISES.md's §5.5
   header echoing the old name — outside bs15 write scope, same
   boat as the R1 corpus decision (open call 1 of the first packet);
   rides into bs16.
3. **The healed qualified call** (lupin 0.1.14 runs `Text.text(n)` on
   an `int`): the §5.5 wolf-run fence can graduate to `run()` and its
   per-machine comment retire. Fence authority = bs16.
4. **R4 remains exemplar-untested** (see correction above). Either
   bless the survivor list on its face a second time and let bs16's
   ch02/ch03 lanes be the proof, or order a third (thin) exemplar on
   ch02 §measurement-credo before fan-out. Recommendation: the
   former — the list is already ratified and the rule is mechanical.

## Sample checks

Prose-only diff (no fenced line touched; fence blocks byte-identical
by extraction diff). The rig ran at the pinned toolchain before and
after the rewrite: 34 failures both runs, failure sets byte-identical,
every failure environmental to this aarch64-darwin host (`wolf build`
targets linux/x86-64 only). ch05 contributes exactly one, its wolf-run
fence; every ch05 lupin-lane sample passes. CI's linux runners hold
the real gate and see a prose-only diff.
