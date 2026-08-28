# bs14 tells audit — front matter (lane A)

Files: `book/front/how-to-read.md`, `book/front/notation.md`, `book/SUMMARY.md`.
Line numbers are raw file lines. Prose only; fenced blocks skipped.

---

## front/how-to-read.md

**Counts:** charter 1 · tell 1 · noted 2.
**Em-dashes:** 5 in 621 prose words = 8.1/1000 — moderate; one pair per
paragraph maximum is respected.
**Negation family:** 0 strict instances ("not X, but Y" and kin absent).
**Banned vocabulary:** none (verified by eye; grep also zero).
**Verdict:** close to the K&R register — the guide voice earns its dry lines
but stacks them past the sass budget in the on-ramp section, and one closing
sentence runs to a subordinate pileup.

- [22–46] «You will be at home in part 1 and ambushed in part 2» / «the first
  place the language declines to guess on your behalf» / «Part 1 is a
  weekend» / «Chapter 9 is the one you came for» — sass density: four wry
  clauses inside roughly one page; TONE §2 budgets one per page, most pages
  none; severity: charter; honesty: each line alone is dry, true, and
  information-bearing — the finding is the stack, not any single sentence;
  suggestion: keep the best one per audience paragraph, flatten the rest.
- [53–56] «part 5 builds programs — spending what the earlier parts taught
  and introducing nothing new, which is the point: by the time you reach it
  the language is one you already know, and none of the projects is a
  trick.» — subordinate-clause pileup (dash + relative clause + colon
  expansion in one sentence); reflexive em-dash where a comma serves;
  severity: tell; suggestion: split after "nothing new." and let "None of
  the projects is a trick" stand as its own sentence.
- [27, 43] «rather than a syntax to learn» / «rather than left to you» —
  "X rather than Y" contrast frame, twice in one short file and recurring
  across the front matter (see notation.md); severity: noted; each instance
  reads fine alone — recorded for the lane-wide pattern count.
- [29–30] «the two places it is not are worth your attention» — mild
  editorialized significance ("worth your attention" tells the reader to
  care rather than showing why); severity: noted; suggestion: "the two
  places it is not are these:" and let the two places argue for themselves.

## front/notation.md

**Counts:** charter 0 · tell 2 · noted 4.
**Em-dashes:** 12 in 747 prose words = 16.1/1000 — the densest real prose in
the front matter; three of the twelve are one recurring template shape (see
second row).
**Negation family:** 0 strict instances. ([116–117] «They are drawings, not
code» is the affirmative-first "X, not Y" direction — defensible, not
counted.)
**Banned vocabulary:** none (verified by eye).
**Verdict:** vocabulary-clean and mostly at register, but the em-dash is this
file's default connective, and one dash-pair-around-a-list shape repeats
often enough to read as a template.

- [55–57, 115, 140–142] «CI derives every line of it — the command, the
  output, the exit status — from that program's declared case» / «Figures —
  ownership trees, tables of boxes and arrows — are set» / «something of its
  own to say — a diagnostic, a binary, a measurement — it says it» —
  em-dash pair enclosing an asyndetic three-item list, three times in one
  file; the same shape opens how-to-read.md line 8; severity: tell;
  honesty: each list enumerates real things — the tell is the identical
  rhythm, not the content; suggestion: keep one, recast the others with
  parens or a plain "such as" clause. SYSTEMIC candidate.
- [123–124] «Tool output is pasted from real runs and never edited — no
  elisions, no retyped error messages, no tidied spans.» — rule-of-three
  rhetorical amplification plus reflexive dash; severity: tell; honesty:
  every claim in the triplet is real and CI-checkable; the rhythm, not the
  facts, is the tell; suggestion: "Tool output is pasted from real runs and
  never edited: nothing is elided or retyped."
- [7–10] «A code block carries a small label at its top edge, set in the
  block's accent over its own left rule and ground tint — read the label and
  you know who is speaking before you read a line of the code.» — reflexive
  em-dash splicing two full sentences; severity: noted; suggestion: full
  stop after "ground tint."
- [66–68] «which is the rule about editing nothing doing its job — the
  compiler had two things to say about that program and the page shows
  both» — dash where a colon serves; severity: noted; suggestion: colon.
- [144–145] «Exit codes recur often enough to be worth memorizing early —
  these are the ones behind the book's `$ echo $?` lines.» — dash where a
  colon or period serves; severity: noted; suggestion: period, then "These
  are the ones behind…".
- [57, 139, 153–154] «a measurement rather than a recollection» / «what a
  program *means* rather than what it costs» / «leaves a tombstone rather
  than renumbering its neighbors» — "X rather than Y" contrast frame, three
  more instances (five across ~1,400 front-matter prose words); severity:
  noted; recorded for the lane-wide pattern count.

## book/SUMMARY.md

**Counts:** charter 1 · tell 0 · noted 2.
**Em-dashes:** 9 in 201 words = 44.8/1000 nominal — all nine are structural
title separators (`# Part 1 — Foundations`, `Appendix A — Grammar summary`);
zero in running prose; the density number is an artifact and not a finding.
**Negation family:** 0 strict instances.
**Banned vocabulary:** none.
**Verdict:** a table of contents with almost no prose surface; titles are at
register except one marketing-flavored promise.

- [12] «Collections and generics without fear» — marketing promise in a
  title (a claim of merit made by neither a program nor a measurement; TONE
  §1 deletes marketing adjectives on sight); severity: charter; honesty:
  reads as a knowing allusion to Rust's "fearless" branding, and the chapter
  may cash the claim — flagged for the human call; suggestion: "Collections
  and generics".
- [19] «The escape hatch is a door, not a cliff» — contrast-frame title;
  severity: noted; honesty: affirmative-first "X, not Y" is the acceptable
  direction of the family, and the image is the chapter's actual thesis —
  defensible as is.
- [9, 37] «Strings, honestly» / «Beating C honestly» — the same earnestness
  adverb carrying two titles; severity: noted; suggestion: keep the stronger
  one (ch21's, where honesty names the benchmark discipline) if either must
  yield.

---

**Front-matter totals:** charter 2 · tell 3 · noted 8. Negation family: 0
strict instances across all three files. Banned vocabulary: zero hits.
