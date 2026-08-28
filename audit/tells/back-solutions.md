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
