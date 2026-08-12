# EXERCISES-PENDING.md — the pending manifest

The ledger model, applied to homework: every exercise below has its
full stem and its solution prose in its chapter file, and — wherever
the solution is a program — a directive-headed `.lu` on disk stating
the expected outcome. None of them is green today, and none is claimed
to be. The day a blocking feature lands, its exercises join the CI run
their headers already describe; until then this manifest is the honest
list. No aspirational green.

Note for the compiler track: the whole corpus — 188 directive-headed
`.lu` files under `principles/exercises/`, runnable and pending alike —
is a seed farm for the toolchain's corpus and fuzz harnesses. The
pending files are the most valuable seeds: each one encodes an expected
verdict for a feature that does not exist yet.

## Entries

| exercise | expected (directive) | blocker | owner |
|---|---|---|---|
| 7-5 (static half) | `fail(E1003)` | borrow-escape checking reaches no static verdict; wolfc leaves `channel` unresolved | s33-channels-select, then s18-tier0-exclusivity |
| 8-7 | `run(exit=0, stdout="c a")` | field writes through a pool index (`pool[h].next = k`) do not denote a place in the interp std subset | s37-core-types (std surface pinning) |
| 13-1 | `run(exit=0, stdout="285")` | `par` absent from the interp std subset | s32-tasks-scheduler / s37-core-types |
| 13-6 | `run(exit=0)` | `par` absent from the interp std subset | s32-tasks-scheduler / s37-core-types |
| 5-8 | `run(exit=0, stdout="marmot 5")` | `sorted_by` / `take` absent from the interp std subset | s37-core-types (std surface pinning) |
| 17-6 | `run(exit=0)` | `--chaos` fault injection at declared effect points — and with it §17.3 of chapter 17, which is why this stem is written and **not printed** (TOC.md §Deltas, bs07) | s36-deterministic-scheduler |
| 19-1 | `run(exit=0, stdout="3")` | perf-contract verification (I15) | s24–s26 WIR fact sprints |
| 20-6 | none — workflow exercise | `wolf bench` harness and `--baseline` workflow | s01-test-and-bench-infrastructure / s44-perf-validation |
| 21-8 | none — measurement exercise | bench rigs and CI perf gates | s44-perf-validation |
| 23-7 | none — workflow exercise | script-mode dependency resolution | s51-package-manager with s31-driver-v0 |
| B-10 | none — the absence is the exercise | `race` needs the dynamic race machine observing a real conflict; `alloc-contract` needs the wolf_rt quarantine allocator | s36-deterministic-scheduler; s23-memory-conformance / wolf_rt |

## What today's tools say

Recorded at authoring time (lupin 0.1.0, wolfc 0.0.1 debug), so the
flip is detectable: 5-8, 8-7, 13-1, 13-6 exit 4 `unsupported` with the
reason on stderr. 17-6: the flag is rejected. 19-1: `#[noalloc]` parses
and is not verified. 7-5: lupin reaches its dynamic verdict; wolfc
reports `unsupported` at resolve. The workflow entries (20-6, 21-8,
23-7) have no tool to run at all. Exercises whose stems
show a worked-example transcript (the ch20 bench-format set) label it
as a worked example in the stem itself; nothing in the corpus presents
invented output as a run.

## What the rp02 pin bump did not move

Recorded because a bump is only honest if the rows that stayed are
checked rather than assumed. At wolf-lang's newest green trunk sha and
lupin v0.1.9 the runner reports five pendings and **zero flips**, and
each of the workflow rows was probed by hand: `par` is still absent from
both implementations (`nothing named 'par' is in scope`), so 13-1 and
13-6 hold and chapter 13 §13.1 stays vacant; `--chaos` is still
`unexpected argument` under lupin, so 17-6 stays unprinted and chapter 17
stays at three sections; `wolf bench` still answers `not yet`, so 19-1,
20-6 and 21-8 hold with chapters 19–21; and the ch05, ch07 and ch08 rows
are unmoved for the reasons their blockers name. Native concurrency and
the compiler's mid-end are what this bump brought, and neither of them is
what any of these rows is waiting on.

## Retired entries

A row leaves this table only when its feature lands and the runner
reports the FLIP. The removals so far, kept here so the ledger reads as
a history rather than a snapshot:

- **18-3**, **18-5**, **18-11** (`run(exit=0, …)`) — retired at the bs09
  pin bump, all three as reported FLIPs. The comptime fold now reaches a
  running program: the compiler publishes its fold table to the lanes
  that execute, and `wolf run` prints the folded value for each of the
  three. lupin still declines `comptime fn` by design, so the three
  directives moved from `run(…)` to a new one, `wolf-run(…)`, which the
  book's runner executes under the compiler — the honest spelling for a
  program only one implementation runs. Chapter 18's ledger predicted
  exactly this fix; chapter 22 §22.3 is the section the same flip made
  writable.
- **22-7** (sketch) — retired at the bs09 pin bump. The comptime registry
  is a real program now: four handler types, a table folded into a
  `const`, a witness that fails the build when the table is the wrong
  length. It is a `wolf-run(…)` sample for the same reason the ch18 three
  are.
- **24-6** (workflow) — retired at the bs09 pin bump. Capability
  manifests and `wolf audit` ship: the exercise is a walkthrough with a
  real transcript (`wolf update`, then `wolf audit --ci` reporting
  `ACQUIRES capability `net`` and exiting 1) instead of a worked example
  in a pinned format. It carried no manifest row because it had no
  program; it still has no `.lu`, because the subject is a project rather
  than a file.
- **13-3** (`fail(E1101)`) and **16-9** (`fail(E1102)`) — retired at the
  bs10 pin bump. Unsynchronized mutable capture across tasks and
  unsendable channel payloads are both static rejections in the compiler
  at this pin, with the diagnostics the two stems predicted: E1101 names
  all three fixes (channel, `Mutex` in a `when`, `par` with a
  reduction), and E1102 names all four admitted payload classes. Both
  exercises are now ordinary green samples with reviewed snapshots, and
  their solution transcripts in `principles/exercises/ch13/` and
  `principles/exercises/ch16/` show the real rejections instead of the
  honest-today runs they used to carry.
  At the rp01 bump the interpreter caught up: E1101, E1102 and E1103 are
  static rejections in *both* tools now, same codes, same spans. 13-3's
  second half was rewritten rather than re-blessed, because its subject
  was the differential — lupin used to run the racy program to exit 0
  with both increments lost — and the differential is gone. Its stem now
  asks about the one thing that still differs, which is how much output
  each tool gives for two offending spawns.
