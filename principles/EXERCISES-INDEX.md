# EXERCISES-INDEX.md — the corpus ledger

Generated view of every exercise in the corpus: 276 total. Sources:
the exemplar batch in `principles/EXERCISES.md` §5 (26 exercises,
folded in unchanged) and the per-chapter files
`principles/exercises/chNN/EXERCISES.md` (+ `appx/` for the
appendix-adjacent sets). Tier meanings — `run (…)`: the solution
program is on disk and was executed by the named checker(s), output
pasted from the run; `prose`: the solution is discussion, no program;
`pending`: the stem and expected outcome exist, the feature does not —
see `EXERCISES-PENDING.md` for each blocker and owner.

Batches written and **not printed in their chapters**: ch13's 13-1,
13-6 and 13-8 — all three `par`'s, whose section §13.1 is held while
the rest of that chapter ships (`book/ch13.md`); ch17's 17-6, whose
section is held for want of fault injection (TOC.md §Deltas, bs07);
ch20's contracts nine (20-5 … 20-13, the old ch19 set renumbered at
the bs18 re-draw), which wait for the verifying compiler while their
chapter ships its printed four (the old ch20 bench-format eight are
retired, two re-homed as 19-3 and 19-5 —
EXERCISES-PENDING.md's re-draw note has the accounting); ch21's 21-4, 21-7 and 21-8, whose sections §21.3 and
§21.5 are held while the rest of that chapter ships
(`book/ch21.md`); ch23's 23-2, 23-3, 23-4 and 23-7, whose two sections
are held (`book/ch23.md`); ch25's eight, unprinted although §25.3
shipped at bs20, because every stem leans on a subject still held
(`book/ch25.md`; the masters' header has the accounting); ch07's 7-5,
whose static verdict does not exist on
either lane (`samples-pending.toml`); and ch05's 5-8, unprinted at bs11
because its whole subject is the combinator chain, and a stem cannot
carry a blocker note in a reader's text (TONE.md §Tense discipline).
All of them stay in the corpus so the day their surface lands the stems
and the sections arrive together. **231 of the 276 are printed**, and
`cargo xtask verify-docs` fails the build if a printed stem has no
published solution.

ch18's 18-3, 18-5 and 18-11 are no longer pending. At the bs09 pin the
comptime fold reaches a running program, so all three are green samples —
under the *compiler*, via the `wolf-run(…)` directive the runner grew for
programs only one implementation executes. They are still not printed in
chapter 18, which is bs08's chapter and not bs09's footprint; the three
stems are ready and the section that wants them needs one sentence added.
Recorded here so the editing pass does not have to rediscover it.

Part 5's first three chapters are in the totals below, and so is ch30,
which shipped with its chapter; ch29's batch lands with its chapter. Chapter 31 — the solo — is absent *by
design*: it publishes milestone checkpoints, not exercises (EXERCISES.md
§4). Chapter 32, the coda, carries no batch.

One reading note for chapters 26–28. Elsewhere the checker tag and the
tier agree; here they can differ, because a project's *chapter* and a
project's *solution program* are not always run by the same tool.
Chapter 26's `count` is compiled (the interpreter has no filesystem), so
several of its stems are tagged `· wolf` while the distilled solution
program on disk runs under lupin — the tier column says what CI
executed, and the exercise file says which is which and why.

Tier totals: 145 run (lupin) · 9 run (lupin REPL) · 27 run (wolf) · 12 run (wolf + lupin) · 73 prose · 10 pending. That is 276.
Four of the 27 `run (wolf)` are the `wolf-run(…)` lane the bs09 bump
added — 18-3, 18-5, 18-11 and 22-7, each holding a `comptime fn` that the
reference interpreter declines by design. 24-6 is the fifth addition and
is a package walkthrough rather than a file, so it has no `.lu`: its
transcript is the verbs' own output, replayed against a fixture. Which
lane executed a sample is bookkeeping; the claim is about the program.
Taxonomy spread (tags, hybrids counted once per kind): fingers 39 ·
comprehension 144 · extension 46 · spelunking 24 · design 33.

Three stems are printed under a section other than the one this index
assigns them, each deliberately: 13-2 (printed in §13.2 while §13.1 is
held), 18-6 (printed in §18.4), and 23-6 (printed in §23.1). Two more
are printed in another *chapter*: 4-3 and 4-4 end §7.4 and §7.5, because
the call-site `mut` they need is taught there and the numbers are stable
(EXERCISES.md §1). 24-8 is a chapter-batch stem printed inside §24.3,
ahead of 24-7; the batch is the chapter's last two either way.

## ch01 — 8 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §1.1 — exemplar batch (EXERCISES.md §5) | 1-1 | fingers · lupin | run (lupin) |
| §1.3 — exemplar batch (EXERCISES.md §5) | 1-2 | comprehension · lupin | run (lupin) |
| §1.1 — exemplar batch (EXERCISES.md §5) | 1-3 | fingers · lupin | run (lupin) |
| §1.4 — The REPL: a spec you can interrogate | 1-4 | fingers · lupin REPL | run (lupin REPL) |
| §1.3 — Scripts before projects | 1-5 | fingers + extension · lupin | run (lupin) |
| §1.5 — What `run` was doing for you | 1-6 | spelunking · lupin | run (lupin) |
| §1.5 — What `run` was doing for you | 1-8 | comprehension · wolf + lupin | run (wolf + lupin) |
| §1.2 — Two implementations, one language | 1-7 | fingers · wolf + lupin | run (wolf + lupin) |

## ch02 — 9 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §2.3 — exemplar batch (EXERCISES.md §5) | 2-1 | comprehension · lupin REPL | run (lupin REPL) |
| §2.1 — exemplar batch (EXERCISES.md §5) | 2-2 | fingers · lupin | run (lupin) |
| §2.3 — exemplar batch (EXERCISES.md §5) | 2-3 | comprehension · lupin | run (lupin) |
| §2.4 — exemplar batch (EXERCISES.md §5) | 2-4 | extension · lupin | run (lupin) |
| §2.2 — Multiline and raw | 2-5 | comprehension · lupin REPL | run (lupin REPL) |
| §2.3 — Bytes, honestly | 2-6 | comprehension · lupin REPL | run (lupin REPL) |
| §2.4 — Iterating meaning | 2-7 | extension · lupin | run (lupin) |
| §2.4 — Iterating meaning | 2-9 | comprehension · wolf | run (wolf) |
| §2.5 — What the machine does | 2-8 | comprehension · lupin REPL | run (lupin REPL) |

## ch03 — 8 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §3.2, §3.4 — exemplar batch (EXERCISES.md §5) | 3-1 | comprehension · lupin | run (lupin) |
| §3.1 — exemplar batch (EXERCISES.md §5) | 3-2 | comprehension · wolf + lupin | run (wolf + lupin) |
| §3.3 — exemplar batch (EXERCISES.md §5) | 3-3 | comprehension · lupin | run (lupin) |
| §3.3 — exemplar batch (EXERCISES.md §5) | 3-4 | comprehension · lupin | run (lupin) |
| §3.2 — exemplar batch (EXERCISES.md §5) | 3-5 | design | prose |
| §3.3 — Arithmetic that traps | 3-6 | extension (break-it-on-purpose) · lupin | run (lupin) |
| §3.3 — Arithmetic that traps | 3-7 | comprehension · lupin | run (lupin) |
| §3.1 — `let`, `var`, and handing values over | 3-8 | comprehension · lupin | run (lupin) |

## ch04 — 7 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §4.2 — exemplar batch (EXERCISES.md §5) | 4-1 | fingers · lupin | run (lupin) |
| §4.3 — exemplar batch (EXERCISES.md §5) | 4-2 | comprehension · lupin | run (lupin) |
| §4.4 — exemplar batch (EXERCISES.md §5) | 4-3 | extension · lupin | run (lupin) |
| §4.4 — exemplar batch (EXERCISES.md §5) | 4-4 | comprehension + spelunking · wolf | run (wolf) |
| §4.1 — Signatures are the contract | 4-5 | comprehension · lupin | run (lupin) |
| §4.3 — `defer` | 4-6 | comprehension · lupin | run (lupin) |
| Chapter batch | 4-7 | extension · lupin | run (lupin) |

## ch05 — 8 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §5.1 — exemplar batch (EXERCISES.md §5) | 5-1 | fingers · lupin | run (lupin) |
| §5.1 — exemplar batch (EXERCISES.md §5) | 5-2 | fingers · lupin | run (lupin) |
| §5.3 — exemplar batch (EXERCISES.md §5) | 5-3 | extension · lupin | run (lupin) |
| §5.4 — exemplar batch (EXERCISES.md §5) | 5-4 | comprehension · lupin | run (lupin) |
| §5.3 — exemplar batch (EXERCISES.md §5) | 5-5 | design | prose |
| §5.1 — `List`, `Map`, `Set`, tuples | 5-6 | extension · lupin | run (lupin) |
| Chapter batch | 5-7 | comprehension + extension · lupin | run (lupin) |
| §5.2 — The combinator style | 5-8 | comprehension · pending | pending |

## ch06 — 10 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §6.1 — exemplar batch (EXERCISES.md §5) | 6-1 | fingers · lupin | run (lupin) |
| §6.2 — exemplar batch (EXERCISES.md §5) | 6-2 | comprehension · lupin | run (lupin) |
| §6.1 — exemplar batch (EXERCISES.md §5) | 6-3 | extension · lupin | run (lupin) |
| §6.2 — exemplar batch (EXERCISES.md §5) | 6-4 | comprehension · lupin | run (lupin) |
| §6.3 — exemplar batch (EXERCISES.md §5) | 6-5 | comprehension · lupin | run (lupin) |
| §6.1 — `!T` and the row | 6-6 | comprehension · lupin | run (lupin) |
| §6.2 — `?`, `else`, `else |err|` | 6-7 | extension · lupin | run (lupin) |
| §6.4 — Hardening by refactor | 6-9 | extension · lupin | run (lupin) |
| §6.5 — Capstone: wordcount | 6-10 | extension · lupin | run (lupin) |
| Chapter batch | 6-8 | design | prose |

## ch07 — 13 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §7.1 — The error we owed you | 7-1 | comprehension · wolf + lupin | run (wolf + lupin) |
| §7.2 — Values are trees | 7-2 | fingers · lupin | run (lupin) |
| §7.2 — Values are trees | 7-3 | extension (break-it-on-purpose) · wolf + lupin | run (wolf + lupin) |
| §7.2 — Values are trees | 7-4 | comprehension · lupin | run (lupin) |
| §7.3 — Borrowing without the word | 7-5 | comprehension · lupin; static verdict · pending | pending |
| §7.4 — `mut` at both ends | 7-6 | fingers + spelunking · lupin | run (lupin) |
| §7.5 — Field-granular exclusivity | 7-7 | comprehension · wolf + lupin | run (wolf + lupin) |
| §7.5 — Field-granular exclusivity | 7-8 | comprehension + fingers · lupin | run (lupin) |
| §7.6 — Why there are no lifetimes | 7-9 | spelunking · wolf | run (wolf) |
| §7.6 — Why there are no lifetimes | 7-10 | design | prose |
| §7.7 — What the machine does | 7-11 | fingers · lupin REPL | run (lupin REPL) |
| Chapter batch | 7-12 | extension · lupin | run (lupin) |
| Chapter batch | 7-13 | comprehension + extension · lupin | run (lupin) |

## ch08 — 16 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §8.1 — You already think in regions | 8-1 | comprehension · prose | prose |
| §8.2 — The block form | 8-2 | fingers · lupin | run (lupin) |
| §8.2 — The block form | 8-3 | comprehension · wolf | run (wolf) |
| §8.3 — Regions are values | 8-4 | fingers · lupin REPL | run (lupin REPL) |
| §8.3 — Regions are values | 8-5 | comprehension · lupin | run (lupin) |
| §8.4 — Cycles are fine here | 8-6 | fingers · lupin | run (lupin) |
| §8.4 — Cycles are fine here | 8-7 | extension · pending | pending |
| §8.5 — Freeze | 8-8 | comprehension · wolf + lupin | run (wolf + lupin) |
| §8.5 — Freeze | 8-9 | comprehension + spelunking · wolf | run (wolf) |
| §8.5 — Freeze | 8-10 | comprehension · lupin | run (lupin) |
| §8.6 — Open, and open again | 8-11 | comprehension · lupin | run (lupin) |
| §8.7 — `shared` and `handle` | 8-12 | comprehension · lupin | run (lupin) |
| §8.7 — `shared` and `handle` | 8-13 | design | prose |
| §8.8 — What the machine does | 8-14 | spelunking · wolf | run (wolf) |
| Chapter batch | 8-15 | extension · lupin | run (lupin) |
| Chapter batch | 8-16 | extension · lupin | run (lupin) |

## ch09 — 14 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §9.1 — The three rings | 9-1 | fingers + spelunking | prose |
| §9.2 — Raw-tier rules | 9-2 | fingers · lupin | run (lupin) |
| §9.2 — Raw-tier rules | 9-3 | comprehension · lupin | run (lupin) |
| §9.2 — Raw-tier rules | 9-4 | comprehension · lupin | run (lupin) |
| §9.3 — The oracle you actually run | 9-5 | fingers + comprehension · lupin | run (lupin) |
| §9.3 — The oracle you actually run | 9-6 | comprehension · lupin | run (lupin) |
| §9.4 — The one door back | 9-7 | comprehension · lupin | run (lupin) |
| §9.5 — `#include`-grade C | 9-8 | fingers · wolf + lupin | run (wolf + lupin) |
| §9.6 — FFI and regions | 9-9 | comprehension · lupin | run (lupin) |
| §9.7 — Auditing: `#[trusted]` and the audit surface | 9-10 | spelunking · lupin | run (lupin) |
| §9.8 — The four-tier picture | 9-11 | comprehension · prose | prose |
| §9.8 — The four-tier picture | 9-12 | design | prose |
| Chapter batch | 9-13 | extension (break-it-on-purpose) · lupin | run (lupin) |
| Chapter batch | 9-14 | comprehension · lupin | run (lupin) |

## ch10 — 10 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §10.1 — The task tree | 10-1 | fingers · lupin | run (lupin) |
| §10.1 — The task tree | 10-2 | comprehension · lupin | run (lupin) |
| §10.1 — The task tree | 10-3 | comprehension · lupin | run (lupin) |
| §10.2 — The leaked goroutine, retired | 10-4 | extension (break-it-on-purpose) · lupin | run (lupin) |
| §10.2 — The leaked goroutine, retired | 10-5 | spelunking · lupin | run (lupin) |
| §10.3 — The dropped error, surfaced | 10-6 | comprehension · lupin | run (lupin) |
| §10.3 — The dropped error, surfaced | 10-7 | extension · lupin | run (lupin) |
| §10.4 — Cancellation | 10-8 | comprehension · lupin | run (lupin) |
| Chapter batch | 10-9 | extension · lupin | run (lupin) |
| Chapter batch | 10-10 | design | prose |

## ch11 — 8 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §11.1 — The scope as a capability | 11-1 | fingers · lupin | run (lupin) |
| §11.1 — The scope as a capability | 11-2 | comprehension · lupin | run (lupin) |
| §11.1 — The scope as a capability | 11-3 | comprehension · lupin | run (lupin) |
| §11.2 — The background refresher | 11-4 | extension · lupin | run (lupin) |
| §11.2 — The background refresher | 11-5 | comprehension + schedule play · lupin | run (lupin) |
| §11.3 — The structured dump | 11-6 | spelunking · lupin REPL | run (lupin REPL) |
| §11.3 — The structured dump | 11-7 | comprehension · lupin REPL | run (lupin REPL) |
| Chapter batch | 11-8 | design | prose |

## ch12 — 9 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §12.1 — Typed channels | 12-1 | fingers · lupin | run (lupin) |
| §12.1 — Typed channels | 12-2 | extension (break-it-on-purpose) · lupin | run (lupin) |
| §12.2 — `select` with timeouts | 12-3 | comprehension · lupin | run (lupin) |
| §12.2 — `select` with timeouts | 12-4 | comprehension + schedule play · lupin | run (lupin) |
| §12.2 — `select` with timeouts | 12-5 | spelunking · lupin | run (lupin) |
| §12.3 — When channels are the wrong queue | 12-6 | extension · lupin | run (lupin) |
| §12.3 — When channels are the wrong queue | 12-7 | design | prose |
| §12.4 — `when (a, b)` | 12-8 | comprehension · lupin | run (lupin) |
| §12.4 — `when (a, b)` | 12-9 | extension (break-it-on-purpose) · wolf + lupin | run (wolf + lupin) |

## ch13 — 8 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §13.1 — `par` (held) | 13-1 | comprehension · pending | pending |
| §13.1 — `par` (held) | 13-2 → printed in §13.2 | fingers · lupin | run (lupin) |
| §13.2 — The race that does not compile | 13-3 | comprehension · wolf + lupin | run (wolf + lupin) |
| §13.2 — The race that does not compile | 13-4 | spelunking · lupin | run (lupin) |
| Chapter batch | 13-5 | extension · lupin | run (lupin) |
| Chapter batch (held with §13.1) | 13-6 | extension · pending | pending |
| Chapter batch | 13-7 | comprehension · lupin | run (lupin) |
| Chapter batch (held with §13.1) | 13-8 | design | prose |

## ch14 — 9 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §14.1 — Armstrong's argument, one page | 14-1 | comprehension · lupin | run (lupin) |
| §14.1 — Armstrong's argument, one page | 14-2 | design | prose |
| §14.2 — Crash means bulk-free | 14-3 | comprehension · lupin | run (lupin) |
| §14.2 — Crash means bulk-free | 14-4 | comprehension · lupin | run (lupin) |
| §14.2 — Crash means bulk-free | 14-5 | comprehension · lupin | run (lupin) |
| §14.3 — Mailboxes | 14-6 | fingers · lupin | run (lupin) |
| §14.3 — Mailboxes | 14-7 | extension · lupin | run (lupin) |
| §14.3 — Mailboxes | 14-8 | design | prose |
| Chapter batch | 14-9 | comprehension + schedule play · lupin | run (lupin) |

## ch15 — 9 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §15.1 — Two primitives | 15-1 | comprehension · lupin | run (lupin) |
| §15.1 — Two primitives | 15-2 | comprehension · lupin | run (lupin) |
| §15.1 — Two primitives | 15-3 | design | prose |
| §15.2 — A supervisor in forty lines | 15-4 | fingers · lupin | run (lupin) |
| §15.2 — A supervisor in forty lines | 15-5 | comprehension · lupin | run (lupin) |
| §15.2 — A supervisor in forty lines | 15-6 | extension · lupin | run (lupin) |
| §15.3 — The root supervisor | 15-7 | design | prose |
| §15.3 — The root supervisor | 15-8 | spelunking · corpus | prose |
| Chapter batch | 15-9 | design | prose |

## ch16 — 9 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §16.1 — `ch.send(move r)` | 16-1 | comprehension · lupin | run (lupin) |
| §16.1 — `ch.send(move r)` | 16-2 | fingers · lupin | run (lupin) |
| §16.1 — `ch.send(move r)` | 16-3 | extension (break-it-on-purpose) · lupin | run (lupin) |
| §16.2 — Freeze, then share | 16-4 | comprehension · lupin | run (lupin) |
| §16.2 — Freeze, then share | 16-5 | design | prose |
| §16.3 — The honest lineup | 16-6 | design | prose |
| Chapter batch | 16-7 | extension · lupin | run (lupin) |
| Chapter batch | 16-8 | comprehension + schedule play · lupin | run (lupin) |
| Chapter batch | 16-9 | comprehension · wolf + lupin | run (wolf + lupin) |

## ch17 — 9 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §17.1 — The bug that typechecks | 17-1 | comprehension · lupin | run (lupin) |
| §17.1 — The bug that typechecks | 17-2 | comprehension (schedule play) · lupin | run (lupin) |
| §17.2 — The seed, the schedule, and the frontier | 17-3 | fingers · lupin | run (lupin) |
| §17.2 — The seed, the schedule, and the frontier | 17-4 | comprehension · lupin | run (lupin) |
| §17.2 — The seed, the schedule, and the frontier | 17-5 | spelunking · lupin | run (lupin) |
| §17.3 (held, not printed) — `--chaos` | 17-6 | comprehension · pending | pending |
| §17.3 — Scope honesty (what exploration cannot see) | 17-7 | comprehension · lupin | run (lupin) |
| §17.3 — Scope honesty (what exploration cannot see) | 17-8 | design | prose |
| Chapter batch | 17-9 | extension (break-it-on-purpose) · lupin | run (lupin) |

## ch18 — 12 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §18.1 — Wolf at compile time | 18-1 | comprehension · wolf | run (wolf) |
| §18.1 — Wolf at compile time | 18-2 | comprehension · wolf | run (wolf) |
| §18.1 — Wolf at compile time (not printed; see the note at the head) | 18-3 | fingers · wolf | run (wolf) |
| §18.2 — Types as values | 18-4 | comprehension · wolf | run (wolf) |
| §18.2 — Types as values (not printed; see the note at the head) | 18-5 | extension · wolf | run (wolf) |
| §18.4 — What it refuses to do (printed there, not §18.3) | 18-6 | spelunking · wolf | run (wolf) |
| §18.4 — What it refuses to do | 18-7 | comprehension · wolf | run (wolf) |
| §18.4 — What it refuses to do | 18-8 | comprehension · wolf | run (wolf) |
| §18.4 — What it refuses to do | 18-9 | comprehension · wolf | run (wolf) |
| §18.4 — What it refuses to do | 18-10 | extension (break-it-on-purpose) · wolf | run (wolf) |
| Chapter batch (not printed; see the note at the head) | 18-11 | extension · wolf | run (wolf) |
| Chapter batch | 18-12 | design | prose |

## ch19 — 5 exercises

*(the bs18 set; the chapter is Reading the release tier since the
re-draw. 19-1's solution transcript also builds and runs the release
binary; the tier names the directive's executor.)*

| section | exercise | type · checker | tier |
|---|---|---|---|
| §19.1 — One program, two binaries | 19-1 | fingers · wolf + lupin | run (lupin) |
| §19.2 — The compiler hands LLVM less | 19-2 | comprehension · prose | prose |
| §19.3 — Reading a loss | 19-3 | comprehension · prose | prose |
| §19.3 — Reading a loss | 19-4 | comprehension · prose | prose |
| Chapter batch | 19-5 | design | prose |

## ch20 — 13 exercises

*(the bs18 printed four, then the contracts corpus — the old ch19
nine, renumbered +4 with their subject at the re-draw — unprinted
until the verifying compiler lands.)*

| section | exercise | type · checker | tier |
|---|---|---|---|
| §20.1 — The promises the language keeps | 20-1 | fingers · lupin | run (lupin) |
| §20.2 — The ledger and the clock | 20-2 | comprehension · prose | prose |
| §20.3 — The floors and the ratchets | 20-3 | comprehension · prose | prose |
| §20.4 — Exceptions, written and capped | 20-4 | design | prose |
| Contracts corpus (unprinted) | 20-5 | comprehension · pending | pending |
| Contracts corpus (unprinted) | 20-6 | comprehension · prose | prose |
| Contracts corpus (unprinted) | 20-7 | comprehension · prose | prose |
| Contracts corpus (unprinted) | 20-8 | fingers · lupin | run (lupin) |
| Contracts corpus (unprinted) | 20-9 | comprehension · prose | prose |
| Contracts corpus (unprinted) | 20-10 | spelunking · wolf | run (wolf) |
| Contracts corpus (unprinted) | 20-11 | comprehension · prose | prose |
| Contracts corpus (unprinted) | 20-12 | design | prose |
| Contracts corpus (unprinted) | 20-13 | extension · lupin | run (lupin) |

## ch21 — 9 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §21.1 — Aliasing | 21-1 | comprehension · prose | prose |
| §21.1 — Aliasing | 21-2 | fingers · lupin | run (lupin) |
| §21.2 — Arenas | 21-3 | comprehension · prose | prose |
| §21.3 — Layout | 21-4 | comprehension · prose | prose |
| §21.4 — Checked arithmetic's bill | 21-5 | comprehension · lupin | run (lupin) |
| §21.4 — Checked arithmetic's bill | 21-6 | spelunking · lupin | run (lupin) |
| §21.5 — Where C wins today | 21-7 | design | prose |
| Chapter batch | 21-8 | comprehension · pending | pending |
| Chapter batch | 21-9 | comprehension · prose | prose |

## ch22 — 8 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §22.1 — Directory = module | 22-1 | fingers · lupin | run (lupin) |
| §22.1 — Directory = module | 22-2 | comprehension · lupin | run (lupin) |
| §22.1 — Directory = module | 22-3 | comprehension · lupin | run (lupin) |
| §22.1 — Directory = module | 22-4 | comprehension · lupin | run (lupin) |
| §22.2 — No cycles | 22-5 | comprehension + extension · lupin | run (lupin) |
| §22.2 — No cycles | 22-6 | comprehension · prose | prose |
| §22.3 — No life before main | 22-7 | comprehension · wolf | run (wolf) |
| Chapter batch | 22-8 | design | prose |

## ch23 — 8 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §23.1 — `wolf.pkg` is data | 23-1 | comprehension · prose | prose |
| §23.2 (held, not printed) — MVS in one page | 23-2 | comprehension · prose | prose |
| §23.2 (held, not printed) — MVS in one page | 23-3 | comprehension · prose | prose |
| §23.2 (held, not printed) — MVS in one page | 23-4 | comprehension · prose | prose |
| §23.3 — `wolf.sum` — the ledger | 23-5 | comprehension · prose | prose |
| §23.1 — `wolf.pkg` is data (printed there, not §23.3) | 23-6 | spelunking · wolf | run (wolf) |
| §23.4 (held, not printed) — Script mode, demystified | 23-7 | comprehension · pending | pending |
| Chapter batch | 23-8 | design | prose |

## ch24 — 8 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §24.1 — The threat, from history | 24-1 | spelunking · prose | prose |
| §24.1 — The threat, from history | 24-2 | comprehension · prose | prose |
| §24.1 — The threat, from history | 24-3 | comprehension · prose | prose |
| §24.2 — What replaces scripts | 24-4 | comprehension · wolf | run (wolf) |
| §24.2 — What replaces scripts | 24-5 | comprehension · wolf | run (wolf) |
| §24.3 — Capabilities and `wolf audit` | 24-6 | comprehension · wolf | run (wolf) |
| §24.4 — What the covenant costs | 24-7 | design | prose |
| Chapter batch | 24-8 | comprehension · prose | prose |

## ch25 — 8 exercises

*§25.3 shipped at bs20; none of these eight is printed — each leans on
a held subject (the masters' header has the accounting).*

| section | exercise | type · checker | tier |
|---|---|---|---|
| §25.1 — Editions per package | 25-1 | comprehension · prose | prose |
| §25.1 — Editions per package | 25-2 | comprehension · prose | prose |
| §25.2 — The stdlib posture | 25-3 | comprehension · prose | prose |
| §25.2 — The stdlib posture | 25-4 | comprehension · prose | prose |
| §25.2 — The stdlib posture | 25-5 | comprehension · prose | prose |
| §25.3 — Publishing | 25-6 | design | prose |
| §25.3 — Publishing | 25-7 | comprehension · prose | prose |
| Chapter batch | 25-8 | design | prose |

## ch26 — 6 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| §26.3 — The same machine as a `match` | 26-1 | fingers · wolf | run (lupin) |
| §26.3 — The same machine as a `match` | 26-2 | comprehension · lupin | run (lupin) |
| §26.4 — Per file, and a total | 26-3 | extension · wolf | run (lupin) |
| §26.4 — Per file, and a total | 26-4 | comprehension · wolf | run (wolf) |
| §26.5 — Where wolf is not shorter | 26-5 | spelunking · wolf | prose |
| §26.5 — Where wolf is not shorter | 26-6 | design | prose |

## ch27 — 8 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| Chapter batch | 27-1 | fingers · lupin | run (lupin) |
| Chapter batch | 27-2 | comprehension · lupin | run (lupin) |
| Chapter batch | 27-3 | extension · lupin | run (lupin) |
| Chapter batch | 27-4 | comprehension · lupin | run (lupin) |
| Chapter batch | 27-5 | spelunking · the C twin | prose |
| Chapter batch | 27-6 | extension · lupin | run (lupin) |
| Chapter batch | 27-7 | design | prose |
| Chapter batch | 27-8 | design | prose |

## ch28 — 7 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| Chapter batch | 28-1 | fingers · lupin | run (lupin) |
| Chapter batch | 28-2 | comprehension · lupin | run (lupin) |
| Chapter batch | 28-3 | extension · lupin | run (lupin) |
| Chapter batch | 28-4 | comprehension · lupin | run (lupin) |
| Chapter batch | 28-5 | spelunking · the C twin | prose |
| Chapter batch | 28-6 | extension · lupin | run (lupin) |
| Chapter batch | 28-7 | design | prose |

## ch30 — 7 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| Chapter batch | 30-1 | fingers · wolf | prose |
| Chapter batch | 30-2 | comprehension · wolf | prose |
| Chapter batch | 30-3 | comprehension · wolf | run (wolf) |
| Chapter batch | 30-4 | extension · wolf | run (wolf) |
| Chapter batch | 30-5 | extension · wolf | run (wolf) |
| Chapter batch | 30-6 | spelunking · wolf | prose |
| Chapter batch | 30-7 | design | prose |

Chapter 31 contributes no row: the solo publishes six milestone
checkpoints instead of exercises, and it is the solutions policy's one
exception (EXERCISES.md §4). Chapter 29 contributes none either — its
chapter is held.

30-1's and 30-2's tier is `prose` for two different reasons. 30-1 needs no
program of its own: the two it measures are `samples/projects/seqgrep/`
and `samples/projects/pargrep/`, both printed in the chapter and both
checked. 30-2's program is deliberately flaky — a buffered `hits` channel
loses hits, which is the exercise — and a sample whose output varies
between runs cannot be a CI sample, so the solution page carries the
twenty-run measurement instead. Both reasons are stated on the solution
pages rather than only here.

## appx — 14 exercises

| section | exercise | type · checker | tier |
|---|---|---|---|
| Appendix B — the trap zoo | B-1 | comprehension · lupin | run (lupin) |
| Appendix B — the trap zoo | B-2 | comprehension · lupin | run (lupin) |
| Appendix B — the trap zoo | B-3 | comprehension · lupin | run (lupin) |
| Appendix B — the trap zoo | B-4 | comprehension · lupin | run (lupin) |
| Appendix B — the trap zoo | B-5 | comprehension · lupin | run (lupin) |
| Appendix B — the trap zoo | B-6 | comprehension · lupin | run (lupin) |
| Appendix B — the trap zoo | B-7 | comprehension · lupin | run (lupin) |
| Appendix B — the trap zoo | B-8 | comprehension · lupin | run (lupin) |
| Appendix B — the trap zoo | B-9 | comprehension · lupin | run (lupin) |
| Appendix B — the trap zoo | B-10 | comprehension · pending | pending |
| Appendix B — the trap zoo | B-11 | comprehension · lupin | run (lupin) |
| Appendix C — the diagnostic catalog | C-1 | spelunking · wolf | run (wolf) |
| Appendix C — the diagnostic catalog | C-2 | comprehension · wolf + lupin | run (wolf + lupin) |
| Appendix C — the diagnostic catalog | C-3 | comprehension · prose | prose |
