# back/solutions — tells audit (bs14, lane C)

**Method note.** 7564 lines, 223 exercises, ~31,700 prose words. Audited
by whole-file mechanical counts (em-dashes, negation-family greps,
banned-vocabulary sweep, curly quotes, bold-sprinkle) plus close reading
of roughly a fifth of the discussion prose sampled across all chapters.
Code blocks, console blocks, and diagnostics excluded throughout.
Exercise stems are quoted chapter text and are audited in their
chapters' catalogs, not re-cataloged here.

**Counts:** charter 0 · tell 5 · noted 3
**Em-dashes (prose only):** 991 total. 546 are template: three per
exercise — the `<summary>Exercise N-M — [§…]` line, the
`**Exercise N-M** *(class · lane)* —` stem lead, and the
`Solution — `chNN/exN-M.lu`:` header. Running-prose dashes: 445 in
~31,700 words = 14.0/1000 — exactly the lane's chapter constant.
**Negation-family count:** 12 strict-form «X is not A; it is B» matches
by grep (lines 2849, 3228, 3363, 3638, 3791, 4176, 4764, 5947, 6429,
6561, 6859, 7258); the looser family (fragments, "rather than" ×34) was
sampled, not exhaustively hand-counted.
**Banned vocabulary:** none. No curly quotes; no stray bold outside the
exercise-stem template.
**Verdict:** The solutions speak in the chapters' exact voice — same
14/1000 dash density, same "not A; it is B" mold, plus a
generalizing-moral closer formula of their own — so every systemic fix
bs15 makes to the chapters has a mirror-image edit waiting here, and the
template dashes are one decision, not 546.

## Findings

- [whole file] the three-per-exercise template dash (summary line, stem
  lead, solution header) — 546 of the file's 991 dashes are one
  template decision; SYSTEMIC — a colon in the `Solution —` header alone
  retires 224 in one edit; severity: noted; suggestion: decide the
  template's punctuation once in bs15, out of band of the prose pass.
- [whole file] 445 running-prose em-dashes at 14.0/1000 — the identical
  density to ch23/24/26/27 marks the habit as the author-voice constant,
  not any one file's; severity: tell; suggestion: the same paired-aside
  pruning the chapter catalogs sketch.
- [2849, 3228, 3363–3364, 3638, 3791, 4176, 5947–5949, 6429, 6561, 6859]
  «UB is not a punishment; it is a price list.» / «the door is not a
  slightly risky shortcut; it is the one thing…» / «The leak is not
  fixed; it is *retired*» / «Buffer size is not a tuning knob here; it is
  part…» / «The dump is not a stack sample; it is the ownership tree…» /
  «The bug is not detected; it is unspellable» / «The refusal is not a
  missing feature; it is the load-bearing wall…» — the «X is not A; it
  is B» sentence mold, eleven-plus times across the file, several with
  emphasis italics on the B-term; individually many are sharp,
  collectively they are the decree-named construction as a house style;
  severity: tell; suggestion: keep the two or three best (the price-list
  line earns its place), recast the rest positively.
- [6561–6562] «The covenant is not a policy document; it is this
  rejection, emitted before anything runs.» — near-verbatim reprise of
  ch24 L85–86 («The covenant is not a policy document. It is enforced in
  three places»), the same flagged sentence shape echoing across files;
  severity: tell; suggestion: whichever form ch24 keeps, this one should
  not duplicate it.
- [2632, 5173, 5200, 5950] «The pattern under all four: …» (verbatim
  twice) / «That is the odd one out: …» / «The distinction to hold onto:
  …» — a generalizing-moral closer formula ending discussion solutions;
  with the fourteen "…is the point" closers (e.g. L1234, L6076, L7095,
  L7139) it is the file's own template-shaped tell; severity: tell;
  suggestion: end discussion solutions on the concrete fact; save the
  moral for the one exercise per chapter that earns it.
- [6427–6430] «`wolf why` for a name that is not there is not a usage
  error — the command was spelled correctly — so it is not exit 2; it is
  a *finding*, and exit 1 is what a finding gets.» — triple negation +
  em-dash pair + emphasis italics in one sentence; severity: tell;
  suggestion: "`wolf why` for an absent name is a finding, not a usage
  error, and exit 1 is what a finding gets."
- [7227–7233] «it may segfault, it may print garbage, and on some
  targets it may appear to work» … «That is the difference the whole
  book is about, arriving in a two-line function.» — anaphoric triple
  (the three are real UB outcomes — defended) closing on editorialized
  significance; severity: noted; suggestion: "That difference is the
  book's subject, in a two-line function" — or end on the byte span.
- [1232–1234] «what has to change, and what — pleasantly — does not?» /
  «Solution — the third impl is three lines, and that is the point:» —
  dash-wrapped adverb aside plus an "is the point" closer in one
  exercise; severity: noted; suggestion: "what changes, and what does
  not?"

## bs16 — dispositions (lane S)

Sequenced after the chapter lanes merged: the mirror follows the
originals. Solutions is generated (`cargo xtask backmatter`), so every
edit below lives in the masters (`principles/EXERCISES.md` §5,
`principles/exercises/*/EXERCISES.md`) or in the generator, and the
page was regenerated after each tranche. Fence content byte-identical
throughout (verified against HEAD per file); sample gates 399/0 held.

- [whole file, template] **bs16: fixed-by-R1** — one decision, period,
  at all three shapes: the generated `<summary>` line (backmatter.rs,
  with verify-docs' marker updated in lockstep), 275 stem leads and
  109 `Solution —` headers across the masters, plus 2 dangling
  stem-lead dashes found later (18-3, 18-11). Solutions template
  dashes 546 → 0; masters 374 → 0. The appendix-D period precedent
  followed; the glossary's colon stands where lane C put it.
- [whole file, 445 running at 14.0/1000] **bs16: fixed-by-R2** — the
  ladder, per chapter, mirror-wide: 445 → 0 running-prose dashes in
  33.3k prose words (0.0/1000w). Zero is the ladder's outcome, not a
  quota: every site read plainly under one of the four substitutes,
  and the back matter is deadpan reference (the appendices' precedent);
  the reveal survivors below carry the interruptions that earn keeping.
- [2849, 3228, 3363–3364, 3638, 3791, 4176, 5947–5949, 6429, 6859]
  **bs16: fixed-by-R3** — recast positive-first, ≤1 survivor per
  chapter kept where the contrast is the teaching: ch09 price-list
  (the catalog's own defense), ch10 leak-retired, ch11 ownership-tree,
  ch12 unspellable, ch15 shared-fate, ch18 load-bearing wall, ch23
  versions-forever, ch30 collector-owner.
- [6561–6562] **bs16: fixed-by-R4** — the covenant echo localized:
  «The rejection lands before anything runs, which is what the
  covenant promises.» ch24's chapter keeps the surviving sentence
  («The covenant is enforced in three places…»); no «policy document»
  phrasing remains on this side.
- [2632, 5173, 5200, 5950 + the "is the point" closers] **bs16:
  fixed-by-R8/R5** — both «The pattern under all four:» sites and the
  odd-one-out/distinction leads dissolved into plain answers; "is the
  point" 14 → 0; discussion solutions end on the concrete fact, ≤1
  earned closer per chapter (e.g. ch06 promises-small-and-kept, ch28
  brace-halves). worth-markers deleted throughout (content uses of
  "worth" stay: ch14 cache-state, ch09 trustworthy).
- [6427–6430] **bs16: fixed** — bs14's suggested sentence adopted
  verbatim; the marker in the same passage deleted.
- [7227–7233] **bs16: fixed** — suggestion adopted («That difference
  is the book's subject, in a two-line function.»); the anaphoric
  triple stays as defended content.
- [1232–1234] **bs16: fixed** — "what changes, and what does not?";
  the "is the point" closer cut.

**Mirror policy applied.** The lanes' chapter rewrites had moved 116 of
223 printed stems out from under their masters. The 159 stems that were
verbatim mirrors before bs16 re-adopted the settled chapter text
verbatim (91 in the sync commit, a handful of fence-spanning ones by
hand); the 59 natively adapted stems (chapter paraphrase vs exercise
page, a pre-bs16 design) keep their form, dash-laddered only. Where a
stem the chapter owns carries "rather than" (ch21 ×2), the mirror does
not diverge from the original.

**Also in this pass.** R9 sweep of published prose: "today"/tracker
narration out (ch03 audit note, ch07 verdict-day, ch12 pin note, ch13
pending parenthetical deleted as printed apparatus, ch23/ch26/ch27/ch30
today-forms restated as present scope); the one remaining prose "yet"
(ch09 malloc bytes "not a value yet") is semantic, kept. "rather than"
rationed to ≤2 per chapter in solution prose. One stray bold
(ch24 confinement) demoted per R10. The generator also stops `### `
repo headings leaking into five published blocks (harvest's stated
intent, enforced). The ch05 master's §5.5 heading follows the
graduated chapter heading («Traits»). The glossary's ch23-epigram echo
was verified settled by lane C — no lane-S work.

**Out of scope, noted.** ch19/ch20/ch25 masters (unprinted — nothing
published to register); file preambles, §1–4 doctrine, §6/§8 apparatus,
EXERCISES-INDEX/PENDING (repo-facing; §8's `**term** — definition`
bullet template ~10 members is a one-decision class if apparatus is
ever registered); the pending-kind annotation's internal «· pending —
blocker:» dash (unprinted template shape, no decision taken).

**Escalations.** (1) verify-docs is RED at trunk 66767a5 before and
after this lane: book/ch10.md L146 "every task has an owner: you name
a scope…" trips the "owner:" substring check (TONE tense-discipline
probe) — lane A prose, outside lane S scope; either the sentence or
the probe needs the fix. (2) Solutions' book-wide 0.0/1000w is below
the ≤5 budget by ladder outcome; if the human prefers visible
survivors in the mirror, the eight R3 reveals are where the voice
lives.
